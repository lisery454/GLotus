use std::{cell::RefCell, rc::Weak};

use slotmap::{SlotMap, new_key_type};

use crate::{TextureConfig, TextureHandle, TextureManager};

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
        config: TextureConfig,
    ) -> Result<FramebufferHandle, FramebufferError> {
        // 1. 升级并借用 TextureManager
        let texture_manager_rc = self
            .texture_manager
            .upgrade()
            .ok_or(FramebufferError::TextureManagerBorrowFail)?;
        let mut texture_manager = texture_manager_rc.borrow_mut();

        // 2. 内部创建一个专门用于 FBO 的纹理
        // 这里的 create_empty 需要你在 TextureManager 中实现，它应该调用 gl::TexImage2D 分配空间
        let texture_handle = texture_manager
            .create_empty(width, height, config)
            .map_err(|_| {
                FramebufferError::CreationFailed("Failed to create color texture".into())
            })?;

        // 3. 获取纹理 ID 用于绑定
        let tex_id = texture_manager.get(texture_handle).map(|t| t.id).ok_or(
            FramebufferError::CreationFailed("Texture ID missing".into()),
        )?;

        // 4. 初始化 FBO 和 RBO
        let (fbo, rbo) = self.setup_gl_resources(width, height, tex_id)?;

        let fb = Framebuffer {
            fbo_id: fbo,
            rbo_id: rbo,
            texture: texture_handle,
            width,
            height,
        };

        Ok(self.framebuffers.insert(fb))
    }

    fn setup_gl_resources(
        &self,
        width: u32,
        height: u32,
        tex_id: u32,
    ) -> Result<(u32, u32), FramebufferError> {
        unsafe {
            let mut fbo = 0;
            let mut rbo = 0;

            gl::GenFramebuffers(1, &mut fbo);
            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);

            // 附加颜色纹理
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                tex_id,
                0,
            );

            // 附加深度模板 RBO
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

            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                gl::DeleteFramebuffers(1, &fbo);
                gl::DeleteRenderbuffers(1, &rbo);
                return Err(FramebufferError::IncompleteFramebuffer);
            }

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            Ok((fbo, rbo))
        }
    }

    pub fn remove(&mut self, handle: FramebufferHandle) {
        if let Some(fb) = self.framebuffers.remove(handle) {
            // 1. 清理 OpenGL 的 FBO 和 RBO
            unsafe {
                gl::DeleteFramebuffers(1, &fb.fbo_id);
                gl::DeleteRenderbuffers(1, &fb.rbo_id);
            }

            // 2. 级联删除：通知 TextureManager 销毁纹理
            if let Some(tm_rc) = self.texture_manager.upgrade() {
                // 注意：这里需要借用 mut，因为要从 SlotMap 中移除
                tm_rc.borrow_mut().remove(fb.texture);
            }
        }
    }

    /// 绑定framebuffer进行渲染
    pub(crate) fn bind(&self, handle: FramebufferHandle) -> Result<(), FramebufferError> {
        if let Some(fb) = self.framebuffers.get(handle) {
            unsafe {
                gl::BindFramebuffer(gl::FRAMEBUFFER, fb.fbo_id);
                gl::Viewport(0, 0, fb.width as i32, fb.height as i32);
            }
            Ok(())
        } else {
            Err(FramebufferError::InvalidHandle)
        }
    }

    /// 解绑framebuffer，恢复到默认framebuffer
    pub(crate) fn unbind() {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    /// 调整framebuffer大小
    pub fn resize(
        &mut self,
        handle: FramebufferHandle,
        width: u32,
        height: u32,
    ) -> Result<(), FramebufferError> {
        let fb = self
            .framebuffers
            .get_mut(handle)
            .ok_or(FramebufferError::InvalidHandle)?;
        let tm_rc = self
            .texture_manager
            .upgrade()
            .ok_or(FramebufferError::TextureManagerBorrowFail)?;

        // 1. 让 TextureManager 调整纹理大小（通常是调用 gl::TexImage2D 重新分配内存）
        tm_rc
            .borrow_mut()
            .resize(fb.texture, width, height)
            .map_err(|_| FramebufferError::CreationFailed("Texture resize failed".into()))?;

        unsafe {
            // 2. 调整 Renderbuffer 大小
            gl::BindRenderbuffer(gl::RENDERBUFFER, fb.rbo_id);
            gl::RenderbufferStorage(
                gl::RENDERBUFFER,
                gl::DEPTH24_STENCIL8,
                width as i32,
                height as i32,
            );

            // 3. 更新 FB 元数据
            fb.width = width;
            fb.height = height;
        }
        Ok(())
    }
    /// 获取framebuffer的颜色附件纹理句柄
    pub fn get_color_texture(
        &self,
        handle: FramebufferHandle,
    ) -> Result<TextureHandle, FramebufferError> {
        if let Some(fb) = self.framebuffers.get(handle) {
            Ok(fb.texture)
        } else {
            Err(FramebufferError::InvalidHandle)
        }
    }
}

impl Drop for FramebufferManager {
    fn drop(&mut self) {
        // 尝试升级一次 TextureManager，如果主引擎已经没了就不管了
        let tm_rc = self.texture_manager.upgrade();

        unsafe {
            for (_, fb) in self.framebuffers.drain() {
                // 销毁 FBO 和 RBO
                gl::DeleteFramebuffers(1, &fb.fbo_id);
                gl::DeleteRenderbuffers(1, &fb.rbo_id);

                // 销毁对应的纹理
                if let Some(ref tm) = tm_rc {
                    tm.borrow_mut().remove(fb.texture);
                }
            }
        }
    }
}
