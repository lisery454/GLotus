use crate::{AppContext, AppEvent, Camera, ISystem, Rotation, Transform, Translation};
use cgmath::{Deg, InnerSpace, Quaternion, Rad, Rotation3, Vector2, Vector3};
use glfw::Key;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct CameraSystem;

impl ISystem for CameraSystem {
    fn name(&self) -> &str {
        "CameraSystem"
    }

    fn update(&mut self, app_context: Rc<RefCell<AppContext>>, _delta_dt: f32) {
        let config_wh = {
            let context = app_context.borrow();
            let config = context.app_config.borrow();
            (config.width, config.height)
        };
        // 根据resize事件缩放相机比例
        let resize_data = {
            let app = app_context.borrow();
            let event_queue = app.event_queue.borrow();
            event_queue.all_events().iter().rev().find_map(|event| {
                if let AppEvent::Resize { width, height } = event {
                    Some((*width, *height))
                } else {
                    None
                }
            })
        };

        {
            let app = app_context.borrow();
            let world = app.world.borrow();
            let mut camera_mgr = world.get_manager_mut::<Camera>();

            if let Some((_entity, main_cam)) = camera_mgr.find_mut(|cam| cam.is_active) {
                if let Some((w, h)) = resize_data {
                    main_cam.set_aspect_ratio(w as u32, h as u32);
                    return;
                }

                if main_cam.is_initialized == false {
                    main_cam.set_aspect_ratio(config_wh.0, config_wh.1);
                    main_cam.is_initialized = true;
                    return;
                }
            }
        }
    }

    fn fixed_update(&mut self, app_context: Rc<RefCell<AppContext>>, delta_dt: f32) {
        // 1. 先提取所有需要的输入数据，然后立即释放 input_state 的锁
        let (movement, scroll_y, cursor_delta) = {
            let app = app_context.borrow(); // app 是 Ref<AppContext>
            let input = app.input_state.borrow(); // input 是 Ref<InputState>

            let move_dir = if input.is_key_down(Key::W) {
                Some(CameraMovement::Forward)
            } else if input.is_key_down(Key::A) {
                Some(CameraMovement::Left)
            } else if input.is_key_down(Key::S) {
                Some(CameraMovement::Backward)
            } else if input.is_key_down(Key::D) {
                Some(CameraMovement::Right)
            } else if input.is_key_down(Key::LeftShift) {
                Some(CameraMovement::Down)
            } else if input.is_key_down(Key::Space) {
                Some(CameraMovement::Up)
            } else {
                None
            };

            let scroll_y = input.get_scroll_delta().y;
            let cursor_delta = Vector2::new(input.get_cursor_delta().x, input.get_cursor_delta().y);

            (move_dir, scroll_y, cursor_delta)
        };

        {
            let app = app_context.borrow();
            let world = app.world.borrow_mut();

            let mut camera_mgr = world.get_manager_mut::<Camera>();
            let mut transform_mgr = world.get_manager_mut::<Transform>();

            if let Some((entity, main_cam)) = camera_mgr.find_mut(|cam| cam.is_active) {
                if let Some(transform) = transform_mgr.get_mut(entity) {
                    if let Some(m) = movement {
                        process_move(transform, m, 10.0, delta_dt);
                    }
                    if scroll_y != 0.0 {
                        process_zoom(main_cam, scroll_y, 0.5);
                    }
                    // 使用解构出来的浮点数值
                    process_turn(transform, cursor_delta.x, cursor_delta.y, 0.005, true);
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CameraMovement {
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
}

fn process_move(
    transform: &mut Transform,
    movement: CameraMovement,
    velocity: f32,
    delta_time: f32,
) {
    let delta_position = match movement {
        CameraMovement::Forward => transform.get_forward(),
        CameraMovement::Backward => transform.get_forward() * -1f32,
        CameraMovement::Right => transform.get_right(),
        CameraMovement::Left => transform.get_right() * -1f32,
        CameraMovement::Up => transform.get_up(),
        CameraMovement::Down => transform.get_up() * -1f32,
    } * velocity
        * delta_time;

    transform
        .get_translation_mut()
        .translate(Translation::from_vec(delta_position));
}

fn process_turn(
    transform: &mut Transform,
    xoffset: f32,
    yoffset: f32,
    sensitivity: f32,
    constrain_pitch: bool,
) {
    let yaw_delta = Rad(-xoffset * sensitivity);
    let pitch_delta = Rad(-yoffset * sensitivity);

    let current_rotation = Quaternion::<f32>::from(transform.get_rotation().get_data());

    // 1. 先应用偏航（绕世界Y轴）
    let yaw_rotation = Quaternion::from_axis_angle(Vector3::unit_y(), yaw_delta);
    let new_rotation = yaw_rotation * current_rotation;

    // 2. 计算当前的右向量（用于俯仰旋转）
    let right = new_rotation * Vector3::unit_x();

    // 3. 应用俯仰（绕局部X轴/右向量）
    let pitch_rotation = Quaternion::from_axis_angle(right, pitch_delta);
    let mut final_rotation = pitch_rotation * new_rotation;

    // 4. 检查俯仰角限制
    if constrain_pitch {
        let forward = final_rotation * Vector3::unit_z();
        let pitch_angle = forward.y.asin();

        const MAX_PITCH: f32 = 89.0;
        let max_pitch_rad = MAX_PITCH.to_radians();

        if pitch_angle.abs() > max_pitch_rad {
            // 只限制俯仰，保留偏航
            // 重新计算限制后的俯仰角
            let clamped_pitch = pitch_angle.signum() * max_pitch_rad;

            // 从偏航旋转开始，应用限制后的俯仰
            let clamped_pitch_rotation = Quaternion::from_axis_angle(
                right,
                Rad(clamped_pitch - (new_rotation * Vector3::unit_z()).y.asin()),
            );
            final_rotation = clamped_pitch_rotation * new_rotation;
        }
    }

    transform.set_rotation(Rotation::from(final_rotation.normalize()));
}

fn process_zoom(camera: &mut Camera, yoffset: f32, sensitivity: f32) {
    // 计算新的FOV值
    let mut new_fov = camera.fov.0 - yoffset * sensitivity;

    // 限制FOV范围（通常在1.0到120度之间）
    new_fov = new_fov.clamp(1.0, 120.0);

    camera.fov = Deg(new_fov);
}
