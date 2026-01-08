use crate::{IComponent, MaterialHandle, MeshHandle, PassId};
use std::collections::HashMap;

pub struct Renderable {
    // 每个pass的material
    pub materials: HashMap<PassId, MaterialHandle>,
    // mesh
    pub mesh: MeshHandle,
}

impl IComponent for Renderable {}

impl Renderable {
    pub fn new(mesh: MeshHandle) -> Self {
        Self {
            materials: HashMap::new(),
            mesh,
        }
    }

    pub fn get_material(&self, pass: impl Into<PassId>) -> Option<MaterialHandle> {
        self.materials.get(&pass.into()).cloned()
    }

    pub fn with_material(mut self, pass: impl Into<PassId>, material: MaterialHandle) -> Self {
        self.materials.insert(pass.into(), material);
        self
    }
}
