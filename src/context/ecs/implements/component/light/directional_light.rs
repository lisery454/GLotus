use crate::{Color, ILight, LightType};

/// 直射光
pub struct DirectionalLight {
    pub color: Color,
    pub intensity: f32,
}

impl DirectionalLight {
    pub fn new() -> Self {
        Self {
            color: Default::default(),
            intensity: 1.0,
        }
    }
}

impl ILight for DirectionalLight {
    fn color(&self) -> Color {
        self.color
    }

    fn intensity(&self) -> f32 {
        self.intensity
    }

    fn light_type(&self) -> LightType {
        LightType::Directional
    }
}
