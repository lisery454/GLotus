use std::{cell::RefCell, ptr, rc::Rc};

use crate::InputState;

use super::Tickable;

/// 间隔执行管理器
pub struct Ticker {
    tickable: Vec<Rc<RefCell<dyn Tickable>>>,
}

impl Ticker {
    pub fn new() -> Self {
        Self {
            tickable: Vec::new(),
        }
    }

    /// 增加一个执行任务
    pub fn add_tickable(&mut self, tickable: Rc<RefCell<dyn Tickable>>) {
        self.tickable.push(tickable);
    }

    /// 移除一个执行任务
    pub fn remove_tickable(&mut self, tickable: Rc<RefCell<dyn Tickable>>) {
        if let Some(pos) = self
            .tickable
            .iter()
            .position(|x| ptr::eq(x.as_ptr(), tickable.as_ptr()))
        {
            self.tickable.remove(pos);
        }
    }

    /// 执行所有的任务
    pub(crate) fn tick_all(&mut self, delta_time: f32, input: Rc<RefCell<InputState>>) {
        for t in self.tickable.iter_mut() {
            t.borrow_mut().tick(delta_time, input.clone());
        }
    }
}
