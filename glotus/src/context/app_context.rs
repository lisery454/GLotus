use super::{AppEventQueue, AssetManager, InputState, Pipeline};
use crate::{AppConfig, World};
use std::cell::RefCell;
use std::rc::Rc;

pub struct AppContext {
    pub app_config: RefCell<AppConfig>,
    pub(crate) event_queue: RefCell<AppEventQueue>,
    pub input_state: RefCell<InputState>,
    pub asset_manager: RefCell<AssetManager>,
    pub pipeline: RefCell<Pipeline>,
    pub world: Rc::<RefCell<World>>,
}

impl AppContext {
    pub fn new(config: AppConfig) -> Self {
        let pipeline = (*config.pipeline_builder)();
        Self {
            app_config: RefCell::new(config),
            event_queue: RefCell::new(AppEventQueue::new()),
            input_state: RefCell::new(InputState::new()),
            asset_manager: RefCell::new(AssetManager::new()),
            pipeline: RefCell::new(pipeline),
            world: Rc::new(RefCell::new(World::new())),
        }
    }
}
