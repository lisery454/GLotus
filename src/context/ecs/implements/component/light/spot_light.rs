use crate::{Color, ILight, LightType};

/// 聚光灯
pub struct SpotLight {
    pub color: Color,
    pub intensity: f32,
    pub range: f32,
    pub inner: f32,
    pub outer: f32,
}

impl SpotLight {
    pub fn new() -> Self {
        Self {
            color: Default::default(),
            intensity: 1.0,
            range: 100.0,
            inner: 1.0,
            outer: 0.8,
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

    pub fn with_inner(mut self, inner: f32) -> Self {
        self.inner = inner;
        self
    }

    pub fn with_outer(mut self, outer: f32) -> Self {
        self.outer = outer;
        self
    }
}

impl ILight for SpotLight {
    fn color(&self) -> Color {
        self.color
    }

    fn intensity(&self) -> f32 {
        self.intensity
    }

    fn light_type(&self) -> LightType {
        LightType::Spot
    }
}
