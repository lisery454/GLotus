use std::collections::HashMap;

use crate::render::*;

/// 实体对象
pub struct Entity {
    pub material_handles: HashMap<String, MaterialHandle>,
    pub mesh_handle: MeshHandle,
}

impl Entity {
    /// 新建
    pub fn new(material_handles: HashMap<String, MaterialHandle>, mesh_handle: MeshHandle) -> Self {
        Self {
            material_handles,
            mesh_handle,
        }
    }
}
