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

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn with_intensity(mut self, intensity: f32) -> Self {
        self.intensity = intensity;
        self
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
