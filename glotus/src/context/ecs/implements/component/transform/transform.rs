use crate::{IComponent, Transform};

pub struct TransformComponent {
    pub transform: Transform,
}

impl IComponent for TransformComponent {}

impl TransformComponent {
    pub fn new(transform: Transform) -> Self {
        Self { transform }
    }
}
