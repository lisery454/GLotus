use std::{cell::RefCell, rc::Rc};

use crate::render::*;

/// 直射光
pub struct DirectionalLight {
    pub transform: Transform,
    pub color: Color,
    pub intensity: f32,
}

impl DirectionalLight {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            transform: Transform::default(),
            color: Default::default(),
            intensity: 1.0,
        }))
    }
}

impl Light for DirectionalLight {
    fn color(&self) -> Color {
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
            color: self.color.to_arr(),
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
