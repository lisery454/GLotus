use gl::types::*;
use image::DynamicImage;

use crate::{AntiPixel, Resolution};

use super::{
    TextureConfig, TextureError,
    texture_mode::{FilteringMode, WrappingMode},
};

/// 二维贴图
#[derive(Debug)]
pub struct Texture2D {
    pub(crate) id: GLuint,
}

impl Texture2D {
    pub fn empty_multi_sample(resolution: Resolution, anti_pixel: AntiPixel) -> Self {
        let mut texture_id: GLuint = 0;
        let samples = anti_pixel.samples();
        unsafe {
            // 创建多重采样纹理
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D_MULTISAMPLE, texture_id);

            gl::TexImage2DMultisample(
                gl::TEXTURE_2D_MULTISAMPLE,
                samples as i32,
                gl::RGBA8,
                resolution.width as i32,
                resolution.height as i32,
                gl::TRUE, // fixedsamplelocations
            );

            // 注意：多重采样纹理不需要设置过滤和wrap参数
            // 这些参数只在resolve后的普通纹理上有效
            gl::BindTexture(gl::TEXTURE_2D_MULTISAMPLE, 0);
        }
        Self { id: texture_id }
    }

    pub fn empty(resolution: Resolution, config: TextureConfig) -> Self {
        let mut texture_id: GLuint = 0;

        unsafe {
            // 创建普通纹理
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            // 应用纹理配置（过滤、wrap等）
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

        Self { id: texture_id }
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

        Self { id: texture_id }
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
                    gl::TexParameterfv(
                        gl::TEXTURE_2D,
                        gl::TEXTURE_BORDER_COLOR,
                        color.to_arr().as_ptr(),
                    );
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
