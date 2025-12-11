use std::{cell::RefCell, rc::Rc};

use gl::types::*;

use super::{
    texture_error::TextureError,
    texture_mode::{FilteringMode, WrappingMode},
};

/// 二维贴图
#[derive(Debug)]
pub struct Texture2D {
    pub(crate) id: GLuint,
}

impl Texture2D {
    /// 用默认配置从文件生成贴图
    pub fn from_file_default(path: &str) -> Result<Rc<RefCell<Self>>, TextureError> {
        Self::from_file(
            path,
            WrappingMode::Repeat,
            WrappingMode::Repeat,
            FilteringMode::LinearMipmapLinear,
            FilteringMode::Linear,
        )
    }

    /// 从文件生成贴图
    pub fn from_file(
        path: &str,
        wrapping_mode_s: WrappingMode,
        wrapping_mode_t: WrappingMode,
        filtering_mode_min: FilteringMode,
        filtering_mode_mag: FilteringMode,
    ) -> Result<Rc<RefCell<Self>>, TextureError> {
        // 1. 用 `image` 库读取图片
        let img_result =
            image::open(path).map_err(|_| TextureError::FileReadError(path.to_string()))?;

        let img = img_result.flipv(); // OpenGL 的纹理坐标原点在左下，需要翻转Y轴

        // 2. 转换为 RGBA 格式
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();

        // 3. 生成 OpenGL 纹理
        let mut texture_id: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            // 设置纹理参数
            Texture2D::set_wrapping_mode(gl::TEXTURE_WRAP_S, wrapping_mode_s);
            Texture2D::set_wrapping_mode(gl::TEXTURE_WRAP_T, wrapping_mode_t);

            Texture2D::set_filtering_mode(gl::TEXTURE_MIN_FILTER, filtering_mode_min);
            Texture2D::set_filtering_mode(gl::TEXTURE_MAG_FILTER, filtering_mode_mag);

            // 将图片数据上传到 GPU
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32, // 内部格式
                width as i32,
                height as i32,
                0,
                gl::RGBA,                  // 数据格式
                gl::UNSIGNED_BYTE,         // 数据类型
                rgba.as_ptr() as *const _, // 图片数据指针
            );

            // 生成 Mipmap（可选）
            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Ok(Rc::new(RefCell::new(Self { id: texture_id })))
    }

    /// 设置循环模式
    fn set_wrapping_mode(wrap: GLenum, wrapping_mode: WrappingMode) {
        unsafe {
            match wrapping_mode {
                WrappingMode::Repeat => gl::TexParameteri(gl::TEXTURE_2D, wrap, gl::REPEAT as i32),
                WrappingMode::MirroreroredRepeat => {
                    gl::TexParameteri(gl::TEXTURE_2D, wrap, gl::MIRRORED_REPEAT as i32)
                }
                WrappingMode::ClampToEdge => {
                    gl::TexParameteri(gl::TEXTURE_2D, wrap, gl::CLAMP_TO_EDGE as i32)
                }
                WrappingMode::ClampToBorder { color } => {
                    gl::TexParameteri(gl::TEXTURE_2D, wrap, gl::CLAMP_TO_BORDER as i32);
                    gl::TexParameterfv(gl::TEXTURE_2D, gl::TEXTURE_BORDER_COLOR, color.as_ptr());
                }
            }
        }
    }

    /// 设置过滤模式
    fn set_filtering_mode(filter: GLenum, filtering_mode: FilteringMode) {
        unsafe {
            match filtering_mode {
                FilteringMode::Nearest => {
                    gl::TexParameteri(gl::TEXTURE_2D, filter, gl::NEAREST as i32)
                }
                FilteringMode::Linear => {
                    gl::TexParameteri(gl::TEXTURE_2D, filter, gl::LINEAR as i32)
                }
                FilteringMode::NearestMipmapNearest => {
                    gl::TexParameteri(gl::TEXTURE_2D, filter, gl::NEAREST_MIPMAP_NEAREST as i32)
                }
                FilteringMode::LinearMipmapNearest => {
                    gl::TexParameteri(gl::TEXTURE_2D, filter, gl::LINEAR_MIPMAP_NEAREST as i32)
                }
                FilteringMode::NearestMipmapLinear => {
                    gl::TexParameteri(gl::TEXTURE_2D, filter, gl::NEAREST_MIPMAP_LINEAR as i32)
                }
                FilteringMode::LinearMipmapLinear => {
                    gl::TexParameteri(gl::TEXTURE_2D, filter, gl::LINEAR_MIPMAP_LINEAR as i32)
                }
            }
        }
    }
}
