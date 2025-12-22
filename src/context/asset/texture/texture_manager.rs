use slotmap::{SlotMap, new_key_type};

use super::{FilteringMode, Texture2D, TextureError, WrappingMode};

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

    pub fn create_from_file(
        &mut self,
        path: &str,
        wrapping_mode_s: WrappingMode,
        wrapping_mode_t: WrappingMode,
        filtering_mode_min: FilteringMode,
        filtering_mode_mag: FilteringMode,
    ) -> Result<TextureHandle, TextureError> {
        let texture = Texture2D::from_file(
            path,
            wrapping_mode_s,
            wrapping_mode_t,
            filtering_mode_min,
            filtering_mode_mag,
        )?;
        Ok(self.textures.insert(texture))
    }

    pub fn create_from_byte(
        &mut self,
        data: &[u8],
        wrapping_mode_s: WrappingMode,
        wrapping_mode_t: WrappingMode,
        filtering_mode_min: FilteringMode,
        filtering_mode_mag: FilteringMode,
    ) -> Result<TextureHandle, TextureError> {
        let texture = Texture2D::from_byte(
            data,
            wrapping_mode_s,
            wrapping_mode_t,
            filtering_mode_min,
            filtering_mode_mag,
        )?;
        Ok(self.textures.insert(texture))
    }

    pub fn remove(&mut self, handle: TextureHandle) {
        self.textures.remove(handle);
    }
}
