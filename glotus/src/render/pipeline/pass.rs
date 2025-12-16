use crate::RenderState;

pub struct Pass {
    pub name: String,
    pub default_state: RenderState,
}

impl Pass {
    pub fn new(name: &str, state: RenderState) -> Self {
        Self {
            name: name.to_string(),
            default_state: state,
        }
    }
}
