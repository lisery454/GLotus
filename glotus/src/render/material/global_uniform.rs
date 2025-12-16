use crate::render::{camera::CameraShaderData, light::LightShaderData};

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
