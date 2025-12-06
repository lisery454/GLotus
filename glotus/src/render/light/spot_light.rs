use std::{cell::RefCell, rc::Rc};

use crate::{
    Color,
    render::{
        light::{Light, LightShaderData, LightType},
        transform::Transform,
    },
};

pub struct SpotLight {
    pub transform: Transform,
    pub color: Color,
    pub intensity: f32,
    pub range: f32,
    pub inner: f32,
    pub outer: f32,
}

impl SpotLight {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            transform: Transform::default(),
            color: Default::default(),
            intensity: 1.0,
            range: 100.0,
            inner: 1.0,
            outer: 0.8,
        }))
    }
}

impl Light for SpotLight {
    fn color(&self) -> Color {
        self.color
    }

    fn intensity(&self) -> f32 {
        self.intensity
    }

    fn light_type(&self) -> LightType {
        LightType::Spot
    }

    fn to_shader_data(&self) -> LightShaderData {
        LightShaderData {
            light_type: 2, // spot
            color: self.color.to_arr(),
            position: self.transform.get_position().get_arr().into(),
            direction: self.transform.get_rotation().forward().into(),
            intensity: self.intensity,
            range: self.range,
            inner_cone: self.inner,
            outer_cone: self.outer,
        }
    }

    fn transform(&self) -> &Transform {
        &self.transform
    }

    fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }
}
