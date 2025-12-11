use std::{cell::RefCell, rc::Rc};

use crate::render::*;

/// 点光源
pub struct PointLight {
    pub transform: Transform,
    pub color: Color,
    pub intensity: f32,
    pub range: f32,
}

impl PointLight {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            transform: Transform::default(),
            color: Default::default(),
            intensity: 1.0,
            range: 100.0,
        }))
    }
}

impl Light for PointLight {
    fn color(&self) -> Color {
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
            color: self.color.to_arr(),
            position: self.transform.get_translation().get_arr().into(),
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
