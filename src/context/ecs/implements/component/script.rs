use crate::IComponent;
use crate::{AppContext, EntityHandle};
use std::{cell::RefCell, rc::Rc};

pub trait IBehavior: 'static {
    fn on_update(&mut self, _entity: EntityHandle, _context: Rc<RefCell<AppContext>>, _dt: f32) {}
    fn on_fixed_update(
        &mut self,
        _entity: EntityHandle,
        _context: Rc<RefCell<AppContext>>,
        _dt: f32,
    ) {
    }
}

pub struct Scriptable {
    pub behaviors: Vec<Box<dyn IBehavior>>,
}

impl IComponent for Scriptable {}

impl Scriptable {
    pub fn new() -> Self {
        Self {
            behaviors: Vec::new(),
        }
    }

    pub fn add<B: IBehavior>(&mut self, behavior: B) {
        self.behaviors.push(Box::new(behavior));
    }

    pub fn with<B: IBehavior>(mut self, behavior: B) -> Self {
        self.add(behavior);
        self
    }
}
