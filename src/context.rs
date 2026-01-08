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
    /// 从文件创建 2D 纹理
    pub fn create_texture_2d_from_file(
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

    /// 从字节数据创建 2D 纹理
    pub fn create_texture_2d_from_bytes(
        &self,
        data: &[u8],
        config: TextureConfig,
    ) -> Result<TextureHandle, TextureError> {
        self.asset_manager
            .borrow_mut()
            .texture_manager
            .borrow_mut()
            .create_from_bytes(data, config)
    }

    /// 创建空的 2D 纹理
    pub fn create_empty_texture_2d(
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

    /// 创建空的多重采样 2D 纹理
    pub fn create_empty_multi_sample_texture_2d(
        &self,
        resolution: Resolution,
        anti_pixel: AntiPixel,
    ) -> Result<TextureHandle, TextureError> {
        self.asset_manager
            .borrow_mut()
            .texture_manager
            .borrow_mut()
            .create_empty_multi_sample(resolution, anti_pixel)
    }

    /// 调整 2D 纹理大小
    pub fn resize_texture_2d(
        &self,
        texture: TextureHandle,
        new_resolution: Resolution,
    ) -> Result<(), TextureError> {
        self.asset_manager
            .borrow_mut()
            .texture_manager
            .borrow_mut()
            .resize_2d(texture, new_resolution)
    }

    /// 创建空的立方体贴图
    pub fn create_empty_cube_map(
        &self,
        resolution: Resolution,
        config: TextureConfig,
    ) -> Result<TextureHandle, TextureError> {
        self.asset_manager
            .borrow_mut()
            .texture_manager
            .borrow_mut()
            .create_empty_cube_map(resolution, config)
    }

    /// 从六个文件创建立方体贴图
    ///
    /// # 参数
    /// - `paths`: 六个文件路径，顺序为 [右, 左, 上, 下, 前, 后] (+X, -X, +Y, -Y, +Z, -Z)
    ///
    /// # 示例
    /// ```
    /// let skybox = ctx.create_cube_map_from_files(
    ///     [
    ///         "skybox/right.jpg",
    ///         "skybox/left.jpg",
    ///         "skybox/top.jpg",
    ///         "skybox/bottom.jpg",
    ///         "skybox/front.jpg",
    ///         "skybox/back.jpg",
    ///     ],
    ///     TextureConfig::default(),
    /// )?;
    /// ```
    pub fn create_cube_map_from_files(
        &self,
        paths: [&str; 6],
        config: TextureConfig,
    ) -> Result<TextureHandle, TextureError> {
        self.asset_manager
            .borrow_mut()
            .texture_manager
            .borrow_mut()
            .create_cube_map_from_files(paths, config)
    }

    /// 从六个字节数组创建立方体贴图
    ///
    /// # 参数
    /// - `data_array`: 六个图像数据，顺序为 [右, 左, 上, 下, 前, 后] (+X, -X, +Y, -Y, +Z, -Z)
    pub fn create_cube_map_from_bytes(
        &self,
        data_array: [&[u8]; 6],
        config: TextureConfig,
    ) -> Result<TextureHandle, TextureError> {
        self.asset_manager
            .borrow_mut()
            .texture_manager
            .borrow_mut()
            .create_cube_map_from_bytes(data_array, config)
    }

    /// 更新立方体贴图的某个面
    ///
    /// # 参数
    /// - `texture`: 立方体贴图的句柄
    /// - `face`: 要更新的面（使用 CubeFace 枚举）
    /// - `img`: 新的图像数据
    pub fn update_cube_map_face(
        &self,
        texture: TextureHandle,
        face: CubeFace,
        img: image::DynamicImage,
    ) -> Result<(), TextureError> {
        self.asset_manager
            .borrow_mut()
            .texture_manager
            .borrow_mut()
            .update_cube_map_face(texture, face, img)
    }

    /// 删除纹理（支持 Texture2D 和 CubeMap）
    pub fn remove_texture(&self, texture: TextureHandle) {
        self.asset_manager
            .borrow_mut()
            .texture_manager
            .borrow_mut()
            .remove(texture);
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

    pub fn create_framebuffer_multi_sample(
        &self,
        resolution: Resolution,
        anti_pixel: AntiPixel,
        config: TextureConfig,
    ) -> Result<FramebufferHandle, FramebufferError> {
        self.asset_manager
            .borrow()
            .framebuffer_manager
            .borrow_mut()
            .create_multi_sample(resolution, anti_pixel, config)
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
    pub fn create_mesh_from_positions(
        &self,
        indices: Vec<u32>,
        positions: Vec<f32>,
    ) -> Result<MeshHandle, MeshError> {
        self.asset_manager
            .borrow_mut()
            .mesh_manager
            .borrow_mut()
            .create_from_positions(indices, positions)
    }

    pub fn create_mesh_from_position_normal(
        &self,
        indices: Vec<u32>,
        positions: Vec<f32>,
        normals: Vec<f32>,
    ) -> Result<MeshHandle, MeshError> {
        self.asset_manager
            .borrow_mut()
            .mesh_manager
            .borrow_mut()
            .create_from_positions_normals(indices, positions, normals)
    }

    pub fn create_mesh_from_positions_uvs(
        &self,
        indices: Vec<u32>,
        positions: Vec<f32>,
        texcoords: Vec<f32>,
    ) -> Result<MeshHandle, MeshError> {
        self.asset_manager
            .borrow_mut()
            .mesh_manager
            .borrow_mut()
            .create_from_positions_uvs(indices, positions, texcoords)
    }

    pub fn create_mesh_from_positions_normals_uvs(
        &self,
        indices: Vec<u32>,
        positions: Vec<f32>,
        normals: Vec<f32>,
        texcoords: Vec<f32>,
    ) -> Result<MeshHandle, MeshError> {
        self.asset_manager
            .borrow_mut()
            .mesh_manager
            .borrow_mut()
            .create_from_positions_normals_uvs(indices, positions, normals, texcoords)
    }

    pub fn create_mesh_from_vertex_data(
        &self,
        indices: Vec<u32>,
        vertex_data: VertexData,
    ) -> Result<MeshHandle, MeshError> {
        self.asset_manager
            .borrow_mut()
            .mesh_manager
            .borrow_mut()
            .create_from_vertex_data(indices, vertex_data)
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
            .create_from_obj_bytes(data)
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
