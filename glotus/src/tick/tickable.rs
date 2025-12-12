use std::{cell::RefCell, rc::Rc};

use crate::InputState;

/// 间隔事件执行器
pub trait Tickable {
    /// 执行一次tick
    fn tick(&mut self, delta_time: f32, input_state: Rc<RefCell<InputState>>);
}
