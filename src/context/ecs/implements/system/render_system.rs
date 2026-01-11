mod global_uniform;
mod uniform_model;

use glam::Mat4;
pub use global_uniform::*;
pub use uniform_model::*;

use crate::*;
use log::{error, warn};
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;

#[derive(thiserror::Error, Debug)]
pub enum RenderError {
    #[error("NotFoundCameraTransform")]
    NotFoundCameraTransform,
    #[error("FailGetLightsShaderData")]
    FailGetLightsShaderData,
    #[error("NotFoundEntityTransform")]
    NotFoundEntityTransform,
    #[error("NotFoundPostProcessQuadMesh")]
    NotFoundPostProcessQuadMesh,
    #[error("NotFoundMesh")]
    NotFoundMesh,
    #[error("MaterialError")]
    Material(#[from] MaterialError),
    #[error("ShaderError")]
    Shader(#[from] ShaderError),
    #[error("MeshError")]
    Mesh(#[from] MeshError),
    #[error("FramebufferError")]
    Framebuffer(#[from] FramebufferError),
    #[error("TransformError")]
    Transform(#[from] TransformError),
}

#[derive(Default)]
pub struct RenderSystem {
    // 全屏四边形 mesh（用于后处理）
    fullscreen_quad: Option<MeshHandle>,

    // 每个相机的临时 framebuffer（key 是 EntityHandle）
    camera_temp_framebuffers: HashMap<EntityHandle, FramebufferHandle>,

    // ping-pong framebuffers（用于多遍后处理）
    ping_pong_framebuffers: [Option<FramebufferHandle>; 2],
    ping_pong_size: Resolution,

    global_uniform: GlobalUniform,

    // instancing framebuffer
    temp_instance_buffer: InstanceBuffer,

    // render stage
    render_stages: HashMap<PassId, Vec<(MeshHandle, MaterialHandle, EntityHandle)>>,
}

impl RenderSystem {
    fn get_all_cameras(camera_mgr: &ComponentManager<Camera>) -> Vec<(EntityHandle, &Camera)> {
        let mut cameras: Vec<(EntityHandle, &Camera)> =
            camera_mgr.iter().map(|(id, cam)| (id, cam)).collect();
        if cameras.is_empty() {
            warn!("No cameras found, render nothing");
        }
        cameras.sort_by_key(|(_, cam)| cam.order);
        return cameras;
    }

    fn clear_frame(color: Color) {
        unsafe {
            let col = color.to_arr();
            gl::ClearColor(col[0], col[1], col[2], col[3]);
            gl::StencilMask(0xFF);
            gl::DepthMask(gl::TRUE);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
        }
    }

    fn clear_frame_for_current_render_target(context: &AppContext, camera: &Camera) {
        let original_target = camera.get_target();

        if original_target == RenderTarget::Screen {
            let bg_color = context.app_config.borrow().bg_color;
            Self::clear_frame(bg_color);
        } else {
            Self::clear_frame(Color::TRANSPARENT);
        }
    }

    fn bind_render_target(
        asset_mgr: &AssetManager,
        target: RenderTarget,
        window_resolution: Resolution,
    ) {
        match target {
            RenderTarget::Screen => unsafe {
                gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
                gl::Viewport(
                    0,
                    0,
                    window_resolution.width as i32,
                    window_resolution.height as i32,
                );
            },
            RenderTarget::Framebuffer(fb) => {
                if let Ok(_) = asset_mgr.framebuffer_manager.borrow().bind(fb) {
                    // 获取 framebuffer 尺寸并设置 viewport
                    if let Ok(reso) = asset_mgr.framebuffer_manager.borrow().get_size(fb) {
                        unsafe {
                            gl::Viewport(0, 0, reso.width as i32, reso.height as i32);
                        }
                    } else {
                        error!("set viewport error");
                    }
                } else {
                    error!("bind fb error");
                }
            }
        }
    }

    fn unbind_render_target(asset_mgr: &AssetManager, target: RenderTarget) {
        match target {
            RenderTarget::Screen => {
                // 屏幕不需要解绑
            }
            RenderTarget::Framebuffer(fb) => {
                // 解绑 framebuffer，恢复到默认
                if let Ok(_) = asset_mgr.framebuffer_manager.borrow().unbind(fb) {
                } else {
                    error!("bind fb error");
                }
            }
        }
    }

    fn get_camera_trasform(
        transform_mgr: &ComponentManager<Transform>,
        camera_entity: EntityHandle,
    ) -> Result<&Transform, RenderError> {
        let Some(camera_transform) = transform_mgr.get(camera_entity) else {
            return Err(RenderError::NotFoundCameraTransform);
        };
        Ok(camera_transform)
    }

    fn get_light_shader_data(
        light_mgr: &ComponentManager<Light>,
        transform_mgr: &ComponentManager<Transform>,
    ) -> Result<Vec<LightShaderData>, RenderError> {
        let raw_lights_shader_data = light_mgr
            .iter()
            .map(|(entity, light)| {
                if let Some(light_transform) = transform_mgr.get(entity) {
                    return Some(LightShaderData::from_light(light, light_transform));
                }
                None
            })
            .collect::<Option<Vec<LightShaderData>>>();

        let Some(lights_shader_data) = raw_lights_shader_data else {
            return Err(RenderError::FailGetLightsShaderData);
        };

        Ok(lights_shader_data)
    }

    fn get_render_jobs_of_pass(
        &self,
        transform_mgr: &ComponentManager<Transform>,
        camera_view_matrix: Mat4,
        pass: &Pass,
        instancing: bool,
    ) -> Result<Vec<RenderJob>, RenderError> {
        let jobs: Vec<RenderJob>;
        if instancing && pass.is_opaque {
            let mut batch_map: HashMap<(MaterialHandle, MeshHandle), Vec<&Transform>> =
                HashMap::new();
            if let Some(batches) = self.render_stages.get(&pass.id) {
                for (mesh, material, entity) in batches.iter() {
                    if let Some(transform) = transform_mgr.get(*entity) {
                        let key = (*material, *mesh);
                        let entry = batch_map.entry(key).or_insert(vec![]);
                        entry.push(transform);
                    }
                }
            }

            jobs = batch_map
                .into_iter() // 并行处理 HashMap 的每个 Entry
                .map(|((material, mesh), transform_refs)| {
                    // 内部并行计算矩阵
                    let matrices: Vec<Mat4> =
                        transform_refs.iter().map(|t| t.to_matrix()).collect();

                    // 创建具体的 InstancedJob 类型
                    let mut instanced_job = InstancedJob::new(mesh, material);
                    instanced_job.set_transforms(matrices);
                    RenderJob::Instanced(instanced_job)
                })
                .collect();
        } else {
            let mut single_jobs = Vec::new();
            if let Some(batches) = self.render_stages.get(&pass.id) {
                for (mesh, material, entity) in batches.iter() {
                    let depth = if let Some(transform) = transform_mgr.get(*entity) {
                        let world_pos_v4 = transform.get_translation().data.to_homogeneous();
                        let view_pos = camera_view_matrix * world_pos_v4;
                        -view_pos.z
                    } else {
                        0.0
                    };

                    single_jobs.push(SingleJob::new(*entity, *mesh, *material, depth));
                }
            }

            // 进行远近排序
            if let Some(sort_func) = &pass.sort_func {
                single_jobs.sort_by(sort_func);
            }
            jobs = single_jobs
                .into_iter()
                .map(|sb| RenderJob::Single(sb))
                .collect();
        }

        Ok(jobs)
    }

    fn do_render_job(
        &mut self,
        job: &RenderJob,
        transform_mgr: &ComponentManager<Transform>,
        asset_mgr: &AssetManager,
    ) -> Result<(), RenderError> {
        match job {
            RenderJob::Single(single_job) => {
                let material_handle = single_job.get_material();
                let mesh_handle = single_job.get_mesh();
                let entity_handle = single_job.get_entity();

                // 计算这个物体相关的数据
                let Some(transform) = transform_mgr.get(entity_handle) else {
                    return Err(RenderError::NotFoundEntityTransform);
                };

                self.global_uniform
                    .update_model_data(&ModelData::new(transform)?);

                // 绑定 Shader
                Self::bind_material(&asset_mgr, material_handle)?;

                // 绘制 Mesh
                Self::draw_mesh(&asset_mgr, mesh_handle)?;

                // 卸载材质
                Self::unbind_material(&asset_mgr, material_handle)?;
            }
            RenderJob::Instanced(instanced_job) => {
                let material_handle = instanced_job.get_material();
                let mesh_handle = instanced_job.get_mesh();
                let transforms = instanced_job.get_transforms();

                Self::bind_material(&asset_mgr, material_handle)?;

                self.draw_mesh_instanced(&asset_mgr, mesh_handle, transforms)?;

                Self::unbind_material(&asset_mgr, material_handle)?;
            }
        }

        Ok(())
    }

    fn bind_material(
        asset_manager: &AssetManager,
        material_handle: MaterialHandle,
    ) -> Result<(), MaterialError> {
        let material_manager = &asset_manager.material_manager.borrow();
        let texture_manager = &asset_manager.texture_manager.borrow();
        let shader_manager = &asset_manager.shader_manager.borrow();
        let material = material_manager
            .get(material_handle)
            .ok_or(MaterialError::FindMatFail)?;
        let shader = shader_manager
            .get(material.shader_handle)
            .ok_or(MaterialError::FindShaderFail)?;

        shader.bind();

        // 给shader设置所有这个材质对应的uniforms
        for (name, value) in &material.uniforms {
            match value {
                UniformValue::Float(v) => shader.set_uniform_f32(name, *v),
                UniformValue::Int(v) => shader.set_uniform_i32(name, *v),
                UniformValue::Vector3(v) => shader.set_uniform_vec3(name, v),
                UniformValue::Vector4(v) => shader.set_uniform_vec4(name, v),
                UniformValue::Matrix3(m) => shader.set_uniform_mat3(name, m),
                UniformValue::Matrix4(m) => shader.set_uniform_mat4(name, m),
                UniformValue::Texture(slot, texture_handle) => {
                    shader
                        .set_uniform_i32(name, *slot as i32)
                        .map_err(|e| MaterialError::BindFail(e))?;

                    let texture = texture_manager
                        .get(*texture_handle)
                        .ok_or(MaterialError::FindTextureFail)?;

                    match texture {
                        Texture::Texture2D(_) => unsafe {
                            gl::ActiveTexture(gl::TEXTURE0 + *slot as u32);
                            gl::BindTexture(gl::TEXTURE_2D, texture.id());
                        },
                        Texture::CubeMap(_) => unsafe {
                            gl::ActiveTexture(gl::TEXTURE0 + *slot as u32);
                            gl::BindTexture(gl::TEXTURE_CUBE_MAP, texture.id());
                        },
                    }

                    Ok(())
                }
            }
            .map_err(|e| MaterialError::BindFail(e))?;
        }

        Ok(())
    }

    fn draw_mesh(asset_manager: &AssetManager, mesh_handle: MeshHandle) -> Result<(), RenderError> {
        let mesh_manager = asset_manager.mesh_manager.borrow();
        let mesh = mesh_manager.get(mesh_handle);

        let Some(mesh) = mesh else {
            Err(RenderError::NotFoundMesh)?
        };

        mesh.draw();

        Ok(())
    }

    fn draw_mesh_instanced(
        &mut self,
        asset_manager: &AssetManager,
        mesh_handle: MeshHandle,
        transforms: &Vec<Mat4>,
    ) -> Result<(), RenderError> {
        let mesh_manager = asset_manager.mesh_manager.borrow();
        let mesh = mesh_manager.get(mesh_handle);
        let Some(mesh) = mesh else {
            Err(RenderError::NotFoundMesh)?
        };
        let buffer = &mut self.temp_instance_buffer;
        mesh.draw_instanced(transforms, buffer);

        Ok(())
    }

    fn unbind_material(
        asset_manager: &AssetManager,
        material_handle: MaterialHandle,
    ) -> Result<(), MaterialError> {
        let material_manager = &asset_manager.material_manager.borrow();
        let shader_manager = &asset_manager.shader_manager.borrow();
        let material = material_manager
            .get(material_handle)
            .ok_or(MaterialError::FindMatFail)?;
        let shader = shader_manager
            .get(material.shader_handle)
            .ok_or(MaterialError::FindShaderFail)?;
        shader.unbind();
        Ok(())
    }

    /// 初始化后处理资源（第一次使用时调用）
    fn create_guad_mesh(&mut self, app_context: &AppContext) -> Result<(), RenderError> {
        if self.fullscreen_quad.is_some() {
            return Ok(());
        }

        // 创建全屏四边形
        let quad = app_context.create_mesh_from_positions_uvs(
            vec![0, 1, 2, 0, 2, 3],
            vec![
                -1.0, 1.0, 0.0, // 左上
                -1.0, -1.0, 0.0, // 左下
                1.0, -1.0, 0.0, // 右下
                1.0, 1.0, 0.0, // 右上
            ],
            vec![
                0.0, 1.0, // 左上 UV
                0.0, 0.0, // 左下 UV
                1.0, 0.0, // 右下 UV
                1.0, 1.0, // 右上 UV
            ],
        )?;

        self.fullscreen_quad = Some(quad);
        Ok(())
    }

    fn init_render_stage(&mut self, app_context: &AppContext) -> Result<(), RenderError> {
        self.render_stages.clear();
        let world = app_context.world.borrow();
        let renderable_mgr = world.get_manager::<Renderable>();
        let pipeline = app_context.pipeline.borrow();
        for (entity, renderable) in renderable_mgr.iter() {
            for pass in &pipeline.passes {
                if let Some(material) = renderable.get_material(pass.id) {
                    let mesh = renderable.mesh;
                    let entry = self.render_stages.entry(pass.id).or_insert(vec![]);
                    entry.push((mesh, material, entity));
                }
            }
        }

        Ok(())
    }
}

impl ISystem for RenderSystem {
    fn name(&self) -> &str {
        "RenderSystem"
    }

    fn init(&mut self, app_context: Rc<RefCell<AppContext>>) -> Result<(), Box<dyn Error>> {
        self.create_guad_mesh(&app_context.borrow())?;
        self.global_uniform.init();
        self.temp_instance_buffer.init();
        Ok(())
    }

    fn update(
        &mut self,
        app_context: Rc<RefCell<AppContext>>,
        _delta_dt: f32,
    ) -> Result<(), Box<dyn Error>> {
        let context = app_context.borrow();
        let world = context.world.borrow();
        let asset_mgr = context.asset_manager.borrow();
        let pipeline = context.pipeline.borrow();
        let camera_mgr = world.get_manager_mut::<Camera>();
        let transform_mgr = world.get_manager_mut::<Transform>();
        let light_mgr = world.get_manager_mut::<Light>();
        let window_state = context.window_state.borrow();
        let window_resolution = window_state.get_resolution();

        let cameras = Self::get_all_cameras(&camera_mgr);

        // 计算光源 shader 信息
        let lights_shader_data = Self::get_light_shader_data(&light_mgr, &transform_mgr)?;

        self.global_uniform
            .update_frame_data(&FrameData::new(&lights_shader_data));

        for (camera_entity, camera) in cameras.iter() {
            // 判断是否需要后处理
            let needs_postprocess = camera.has_postprocess();
            let original_target = camera.get_target();

            // 如果需要后处理，先渲染到临时 framebuffer
            let render_target = if needs_postprocess {
                // 获取或创建临时 framebuffer
                match self.get_or_create_temp_framebuffer(
                    &context,
                    *camera_entity,
                    window_resolution,
                ) {
                    Ok(fb) => RenderTarget::Framebuffer(fb),
                    Err(e) => {
                        error!("Failed to create temp framebuffer: {:?}", e);
                        original_target
                    }
                }
            } else {
                original_target
            };

            // 绑定相机对应的渲染目标
            Self::bind_render_target(&asset_mgr, render_target, window_resolution);
            // 清空缓冲区
            Self::clear_frame_for_current_render_target(&context, &camera);

            // 找到这个相机的 transform
            let camera_transform = Self::get_camera_trasform(&transform_mgr, *camera_entity)?;

            // 计算相机相关矩阵
            let view_matrix = camera_transform.get_view_matrix();

            self.global_uniform
                .update_camera_data(&CameraData::new(camera, camera_transform));

            self.init_render_stage(&context)?;

            // 按 pass 渲染
            for pass in &pipeline.passes {
                // 应用渲染状态
                pass.default_state.apply();

                // 收集当前 Pass 需要渲染的所有物体
                let jobs = self.get_render_jobs_of_pass(
                    &transform_mgr,
                    view_matrix,
                    pass,
                    context.app_config.borrow().instancing,
                )?;

                // 渲染所有 job
                for job in jobs {
                    if let Err(e) = self.do_render_job(&job, &transform_mgr, &asset_mgr) {
                        error!("one of render job fail: {:?}", e);
                        continue;
                    }
                }
            }

            // 相机渲染完成后解绑 rendertarget
            Self::unbind_render_target(&asset_mgr, render_target);

            if needs_postprocess {
                if let Err(e) = self.apply_postprocess(
                    &app_context.borrow(),
                    render_target,
                    original_target,
                    &camera.postprocess_materials,
                    window_resolution,
                ) {
                    error!("Failed to apply postprocess: {:?}", e);
                }
            }
        }

        Ok(())
    }
}

// 后处理
impl RenderSystem {
    /// 获取或创建相机的临时 framebuffer
    fn get_or_create_temp_framebuffer(
        &mut self,
        context: &AppContext,
        camera_entity: EntityHandle,
        resolution: Resolution,
    ) -> Result<FramebufferHandle, RenderError> {
        let anti_pixel = context.app_config.borrow().anti_pixel;
        // 检查是否已存在
        if let Some(&fb_handle) = self.camera_temp_framebuffers.get(&camera_entity) {
            // 如果大小不一致重新resize
            let asset_mgr = context.asset_manager.borrow();
            let mut framebuffer_mgr = asset_mgr.framebuffer_manager.borrow_mut();
            let size = framebuffer_mgr.get_size(fb_handle)?;
            if size != resolution {
                framebuffer_mgr.resize(fb_handle, resolution)?;
            }
            return Ok(fb_handle);
        }

        // 不存在就创建新的
        let texture_config = TextureConfig::new()
            .with_wrapping(WrappingMode::ClampToEdge, WrappingMode::ClampToEdge)
            .with_filtering(FilteringMode::Linear, FilteringMode::Linear);

        let fb = context.create_framebuffer_multi_sample(resolution, anti_pixel, texture_config)?;
        self.camera_temp_framebuffers.insert(camera_entity, fb);
        Ok(fb)
    }

    /// 确保 ping-pong framebuffers 存在
    fn ensure_ping_pong_framebuffers(
        &mut self,
        context: &AppContext,
        resolution: Resolution,
    ) -> Result<(), RenderError> {
        let anti_pixel = context.app_config.borrow().anti_pixel;

        let needs_recreate =
            self.ping_pong_framebuffers[0].is_none() || self.ping_pong_size != resolution;

        if needs_recreate {
            // 删除旧的
            for fb_opt in &self.ping_pong_framebuffers {
                if let Some(fb) = fb_opt {
                    context.remove_framebuffer(*fb)?;
                }
            }

            // 创建新的
            let texture_config = TextureConfig::new()
                .with_wrapping(WrappingMode::ClampToEdge, WrappingMode::ClampToEdge)
                .with_filtering(FilteringMode::Linear, FilteringMode::Linear);

            self.ping_pong_framebuffers[0] = Some(context.create_framebuffer_multi_sample(
                resolution,
                anti_pixel,
                texture_config,
            )?);
            self.ping_pong_framebuffers[1] = Some(context.create_framebuffer_multi_sample(
                resolution,
                anti_pixel,
                texture_config,
            )?);

            self.ping_pong_size = resolution;
        }

        Ok(())
    }

    /// 应用后处理
    fn apply_postprocess(
        &mut self,
        context: &AppContext,
        source_target: RenderTarget,
        final_target: RenderTarget,
        materials: &Vec<MaterialHandle>,
        window_resolution: Resolution,
    ) -> Result<(), RenderError> {
        if materials.is_empty() {
            return Ok(());
        }

        // 获取源纹理
        let source_texture = match source_target {
            RenderTarget::Framebuffer(fb) => context.get_texture_of_framebuffer(fb)?,
            RenderTarget::Screen => {
                error!("Cannot apply postprocess from screen");
                return Ok(());
            }
        };

        // 如果只有一个后处理，直接渲染到最终目标
        if materials.len() == 1 {
            return self.render_postprocess_pass(
                context,
                source_texture,
                final_target,
                materials[0],
                window_resolution,
            );
        }

        // 多个后处理，需要 ping-pong
        self.ensure_ping_pong_framebuffers(context, window_resolution)?;

        let mut current_source = source_texture;
        let num_materials = materials.len();

        for (i, &material) in materials.iter().enumerate() {
            let is_last = i == num_materials - 1;

            let current_target = if is_last {
                final_target
            } else {
                match self.ping_pong_framebuffers[i % 2] {
                    Some(fb) => RenderTarget::Framebuffer(fb),
                    None => RenderTarget::Screen,
                }
            };

            self.render_postprocess_pass(
                context,
                current_source,
                current_target,
                material,
                window_resolution,
            )?;

            // 更新 source
            if !is_last {
                let fb = self.ping_pong_framebuffers[i % 2].unwrap();
                current_source = context.get_texture_of_framebuffer(fb)?;
            }
        }

        Ok(())
    }

    /// 渲染单个后处理 pass
    fn render_postprocess_pass(
        &self,
        context: &AppContext,
        source_texture: TextureHandle,
        target: RenderTarget,
        material_handle: MaterialHandle,
        window_resolution: Resolution,
    ) -> Result<(), RenderError> {
        let asset_mgr = context.asset_manager.borrow();

        // 只清空非屏幕目标
        match target {
            RenderTarget::Framebuffer(_) => Self::clear_frame(Color::TRANSPARENT),
            RenderTarget::Screen => {} // 不清空，全屏quad会覆盖整个屏幕
        }

        // 绑定目标 framebuffer
        Self::bind_render_target(&asset_mgr, target, window_resolution);

        unsafe {
            gl::Disable(gl::DEPTH_TEST);
            gl::DepthMask(gl::FALSE);
            gl::Disable(gl::CULL_FACE);
            gl::Disable(gl::BLEND);
            gl::Disable(gl::SCISSOR_TEST);
            gl::Disable(gl::STENCIL_TEST);
        }

        // 清空测试 - 用明显的颜色
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::ClearColor(1.0, 0.0, 1.0, 1.0); // 洋红色
        }

        // 绑定材质（材质里应该已经包含了纹理）
        Self::bind_material(&asset_mgr, material_handle)?;

        let material_manager = asset_mgr.material_manager.borrow();
        let texture_manager = asset_mgr.texture_manager.borrow();
        let shader_manager = asset_mgr.shader_manager.borrow();
        // 注入源纹理到 material（假设 uniform 名为 "screenTexture"）
        let material = material_manager
            .get(material_handle)
            .ok_or(MaterialError::FindMatFail)?;

        let shader = shader_manager
            .get(material.shader_handle)
            .ok_or(MaterialError::FindShaderFail)?;

        {
            let name = "screenTexture";
            shader
                .set_uniform_i32(name, 0)
                .map_err(|e| MaterialError::BindFail(e))?;

            let texture = texture_manager
                .get(source_texture)
                .ok_or(MaterialError::FindTextureFail)?;

            unsafe {
                gl::ActiveTexture(gl::TEXTURE0 + 0);
                gl::BindTexture(gl::TEXTURE_2D, texture.id());
            }
        }

        // 绘制全屏四边形
        let Some(quad) = self.fullscreen_quad else {
            Err(RenderError::NotFoundPostProcessQuadMesh)?
        };

        Self::draw_mesh(&asset_mgr, quad)?;

        // 解绑
        Self::unbind_material(&asset_mgr, material_handle)?;

        Self::unbind_render_target(&asset_mgr, target);

        Ok(())
    }
}

// TODO: 清空camera已经删除的fb
