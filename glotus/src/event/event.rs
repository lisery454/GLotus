use glfw::{Action, Key, MouseButton};

/// 应用事件
#[derive(Debug, Clone, Copy)]
pub enum AppEvent {
    /// 按键
    Key { key: Key, action: Action },
    /// 关闭窗口
    Close,
    /// 滚动
    Scroll { x: f64, y: f64 },
    /// 鼠标位置
    CursorPos { x: f64, y: f64 },
    /// 鼠标点击
    MouseButton { button: MouseButton, action: Action },
    /// resize窗口
    Resize { width: i32, height: i32 },
}
