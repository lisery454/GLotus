mod asset;
mod ecs;
mod event;
mod input;
mod pipeline;
mod window;

pub use asset::*;
pub use ecs::*;
pub use event::*;
pub use input::*;
pub use pipeline::*;
pub use window::*;

use crate::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct AppContext {
    pub app_config: RefCell<AppConfig>,
    pub event_queue: RefCell<AppEventQueue>,
    pub input_state: RefCell<InputState>,
    pub window_state: RefCell<WindowState>,
    pub asset_manager: RefCell<AssetManager>,
    pub pipeline: RefCell<Pipeline>,
    pub world: Rc<RefCell<World>>,
}

impl AppContext {
    pub fn new(config: AppConfig) -> Self {
        let mut pipeline = DefaultPipeline::build_default_pipeline();

        if let Some(configurer) = &config.pipeline_configurer {
            configurer(&mut pipeline);
        }

        let init_resolution = config.resolution;

        Self {
            app_config: RefCell::new(config),
            event_queue: RefCell::new(AppEventQueue::new()),
            input_state: RefCell::new(InputState::new()),
            window_state: RefCell::new(WindowState::new(init_resolution)),
            asset_manager: RefCell::new(AssetManager::new()),
            pipeline: RefCell::new(pipeline),
            world: Rc::new(RefCell::new(World::new_with_default_registry())),
        }
    }
}

impl AppContext {
    pub fn with_sdr_mgr<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut ShaderManager) -> R,
    {
        let asset_mgr = self.asset_manager.borrow();
        let mut shader_mgr = asset_mgr.shader_manager.borrow_mut();
        f(&mut *shader_mgr)
    }

    pub fn with_mat_mgr<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut MaterialManager) -> R,
    {
        let asset_mgr = self.asset_manager.borrow();
        let mut material_mgr = asset_mgr.material_manager.borrow_mut();
        f(&mut *material_mgr)
    }

    pub fn with_tex_mgr<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut TextureManager) -> R,
    {
        let asset_mgr = self.asset_manager.borrow();
        let mut texture_mgr = asset_mgr.texture_manager.borrow_mut();
        f(&mut *texture_mgr)
    }

    pub fn with_fbr_mgr<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut FramebufferManager) -> R,
    {
        let asset_mgr = self.asset_manager.borrow();
        let mut framebuffer_mgr = asset_mgr.framebuffer_manager.borrow_mut();
        f(&mut *framebuffer_mgr)
    }

    pub fn with_msh_mgr<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut MeshManager) -> R,
    {
        let asset_mgr = self.asset_manager.borrow();
        let mut mesh_mgr = asset_mgr.mesh_manager.borrow_mut();
        f(&mut *mesh_mgr)
    }

    pub fn with_world<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut World) -> R,
    {
        f(&mut self.world.borrow_mut())
    }
}
