use rayon::prelude::*;

use gl::types::*;
use image::{DynamicImage, GenericImageView};

use crate::Resolution;

use super::{
    TextureConfig, TextureError,
    texture_mode::{FilteringMode, WrappingMode},
};

/// 立方体贴图的六个面
#[derive(Debug, Clone, Copy)]
pub enum CubeFace {
    PositiveX = 0, // 右
    NegativeX = 1, // 左
    PositiveY = 2, // 上
    NegativeY = 3, // 下
    PositiveZ = 4, // 前
    NegativeZ = 5, // 后
}

impl CubeFace {
    /// 获取对应的 OpenGL 常量
    pub fn to_gl_enum(&self) -> GLenum {
        match self {
            CubeFace::PositiveX => gl::TEXTURE_CUBE_MAP_POSITIVE_X,
            CubeFace::NegativeX => gl::TEXTURE_CUBE_MAP_NEGATIVE_X,
            CubeFace::PositiveY => gl::TEXTURE_CUBE_MAP_POSITIVE_Y,
            CubeFace::NegativeY => gl::TEXTURE_CUBE_MAP_NEGATIVE_Y,
            CubeFace::PositiveZ => gl::TEXTURE_CUBE_MAP_POSITIVE_Z,
            CubeFace::NegativeZ => gl::TEXTURE_CUBE_MAP_NEGATIVE_Z,
        }
    }

    /// 获取所有面的数组
    pub fn all() -> [CubeFace; 6] {
        [
            CubeFace::PositiveX,
            CubeFace::NegativeX,
            CubeFace::PositiveY,
            CubeFace::NegativeY,
            CubeFace::PositiveZ,
            CubeFace::NegativeZ,
        ]
    }
}

/// 立方体贴图
#[derive(Debug)]
pub struct TextureCubeMap {
    pub(crate) id: GLuint,
    config: TextureConfig,
    resolution: Resolution, // 每个面的分辨率（立方体贴图要求所有面尺寸相同）
}

impl TextureCubeMap {
    /// 创建空的立方体贴图
    pub fn empty(resolution: Resolution, config: TextureConfig) -> Self {
        let mut texture_id: GLuint = 0;

        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, texture_id);

            // 为六个面分配空间
            for face in CubeFace::all() {
                gl::TexImage2D(
                    face.to_gl_enum(),
                    0,
                    gl::RGBA as i32,
                    resolution.width as i32,
                    resolution.height as i32,
                    0,
                    gl::RGBA,
                    gl::UNSIGNED_BYTE,
                    std::ptr::null(),
                );
            }

            // 应用纹理配置
            Self::apply_config(config);

