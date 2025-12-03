use cgmath::{Deg, InnerSpace, Quaternion, Rad, Rotation3, Vector3};

use crate::render::transform::Rotation;

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
            .get_position_mut()
            .translate(delta_position);
    }

    pub fn process_turn(
        &mut self,
        xoffset: f32,
        yoffset: f32,
        sensitivity: f32,
        constrain_pitch: bool,
    ) {
        // 将偏移量转换为弧度
        let yaw = Rad(-xoffset * sensitivity);
        let pitch = Rad(-yoffset * sensitivity);

        // 创建绕Y轴(偏航)和X轴(俯仰)的旋转四元数
        let yaw_rotation = Quaternion::from_axis_angle(Vector3::unit_y(), yaw);
        let pitch_rotation = Quaternion::from_axis_angle(Vector3::unit_x(), pitch);

        let mut new_rotation =
            Quaternion::<f32>::from(self.get_transform().get_rotation().get_data());

        new_rotation = yaw_rotation * new_rotation * pitch_rotation;

        // 从新旋转中提取前向向量
        let forward = new_rotation * Vector3::unit_z();

        // 计算当前俯仰角(与水平面的夹角)
        let pitch_angle = forward.y.asin(); // 返回弧度值

        if constrain_pitch {
            // 限制俯仰角度在±89度内
            if pitch_angle.abs() < 89.0f32.to_radians() {
                self.transform
                    .set_rotation(Rotation::from(new_rotation.normalize()));
            }
        }
    }

    pub fn process_zoom(&mut self, yoffset: f32, sensitivity: f32) {
        // 计算新的FOV值
        let mut new_fov = self.fov.0 - yoffset * sensitivity;

        // 限制FOV范围（通常在1.0到120度之间）
        new_fov = new_fov.clamp(1.0, 120.0);

        self.fov = Deg(new_fov);
    }
}
