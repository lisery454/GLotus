use glfw::{Action, Key, MouseButton};

#[derive(Debug, Clone, Copy)]
pub enum AppEvent {
    Key { key: Key, action: Action },
    Close,
    Scroll { x: f64, y: f64 },
    CursorPos { x: f64, y: f64 },
    MouseButton { button: MouseButton, action: Action },
    Resize { width: i32, height: i32 },
}
