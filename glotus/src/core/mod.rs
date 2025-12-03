use std::{cell::RefCell, rc::Rc};

use crate::input::input_state::InputState;

pub trait UpdateAble {
    fn update(&mut self, delta_time: f32, input_state: Rc<RefCell<InputState>>);
}

pub trait FixedUpdateAble {
    fn fixed_update(&mut self, delta_time: f32, input_state: Rc<RefCell<InputState>>);
}
