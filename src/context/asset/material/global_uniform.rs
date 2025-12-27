/// 相机的shader数据，用来传递给shader
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CameraShaderData {
    pub camera_type: i32,
    pub fov: f32,
    pub direction: [f32; 3],
    pub position: [f32; 3],
    pub aspect_ratio: f32,
    pub near_plane: f32,
    pub far_plane: f32,
}

/// 光源shader数据
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LightShaderData {
    pub light_type: i32,
    pub color: [f32; 4],
    pub position: [f32; 3],
    pub direction: [f32; 3],
    pub intensity: f32,
    pub range: f32,
    pub inner_cone: f32,
    pub outer_cone: f32,
}

/// 全局uniform数据，用来填充glotus.glsl中的uniform
pub struct GlobalUniform<'a> {
    pub view_position: &'a [f32; 3],
    pub model_matrix: &'a [[f32; 4]; 4],
    pub normal_matrix: &'a [[f32; 3]; 3],
    pub view_matrix: &'a [[f32; 4]; 4],
    pub projection_matrix: &'a [[f32; 4]; 4],
    pub light_count: &'a i32,
    pub lights_shader_data: &'a Vec<LightShaderData>,
    pub camera_shader_data: &'a CameraShaderData,
}
