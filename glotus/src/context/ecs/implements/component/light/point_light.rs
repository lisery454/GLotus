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
