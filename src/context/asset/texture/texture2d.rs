use gl::types::*;

use super::texture_mode::{FilteringMode, WrappingMode};

#[derive(Debug, Clone, Copy)]
pub struct TextureConfig {
    pub wrapping_s: WrappingMode,
    pub wrapping_t: WrappingMode,
    pub min_filter: FilteringMode,
    pub mag_filter: FilteringMode,
}

impl TextureConfig {
    pub fn new() -> Self {
        Self::default()
    }

    /// 修改循环模式（同时修改 S 和 T 轴）
    pub fn with_wrapping(mut self, mode_s: WrappingMode, mode_t: WrappingMode) -> Self {
        self.wrapping_s = mode_s;
        self.wrapping_t = mode_t;
        self
    }

    /// 修改过滤模式
    pub fn with_filtering(mut self, min: FilteringMode, mag: FilteringMode) -> Self {
        self.min_filter = min;
        self.mag_filter = mag;
        self
    }
}

impl Default for TextureConfig {
    fn default() -> Self {
        Self {
            wrapping_s: WrappingMode::ClampToEdge,
            wrapping_t: WrappingMode::ClampToEdge,
            min_filter: FilteringMode::Linear,
            mag_filter: FilteringMode::Linear,
        }
    }
}

/// 二维贴图
#[derive(Debug)]
pub struct Texture2D {
    pub(crate) id: GLuint,
    pub width: u32,
    pub height: u32,
}
