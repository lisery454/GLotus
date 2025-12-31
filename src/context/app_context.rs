use super::{AppEventQueue, AssetManager, DefaultPipeline, InputState, Pipeline};
use crate::*;
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
            .borrow_mut()
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
            .borrow_mut()
            .create_from_sources(vertex_source, fragment_source)
    }

    pub fn remove_shader(&self, shader: ShaderHandle) {
        self.asset_manager
            .borrow_mut()
            .shader_manager
            .borrow_mut()
            .remove(shader);
    }
}

// material
pub struct MaterialBuilder<'a> {
    app_context: &'a AppContext,
    material: MaterialHandle,
}

impl<'a> MaterialBuilder<'a> {
    pub fn with(self, name: &str, value: UniformValue) -> MaterialBuilder<'a> {
        self.app_context
            .insert_uniform_to_material(self.material, name, value);
        self
    }

    pub fn build(self) -> MaterialHandle {
        self.material
    }
}
impl AppContext {
    pub fn get_material_builder(
        &self,
        shader: ShaderHandle,
    ) -> Result<MaterialBuilder<'_>, MaterialError> {
        let material = self.create_material(shader)?;
        Ok(MaterialBuilder {
            app_context: self,
            material,
        })
    }

    pub fn create_material(&self, shader: ShaderHandle) -> Result<MaterialHandle, MaterialError> {
        self.asset_manager
            .borrow_mut()
            .material_manager
            .borrow_mut()
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
            .borrow_mut()
            .insert_uniform(material, name, value);
    }

    pub fn remove_material(&self, material: MaterialHandle) {
        self.asset_manager
            .borrow_mut()
            .material_manager
            .borrow_mut()
            .remove(material);
    }
}

// texture
impl AppContext {
    pub fn create_texture_from_file(
        &self,
        path: &str,
        config: TextureConfig,
    ) -> Result<TextureHandle, TextureError> {
        self.asset_manager
            .borrow_mut()
            .texture_manager
            .borrow_mut()
            .create_from_file(path, config)
    }

    pub fn create_texture_from_byte(
        &self,
        data: &[u8],
        config: TextureConfig,
    ) -> Result<TextureHandle, TextureError> {
        self.asset_manager
            .borrow_mut()
            .texture_manager
            .borrow_mut()
            .create_from_byte(data, config)
    }

    pub fn remove_texture(&mut self, texture: TextureHandle) {
        self.asset_manager
            .borrow_mut()
            .texture_manager
            .borrow_mut()
            .remove(texture);
    }

    pub fn create_empty_texture(
        &self,
        resolution: Resolution,
        config: TextureConfig,
    ) -> Result<TextureHandle, TextureError> {
        self.asset_manager
            .borrow_mut()
            .texture_manager
            .borrow_mut()
            .create_empty(resolution, config)
    }
}

// framebuffer
impl AppContext {
    pub fn create_framebuffer(
        &self,
        resolution: Resolution,
        config: TextureConfig,
    ) -> Result<FramebufferHandle, FramebufferError> {
        self.asset_manager
            .borrow()
            .framebuffer_manager
            .borrow_mut()
            .create(resolution, config)
    }

    pub fn remove_framebuffer(&self, handle: FramebufferHandle) -> Result<(), FramebufferError> {
        self.asset_manager
            .borrow()
            .framebuffer_manager
            .borrow_mut()
            .remove(handle)
    }

    pub fn get_texture_of_framebuffer(
        &self,
        framebuffer: FramebufferHandle,
    ) -> Result<TextureHandle, FramebufferError> {
        self.asset_manager
            .borrow()
            .framebuffer_manager
            .borrow_mut()
            .get_color_texture(framebuffer)
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
            .borrow_mut()
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
            .borrow_mut()
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
            .borrow_mut()
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
            .borrow_mut()
            .create_from_position_normal_texcoord(indices, positions, normals, texcoords)
    }

    pub fn create_mesh_from_obj_path(&self, path: &str) -> Result<MeshHandle, MeshError> {
        self.asset_manager
            .borrow_mut()
            .mesh_manager
            .borrow_mut()
            .create_from_obj_path(path)
    }

    pub fn create_mesh_from_obj_in_bytes(&self, data: &[u8]) -> Result<MeshHandle, MeshError> {
        self.asset_manager
            .borrow_mut()
            .mesh_manager
            .borrow_mut()
            .create_from_obj_in_bytes(data)
    }

    pub fn remove_mesh(&mut self, mesh: MeshHandle) {
        self.asset_manager
            .borrow_mut()
            .mesh_manager
            .borrow_mut()
            .remove(mesh);
    }
}

// entity
// 定义 Bundle trait
pub trait ComponentBundle {
    fn add_to_entity(self, ctx: &AppContext, entity: EntityHandle);
}

// 为单个组件实现
impl<T: IComponent> ComponentBundle for T {
    fn add_to_entity(self, ctx: &AppContext, entity: EntityHandle) {
        ctx.add_component(entity, self);
    }
}

// 为元组实现
impl<T1, T2> ComponentBundle for (T1, T2)
where
    T1: IComponent,
    T2: IComponent,
{
    fn add_to_entity(self, ctx: &AppContext, entity: EntityHandle) {
        ctx.add_component(entity, self.0);
        ctx.add_component(entity, self.1);
    }
}

impl<T1, T2, T3> ComponentBundle for (T1, T2, T3)
where
    T1: IComponent,
    T2: IComponent,
    T3: IComponent,
{
    fn add_to_entity(self, ctx: &AppContext, entity: EntityHandle) {
        ctx.add_component(entity, self.0);
        ctx.add_component(entity, self.1);
        ctx.add_component(entity, self.2);
    }
}
impl AppContext {
    pub fn spawn_entity(&self) -> EntityHandle {
        self.world.borrow_mut().spawn_entity()
    }

    pub fn despawn_entity(&self, entity: EntityHandle) {
        self.world.borrow_mut().despawn_entity(entity)
    }

    pub fn spawn_entity_with<B: ComponentBundle>(&self, bundle: B) -> EntityHandle {
        let entity = self.spawn_entity();
        bundle.add_to_entity(self, entity);
        entity
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
