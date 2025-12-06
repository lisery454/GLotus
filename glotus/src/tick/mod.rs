use std::{cell::RefCell, rc::Rc};

use crate::input::input_state::InputState;

pub mod ticker;
pub mod camera_tickable;

pub trait ITickable {
    fn tick(&mut self, delta_time: f32, input_state: Rc<RefCell<InputState>>);
}
