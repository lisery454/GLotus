use std::{cell::RefCell, rc::Rc};

use glfw::Key;

use crate::{
    input::input_state::InputState,
    render::camera::{Camera, CameraMovement},
};

use super::ITickable;

pub struct CameraTickable {
    camera: Rc<RefCell<Camera>>,
}
impl CameraTickable {
    pub fn new(camera: Rc<RefCell<Camera>>) -> Self {
        Self { camera }
    }
}
impl ITickable for CameraTickable {
    fn tick(&mut self, delta_time: f32, input_state: Rc<RefCell<InputState>>) {
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
            self.camera
                .borrow_mut()
                .process_move(movement, velocity, delta_time);
        }

        let scroll_y_offset = input_state.borrow().get_scroll_delta().y;
        if scroll_y_offset != 0.0 {
            self.camera
                .borrow_mut()
                .process_zoom(scroll_y_offset as f32, 0.5);
        }

        let cursor_x_offset = input_state.borrow().get_cursor_delta().x;
        let cursor_y_offset = input_state.borrow().get_cursor_delta().y;

        self.camera.borrow_mut().process_turn(
            cursor_x_offset as f32,
            cursor_y_offset as f32,
            0.015,
            true,
        );
    }
}
