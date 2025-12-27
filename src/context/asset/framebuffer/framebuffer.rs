use std::cell::Ref;

use crate::{TextureHandle, TextureManager};

use super::FramebufferError;

pub struct Framebuffer {
    pub(crate) fbo_id: u32,
    pub texture: TextureHandle,
    pub(crate) rbo_id: u32,
    pub width: u32,
    pub height: u32,
}

impl Framebuffer {
    pub(crate) fn new(
        width: u32,
        height: u32,
        texture_handle: TextureHandle,
        texture_manager: Ref<TextureManager>,
    ) -> Result<Self, FramebufferError> {
        let mut fbo = 0;
        let mut rbo = 0;

        unsafe {
            // 创建颜色framebuffer
            gl::GenFramebuffers(1, &mut fbo);
            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);

            // 获取纹理的OpenGL ID并附加到framebuffer
            if let Some(texture) = texture_manager.get(texture_handle) {
                gl::FramebufferTexture2D(
                    gl::FRAMEBUFFER,
                    gl::COLOR_ATTACHMENT0,
                    gl::TEXTURE_2D,
                    texture.id,
                    0,
                );
            } else {
                return Err(FramebufferError::CreationFailed(
                    "Failed to get texture".to_string(),
                ));
            }

            // 创建深度模板renderbuffer
            gl::GenRenderbuffers(1, &mut rbo);
            gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
            gl::RenderbufferStorage(
                gl::RENDERBUFFER,
                gl::DEPTH24_STENCIL8,
                width as i32,
                height as i32,
            );
            gl::FramebufferRenderbuffer(
                gl::FRAMEBUFFER,
                gl::DEPTH_STENCIL_ATTACHMENT,
                gl::RENDERBUFFER,
                rbo,
            );

            // 检查framebuffer完整性
            let status = gl::CheckFramebufferStatus(gl::FRAMEBUFFER);
            if status != gl::FRAMEBUFFER_COMPLETE {
                gl::DeleteFramebuffers(1, &fbo);
                gl::DeleteRenderbuffers(1, &rbo);
                return Err(FramebufferError::IncompleteFramebuffer);
            }

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }

        Ok(Self {
            fbo_id: fbo,
            texture: texture_handle,
            rbo_id: rbo,
            width,
            height,
        })
    }

    pub(crate) fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo_id);
            gl::Viewport(0, 0, self.width as i32, self.height as i32);
        }
    }

    pub(crate) fn unbind() {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.fbo_id);
            gl::DeleteRenderbuffers(1, &self.rbo_id);
        }
    }
}
