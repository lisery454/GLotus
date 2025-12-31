#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Resolution {
    pub(crate) height: u32,
    pub(crate) width: u32,
}

impl Resolution {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }
}
