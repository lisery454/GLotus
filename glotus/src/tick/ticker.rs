use std::{cell::RefCell, ptr, rc::Rc};

use crate::input::input_state::InputState;

use super::ITickable;

pub struct Ticker {
    tickable: Vec<Box<dyn ITickable>>,
}

impl Ticker {
    pub fn new() -> Self {
        Self {
            tickable: Vec::new(),
        }
    }

    pub fn add_tickable(&mut self, tickable: Box<dyn ITickable>) {
        self.tickable.push(tickable);
    }

    pub fn remove_tickable(&mut self, tickable: &Box<dyn ITickable>) {
        if let Some(pos) = self.tickable.iter().position(|x| ptr::eq(x, tickable)) {
            self.tickable.remove(pos);
        }
    }

    pub(crate) fn tick_all(&mut self, delta_time: f32, input: Rc<RefCell<InputState>>) {
        for t in self.tickable.iter_mut() {
            t.tick(delta_time, input.clone());
        }
    }
}
