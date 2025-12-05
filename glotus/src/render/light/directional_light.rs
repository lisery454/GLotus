use std::{cell::RefCell, rc::Rc};

use cgmath::Vector3;

use crate::{
    render::light::{Light, LightShaderData, LightType},
    render::transform::Transform,
};

pub struct DirectionalLight {
    pub transform: Transform,
    pub color: Vector3<f32>,
    pub intensity: f32,
}

impl DirectionalLight {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            transform: Transform::default(),
            color: Vector3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            intensity: 1.0,
        }))
    }
}

impl Light for DirectionalLight {
    fn color(&self) -> Vector3<f32> {
        self.color
    }

    fn intensity(&self) -> f32 {
        self.intensity
    }

    fn light_type(&self) -> LightType {
        LightType::Directional
    }

    fn to_shader_data(&self) -> LightShaderData {
        LightShaderData {
            light_type: 0, // directional
            color: self.color.into(),
            position: [0.0; 3],
            direction: self.transform.get_rotation().forward().into(),
            intensity: self.intensity,
            range: 0.0,
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
