mod texture;
mod texture_config;
mod texture_error;
mod texture_mode;

pub use texture::*;
pub use texture_config::*;
pub use texture_error::TextureError;
pub use texture_mode::FilteringMode;
pub use texture_mode::WrappingMode;

use crate::{AntiPixel, Resolution};
use slotmap::{SlotMap, new_key_type};

new_key_type! {
    pub struct TextureHandle;
}

pub struct TextureManager {
    textures: SlotMap<TextureHandle, Texture>,
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            textures: SlotMap::with_key(),
        }
    }

    /// 获取纹理（任意类型）
    pub fn get(&self, handle: TextureHandle) -> Option<&Texture> {
        self.textures.get(handle)
    }

    /// 获取可变纹理（任意类型）
    pub fn get_mut(&mut self, handle: TextureHandle) -> Option<&mut Texture> {
        self.textures.get_mut(handle)
    }

    /// 获取 Texture2D（类型安全）
    pub fn get_texture(&self, handle: TextureHandle) -> Option<&Texture> {
        self.textures.get(handle)
    }

    /// 获取可变 Texture2D（类型安全）
    pub fn get_texture_mut(&mut self, handle: TextureHandle) -> Option<&mut Texture> {
        self.textures.get_mut(handle)
    }

    /// 删除纹理
    pub fn remove(&mut self, handle: TextureHandle) {
        self.textures.remove(handle);
    }
}

// ============ Texture2D 相关方法 ============
impl TextureManager {
    /// 创建空的多重采样 2D 纹理
    pub fn create_empty_multi_sample(
        &mut self,
        resolution: Resolution,
        anti_pixel: AntiPixel,
    ) -> Result<TextureHandle, TextureError> {
        let texture = Texture::empty(resolution, TextureConfig::MultiSample { anti_pixel });
        Ok(self.textures.insert(texture))
    }

    /// 创建空的 2D 纹理
    pub fn create_empty(
        &mut self,
        resolution: Resolution,
        config: TextureConfig,
    ) -> Result<TextureHandle, TextureError> {
        let texture = Texture::empty(resolution, config);
        Ok(self.textures.insert(texture))
    }

    /// 从文件创建 2D 纹理
    pub fn create_from_file(
        &mut self,
        path: &str,
        config: TextureConfig,
    ) -> Result<TextureHandle, TextureError> {
        let texture = Texture::from_file(path, config)?;
        Ok(self.textures.insert(texture))
    }

    /// 从字节数据创建 2D 纹理
    pub fn create_from_bytes(
        &mut self,
        data: &[u8],
        config: TextureConfig,
    ) -> Result<TextureHandle, TextureError> {
        let texture = Texture::from_bytes(data, config)?;
        Ok(self.textures.insert(texture))
    }

    /// 调整 2D 纹理大小
    pub fn resize_2d(
        &mut self,
        handle: TextureHandle,
        new_resolution: Resolution,
    ) -> Result<(), TextureError> {
        let texture = self
            .get_texture_mut(handle)
            .ok_or(TextureError::InvalidHandle)?;
        texture.resize(new_resolution)?;
        Ok(())
    }
}

// ============ CubeMap 相关方法 ============
impl TextureManager {
    /// 创建空的立方体贴图
    pub fn create_empty_cube_map(
        &mut self,
        resolution: Resolution,
        config: TextureConfig,
    ) -> Result<TextureHandle, TextureError> {
        let texture = Texture::empty(resolution, config);
        Ok(self.textures.insert(texture))
    }

    /// 从六个文件创建立方体贴图
    pub fn create_cube_map_from_files(
        &mut self,
        paths: [&str; 6],
        config: TextureConfig,
    ) -> Result<TextureHandle, TextureError> {
        let texture = Texture::from_files(paths, config)?;
        Ok(self.textures.insert(texture))
    }

    /// 从六个字节数组创建立方体贴图
    pub fn create_cube_map_from_bytes(
        &mut self,
        data_array: [&[u8]; 6],
        config: TextureConfig,
    ) -> Result<TextureHandle, TextureError> {
        let texture = Texture::from_bytes_array(data_array, config)?;
        Ok(self.textures.insert(texture))
    }

    /// 更新立方体贴图的某个面
    pub fn update_cube_map_face(
        &mut self,
        handle: TextureHandle,
        face: CubeFace,
        img: image::DynamicImage,
    ) -> Result<(), TextureError> {
        let texture = self
            .get_texture_mut(handle)
            .ok_or(TextureError::InvalidHandle)?;
        texture.update_face(face, img)?;
        Ok(())
    }
}

impl Default for TextureManager {
    fn default() -> Self {
        Self::new()
    }
}
