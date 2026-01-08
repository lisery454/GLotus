use crate::{Color, IComponent};
use log::warn;

/// 光源
pub struct Light {
    pub color: Color,
    pub intensity: f32,
    pub data: LightData,
}

#[derive(Clone, Copy)]
pub enum LightData {
    Directional,
    Point { range: f32 },
    Spot { range: f32, inner: f32, outer: f32 },
}

impl IComponent for Light {}

impl Light {
    pub fn directional() -> Self {
        Self {
            color: Default::default(),
            intensity: 1.0,
            data: LightData::Directional,
        }
    }

    pub fn point() -> Self {
        Self {
            color: Default::default(),
            intensity: 1.0,
            data: LightData::Point { range: 100.0 },
        }
    }

    pub fn spot() -> Self {
        Self {
            color: Default::default(),
            intensity: 1.0,
            data: LightData::Spot {
                range: 100.0,
                inner: 1.0,
                outer: 0.8,
            },
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

    pub fn with_range(mut self, new_range: f32) -> Self {
        match &mut self.data {
            // 对于有 range 字段的变体，直接修改
            LightData::Point { range } => {
                *range = new_range;
            }
            LightData::Spot { range, .. } => {
                *range = new_range;
            }
            LightData::Directional => {
                warn!("should not set directional light range");
            }
        }
        self
    }

    pub fn with_inner(mut self, inner_angle: f32) -> Self {
        if let LightData::Spot { inner, .. } = &mut self.data {
            *inner = inner_angle;
        } else {
            warn!("should not set directional or point light inner");
        }
        self
    }

    pub fn with_outer(mut self, outer_angle: f32) -> Self {
        if let LightData::Spot { outer, .. } = &mut self.data {
            *outer = outer_angle;
        } else {
            warn!("should not set directional or point light outer");
        }
        self
    }
}
