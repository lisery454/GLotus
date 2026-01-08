mod framebuffer;
mod framebuffer_error;

pub use framebuffer::*;
pub use framebuffer_error::*;

use std::{cell::RefCell, rc::Weak};

use slotmap::{SecondaryMap, SlotMap, new_key_type};

use crate::{AntiPixel, Resolution, TextureConfig, TextureHandle, TextureManager};


new_key_type! {
    pub struct FramebufferHandle;
}

pub struct FramebufferManager {
    framebuffers: SlotMap<FramebufferHandle, Framebuffer>,
    fbo_textures: SecondaryMap<FramebufferHandle, TextureHandle>,
    msaa_fbo_textures: SecondaryMap<FramebufferHandle, TextureHandle>,
    texture_manager: Weak<RefCell<TextureManager>>,
}

impl FramebufferManager {
    pub fn new(texture_manager: Weak<RefCell<TextureManager>>) -> Self {
        Self {
            framebuffers: SlotMap::with_key(),
            fbo_textures: SecondaryMap::new(),
            msaa_fbo_textures: SecondaryMap::new(),
            texture_manager,
        }
    }

    pub fn create(
        &mut self,
        resolution: Resolution,
        config: TextureConfig,
    ) -> Result<FramebufferHandle, FramebufferError> {
        // get tex mgr
        let texture_manager = self
            .texture_manager
            .upgrade()
            .ok_or(FramebufferError::TextureManagerBorrowFail)?;
        let mut texture_manager = texture_manager.borrow_mut();

        // create new tex
        let texture_handle = texture_manager.create_empty(resolution, config)?;

        // get tex id
        let tex_id = texture_manager
            .get(texture_handle)
            .map(|t| t.id())
            .ok_or(FramebufferError::NotFoundTexture)?;

        // create fb
        let framebuffer = Framebuffer::new(resolution, tex_id)?;

        // insert slotmap
        let framebuffer_handle = self.framebuffers.insert(framebuffer);
        self.fbo_textures.insert(framebuffer_handle, texture_handle);

        Ok(framebuffer_handle)
    }

    pub fn create_multi_sample(
        &mut self,
        resolution: Resolution,
        anti_pixel: AntiPixel,
        config: TextureConfig,
    ) -> Result<FramebufferHandle, FramebufferError> {
        // get tex mgr
        let texture_manager = self
            .texture_manager
            .upgrade()
            .ok_or(FramebufferError::TextureManagerBorrowFail)?;
        let mut texture_manager = texture_manager.borrow_mut();

        // create new tex
        let texture_handle = texture_manager.create_empty(resolution, config)?;
        let mass_texture_handle =
            texture_manager.create_empty_multi_sample(resolution, anti_pixel)?;

        // get tex id
        let tex_id = texture_manager
            .get(texture_handle)
            .map(|t| t.id())
            .ok_or(FramebufferError::NotFoundTexture)?;
        let mass_tex_id = texture_manager
            .get(mass_texture_handle)
            .map(|t| t.id())
            .ok_or(FramebufferError::NotFoundTexture)?;

        // create fb
        let framebuffer =
            Framebuffer::new_multi_sample(resolution, mass_tex_id, tex_id, anti_pixel.samples())?;

        // insert slotmap
        let framebuffer_handle = self.framebuffers.insert(framebuffer);
        self.fbo_textures.insert(framebuffer_handle, texture_handle);
        self.msaa_fbo_textures
            .insert(framebuffer_handle, mass_texture_handle);

        Ok(framebuffer_handle)
    }

    pub fn remove(&mut self, handle: FramebufferHandle) -> Result<(), FramebufferError> {
        let Some(_) = self.framebuffers.remove(handle) else {
            return Err(FramebufferError::InvalidHandle);
        };
        let Some(texture_manager) = self.texture_manager.upgrade() else {
            return Err(FramebufferError::TextureManagerBorrowFail);
        };
        if let Some(texture_handle) = self.fbo_textures.remove(handle) {
            texture_manager.borrow_mut().remove(texture_handle);
        };
        if let Some(msaa_texture_handle) = self.msaa_fbo_textures.remove(handle) {
            texture_manager.borrow_mut().remove(msaa_texture_handle);
        };

        Ok(())
    }

    pub(crate) fn bind(&self, handle: FramebufferHandle) -> Result<(), FramebufferError> {
        let Some(fb) = self.framebuffers.get(handle) else {
            return Err(FramebufferError::InvalidHandle);
        };

        fb.bind();
        Ok(())
    }

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

    pub fn resize(
        &mut self,
        handle: FramebufferHandle,
        new_resolution: Resolution,
    ) -> Result<(), FramebufferError> {
        // 1. 获取 TextureManager 的可变借用
        let tm_rc = self
            .texture_manager
            .upgrade()
            .ok_or(FramebufferError::TextureManagerBorrowFail)?;
        let mut tm = tm_rc.borrow_mut();

        // 2. 更新关联的所有纹理尺寸
        // 处理普通颜色纹理
        let color_tex_handle = self
            .fbo_textures
            .get(handle)
            .ok_or(FramebufferError::NotFoundTexture)?;
        tm.resize_2d(*color_tex_handle, new_resolution)?;
        let color_tex_id = tm
            .get(*color_tex_handle)
            .map(|t| t.id())
            .ok_or(FramebufferError::NotFoundTexture)?;

        // 处理 MSAA 纹理（如果有）
        let mut msaa_config = None;
        if let Some(msaa_tex_handle) = self.msaa_fbo_textures.get(handle) {
            tm.resize_2d(*msaa_tex_handle, new_resolution)?;
            let msaa_tex = tm
                .get(*msaa_tex_handle)
                .ok_or(FramebufferError::NotFoundTexture)?;

            // 注意：这里需要知道之前的采样率。
            // 如果你的 Framebuffer 结构体没存 samples，建议在此处获取（或从配置获取）
            let samples = 4; // 示例：建议通过 fb.samples 获取
            msaa_config = Some((msaa_tex.id(), samples));
        }

        // 3. 创建新的 Framebuffer 对象
        let new_fb = if let Some((msaa_id, samples)) = msaa_config {
            Framebuffer::new_multi_sample(new_resolution, msaa_id, color_tex_id, samples)?
        } else {
            Framebuffer::new(new_resolution, color_tex_id)?
        };

        // 4. 【核心点】更新 SlotMap 中的值并触发旧对象的 Drop
        if let Some(fb_entry) = self.framebuffers.get_mut(handle) {
            // 解引用赋值：这会销毁旧的 fb_entry（调用其 Drop 释放 OpenGL 资源）
            // 并将 new_fb 的内容移动进去
            *fb_entry = new_fb;
        } else {
            return Err(FramebufferError::InvalidHandle);
        }

        Ok(())
    }

    pub fn get_color_texture(
        &self,
        handle: FramebufferHandle,
    ) -> Result<TextureHandle, FramebufferError> {
        let Some(texture_handle) = self.fbo_textures.get(handle) else {
            return Err(FramebufferError::InvalidHandle);
        };

        Ok(texture_handle.clone())
    }
}
