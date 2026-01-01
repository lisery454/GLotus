use std::{cell::RefCell, rc::Weak};

use slotmap::{SecondaryMap, SlotMap, new_key_type};

use crate::{AntiPixel, Resolution, TextureConfig, TextureHandle, TextureManager};

use super::{Framebuffer, FramebufferError};

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
            .map(|t| t.id)
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
            .map(|t| t.id)
            .ok_or(FramebufferError::NotFoundTexture)?;
        let mass_tex_id = texture_manager
            .get(mass_texture_handle)
            .map(|t| t.id)
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
