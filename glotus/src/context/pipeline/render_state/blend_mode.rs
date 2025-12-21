#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum BlendMode {
    Alpha,
    Additive,
    None,
}

impl Default for BlendMode {
    fn default() -> Self {
        BlendMode::None
    }
}

impl BlendMode {
    pub fn apply(&self) {
        unsafe {
            match self {
                BlendMode::Alpha => {
                    gl::Enable(gl::BLEND);
                    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
                }
                BlendMode::Additive => {
                    gl::Enable(gl::BLEND);
                    gl::BlendFunc(gl::ONE, gl::ONE);
                }
                BlendMode::None => gl::Disable(gl::BLEND),
            }
        }
    }
}
