use std::{cell::RefCell, rc::Rc};

use cgmath::Vector3;

use crate::{
    render::light::{Light, LightShaderData, LightType},
    render::transform::Transform,
};

pub struct PointLight {
    pub transform: Transform,
    pub color: Vector3<f32>,
    pub intensity: f32,
    pub range: f32,
}

impl PointLight {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            transform: Transform::default(),
            color: Vector3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            intensity: 1.0,
            range: 100.0,
        }))
    }
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

    fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }
}
