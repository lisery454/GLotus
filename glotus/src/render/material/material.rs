use std::collections::HashMap;

use crate::*;

use super::uniform_value::UniformValue;

pub struct Material {
    pub shader_handle: ShaderHandle,
    pub uniforms: HashMap<String, UniformValue>,
}

impl Material {
    pub fn new(shader: ShaderHandle) -> Self {
        Self {
            shader_handle: shader,
            uniforms: HashMap::new(),
        }
    }

    pub fn insert_uniform(&mut self, name: &str, value: UniformValue) {
        self.uniforms.insert(name.to_string(), value);
    }

   
}
