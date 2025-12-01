use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{shader::Shader, texture::Texture2D};

use super::uniform_value::UniformValue;

pub struct Material {
    pub shader: Rc<RefCell<Shader>>,
    pub uniforms: HashMap<String, UniformValue>,
    pub textures: HashMap<u32, Rc<RefCell<Texture2D>>>,
}

impl Material {
    pub fn new(shader: Rc<RefCell<Shader>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            shader: shader,
            uniforms: HashMap::new(),
            textures: HashMap::new(),
        }))
    }

    pub fn insert_uniform(&mut self, name: &str, value: UniformValue) {
        self.uniforms.insert(name.to_string(), value);
    }

    pub fn insert_textures(&mut self, slot_id: u32, value: Rc<RefCell<Texture2D>>) {
        self.textures.insert(slot_id, value);
    }
}
