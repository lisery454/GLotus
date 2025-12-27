use std::{cell::RefCell, rc::Weak};

use slotmap::{SlotMap, new_key_type};

use crate::{TextureHandle, TextureManager};

use super::{Framebuffer, FramebufferError};

new_key_type! {
    pub struct FramebufferHandle;
}

pub struct FramebufferManager {
    framebuffers: SlotMap<FramebufferHandle, Framebuffer>,
    texture_manager: Weak<RefCell<TextureManager>>,
}

impl FramebufferManager {
    pub fn new(texture_manager: Weak<RefCell<TextureManager>>) -> Self {
        Self {
            framebuffers: SlotMap::with_key(),
            texture_manager,
        }
    }

    pub(crate) fn get(&self, handle: FramebufferHandle) -> Option<&Framebuffer> {
        self.framebuffers.get(handle)
    }

    pub(crate) fn get_mut(&mut self, handle: FramebufferHandle) -> Option<&mut Framebuffer> {
        self.framebuffers.get_mut(handle)
    }

    /// 创建一个新的framebuffer
    pub fn create(
        &mut self,
        width: u32,
        height: u32,
        texture_handle: TextureHandle,
    ) -> Result<FramebufferHandle, FramebufferError> {
        let texture_manager = self
            .texture_manager
            .upgrade()
            .ok_or(FramebufferError::TextureManagerBorrowFail)?;
        let framebuffer =
            Framebuffer::new(width, height, texture_handle, texture_manager.borrow())?;
        Ok(self.framebuffers.insert(framebuffer))
    }

    /// 删除framebuffer（注意：关联的纹理需要手动从TextureManager删除）
    pub fn remove(&mut self, handle: FramebufferHandle) -> Option<TextureHandle> {
        self.framebuffers.remove(handle).map(|fb| fb.texture)
    }

    /// 绑定framebuffer进行渲染
    pub(crate) fn bind(&self, handle: FramebufferHandle) -> Result<(), FramebufferError> {
        if let Some(fb) = self.framebuffers.get(handle) {
            fb.bind();
            Ok(())
        } else {
            Err(FramebufferError::InvalidHandle)
        }
    }

    /// 解绑framebuffer，恢复到默认framebuffer
    pub(crate) fn unbind() {
        Framebuffer::unbind();
    }

    /// 调整framebuffer大小
    pub fn resize(
        &mut self,
        handle: FramebufferHandle,
        width: u32,
        height: u32,
        new_texture_handle: TextureHandle,
    ) -> Result<(), FramebufferError> {
        let texture_manager = self
            .texture_manager
            .upgrade()
            .ok_or(FramebufferError::TextureManagerBorrowFail)?;
        if let Some(old_fb) = self.framebuffers.remove(handle) {
            // 删除旧纹理
            texture_manager.borrow_mut().remove(old_fb.texture);

            // 创建新的framebuffer
            let new_fb =
                Framebuffer::new(width, height, new_texture_handle, texture_manager.borrow())?;
            self.framebuffers.insert_with_key(|_| new_fb);
            Ok(())
        } else {
            Err(FramebufferError::InvalidHandle)
        }
    }

    /// 获取framebuffer的颜色附件纹理句柄
    pub fn get_color_texture(&self, handle: FramebufferHandle) -> Option<TextureHandle> {
        self.framebuffers.get(handle).map(|fb| fb.texture)
    }
}
