use cgmath::Vector3;

use crate::{
    light::{Light, LightShaderData, LightType},
    transform::Transform,
};

pub struct PointLight {
    pub transform: Transform,
    pub color: Vector3<f32>,
    pub intensity: f32,
    pub range: f32,
}

impl Light for PointLight {
    fn color(&self) -> Vector3<f32> {
        self.color
    }

    fn intensity(&self) -> f32 {
        self.intensity
    }

    fn light_type(&self) -> LightType {
        LightType::Point
    }

    fn to_shader_data(&self) -> LightShaderData {
        LightShaderData {
            light_type: 1, // point
            color: self.color.into(),
            position: self.transform.get_position().get_arr().into(),
            direction: [0.0; 3],
            intensity: self.intensity,
            range: self.range,
            inner_cone: 0.0,
            outer_cone: 0.0,
        }
    }

    fn transform(&self) -> &Transform {
        &self.transform
    }
}
