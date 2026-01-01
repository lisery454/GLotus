use crate::*;
use cgmath::{Matrix4, One};
use log::{error, warn};
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;

#[derive(Default)]
pub struct RenderSystem {
    // 全屏四边形 mesh（用于后处理）
    fullscreen_quad: Option<MeshHandle>,

    // 每个相机的临时 framebuffer（key 是 EntityHandle）
    camera_temp_framebuffers: HashMap<EntityHandle, FramebufferHandle>,

    // ping-pong framebuffers（用于多遍后处理）
    ping_pong_framebuffers: [Option<FramebufferHandle>; 2],
    ping_pong_size: Resolution,
}

impl ISystem for RenderSystem {
    fn name(&self) -> &str {
        "RenderSystem"
    }

    fn init(&mut self, app_context: Rc<RefCell<AppContext>>) {
        // 初始化后处理资源
        if let Err(e) = self.ensure_postprocess_initialized(&app_context.borrow()) {
            error!("Failed to initialize postprocess: {:?}", e);
        }
    }

    fn update(&mut self, app_context: Rc<RefCell<AppContext>>, _delta_dt: f32) {
        let context = app_context.borrow();
        let world = context.world.borrow();
        let config = context.app_config.borrow();
        let asset_mgr = context.asset_manager.borrow();
        let pipeline = context.pipeline.borrow();
        let camera_mgr = world.get_manager_mut::<Camera>();
        let transform_mgr = world.get_manager_mut::<Transform>();
        let light_mgr = world.get_manager_mut::<Light>();
        let renderable_mgr = world.get_manager_mut::<Renderable>();
        let window_resolution = config.resolution;

        // 收集所有相机
        let mut cameras: Vec<(EntityHandle, &Camera)> =
            camera_mgr.iter().map(|(id, cam)| (id, cam)).collect();
        cameras.sort_by_key(|(_, cam)| cam.order);
        if cameras.is_empty() {
            warn!("No cameras found, render fail...");
            return;
        }
        for (camera_entity_id, camera) in cameras.iter() {
            // 判断是否需要后处理
            let needs_postprocess = camera.has_postprocess();
            let original_target = camera.get_target();

            // 如果需要后处理，先渲染到临时 framebuffer
            let render_target = if needs_postprocess {
                // 获取或创建临时 framebuffer
                match self.get_or_create_temp_framebuffer(
                    &context,
                    *camera_entity_id,
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
            self.bind_render_target(&asset_mgr, render_target, window_resolution);
            // 清空缓冲区
            if render_target == RenderTarget::Screen {
                clear_frame(config.bg_color);
            } else if needs_postprocess {
                clear_frame(config.bg_color);
            } else {
                clear_frame(Color::TRANSPARENT);
            }
            // 找到这个相机的 transform
            let Some(camera_transform) = transform_mgr.get(*camera_entity_id) else {
                warn!("Cannot find transform for camera {:?}", camera_entity_id);
                continue;
            };
            // 计算相机相关矩阵
            let view_matrix = camera_transform.get_view_matrix();
            let view_matrix_arr = view_matrix.into();

            let projection_matrix = if camera_transform.space == TransformSpace::World {
                camera.get_projection_matrix()
            } else {
                Matrix4::one().into()
            };

            let view_position = get_view_position(&camera_transform);
            let camera_shader_data = camera_to_shader_data(camera, camera_transform);

            // 计算光源 shader 信息
            let raw_lights_shader_data = light_mgr
                .iter()
                .map(|(entity, light)| {
                    if let Some(light_transform) = transform_mgr.get(entity) {
                        return Some(light_to_shader_data(light, light_transform));
                    }
                    None
                })
                .collect::<Option<Vec<LightShaderData>>>();

            let Some(lights_shader_data) = raw_lights_shader_data else {
                warn!("Cannot create lights shader data");
                continue;
            };
            let light_count = lights_shader_data.len() as i32;

            // 按 pass 渲染
            for pass in &pipeline.passes {
                // 应用渲染状态
                pass.default_state.apply();

                // 收集当前 Pass 需要渲染的所有物体
                let mut jobs = Vec::new();

                for (entity, renderable) in renderable_mgr.iter() {
                    if let Some(material) = renderable.get_material(pass.id) {
                        let mesh = renderable.mesh;

                        // 获取 transform 用于计算深度
                        let depth = if let Some(transform) = transform_mgr.get(entity) {
                            let world_pos_v4 = transform.translation.data.to_homogeneous();
                            let view_pos = view_matrix * world_pos_v4;
                            -view_pos.z
                        } else {
                            0.0
                        };

                        jobs.push(RenderJob::new(entity, mesh, material, depth));
                    }
                }

                // 进行远近排序
                if let Some(sort_func) = &pass.sort_func {
                    jobs.sort_by(sort_func);
                }

                // 渲染所有 job
                for job in jobs {
                    let material_handle = job.get_material();
                    let mesh_handle = job.get_mesh();
                    let entity_handle = job.get_entity();

                    // 计算这个物体相关的数据
                    let Some(transform) = transform_mgr.get(entity_handle) else {
                        warn!("Cannot find transform for entity {:?}", entity_handle);
                        continue;
                    };
                    let model_matrix = transform.to_matrix();
                    let normal_matrix = transform.to_normal_matrix().unwrap();

                    // 注入全局 Uniform
                    let global_uniform = GlobalUniform {
                        view_matrix: &view_matrix_arr,
                        projection_matrix: &projection_matrix,
                        view_position: &view_position,
                        model_matrix: &model_matrix,
                        normal_matrix: &normal_matrix,
                        light_count: &light_count,
                        lights_shader_data: &lights_shader_data,
                        camera_shader_data: &camera_shader_data,
                    };

                    // 绑定 Shader
                    if let Err(e) = bind_material(&asset_mgr, material_handle) {
                        error!("bind material fail: {:?}", e);
                        continue;
                    }

                    // 给这个材质注入全局变量
                    if let Err(_) =
                        inject_global_uniform(&asset_mgr, material_handle, &global_uniform)
                    {
                        error!("inject global uniform fail");
                        continue;
                    }

                    // 绘制 Mesh
                    if let Err(_) = draw_mesh(&asset_mgr, mesh_handle) {
                        error!("draw mesh fail");
                        continue;
                    }

                    // 卸载材质
                    if let Err(_) = unbind_material(&asset_mgr, material_handle) {
                        error!("unbind material fail");
                        continue;
                    }
                }
            }

            // 相机渲染完成后解绑 rendertarget
            self.unbind_render_target(&asset_mgr, render_target);

            // ========== 应用后处理 ==========
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
    }
}

// 后处理
impl RenderSystem {
    /// 初始化后处理资源（第一次使用时调用）
    fn ensure_postprocess_initialized(
        &mut self,
        app_context: &AppContext,
    ) -> Result<(), Box<dyn Error>> {
        if self.fullscreen_quad.is_some() {
            return Ok(());
        }

        // 创建全屏四边形
        let quad = app_context.create_mesh_from_position_texcoord(
            &vec![0, 1, 2, 0, 2, 3],
            &vec![
                -1.0, 1.0, 0.0, // 左上
                -1.0, -1.0, 0.0, // 左下
                1.0, -1.0, 0.0, // 右下
                1.0, 1.0, 0.0, // 右上
            ],
            &vec![
                0.0, 1.0, // 左上 UV
                0.0, 0.0, // 左下 UV
                1.0, 0.0, // 右下 UV
                1.0, 1.0, // 右上 UV
            ],
        )?;

        self.fullscreen_quad = Some(quad);
        Ok(())
    }

    /// 获取或创建相机的临时 framebuffer
    fn get_or_create_temp_framebuffer(
        &mut self,
        context: &AppContext,
        camera_entity: EntityHandle,
        resolution: Resolution,
    ) -> Result<FramebufferHandle, Box<dyn Error>> {
        let anti_pixel = context.app_config.borrow().anti_pixel;
        // 检查是否已存在
        if let Some(&fb) = self.camera_temp_framebuffers.get(&camera_entity) {
            // TODO: 检查尺寸是否匹配
            return Ok(fb);
        }

        // 创建新的
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
    ) -> Result<(), Box<dyn Error>> {
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
    ) -> Result<(), Box<dyn Error>> {
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
    ) -> Result<(), Box<dyn Error>> {
        let asset_mgr = context.asset_manager.borrow();

        // 只清空非屏幕目标
        match target {
            RenderTarget::Framebuffer(_) => clear_frame(Color::TRANSPARENT),
            RenderTarget::Screen => {} // 不清空，全屏quad会覆盖整个屏幕
        }

        // 绑定目标 framebuffer
        self.bind_render_target(&asset_mgr, target, window_resolution);

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
        bind_material(&asset_mgr, material_handle)?;

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
                gl::BindTexture(gl::TEXTURE_2D, texture.id);
            }
        }

        // 绘制全屏四边形
        let Some(quad) = self.fullscreen_quad else {
            Err("post process quad Mesh does not exist")?
        };

        draw_mesh(&asset_mgr, quad)?;

        // 解绑
        unbind_material(&asset_mgr, material_handle)?;

        self.unbind_render_target(&asset_mgr, target);

        Ok(())
    }
}

// 相机绑定目标
impl RenderSystem {
    fn bind_render_target(
        &self,
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

    fn unbind_render_target(&self, asset_mgr: &AssetManager, target: RenderTarget) {
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

fn get_view_position(camera_transform: &Transform) -> [f32; 3] {
    camera_transform.get_translation().get_arr()
}

pub fn camera_to_shader_data(camera: &Camera, transform: &Transform) -> CameraShaderData {
    CameraShaderData {
        camera_type: if camera.projection_type == ProjectionType::Perspective {
            0
        } else {
            1
        },
        fov: camera.fov.0, // Deg<f32> 解包成 f32
        position: transform.get_translation().get_arr().into(),
        direction: transform.get_rotation().forward().into(),
        aspect_ratio: camera.aspect_ratio,
        near_plane: camera.near_plane,
        far_plane: camera.far_plane,
    }
}

pub fn light_to_shader_data(light: &Light, transform: &Transform) -> LightShaderData {
    // 预提取通用属性
    let color = light.color.to_arr();
    let intensity = light.intensity;
    let position = transform.get_translation().get_arr();
    let direction = transform.get_rotation().forward().into();

    match light.data {
        LightData::Directional => LightShaderData {
            light_type: 0,
            color,
            position: [0.0; 3], // 方向光通常不需要位置
            direction,
            intensity,
            range: 0.0,
            inner_cone: 0.0,
            outer_cone: 0.0,
        },
        LightData::Point { range } => LightShaderData {
            light_type: 1,
            color,
            position,
            direction: [0.0; 3],
            intensity,
            range,
            inner_cone: 0.0,
            outer_cone: 0.0,
        },
        LightData::Spot {
            range,
            inner,
            outer,
        } => LightShaderData {
            light_type: 2,
            color,
            position,
            direction,
            intensity,
            range,
            inner_cone: inner,
            outer_cone: outer,
        },
    }
}

pub(crate) fn inject_global_uniform(
    asset_manager: &AssetManager,
    material_handle: MaterialHandle,
    global_uniform: &GlobalUniform,
) -> Result<(), Box<dyn Error>> {
    let material_manager = &asset_manager.material_manager.borrow();
    let shader_manager = &asset_manager.shader_manager.borrow();
    let material = material_manager
        .get(material_handle)
        .ok_or(MaterialError::FindMatFail)?;

    let shader = shader_manager
        .get(material.shader_handle)
        .ok_or(MaterialError::FindShaderFail)?;

    shader.set_uniform_vec3("g_view_position", &global_uniform.view_position)?;
    shader.set_uniform_mat4("g_model_matrix", &global_uniform.model_matrix)?;
    shader.set_uniform_mat3("g_normal_matrix", &global_uniform.normal_matrix)?;
    shader.set_uniform_mat4("g_view_matrix", &global_uniform.view_matrix)?;
    shader.set_uniform_mat4("g_projection_matrix", &global_uniform.projection_matrix)?;
    shader.set_uniform_i32("g_light_count", *global_uniform.light_count)?;

    for (i, v) in global_uniform.lights_shader_data.iter().enumerate() {
        shader.set_uniform_i32(&format!("g_lights[{}].light_type", i), v.light_type)?;
        shader.set_uniform_vec4(&format!("g_lights[{}].color", i), &v.color)?;
        shader.set_uniform_vec3(&format!("g_lights[{}].position", i), &v.position)?;
        shader.set_uniform_vec3(&format!("g_lights[{}].direction", i), &v.direction)?;
        shader.set_uniform_f32(&format!("g_lights[{}].intensity", i), v.intensity)?;
        shader.set_uniform_f32(&format!("g_lights[{}].range", i), v.range)?;
        shader.set_uniform_f32(&format!("g_lights[{}].inner_cone", i), v.inner_cone)?;
        shader.set_uniform_f32(&format!("g_lights[{}].outer_cone", i), v.outer_cone)?;
    }
    shader.set_uniform_i32(
        "g_camera.camera_type",
        global_uniform.camera_shader_data.camera_type,
    )?;

    shader.set_uniform_vec3(
        "g_camera.direction",
        &global_uniform.camera_shader_data.direction,
    )?;

    shader.set_uniform_vec3(
        "g_camera.position",
        &global_uniform.camera_shader_data.position,
    )?;

    shader.set_uniform_f32(
        "g_camera.aspect_ratio",
        global_uniform.camera_shader_data.aspect_ratio,
    )?;

    shader.set_uniform_f32(
        "g_camera.near_plane",
        global_uniform.camera_shader_data.near_plane,
    )?;

    shader.set_uniform_f32(
        "g_camera.far_plane",
        global_uniform.camera_shader_data.far_plane,
    )?;

    Ok(())
}

pub(crate) fn bind_material(
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

                unsafe {
                    gl::ActiveTexture(gl::TEXTURE0 + *slot as u32);
                    gl::BindTexture(gl::TEXTURE_2D, texture.id);
                }

                Ok(())
            }
        }
        .map_err(|e| MaterialError::BindFail(e))?;
    }

    Ok(())
}

pub(crate) fn draw_mesh(
    asset_manager: &AssetManager,
    mesh_handle: MeshHandle,
) -> Result<(), Box<dyn Error>> {
    let mesh_manager = asset_manager.mesh_manager.borrow();
    let mesh = mesh_manager.get(mesh_handle);

    let Some(mesh) = mesh else {
        Err("Mesh does not exist")?
    };

    mesh.draw();

    Ok(())
}

pub(crate) fn unbind_material(
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
