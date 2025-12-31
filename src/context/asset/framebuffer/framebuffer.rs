use crate::Resolution;

use super::FramebufferError;

pub struct Framebuffer {
    pub(crate) fbo_id: u32,
    pub(crate) rbo_id: u32,
    pub(crate) resolution: Resolution,
}

impl Framebuffer {
    pub fn new(resolution: Resolution, tex_id: u32) -> Result<Self, FramebufferError> {
        unsafe {
            let mut fbo_id = 0;
            let mut rbo_id = 0;

            gl::GenFramebuffers(1, &mut fbo_id);
            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo_id);

            // 附加颜色纹理
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                tex_id,
                0,
            );

            // 附加深度模板 RBO
            gl::GenRenderbuffers(1, &mut rbo_id);
            gl::BindRenderbuffer(gl::RENDERBUFFER, rbo_id);
            gl::RenderbufferStorage(
                gl::RENDERBUFFER,
                gl::DEPTH24_STENCIL8,
                resolution.width as i32,
                resolution.height as i32,
            );
            gl::FramebufferRenderbuffer(
                gl::FRAMEBUFFER,
                gl::DEPTH_STENCIL_ATTACHMENT,
                gl::RENDERBUFFER,
                rbo_id,
            );

            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                gl::DeleteFramebuffers(1, &fbo_id);
                gl::DeleteRenderbuffers(1, &rbo_id);
                return Err(FramebufferError::IncompleteFramebuffer);
            }

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

            Ok(Self {
                fbo_id,
                rbo_id,
                resolution,
            })
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo_id);
            gl::Viewport(
                0,
                0,
                self.resolution.width as i32,
                self.resolution.height as i32,
            );
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0); // binding to screen
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
