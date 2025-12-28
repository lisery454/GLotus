use crate::TextureHandle;

pub struct Framebuffer {
    pub(crate) fbo_id: u32,
    pub(crate) rbo_id: u32, // 深度模板缓冲
    pub texture: TextureHandle,
    pub width: u32,
    pub height: u32,
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.fbo_id);
            gl::DeleteRenderbuffers(1, &self.rbo_id);
        }
    }
}
