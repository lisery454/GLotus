use crate::Resolution;

use super::FramebufferError;

pub struct Framebuffer {
    pub(crate) fbo_id: u32,              // 普通 FBO (用于存储最终纹理)
    pub(crate) msaa_fbo_id: Option<u32>, // 多重采样 FBO (可选)
    pub(crate) rbo_id: u32,              // 深度模板缓冲
    pub(crate) resolution: Resolution,
    pub(crate) samples: u32,
}

impl Framebuffer {
    /// 创建普通纹理的 Framebuffer
    pub fn new(resolution: Resolution, tex_id: u32) -> Result<Self, FramebufferError> {
        Self::create_internal(resolution, tex_id, None)
    }

    /// 创建多重采样的 Framebuffer
    /// `msaa_tex_id`: 多重采样纹理的 ID
    /// `resolve_tex_id`: 最终输出的普通纹理 ID (用于解析 MSAA)
    pub fn new_multi_sample(
        resolution: Resolution,
        msaa_tex_id: u32,
        resolve_tex_id: u32,
        samples: u32,
    ) -> Result<Self, FramebufferError> {
        Self::create_internal(resolution, resolve_tex_id, Some((msaa_tex_id, samples)))
    }

    fn create_internal(
        resolution: Resolution,
        resolve_tex_id: u32,
        msaa_config: Option<(u32, u32)>,
    ) -> Result<Self, FramebufferError> {
        unsafe {
            // 创建解析用的普通 FBO
            let mut fbo_id = 0;
            gl::GenFramebuffers(1, &mut fbo_id);
            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo_id);
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                resolve_tex_id,
                0,
            );

            let mut msaa_fbo_id = None;
            let mut rbo_id = 0;
            let mut samples = 1;

            // 如果有 MSAA 配置，创建第二个渲染用的 FBO
            if let Some((msaa_tex_id, s)) = msaa_config {
                samples = s;
                let mut mfbo = 0;
                gl::GenFramebuffers(1, &mut mfbo);
                gl::BindFramebuffer(gl::FRAMEBUFFER, mfbo);

                // 附加多重采样纹理
                gl::FramebufferTexture2D(
                    gl::FRAMEBUFFER,
                    gl::COLOR_ATTACHMENT0,
                    gl::TEXTURE_2D_MULTISAMPLE,
                    msaa_tex_id,
                    0,
                );

                // 附加多重采样 RBO
                gl::GenRenderbuffers(1, &mut rbo_id);
                gl::BindRenderbuffer(gl::RENDERBUFFER, rbo_id);
                gl::RenderbufferStorageMultisample(
                    gl::RENDERBUFFER,
                    samples as i32,
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

                msaa_fbo_id = Some(mfbo);
            } else {
                // 普通模式：直接在主 FBO 上附加 RBO
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
            }

            // 检查当前绑定的 FBO 是否完整
            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                return Err(FramebufferError::IncompleteFramebuffer);
            }

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

            Ok(Self {
                fbo_id,
                msaa_fbo_id,
                rbo_id,
                resolution,
                samples,
            })
        }
    }

    /// 绑定用于“写入”的 FBO
    pub fn bind(&self) {
        unsafe {
            // 如果有 MSAA FBO，优先绑定它，否则绑定普通 FBO
            let bind_id = self.msaa_fbo_id.unwrap_or(self.fbo_id);
            gl::BindFramebuffer(gl::FRAMEBUFFER, bind_id);
            gl::Viewport(
                0,
                0,
                self.resolution.width as i32,
                self.resolution.height as i32,
            );
        }
    }

    /// 解绑，并如果是 MSAA 模式，自动执行 Resolve
    pub fn unbind(&self) {
        unsafe {
            if let Some(msaa_id) = self.msaa_fbo_id {
                // 将数据从多重采样 FBO 拷贝到普通 FBO
                gl::BindFramebuffer(gl::READ_FRAMEBUFFER, msaa_id);
                gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, self.fbo_id);
                gl::BlitFramebuffer(
                    0,
                    0,
                    self.resolution.width as i32,
                    self.resolution.height as i32,
                    0,
                    0,
                    self.resolution.width as i32,
                    self.resolution.height as i32,
                    gl::COLOR_BUFFER_BIT,
                    gl::NEAREST,
                );
            }
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.fbo_id);
            if let Some(msaa_id) = self.msaa_fbo_id {
                gl::DeleteFramebuffers(1, &msaa_id);
            }
            gl::DeleteRenderbuffers(1, &self.rbo_id);
        }
    }
}
