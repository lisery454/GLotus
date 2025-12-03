use std::{cell::RefCell, rc::Rc};

use cgmath::{Deg, InnerSpace, Matrix4, Ortho, PerspectiveFov, Rad, Vector2, Vector3};
use glfw::Key;
use log::{info, log};

use crate::{
    core::FixedUpdateAble, input::input_state::InputState, render::camera::CameraMovement,
    render::transform::Transform,
};

use super::projection_type::ProjectionType;

pub struct Camera {
    pub(crate) transform: Transform,
    pub(crate) fov: Deg<f32>,
    pub(crate) aspect_ratio: f32,
    pub(crate) near_plane: f32,
    pub(crate) far_plane: f32,
    pub(crate) projection_type: ProjectionType,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            transform: Transform::default(),
            fov: Deg(45.0),
            aspect_ratio: 16.0 / 9.0,
            near_plane: 0.1,
            far_plane: 100.0,
            projection_type: ProjectionType::Perspective,
        }
    }

    pub fn get_transform(&self) -> &Transform {
        &self.transform
    }

    pub fn get_transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    pub fn set_transform(&mut self, transform: Transform) {
        self.transform = transform
    }

    pub fn set_aspect_ratio(&mut self, width: u32, height: u32) {
        self.aspect_ratio = width as f32 / height as f32;
    }

    pub fn get_aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }

    pub(crate) fn get_view_matrix(&self) -> [[f32; 4]; 4] {
        Matrix4::look_to_rh(
            self.transform.get_position().get_data(),
            self.get_forward(),
            self.get_up(),
        )
        .into()
    }

    pub(crate) fn get_view_position(&self) -> [f32; 3] {
        self.get_transform().get_position().get_arr()
    }

    pub(crate) fn get_projection_matrix(&self) -> [[f32; 4]; 4] {
        let matrix: Matrix4<f32> = match self.projection_type {
            ProjectionType::Perspective => PerspectiveFov {
                fovy: Rad::from(self.fov),
                aspect: self.aspect_ratio,
                near: self.near_plane,
                far: self.far_plane,
            }
            .into(),
            ProjectionType::Orthographic => {
                let half_height = self.fov.0 / 2.0;
                let half_width = half_height * self.aspect_ratio;
                Ortho {
                    left: -half_width,
                    right: half_width,
                    bottom: -half_height,
                    top: half_height,
                    near: self.near_plane,
                    far: self.far_plane,
                }
                .into()
            }
        };

        matrix.into()
    }

    pub fn get_forward(&self) -> Vector3<f32> {
        self.transform.get_rotation().get_data() * -Vector3::unit_z()
    }

    pub fn get_right(&self) -> Vector3<f32> {
        self.transform.get_rotation().get_data() * Vector3::unit_x()
    }

    pub fn get_up(&self) -> Vector3<f32> {
        self.transform.get_rotation().get_data() * Vector3::unit_y()
    }
}

impl FixedUpdateAble for Camera {
    fn fixed_update(&mut self, delta_time: f32, input_state: Rc<RefCell<InputState>>) {
        let velocity = 10.0;
        let movement = if input_state.borrow().is_key_down(Key::W) {
            Some(CameraMovement::Forward)
        } else if input_state.borrow().is_key_down(Key::A) {
            Some(CameraMovement::Left)
        } else if input_state.borrow().is_key_down(Key::S) {
            Some(CameraMovement::Backward)
        } else if input_state.borrow().is_key_down(Key::D) {
            Some(CameraMovement::Right)
        } else if input_state.borrow().is_key_down(Key::LeftShift) {
            Some(CameraMovement::Down)
        } else if input_state.borrow().is_key_down(Key::Space) {
            Some(CameraMovement::Up)
        } else {
            None
        };
        if let Some(movement) = movement {
            self.process_move(movement, velocity, delta_time);
        }

        let scroll_y_offset = input_state.borrow().get_scroll_delta().y;
        if scroll_y_offset != 0.0 {
            self.process_zoom(scroll_y_offset as f32, 0.5);
        }

        let cursor_x_offset = input_state.borrow().get_cursor_delta().x;
        let cursor_y_offset = input_state.borrow().get_cursor_delta().y;

        self.process_turn(cursor_x_offset as f32, cursor_y_offset as f32, 0.015, true);
    }
}
