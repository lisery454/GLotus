use crate::IComponent;

use super::IBehavior;

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
