use crate::{IComponent, MaterialHandle, MeshHandle};
use std::collections::HashMap;

pub struct RenderableComponent {
    // 每个pass的material
    pub materials: HashMap<String, MaterialHandle>,
    // mesh
    pub mesh: MeshHandle,
}

impl IComponent for RenderableComponent {}

impl RenderableComponent {
    pub fn new(materials: HashMap<String, MaterialHandle>, mesh: MeshHandle) -> Self {
        Self { materials, mesh }
    }
}
