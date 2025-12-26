use crate::{Color, ILight, LightType};

/// 点光源
pub struct PointLight {
    pub color: Color,
    pub intensity: f32,
    pub range: f32,
}

impl PointLight {
    pub fn new() -> Self {
        Self {
            color: Default::default(),
            intensity: 1.0,
            range: 100.0,
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

    pub fn with_range(mut self, range: f32) -> Self {
        self.range = range;
        self
    }
}

impl ILight for PointLight {
    fn color(&self) -> Color {
        self.color
    }

    fn intensity(&self) -> f32 {
        self.intensity
    }

    fn light_type(&self) -> LightType {
        LightType::Point
    }
}
