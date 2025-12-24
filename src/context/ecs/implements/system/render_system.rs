use crate::*;
use cgmath::Matrix4;
use log::{error, warn};
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

#[derive(Default)]
pub struct RenderSystem;

impl ISystem for RenderSystem {
    fn name(&self) -> &str {
        "RenderSystem"
    }

    fn update(&mut self, app_context: Rc<RefCell<AppContext>>, _delta_dt: f32) {
        let context = app_context.borrow();
        let world = context.world.borrow();
        let config = context.app_config.borrow();
        // 清空缓冲区
        clear_frame(config.bg_color);

        // 计算全局数据
        let mut camera_mgr = world.get_manager_mut::<CameraComponent>();
        let mut transform_mgr = world.get_manager_mut::<TransformComponent>();
        let light_mgr = world.get_manager_mut::<LightComponent>();
        let renderable_mgr = world.get_manager_mut::<RenderableComponent>();
        let Some((_camera_entity_id, main_cam)) = camera_mgr.find_mut(|cam| cam.is_active) else {
            warn!("Can not find active camera");
            return;
        };
        let Some(main_cam_transform) = transform_mgr.get_mut(_camera_entity_id) else {
            warn!("Can not find active camera transform");
            return;
        };

        let view_matrix = get_view_matrix(&main_cam_transform.transform);
        let projection_matrix = main_cam.get_projection_matrix();
        let view_position = get_view_position(&main_cam_transform.transform);
        let camera_shader_data = camera_to_shader_data(main_cam, main_cam_transform);
        let raw_lights_shader_data = light_mgr
            .iter()
            .map(|(entity, light)| {
                if let Some(light_transform) = transform_mgr.get_mut(entity) {
                    return light_to_shader_data(light, light_transform);
                }
                None
            })
            .collect::<Option<Vec<LightShaderData>>>();
        let Some(lights_shader_data) = raw_lights_shader_data else {
            warn!("Can not create lights shader data");
            return;
        };
        let light_count = lights_shader_data.len() as i32;

        // 按pass渲染
        let pipeline = context.pipeline.borrow();
        for pass in &pipeline.passes {
            pass.default_state.apply();
            for (entity, renderable) in renderable_mgr.iter() {
                // 检查这个 Entity 的 Material 是否包含当前 Pass
                let material_handle = match renderable.materials.get(&pass.id) {
                    Some(mat) => mat.clone(),
                    None => continue, // 跳过这个 Pass
                };
                let mesh_handle = renderable.mesh;

                // 计算这个物体相关的数据
                let Some(transform) = transform_mgr.get_mut(entity) else {
                    warn!("Can not find transform for entity {:?}", entity);
                    return;
                };
                let model_matrix = transform.transform.to_matrix();
                let normal_matrix = transform.transform.to_normal_matrix().unwrap();

                // 注入全局 Uniform
                let global_uniform = GlobalUniform {
                    view_matrix: &view_matrix,
                    projection_matrix: &projection_matrix,
                    view_position: &view_position,
                    model_matrix: &model_matrix,
                    normal_matrix: &normal_matrix,
                    light_count: &light_count,
                    lights_shader_data: &lights_shader_data,
                    camera_shader_data: &camera_shader_data,
                };
                let asset_mgr = context.asset_manager.borrow();

                // 绑定 Shader
                if let Err(_) = bind_material(&asset_mgr, material_handle) {
                    error!("bind material fail");
                    continue;
                }

                // 给这个材质注入全局变量
                if let Err(_) = inject_global_uniform(&asset_mgr, material_handle, &global_uniform)
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
    }
}

fn clear_frame(color: Color) {
    unsafe {
        let col = color.to_arr();
        gl::ClearColor(col[0], col[1], col[2], 1.0);
        gl::StencilMask(0xFF);
        gl::DepthMask(gl::TRUE);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
    }
}

fn get_view_matrix(camera_transform: &Transform) -> [[f32; 4]; 4] {
    Matrix4::look_to_rh(
        camera_transform.get_translation().data,
        camera_transform.get_forward(),
        camera_transform.get_up(),
    )
    .into()
}

fn get_view_position(camera_transform: &Transform) -> [f32; 3] {
    camera_transform.get_translation().get_arr()
}

pub fn camera_to_shader_data(
    camera: &CameraComponent,
    transform: &TransformComponent,
) -> CameraShaderData {
    CameraShaderData {
        camera_type: if camera.projection_type == ProjectionType::Perspective {
            0
        } else {
            1
        },
        fov: camera.fov.0, // Deg<f32> 解包成 f32
        position: transform.transform.get_translation().get_arr().into(),
        direction: transform.transform.get_rotation().forward().into(),
        aspect_ratio: camera.aspect_ratio,
        near_plane: camera.near_plane,
        far_plane: camera.far_plane,
    }
}

pub fn light_to_shader_data(
    light: &LightComponent,
    transform: &TransformComponent,
) -> Option<LightShaderData> {
    if let Some(directional_light) = light.light.downcast_ref::<DirectionalLight>() {
        return Some(LightShaderData {
            light_type: 0, // directional
            color: directional_light.color.to_arr(),
            position: [0.0; 3],
            direction: transform.transform.get_rotation().forward().into(),
            intensity: directional_light.intensity,
            range: 0.0,
            inner_cone: 0.0,
            outer_cone: 0.0,
        });
    } else if let Some(point_light) = light.light.downcast_ref::<PointLight>() {
        return Some(LightShaderData {
            light_type: 1, // point
            color: point_light.color.to_arr(),
            position: transform.transform.get_translation().get_arr().into(),
            direction: [0.0; 3],
            intensity: point_light.intensity,
            range: point_light.range,
            inner_cone: 0.0,
            outer_cone: 0.0,
        });
    } else if let Some(spot_light) = light.light.downcast_ref::<SpotLight>() {
        return Some(LightShaderData {
            light_type: 2, // spot
            color: spot_light.color.to_arr(),
            position: transform.transform.get_translation().get_arr().into(),
            direction: transform.transform.get_rotation().forward().into(),
            intensity: spot_light.intensity,
            range: spot_light.range,
            inner_cone: spot_light.inner,
            outer_cone: spot_light.outer,
        });
    }

    None
}

pub(crate) fn inject_global_uniform(
    asset_manager: &AssetManager,
    material_handle: MaterialHandle,
    global_uniform: &GlobalUniform,
) -> Result<(), Box<dyn Error>> {
    let material_manager = &asset_manager.material_manager;
    let shader_manager = &asset_manager.shader_manager;
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
        shader.set_uniform_vec3(&format!("g_lights[{}].color", i), &v.color)?;
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
    let material_manager = &asset_manager.material_manager;
    let texture_manager = &asset_manager.texture_manager;
    let shader_manager = &asset_manager.shader_manager;
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
    let mesh = asset_manager.mesh_manager.get(mesh_handle);

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
    let material_manager = &asset_manager.material_manager;
    let shader_manager = &asset_manager.shader_manager;
    let material = material_manager
        .get(material_handle)
        .ok_or(MaterialError::FindMatFail)?;
    let shader = shader_manager
        .get(material.shader_handle)
        .ok_or(MaterialError::FindShaderFail)?;
    shader.unbind();
    Ok(())
}
