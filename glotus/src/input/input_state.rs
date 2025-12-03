use std::collections::HashSet;

use cgmath::Vector2;
use glfw::{Key, MouseButton};
use log::debug;

pub struct InputState {
    pressed_keys: HashSet<Key>,

    is_first_cursor_move: bool,
    cursor_pos: Vector2<f64>,
    cursor_delta: Vector2<f64>,

    scroll_delta: Vector2<f64>,

    pressed_mouse_buttons: HashSet<MouseButton>,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            pressed_keys: HashSet::new(),
            is_first_cursor_move: true,
            cursor_pos: Vector2::new(0.0, 0.0),
            cursor_delta: Vector2::new(0.0, 0.0),
            scroll_delta: Vector2::new(0.0, 0.0),
            pressed_mouse_buttons: HashSet::new(),
        }
    }

    pub fn clear_delta(&mut self) {
        self.cursor_delta.x = 0.0;
        self.cursor_delta.y = 0.0;
        self.scroll_delta.x = 0.0;
        self.scroll_delta.y = 0.0;
    }

    pub fn is_key_down(&self, k: Key) -> bool {
        self.pressed_keys.contains(&k)
    }

    pub fn press_key(&mut self, k: Key) {
        debug!("Trigger Key {:?} Press", k);

        self.pressed_keys.insert(k);
    }

    pub fn release_key(&mut self, k: &Key) {
        debug!("Trigger Key {:?} Release", k);
        self.pressed_keys.remove(k);
    }

    pub fn get_scroll_delta(&self) -> &Vector2<f64> {
        &self.scroll_delta
    }

    pub fn set_scroll_delta(&mut self, x: f64, y: f64) {
        debug!("Trigger Mouse Scroll: X={}, Y={}", x, y);

        self.scroll_delta.x = x;
        self.scroll_delta.y = y;
    }

    pub fn get_cursor_delta(&self) -> &Vector2<f64> {
        &self.cursor_delta
    }

    pub fn get_cursor_pos(&self) -> &Vector2<f64> {
        &self.cursor_pos
    }

    pub fn set_cursor_delta(&mut self, x: f64, y: f64) {
        debug!("Trigger Cursor Move: X={}, Y={}", x, y);

        if self.is_first_cursor_move {
            self.is_first_cursor_move = false;
            self.cursor_pos = Vector2::new(x, y);
        } else {
            self.cursor_delta.x = x - self.cursor_pos.x;
            self.cursor_delta.y = y - self.cursor_pos.y;
            self.cursor_pos = Vector2::new(x, y);
        }
    }

    pub fn press_mouse_button(&mut self, b: MouseButton) {
        debug!("Trigger Mouse button: {:?}, Action: Press", b);
        self.pressed_mouse_buttons.insert(b);
    }

    pub fn release_mouse_button(&mut self, b: &MouseButton) {
        debug!("Trigger Mouse button: {:?}, Action: Release", b);
        self.pressed_mouse_buttons.remove(b);
    }
}
