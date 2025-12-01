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

    pub(crate) fn bind(&self) {
        let shader = self.shader.borrow();
        shader.bind();
        
        // 给shader设置所有这个材质对应的uniforms
        for (name, value) in &self.uniforms {
            match value {
                UniformValue::Float(v) => shader.set_uniform_f32(name, *v),
                UniformValue::Int(v) => shader.set_uniform_i32(name, *v),
                UniformValue::Vector3(v) => shader.set_uniform_vec3(name, v),
                UniformValue::Vector4(v) => shader.set_uniform_vec4(name, v),
                UniformValue::Matrix3(m) => shader.set_uniform_mat3(name, m),
                UniformValue::Matrix4(m) => shader.set_uniform_mat4(name, m),
                UniformValue::Texture(slot) => shader.set_uniform_i32(name, *slot as i32),
            }
        }

        for (texture_slot_id, texture) in &self.textures {
            unsafe {
                gl::ActiveTexture(gl::TEXTURE0 + texture_slot_id);
                gl::BindTexture(gl::TEXTURE_2D, texture.borrow().get_id());
            }
        }
    }

    pub(crate) fn unbind(&self) {
        self.shader.borrow().unbind();
    }
}
