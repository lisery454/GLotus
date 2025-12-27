use std::{cell::RefCell, rc::Rc};

use super::{FramebufferManager, MaterialManager, MeshManager, ShaderManager, TextureManager};

pub struct AssetManager {
    pub texture_manager: Rc<RefCell<TextureManager>>,
    pub shader_manager: Rc<RefCell<ShaderManager>>,
    pub mesh_manager: Rc<RefCell<MeshManager>>,
    pub material_manager: Rc<RefCell<MaterialManager>>,
    pub framebuffer_manager: Rc<RefCell<FramebufferManager>>,
}

impl AssetManager {
    pub fn new() -> Self {
        let texture_manager = Rc::new(RefCell::new(TextureManager::new()));
        let shader_manager = Rc::new(RefCell::new(ShaderManager::new()));
        let mesh_manager = Rc::new(RefCell::new(MeshManager::new()));
        let material_manager = Rc::new(RefCell::new(MaterialManager::new()));
        let framebuffer_manager = Rc::new(RefCell::new(FramebufferManager::new(Rc::downgrade(
            &texture_manager,
        ))));
        Self {
            texture_manager,
            shader_manager,
            mesh_manager,
            material_manager,
            framebuffer_manager,
        }
    }
}
