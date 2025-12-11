use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::render::{
    material::{GlobalUniform, MaterialError},
    shader::Shader,
    texture::Texture2D,
};

use super::uniform_value::UniformValue;

pub struct Material {
    pub shader: Rc<RefCell<Shader>>,
    pub uniforms: HashMap<String, UniformValue>,
    pub textures: HashMap<u32, Rc<RefCell<Texture2D>>>,
}

impl Material {
    pub fn new(shader: Rc<RefCell<Shader>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            shader,
            uniforms: HashMap::new(),
            textures: HashMap::new(),
        }))
    }

    pub fn insert_uniform(&mut self, name: &str, value: UniformValue) {
        if let UniformValue::Texture(slot_id, texture) = &value {
            self.insert_textures(*slot_id as u32, texture.clone());
        }
        self.uniforms.insert(name.to_string(), value);
    }

    fn insert_textures(&mut self, slot_id: u32, value: Rc<RefCell<Texture2D>>) {
        self.textures.insert(slot_id, value);
    }

    pub(crate) fn inject_global_uniform(&mut self, global_uniform: &GlobalUniform) {
        self.insert_uniform(
            "g_view_position",
            UniformValue::Vector3(global_uniform.view_position),
        );
        self.insert_uniform(
            "g_model_matrix",
            UniformValue::Matrix4(global_uniform.model_matrix),
        );
        self.insert_uniform(
            "g_normal_matrix",
            UniformValue::Matrix3(global_uniform.normal_matrix),
        );
        self.insert_uniform(
            "g_view_matrix",
            UniformValue::Matrix4(global_uniform.view_matrix),
        );
        self.insert_uniform(
            "g_projection_matrix",
            UniformValue::Matrix4(global_uniform.projection_matrix),
        );
        self.insert_uniform(
            "g_light_count",
            UniformValue::Int(global_uniform.light_count),
        );

        for (i, v) in global_uniform.lights_shader_data.iter().enumerate() {
            self.insert_uniform(
                &format!("g_lights[{}].light_type", i),
                UniformValue::Int(v.light_type),
            );
            self.insert_uniform(
                &format!("g_lights[{}].color", i),
                UniformValue::Vector3(v.color),
            );
            self.insert_uniform(
                &format!("g_lights[{}].position", i),
                UniformValue::Vector3(v.position),
            );
            self.insert_uniform(
                &format!("g_lights[{}].direction", i),
                UniformValue::Vector3(v.direction),
            );
            self.insert_uniform(
                &format!("g_lights[{}].intensity", i),
                UniformValue::Float(v.intensity),
            );
            self.insert_uniform(
                &format!("g_lights[{}].range", i),
                UniformValue::Float(v.range),
            );
            self.insert_uniform(
                &format!("g_lights[{}].inner_cone", i),
                UniformValue::Float(v.inner_cone),
            );
            self.insert_uniform(
                &format!("g_lights[{}].outer_cone", i),
                UniformValue::Float(v.outer_cone),
            );
        }

        //         struct Camera {
        //   int camera_type;
        //   vec3 direction;
        //   vec3 position;
        //   float aspect_ratio;
        //   float near_plane;
        //   float far_plane;
        // }

        self.insert_uniform(
            "g_camera.camera_type",
            UniformValue::Int(global_uniform.camera_shader_data.camera_type),
        );
        self.insert_uniform(
            "g_camera.direction",
            UniformValue::Vector3(global_uniform.camera_shader_data.direction),
        );
        self.insert_uniform(
            "g_camera.position",
            UniformValue::Vector3(global_uniform.camera_shader_data.position),
        );
        self.insert_uniform(
            "g_camera.aspect_ratio",
            UniformValue::Float(global_uniform.camera_shader_data.aspect_ratio),
        );
        self.insert_uniform(
            "g_camera.near_plane",
            UniformValue::Float(global_uniform.camera_shader_data.near_plane),
        );
        self.insert_uniform(
            "g_camera.far_plane",
            UniformValue::Float(global_uniform.camera_shader_data.far_plane),
        );
    }

    pub(crate) fn bind(&self) -> Result<(), MaterialError> {
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
                UniformValue::Texture(slot, _) => shader.set_uniform_i32(name, *slot as i32),
            }
            .map_err(|e| MaterialError::BindFail(e))?;
        }

        for (texture_slot_id, texture) in &self.textures {
            unsafe {
                gl::ActiveTexture(gl::TEXTURE0 + texture_slot_id);
                gl::BindTexture(gl::TEXTURE_2D, texture.borrow().get_id());
            }
        }

        Ok(())
    }

    pub(crate) fn unbind(&self) {
        self.shader.borrow().unbind();
    }
}
