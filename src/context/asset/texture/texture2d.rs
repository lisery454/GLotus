use gl::types::*;
use image::DynamicImage;

use crate::Resolution;

use super::{
    TextureError,
    texture_mode::{FilteringMode, WrappingMode},
};

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
    pub config: TextureConfig,
}

impl Texture2D {
    pub fn empty(resolution: Resolution, config: TextureConfig) -> Self {
        let mut texture_id: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            // 统一设置参数
            Self::apply_config(config);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                resolution.width as i32,
                resolution.height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                std::ptr::null() as *const _,
            );

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Self {
            id: texture_id,
            config,
        }
    }

    pub fn from_file(path: &str, config: TextureConfig) -> Result<Self, TextureError> {
        let img = image::open(path).map_err(|_| TextureError::FileReadError(path.to_string()))?;
        Ok(Self::load(img, config))
    }

    pub fn from_bytes(data: &[u8], config: TextureConfig) -> Result<Self, TextureError> {
        let img = image::load_from_memory(data).map_err(|_| TextureError::ByteReadError)?;
        Ok(Self::load(img, config))
    }

    fn load(img: DynamicImage, config: TextureConfig) -> Self {
        let img = img.flipv();
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();

        let mut texture_id: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            // 统一设置参数
            Self::apply_config(config);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                rgba.as_ptr() as *const _,
            );

            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Self {
            id: texture_id,
            config,
        }
    }

    fn apply_config(config: TextureConfig) {
        unsafe {
            let set_wrap = |target, mode| match mode {
                WrappingMode::Repeat => {
                    gl::TexParameteri(gl::TEXTURE_2D, target, gl::REPEAT as i32)
                }
                WrappingMode::MirroreroredRepeat => {
                    gl::TexParameteri(gl::TEXTURE_2D, target, gl::MIRRORED_REPEAT as i32)
                }
                WrappingMode::ClampToEdge => {
                    gl::TexParameteri(gl::TEXTURE_2D, target, gl::CLAMP_TO_EDGE as i32)
                }
                WrappingMode::ClampToBorder { color } => {
                    gl::TexParameteri(gl::TEXTURE_2D, target, gl::CLAMP_TO_BORDER as i32);
                    gl::TexParameterfv(gl::TEXTURE_2D, gl::TEXTURE_BORDER_COLOR, color.as_ptr());
                }
            };

            set_wrap(gl::TEXTURE_WRAP_S, config.wrapping_s);
            set_wrap(gl::TEXTURE_WRAP_T, config.wrapping_t);

            let set_filter = |target, mode: FilteringMode| {
                let val = match mode {
                    FilteringMode::Nearest => gl::NEAREST,
                    FilteringMode::Linear => gl::LINEAR,
                    FilteringMode::NearestMipmapNearest => gl::NEAREST_MIPMAP_NEAREST,
                    FilteringMode::LinearMipmapNearest => gl::LINEAR_MIPMAP_NEAREST,
                    FilteringMode::NearestMipmapLinear => gl::NEAREST_MIPMAP_LINEAR,
                    FilteringMode::LinearMipmapLinear => gl::LINEAR_MIPMAP_LINEAR,
                };
                gl::TexParameteri(gl::TEXTURE_2D, target, val as i32);
            };

            set_filter(gl::TEXTURE_MIN_FILTER, config.min_filter);
            set_filter(gl::TEXTURE_MAG_FILTER, config.mag_filter);
        }
    }
}

impl Drop for Texture2D {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
