use gl::types::GLuint;
use image::DynamicImage;
use slotmap::{SlotMap, new_key_type};

use super::{FilteringMode, Texture2D, TextureError, WrappingMode, texture2d::TextureConfig};

new_key_type! {
    pub struct TextureHandle;
}

pub struct TextureManager {
    textures: SlotMap<TextureHandle, Texture2D>,
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            textures: SlotMap::with_key(),
        }
    }
    pub(crate) fn get(&self, handle: TextureHandle) -> Option<&Texture2D> {
        self.textures.get(handle)
    }

    pub(crate) fn get_mut(&mut self, handle: TextureHandle) -> Option<&mut Texture2D> {
        self.textures.get_mut(handle)
    }
}

// create
impl TextureManager {
    pub fn create_empty(
        &mut self,
        width: u32,
        height: u32,
        config: TextureConfig,
    ) -> Result<TextureHandle, TextureError> {
        let id = self.alloc_and_setup(width, height, None, config)?;
        Ok(self.textures.insert(Texture2D { id, width, height }))
    }

    pub fn create_from_file(
        &mut self,
        path: &str,
        config: TextureConfig,
    ) -> Result<TextureHandle, TextureError> {
        let img = image::open(path).map_err(|_| TextureError::FileReadError(path.to_string()))?;
        let (id, w, h) = self.load_image_to_gpu(img, config)?;
        Ok(self.textures.insert(Texture2D {
            id,
            width: w,
            height: h,
        }))
    }

    fn load_image_to_gpu(
        &self,
        img: DynamicImage,
        config: TextureConfig,
    ) -> Result<(GLuint, u32, u32), TextureError> {
        let img = img.flipv();
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();

        let id = self.alloc_and_setup(width, height, Some(rgba.as_ptr()), config)?;

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Ok((id, width, height))
    }

    fn alloc_and_setup(
        &self,
        width: u32,
        height: u32,
        data_ptr: Option<*const u8>,
        config: TextureConfig,
    ) -> Result<GLuint, TextureError> {
        let mut texture_id: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            // 统一设置参数
            self.apply_config(&config);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data_ptr.map_or(std::ptr::null(), |p| p as *const _),
            );

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        Ok(texture_id)
    }

    fn apply_config(&self, config: &TextureConfig) {
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

    pub fn create_from_byte(
        &mut self,
        data: &[u8],
        config: TextureConfig,
    ) -> Result<TextureHandle, TextureError> {
        // 1. 使用 image 库从内存加载图片
        let img = image::load_from_memory(data).map_err(|_| TextureError::ByteReadError)?;

        // 2. 调用内部通用的加载逻辑
        let (id, w, h) = self.load_image_to_gpu(img, config)?;

        // 3. 存入 SlotMap
        Ok(self.textures.insert(Texture2D {
            id,
            width: w,
            height: h,
        }))
    }
}

// resize
impl TextureManager {
    pub fn resize(
        &mut self,
        handle: TextureHandle,
        width: u32,
        height: u32,
    ) -> Result<(), TextureError> {
        let tex = self
            .textures
            .get_mut(handle)
            .ok_or(TextureError::InvalidHandle)?;

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, tex.id);
            // 重新定义纹理大小
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                std::ptr::null(), // 不上传新数据，只改变尺寸
            );
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        // 更新数据结构中的尺寸信息
        tex.width = width;
        tex.height = height;

        Ok(())
    }
}

// remove
impl TextureManager {
    pub fn remove(&mut self, handle: TextureHandle) {
        if let Some(tex) = self.textures.remove(handle) {
            unsafe {
                gl::DeleteTextures(1, &tex.id);
            }
        }
    }
}

impl Drop for TextureManager {
    fn drop(&mut self) {
        unsafe {
            for (_, tex) in self.textures.drain() {
                gl::DeleteTextures(1, &tex.id);
            }
        }
    }
}