            gl::BindTexture(gl::TEXTURE_CUBE_MAP, 0);
        }

        Self {
            id: texture_id,
            config,
            resolution,
        }
    }

    /// 从六个文件加载立方体贴图
    /// 顺序：右、左、上、下、前、后 (+X, -X, +Y, -Y, +Z, -Z)
    pub fn from_files(paths: [&str; 6], config: TextureConfig) -> Result<Self, TextureError> {
        let mut images = Vec::with_capacity(6);

        for path in &paths {
            let img =
                image::open(path).map_err(|_| TextureError::FileReadError(path.to_string()))?;
            images.push(img);
        }

        Ok(Self::load_from_images(images.try_into().unwrap(), config))
    }

    /// 从六个字节数组加载立方体贴图
    pub fn from_bytes_array(
        data_array: [&[u8]; 6],
        config: TextureConfig,
    ) -> Result<Self, TextureError> {
        let images: Result<Vec<_>, TextureError> = data_array
            .par_iter()
            .map(|data| image::load_from_memory(data).map_err(|_| TextureError::ByteReadError))
            .collect();

        let images = images?;

        Ok(Self::load_from_images(images.try_into().unwrap(), config))
    }

    /// 更新单个面的数据
    pub fn update_face(&mut self, face: CubeFace, img: DynamicImage) -> Result<(), TextureError> {
        let (width, height) = img.dimensions();

        // 检查尺寸是否匹配
        if width != self.resolution.width || height != self.resolution.height {
            return Err(TextureError::InvalidResolution);
        }

        unsafe {
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, self.id);

            // 根据图片格式选择最佳上传方式
            match img {
                DynamicImage::ImageRgba8(rgba) => {
                    // 已经是 RGBA8，直接上传
                    gl::TexSubImage2D(
                        face.to_gl_enum(),
                        0,
                        0,
                        0,
                        width as i32,
                        height as i32,
                        gl::RGBA,
                        gl::UNSIGNED_BYTE,
                        rgba.as_ptr() as *const _,
                    );
                }
                DynamicImage::ImageRgb8(rgb) => {
                    // RGB8 直接上传为 RGB
                    gl::TexSubImage2D(
                        face.to_gl_enum(),
                        0,
                        0,
                        0,
                        width as i32,
                        height as i32,
                        gl::RGB,
                        gl::UNSIGNED_BYTE,
                        rgb.as_ptr() as *const _,
                    );
                }
                DynamicImage::ImageLuma8(gray) => {
                    // 灰度图上传为单通道
                    gl::TexSubImage2D(
                        face.to_gl_enum(),
                        0,
                        0,
                        0,
                        width as i32,
                        height as i32,
                        gl::RED,
                        gl::UNSIGNED_BYTE,
                        gray.as_ptr() as *const _,
                    );
                }
                _ => {
                    // 其他格式才转换
                    let rgba = img.to_rgba8();
                    gl::TexSubImage2D(
                        face.to_gl_enum(),
                        0,
                        0,
                        0,
                        width as i32,
                        height as i32,
                        gl::RGBA,
                        gl::UNSIGNED_BYTE,
                        rgba.as_ptr() as *const _,
                    );
                }
            }

            gl::BindTexture(gl::TEXTURE_CUBE_MAP, 0);
        }

        Ok(())
    }

    /// 从六个图像加载立方体贴图
    fn load_from_images(images: [DynamicImage; 6], config: TextureConfig) -> Self {
        let (width, height) = images[0].dimensions();

        let mut texture_id: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, texture_id);

            // 加载六个面
            for (i, img) in images.iter().enumerate() {
                let (w, h) = img.dimensions();

                // 确保所有面的尺寸相同
                assert_eq!(w, width, "all cube map tex should have equal width");
                assert_eq!(h, height, "all cube map tex should have equal height");

                let face = CubeFace::all()[i];

                match img {
                    DynamicImage::ImageRgba8(data) => {
                        gl::TexImage2D(
                            face.to_gl_enum(),
                            0,
                            gl::RGBA as i32,
                            width as i32,
                            height as i32,
                            0,
                            gl::RGBA,
                            gl::UNSIGNED_BYTE,
                            data.as_ptr() as *const _,
                        );
                    }
                    DynamicImage::ImageRgb8(data) => {
                        gl::TexImage2D(
                            face.to_gl_enum(),
                            0,
                            gl::RGB as i32,
                            width as i32,
                            height as i32,
                            0,
                            gl::RGB,
                            gl::UNSIGNED_BYTE,
                            data.as_ptr() as *const _,
                        );
                    }
                    DynamicImage::ImageRgba16(data) => {
                        gl::TexImage2D(
                            face.to_gl_enum(),
                            0,
                            gl::RGBA16 as i32,
                            width as i32,
                            height as i32,
                            0,
                            gl::RGBA,
                            gl::UNSIGNED_SHORT,
                            data.as_ptr() as *const _,
                        );
                    }
                    _ => {
                        // 不常见格式才转换
                        let rgba = img.to_rgba8();
                        gl::TexImage2D(
                            face.to_gl_enum(),
                            0,
                            gl::RGBA as i32,
                            width as i32,
                            height as i32,
                            0,
                            gl::RGBA,
                            gl::UNSIGNED_BYTE,
                            rgba.as_ptr() as *const _,
                        );
                    }
                }
            }

            // 应用纹理配置
            Self::apply_config(config);

            // 生成 mipmap（如果需要）
            gl::GenerateMipmap(gl::TEXTURE_CUBE_MAP);

            gl::BindTexture(gl::TEXTURE_CUBE_MAP, 0);
        }

        Self {
            id: texture_id,
            config,
            resolution: Resolution { width, height },
        }
    }

    /// 应用纹理配置
    fn apply_config(config: TextureConfig) {
        unsafe {
            // 设置包裹模式（立方体贴图使用 S, T, R 三个方向）
            let set_wrap = |target, mode| match mode {
                WrappingMode::Repeat => {
                    gl::TexParameteri(gl::TEXTURE_CUBE_MAP, target, gl::REPEAT as i32)
                }
                WrappingMode::MirroreroredRepeat => {
                    gl::TexParameteri(gl::TEXTURE_CUBE_MAP, target, gl::MIRRORED_REPEAT as i32)
                }
                WrappingMode::ClampToEdge => {
                    gl::TexParameteri(gl::TEXTURE_CUBE_MAP, target, gl::CLAMP_TO_EDGE as i32)
                }
                WrappingMode::ClampToBorder { color } => {
                    gl::TexParameteri(gl::TEXTURE_CUBE_MAP, target, gl::CLAMP_TO_BORDER as i32);
                    gl::TexParameterfv(
                        gl::TEXTURE_CUBE_MAP,
                        gl::TEXTURE_BORDER_COLOR,
                        color.to_arr().as_ptr(),
                    );
                }
            };

            set_wrap(gl::TEXTURE_WRAP_S, config.wrapping_s);
            set_wrap(gl::TEXTURE_WRAP_T, config.wrapping_t);
            // 立方体贴图还需要设置 R 方向
            set_wrap(gl::TEXTURE_WRAP_R, config.wrapping_s); // 通常使用相同的包裹模式

            // 设置过滤模式
            let set_filter = |target, mode: FilteringMode| {
                let val = match mode {
                    FilteringMode::Nearest => gl::NEAREST,
                    FilteringMode::Linear => gl::LINEAR,
                    FilteringMode::NearestMipmapNearest => gl::NEAREST_MIPMAP_NEAREST,
                    FilteringMode::LinearMipmapNearest => gl::LINEAR_MIPMAP_NEAREST,
                    FilteringMode::NearestMipmapLinear => gl::NEAREST_MIPMAP_LINEAR,
                    FilteringMode::LinearMipmapLinear => gl::LINEAR_MIPMAP_LINEAR,
                };
                gl::TexParameteri(gl::TEXTURE_CUBE_MAP, target, val as i32);
            };

            set_filter(gl::TEXTURE_MIN_FILTER, config.min_filter);
            set_filter(gl::TEXTURE_MAG_FILTER, config.mag_filter);
        }
    }

    /// 获取纹理 ID
    pub fn id(&self) -> GLuint {
        self.id
    }

    /// 获取分辨率
    pub fn resolution(&self) -> Resolution {
        self.resolution
    }

    /// 绑定纹理到指定的纹理单元
    pub fn bind(&self, texture_unit: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + texture_unit);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, self.id);
        }
    }

    /// 解绑纹理
    pub fn unbind() {
        unsafe {
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, 0);
        }
    }
}

impl Drop for TextureCubeMap {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
