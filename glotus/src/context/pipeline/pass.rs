use super::RenderState;

pub struct Pass {
    pub name: String,
    pub default_state: RenderState,
}

impl Pass {
    pub fn new(name: String, state: RenderState) -> Self {
        Self {
            name,
            default_state: state,
        }
    }
}
