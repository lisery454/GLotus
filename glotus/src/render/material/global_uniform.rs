use crate::render::{camera::CameraShaderData, light::LightShaderData};

pub struct GlobalUniform<'a> {
    pub view_position: [f32; 3],
    pub model_matrix: [[f32; 4]; 4],
    pub normal_matrix: [[f32; 3]; 3],
    pub view_matrix: [[f32; 4]; 4],
    pub projection_matrix: [[f32; 4]; 4],
    pub light_count: i32,
    pub lights_shader_data: &'a Vec<LightShaderData>,
    pub camera_shader_data: &'a CameraShaderData,
}
