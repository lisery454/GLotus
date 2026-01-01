use crate::Resolution;

pub struct WindowState {
    resolution: Resolution,
}

impl WindowState {
    pub fn new(resolution: Resolution) -> Self {
        Self { resolution }
    }

    pub fn set_resolution(&mut self, resolution: Resolution) {
        self.resolution = resolution;
    }

    pub fn get_resolution(&self) -> Resolution {
        self.resolution
    }
}
