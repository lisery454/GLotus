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
