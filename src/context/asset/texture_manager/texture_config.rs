use crate::AntiPixel;

use super::{FilteringMode, FormatType, WrappingMode};

#[derive(Debug, Clone, Copy)]
pub enum TextureConfig {
    Common {
        wrapping_s: WrappingMode,
        wrapping_t: WrappingMode,
        min_filter: FilteringMode,
        mag_filter: FilteringMode,
        format_type: FormatType,
    },
    Cube {
        wrapping_s: WrappingMode,
        wrapping_t: WrappingMode,
        min_filter: FilteringMode,
        mag_filter: FilteringMode,
    },
    MultiSample {
        anti_pixel: AntiPixel,
        format_type: FormatType,
    },
}

impl TextureConfig {
    pub fn common(
        wrapping_s: WrappingMode,
        wrapping_t: WrappingMode,
        min_filter: FilteringMode,
        mag_filter: FilteringMode,
        format_type: FormatType,
    ) -> Self {
        Self::Common {
            wrapping_s,
            wrapping_t,
            min_filter,
            mag_filter,
            format_type,
        }
    }

    pub fn cube(
        wrapping_s: WrappingMode,
        wrapping_t: WrappingMode,
        min_filter: FilteringMode,
        mag_filter: FilteringMode,
    ) -> Self {
        Self::Cube {
            wrapping_s,
            wrapping_t,
            min_filter,
            mag_filter,
        }
    }

    pub fn multi_sample(anti_pixel: AntiPixel, format_type: FormatType) -> Self {
        Self::MultiSample {
            anti_pixel,
            format_type,
        }
    }
}

impl Default for TextureConfig {
    fn default() -> Self {
        Self::common(
            WrappingMode::ClampToEdge,
            WrappingMode::ClampToEdge,
            FilteringMode::Linear,
            FilteringMode::Linear,
            FormatType::SRGBA,
        )
    }
}
