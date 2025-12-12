use std::collections::HashSet;

use cgmath::Vector2;
use glfw::{Key, MouseButton};
use log::debug;

/// 输入状态
pub struct InputState {
    pressed_keys: HashSet<Key>,

    is_first_cursor_move: bool,
    cursor_pos: Vector2<f64>,
    cursor_delta: Vector2<f64>,

    scroll_delta: Vector2<f64>,

    pressed_mouse_buttons: HashSet<MouseButton>,
}

impl InputState {
    /// 空的输入状态
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

    /// 清空偏移的数据
    pub(crate) fn clear_delta(&mut self) {
        self.cursor_delta.x = 0.0;
        self.cursor_delta.y = 0.0;
        self.scroll_delta.x = 0.0;
        self.scroll_delta.y = 0.0;
    }

    /// 是否按下某个按键
    pub fn is_key_down(&self, k: Key) -> bool {
        self.pressed_keys.contains(&k)
    }

    /// 按下按键
    pub(crate) fn press_key(&mut self, k: Key) {
        debug!("Trigger Key {:?} Press", k);

        self.pressed_keys.insert(k);
    }

    /// 释放按键
    pub(crate) fn release_key(&mut self, k: &Key) {
        debug!("Trigger Key {:?} Release", k);
        self.pressed_keys.remove(k);
    }

    /// 获取滚动的delta数据
    pub fn get_scroll_delta(&self) -> &Vector2<f64> {
        &self.scroll_delta
    }

    /// 设置滚动的delta数据
    pub(crate) fn set_scroll_delta(&mut self, x: f64, y: f64) {
        debug!("Trigger Mouse Scroll: X={}, Y={}", x, y);

        self.scroll_delta.x = x;
        self.scroll_delta.y = y;
    }

    /// 获取鼠标的位移
    pub fn get_cursor_delta(&self) -> &Vector2<f64> {
        &self.cursor_delta
    }

    /// 获取鼠标的位置
    pub fn get_cursor_pos(&self) -> &Vector2<f64> {
        &self.cursor_pos
    }

    /// 设置鼠标的delta值
    pub(crate) fn set_cursor_delta(&mut self, x: f64, y: f64) {
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

    /// 按下鼠标按键
    pub(crate) fn press_mouse_button(&mut self, b: MouseButton) {
        debug!("Trigger Mouse button: {:?}, Action: Press", b);
        self.pressed_mouse_buttons.insert(b);
    }

    /// 抬起鼠标按钮
    pub(crate) fn release_mouse_button(&mut self, b: &MouseButton) {
        debug!("Trigger Mouse button: {:?}, Action: Release", b);
        self.pressed_mouse_buttons.remove(b);
    }
}
