use super::{Texture2D, TextureError, texture2d::TextureConfig};
use crate::Resolution;
use slotmap::{SlotMap, new_key_type};

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
        resolution: Resolution,
        config: TextureConfig,
    ) -> Result<TextureHandle, TextureError> {
        let texture = Texture2D::empty(resolution, config);
        Ok(self.textures.insert(texture))
    }

    pub fn create_from_file(
        &mut self,
        path: &str,
        config: TextureConfig,
    ) -> Result<TextureHandle, TextureError> {
        let texture = Texture2D::from_file(path, config)?;
        Ok(self.textures.insert(texture))
    }

    pub fn create_from_byte(
        &mut self,
        data: &[u8],
        config: TextureConfig,
    ) -> Result<TextureHandle, TextureError> {
        let texture = Texture2D::from_bytes(data, config)?;
        Ok(self.textures.insert(texture))
    }

    pub fn remove(&mut self, handle: TextureHandle) {
        self.textures.remove(handle);
    }
}
