use super::{MaterialManager, MeshManager, ShaderManager, TextureManager};

pub struct AssetManager {
    pub texture_manager: TextureManager,
    pub shader_manager: ShaderManager,
    pub mesh_manager: MeshManager,
    pub material_manager: MaterialManager,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            texture_manager: TextureManager::new(),
            shader_manager: ShaderManager::new(),
            mesh_manager: MeshManager::new(),
            material_manager: MaterialManager::new(),
        }
    }
}
