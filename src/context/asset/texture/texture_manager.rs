use super::{Texture2D, TextureConfig, TextureCubeMap, TextureError};
use crate::{AntiPixel, Resolution};
use slotmap::{SlotMap, new_key_type};

new_key_type! {
    pub struct TextureHandle;
}

/// 纹理类型的枚举包装
#[derive(Debug)]
pub enum Texture {
    Texture2D(Texture2D),
    CubeMap(TextureCubeMap),
}

impl Texture {
    /// 获取纹理 ID（用于绑定）
    pub fn id(&self) -> gl::types::GLuint {
        match self {
            Texture::Texture2D(tex) => tex.id,
            Texture::CubeMap(tex) => tex.id(),
        }
    }

    /// 尝试获取 Texture2D 的引用
    pub fn as_texture_2d(&self) -> Option<&Texture2D> {
        match self {
            Texture::Texture2D(tex) => Some(tex),
            _ => None,
        }
    }

    /// 尝试获取 Texture2D 的可变引用
    pub fn as_texture_2d_mut(&mut self) -> Option<&mut Texture2D> {
        match self {
            Texture::Texture2D(tex) => Some(tex),
            _ => None,
        }
    }

    /// 尝试获取 TextureCubeMap 的引用
    pub fn as_cube_map(&self) -> Option<&TextureCubeMap> {
        match self {
            Texture::CubeMap(tex) => Some(tex),
            _ => None,
        }
    }

    /// 尝试获取 TextureCubeMap 的可变引用
    pub fn as_cube_map_mut(&mut self) -> Option<&mut TextureCubeMap> {
        match self {
            Texture::CubeMap(tex) => Some(tex),
            _ => None,
        }
    }
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
    pub fn get_texture_2d(&self, handle: TextureHandle) -> Option<&Texture2D> {
        self.textures.get(handle)?.as_texture_2d()
    }

    /// 获取可变 Texture2D（类型安全）
    pub fn get_texture_2d_mut(&mut self, handle: TextureHandle) -> Option<&mut Texture2D> {
        self.textures.get_mut(handle)?.as_texture_2d_mut()
    }

    /// 获取 TextureCubeMap（类型安全）
    pub fn get_cube_map(&self, handle: TextureHandle) -> Option<&TextureCubeMap> {
        self.textures.get(handle)?.as_cube_map()
    }

    /// 获取可变 TextureCubeMap（类型安全）
    pub fn get_cube_map_mut(&mut self, handle: TextureHandle) -> Option<&mut TextureCubeMap> {
        self.textures.get_mut(handle)?.as_cube_map_mut()
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
        let texture = Texture2D::empty_multi_sample(resolution, anti_pixel);
        Ok(self.textures.insert(Texture::Texture2D(texture)))
    }

    /// 创建空的 2D 纹理
    pub fn create_empty(
        &mut self,
        resolution: Resolution,
        config: TextureConfig,
    ) -> Result<TextureHandle, TextureError> {
        let texture = Texture2D::empty(resolution, config);
        Ok(self.textures.insert(Texture::Texture2D(texture)))
    }

    /// 从文件创建 2D 纹理
    pub fn create_from_file(
        &mut self,
        path: &str,
        config: TextureConfig,
    ) -> Result<TextureHandle, TextureError> {
        let texture = Texture2D::from_file(path, config)?;
        Ok(self.textures.insert(Texture::Texture2D(texture)))
    }

    /// 从字节数据创建 2D 纹理
    pub fn create_from_bytes(
        &mut self,
        data: &[u8],
        config: TextureConfig,
    ) -> Result<TextureHandle, TextureError> {
        let texture = Texture2D::from_bytes(data, config)?;
        Ok(self.textures.insert(Texture::Texture2D(texture)))
    }

    /// 调整 2D 纹理大小
    pub fn resize_2d(
        &mut self,
        handle: TextureHandle,
        new_resolution: Resolution,
    ) -> Result<(), TextureError> {
        let texture = self
            .get_texture_2d_mut(handle)
            .ok_or(TextureError::InvalidHandle)?;
        texture.resize(new_resolution);
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
        let texture = TextureCubeMap::empty(resolution, config);
        Ok(self.textures.insert(Texture::CubeMap(texture)))
    }

    /// 从六个文件创建立方体贴图
    pub fn create_cube_map_from_files(
        &mut self,
        paths: [&str; 6],
        config: TextureConfig,
    ) -> Result<TextureHandle, TextureError> {
        let texture = TextureCubeMap::from_files(paths, config)?;
        Ok(self.textures.insert(Texture::CubeMap(texture)))
    }

    /// 从六个字节数组创建立方体贴图
    pub fn create_cube_map_from_bytes(
        &mut self,
        data_array: [&[u8]; 6],
        config: TextureConfig,
    ) -> Result<TextureHandle, TextureError> {
        let texture = TextureCubeMap::from_bytes_array(data_array, config)?;
        Ok(self.textures.insert(Texture::CubeMap(texture)))
    }

    /// 更新立方体贴图的某个面
    pub fn update_cube_map_face(
        &mut self,
        handle: TextureHandle,
        face: super::CubeFace,
        img: image::DynamicImage,
    ) -> Result<(), TextureError> {
        let texture = self
            .get_cube_map_mut(handle)
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
