use super::{AppEventQueue, AssetManager, DefaultPipeline, InputState, Pipeline};
use crate::{
    AppConfig, EntityHandle, FilteringMode, IComponent, MaterialError, MaterialHandle, MeshError,
    MeshHandle, ShaderError, ShaderHandle, TextureError, TextureHandle, UniformValue, World,
    WrappingMode,
};
use std::cell::RefCell;
use std::rc::Rc;

pub struct AppContext {
    pub app_config: RefCell<AppConfig>,
    pub(crate) event_queue: RefCell<AppEventQueue>,
    pub input_state: RefCell<InputState>,
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

        Self {
            app_config: RefCell::new(config),
            event_queue: RefCell::new(AppEventQueue::new()),
            input_state: RefCell::new(InputState::new()),
            asset_manager: RefCell::new(AssetManager::new()),
            pipeline: RefCell::new(pipeline),
            world: Rc::new(RefCell::new(World::new_with_default_registry())),
        }
    }
}

// shader
impl AppContext {
    pub fn create_shader_from_files(
        &self,
        vertex_path: &str,
        fragment_path: &str,
    ) -> Result<ShaderHandle, ShaderError> {
        self.asset_manager
            .borrow_mut()
            .shader_manager
            .create_from_files(vertex_path, fragment_path)
    }

    pub fn create_shader_from_sources(
        &self,
        vertex_source: &str,
        fragment_source: &str,
    ) -> Result<ShaderHandle, ShaderError> {
        self.asset_manager
            .borrow_mut()
            .shader_manager
            .create_from_sources(vertex_source, fragment_source)
    }

    pub fn remove_shader(&self, shader: ShaderHandle) {
        self.asset_manager
            .borrow_mut()
            .shader_manager
            .remove(shader);
    }
}

// material
impl AppContext {
    pub fn create_material(&self, shader: ShaderHandle) -> Result<MaterialHandle, MaterialError> {
        self.asset_manager
            .borrow_mut()
            .material_manager
            .create(shader)
    }

    pub fn insert_uniform_to_material(
        &self,
        material: MaterialHandle,
        name: &str,
        value: UniformValue,
    ) {
        self.asset_manager
            .borrow_mut()
            .material_manager
            .insert_uniform(material, name, value);
    }

    pub fn remove_material(&self, material: MaterialHandle) {
        self.asset_manager
            .borrow_mut()
            .material_manager
            .remove(material);
    }
}

// texture
impl AppContext {
    pub fn create_texture_from_file(
        &self,
        path: &str,
        wrapping_mode_s: WrappingMode,
        wrapping_mode_t: WrappingMode,
        filtering_mode_min: FilteringMode,
        filtering_mode_mag: FilteringMode,
    ) -> Result<TextureHandle, TextureError> {
        self.asset_manager
            .borrow_mut()
            .texture_manager
            .create_from_file(
                path,
                wrapping_mode_s,
                wrapping_mode_t,
                filtering_mode_min,
                filtering_mode_mag,
            )
    }

    pub fn create_texture_from_byte(
        &self,
        data: &[u8],
        wrapping_mode_s: WrappingMode,
        wrapping_mode_t: WrappingMode,
        filtering_mode_min: FilteringMode,
        filtering_mode_mag: FilteringMode,
    ) -> Result<TextureHandle, TextureError> {
        self.asset_manager
            .borrow_mut()
            .texture_manager
            .create_from_byte(
                data,
                wrapping_mode_s,
                wrapping_mode_t,
                filtering_mode_min,
                filtering_mode_mag,
            )
    }

    pub fn remove_texture(&mut self, texture: TextureHandle) {
        self.asset_manager
            .borrow_mut()
            .texture_manager
            .remove(texture);
    }
}

// mesh
impl AppContext {
    pub fn create_mesh_from_position(
        &self,
        indices: &Vec<usize>,
        positions: &Vec<f32>,
    ) -> Result<MeshHandle, MeshError> {
        self.asset_manager
            .borrow_mut()
            .mesh_manager
            .create_from_position(indices, positions)
    }

    pub fn create_mesh_from_position_normal(
        &self,
        indices: &Vec<usize>,
        positions: &Vec<f32>,
        normals: &Vec<f32>,
    ) -> Result<MeshHandle, MeshError> {
        self.asset_manager
            .borrow_mut()
            .mesh_manager
            .create_from_position_normal(indices, positions, normals)
    }

    pub fn create_mesh_from_position_texcoord(
        &self,
        indices: &Vec<usize>,
        positions: &Vec<f32>,
        texcoords: &Vec<f32>,
    ) -> Result<MeshHandle, MeshError> {
        self.asset_manager
            .borrow_mut()
            .mesh_manager
            .create_from_position_texcoord(indices, positions, texcoords)
    }

    pub fn create_mesh_from_position_normal_texcoord(
        &self,
        indices: &Vec<usize>,
        positions: &Vec<f32>,
        normals: &Vec<f32>,
        texcoords: &Vec<f32>,
    ) -> Result<MeshHandle, MeshError> {
        self.asset_manager
            .borrow_mut()
            .mesh_manager
            .create_from_position_normal_texcoord(indices, positions, normals, texcoords)
    }

    pub fn create_mesh_from_obj_path(&self, path: &str) -> Result<MeshHandle, MeshError> {
        self.asset_manager
            .borrow_mut()
            .mesh_manager
            .create_from_obj_path(path)
    }

    pub fn create_mesh_from_obj_in_bytes(&self, data: &[u8]) -> Result<MeshHandle, MeshError> {
        self.asset_manager
            .borrow_mut()
            .mesh_manager
            .create_from_obj_in_bytes(data)
    }

    pub fn remove_mesh(&mut self, mesh: MeshHandle) {
        self.asset_manager.borrow_mut().mesh_manager.remove(mesh);
    }
}

// entity
impl AppContext {
    pub fn spawn_entity(&self) -> EntityHandle {
        self.world.borrow_mut().spawn_entity()
    }

    pub fn despawn_entity(&self, entity: EntityHandle) {
        self.world.borrow_mut().despawn_entity(entity)
    }

    pub fn add_component<T: IComponent>(&self, entity: EntityHandle, component: T) {
        self.world
            .borrow_mut()
            .get_manager_mut::<T>()
            .add(entity, component);
    }

    pub fn remove_component<T: IComponent>(&self, entity: EntityHandle) -> Option<T> {
        self.world
            .borrow_mut()
            .get_manager_mut::<T>()
            .remove(entity)
    }
}
