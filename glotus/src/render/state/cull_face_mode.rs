#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum CullFaceMode {
    Back,
    Front,
    None,
}

impl Default for CullFaceMode {
    fn default() -> Self {
        CullFaceMode::None
    }
}

impl CullFaceMode {
    pub fn apply(&self) {
        unsafe {
            match self {
                CullFaceMode::Back => {
                    gl::Enable(gl::CULL_FACE);
                    gl::CullFace(gl::BACK);
                }
                CullFaceMode::Front => {
                    gl::Enable(gl::CULL_FACE);
                    gl::CullFace(gl::FRONT);
                }
                CullFaceMode::None => gl::Disable(gl::CULL_FACE),
            }
        }
    }
}
