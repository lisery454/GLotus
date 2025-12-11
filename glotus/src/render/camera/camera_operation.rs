use cgmath::{Deg, InnerSpace, Quaternion, Rad, Rotation3, Vector3};

use crate::{Translation, render::transform::Rotation};

use super::camera::Camera;

pub enum CameraMovement {
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
}

impl Camera {
    pub fn process_move(&mut self, movement: CameraMovement, velocity: f32, delta_time: f32) {
        let delta_position = match movement {
            CameraMovement::Forward => self.get_forward(),
            CameraMovement::Backward => self.get_forward() * -1f32,
            CameraMovement::Right => self.get_right(),
            CameraMovement::Left => self.get_right() * -1f32,
            CameraMovement::Up => self.get_up(),
            CameraMovement::Down => self.get_up() * -1f32,
        } * velocity
            * delta_time;

        self.get_transform_mut()
            .get_translation_mut()
            .translate(Translation::from_vec(delta_position));
    }

    pub fn process_turn(
        &mut self,
        xoffset: f32,
        yoffset: f32,
        sensitivity: f32,
        constrain_pitch: bool,
    ) {
        let yaw_delta = Rad(-xoffset * sensitivity);
        let pitch_delta = Rad(-yoffset * sensitivity);

        let current_rotation =
            Quaternion::<f32>::from(self.get_transform().get_rotation().get_data());

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

        self.transform
            .set_rotation(Rotation::from(final_rotation.normalize()));
    }

    pub fn process_zoom(&mut self, yoffset: f32, sensitivity: f32) {
        // 计算新的FOV值
        let mut new_fov = self.fov.0 - yoffset * sensitivity;

        // 限制FOV范围（通常在1.0到120度之间）
        new_fov = new_fov.clamp(1.0, 120.0);

        self.fov = Deg(new_fov);
    }
}
