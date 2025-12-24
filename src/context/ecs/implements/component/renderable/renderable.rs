use crate::{IComponent, MaterialHandle, MeshHandle, PassId};
use std::collections::HashMap;

pub struct RenderableComponent {
    // 每个pass的material
    pub materials: HashMap<PassId, MaterialHandle>,
    // mesh
    pub mesh: MeshHandle,
}

impl IComponent for RenderableComponent {}

impl RenderableComponent {
    pub fn new(mesh: MeshHandle) -> Self {
        Self {
            materials: HashMap::new(),
            mesh,
        }
    }

    pub fn with_material(mut self, pass: impl Into<PassId>, material: MaterialHandle) -> Self {
        self.materials.insert(pass.into(), material);
        self
    }
}
