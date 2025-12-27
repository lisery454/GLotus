use crate::FramebufferHandle;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RenderTarget {
    Screen,
    Framebuffer(FramebufferHandle),
}

impl Default for RenderTarget {
    fn default() -> Self {
        RenderTarget::Screen
    }
}
