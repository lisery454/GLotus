use std::{cell::RefCell, rc::Weak};

use slotmap::{SecondaryMap, SlotMap, new_key_type};

use crate::{Resolution, TextureConfig, TextureHandle, TextureManager};

use super::{Framebuffer, FramebufferError};

new_key_type! {
    pub struct FramebufferHandle;
}

pub struct FramebufferManager {
    framebuffers: SlotMap<FramebufferHandle, Framebuffer>,
    textures: SecondaryMap<FramebufferHandle, TextureHandle>,
    texture_manager: Weak<RefCell<TextureManager>>,
}

impl FramebufferManager {
    pub fn new(texture_manager: Weak<RefCell<TextureManager>>) -> Self {
        Self {
            framebuffers: SlotMap::with_key(),
            textures: SecondaryMap::new(),
            texture_manager,
        }
    }

    /// 创建一个新的framebuffer
    pub fn create(
        &mut self,
        resolution: Resolution,
        config: TextureConfig,
    ) -> Result<FramebufferHandle, FramebufferError> {
        let texture_manager = self
            .texture_manager
            .upgrade()
            .ok_or(FramebufferError::TextureManagerBorrowFail)?;
        let mut texture_manager = texture_manager.borrow_mut();

        let texture_handle =
            texture_manager.create_empty(resolution, config)?;

        let tex_id = texture_manager.get(texture_handle).map(|t| t.id).ok_or(
            FramebufferError::CreationFailed("Texture ID missing".into()),
        )?;
        let framebuffer = Framebuffer::new(resolution, tex_id)?;

        let framebuffer_handle = self.framebuffers.insert(framebuffer);
        self.textures.insert(framebuffer_handle, texture_handle);

        Ok(framebuffer_handle)
    }

    pub fn remove(&mut self, handle: FramebufferHandle) -> Result<(), FramebufferError> {
        let Some(_) = self.framebuffers.remove(handle) else {
            return Err(FramebufferError::InvalidHandle);
        };
        let Some(texture_manager) = self.texture_manager.upgrade() else {
            return Err(FramebufferError::TextureManagerBorrowFail);
        };
        let Some(texture_handle) = self.textures.remove(handle) else {
            return Err(FramebufferError::InvalidHandle);
        };

        texture_manager.borrow_mut().remove(texture_handle);
        Ok(())
    }

    /// 绑定framebuffer进行渲染
    pub(crate) fn bind(&self, handle: FramebufferHandle) -> Result<(), FramebufferError> {
        let Some(fb) = self.framebuffers.get(handle) else {
            return Err(FramebufferError::InvalidHandle);
        };

        fb.bind();
        Ok(())
    }

    /// 解绑framebuffer，恢复到默认framebuffer
    pub(crate) fn unbind(&self, handle: FramebufferHandle) -> Result<(), FramebufferError> {
        let Some(fb) = self.framebuffers.get(handle) else {
            return Err(FramebufferError::InvalidHandle);
        };

        fb.unbind();
        Ok(())
    }

    pub fn get_size(&self, handle: FramebufferHandle) -> Result<Resolution, FramebufferError> {
        let fb = self
            .framebuffers
            .get(handle)
            .ok_or(FramebufferError::InvalidHandle)?;

        Ok(fb.resolution)
    }

    pub fn get_color_texture(
        &self,
        handle: FramebufferHandle,
    ) -> Result<TextureHandle, FramebufferError> {
        let Some(texture_handle) = self.textures.get(handle) else {
            return Err(FramebufferError::InvalidHandle);
        };

        Ok(texture_handle.clone())
    }
}
