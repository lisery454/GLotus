use slotmap::{SlotMap, new_key_type};

use crate::{ShaderHandle, ShaderManager, TextureManager};

use super::{GlobalUniform, Material, MaterialError, UniformValue};

new_key_type! {
    pub struct MaterialHandle;
}

pub struct MaterialManager {
    materials: SlotMap<MaterialHandle, Material>,
}

impl MaterialManager {
    pub fn new() -> Self {
        Self {
            materials: SlotMap::with_key(),
        }
    }
    pub(crate) fn get(&self, handle: MaterialHandle) -> Option<&Material> {
        self.materials.get(handle)
    }

    pub(crate) fn get_mut(&mut self, handle: MaterialHandle) -> Option<&mut Material> {
        self.materials.get_mut(handle)
    }

    pub fn create(&mut self, shader_handle: ShaderHandle) -> Result<MaterialHandle, MaterialError> {
        let material = Material::new(shader_handle);
        Ok(self.materials.insert(material))
    }

    pub fn remove(&mut self, handle: MaterialHandle) {
        self.materials.remove(handle);
    }

    pub(crate) fn inject_global_uniform(
        &mut self,
        material_handle: MaterialHandle,
        global_uniform: &GlobalUniform,
    ) -> Result<(), MaterialError> {
        let material = self
            .get_mut(material_handle)
            .ok_or(MaterialError::FindMatFail)?;

        material.insert_uniform(
            "g_view_position",
            UniformValue::Vector3(global_uniform.view_position.clone()),
        );
        material.insert_uniform(
            "g_model_matrix",
            UniformValue::Matrix4(global_uniform.model_matrix.clone()),
        );
        material.insert_uniform(
            "g_normal_matrix",
            UniformValue::Matrix3(global_uniform.normal_matrix.clone()),
        );
        material.insert_uniform(
            "g_view_matrix",
            UniformValue::Matrix4(global_uniform.view_matrix.clone()),
        );
        material.insert_uniform(
            "g_projection_matrix",
            UniformValue::Matrix4(global_uniform.projection_matrix.clone()),
        );
        material.insert_uniform(
            "g_light_count",
            UniformValue::Int(global_uniform.light_count.clone()),
        );

        for (i, v) in global_uniform.lights_shader_data.iter().enumerate() {
            material.insert_uniform(
                &format!("g_lights[{}].light_type", i),
                UniformValue::Int(v.light_type),
            );
            material.insert_uniform(
                &format!("g_lights[{}].color", i),
                UniformValue::Vector3(v.color),
            );
            material.insert_uniform(
                &format!("g_lights[{}].position", i),
                UniformValue::Vector3(v.position),
            );
            material.insert_uniform(
                &format!("g_lights[{}].direction", i),
                UniformValue::Vector3(v.direction),
            );
            material.insert_uniform(
                &format!("g_lights[{}].intensity", i),
                UniformValue::Float(v.intensity),
            );
            material.insert_uniform(
                &format!("g_lights[{}].range", i),
                UniformValue::Float(v.range),
            );
            material.insert_uniform(
                &format!("g_lights[{}].inner_cone", i),
                UniformValue::Float(v.inner_cone),
            );
            material.insert_uniform(
                &format!("g_lights[{}].outer_cone", i),
                UniformValue::Float(v.outer_cone),
            );
        }

        material.insert_uniform(
            "g_camera.camera_type",
            UniformValue::Int(global_uniform.camera_shader_data.camera_type),
        );
        material.insert_uniform(
            "g_camera.direction",
            UniformValue::Vector3(global_uniform.camera_shader_data.direction),
        );
        material.insert_uniform(
            "g_camera.position",
            UniformValue::Vector3(global_uniform.camera_shader_data.position),
        );
        material.insert_uniform(
            "g_camera.aspect_ratio",
            UniformValue::Float(global_uniform.camera_shader_data.aspect_ratio),
        );
        material.insert_uniform(
            "g_camera.near_plane",
            UniformValue::Float(global_uniform.camera_shader_data.near_plane),
        );
        material.insert_uniform(
            "g_camera.far_plane",
            UniformValue::Float(global_uniform.camera_shader_data.far_plane),
        );

        Ok(())
    }

    pub(crate) fn bind(
        &self,
        material_handle: MaterialHandle,
        texture_manager: &TextureManager,
        shader_manager: &ShaderManager,
    ) -> Result<(), MaterialError> {
        let material = self
            .get(material_handle)
            .ok_or(MaterialError::FindMatFail)?;
        let shader = shader_manager
            .get(material.shader_handle)
            .ok_or(MaterialError::FindShaderFail)?;

        shader.bind();

        // 给shader设置所有这个材质对应的uniforms
        for (name, value) in &material.uniforms {
            match value {
                UniformValue::Float(v) => shader.set_uniform_f32(name, *v),
                UniformValue::Int(v) => shader.set_uniform_i32(name, *v),
                UniformValue::Vector3(v) => shader.set_uniform_vec3(name, v),
                UniformValue::Vector4(v) => shader.set_uniform_vec4(name, v),
                UniformValue::Matrix3(m) => shader.set_uniform_mat3(name, m),
                UniformValue::Matrix4(m) => shader.set_uniform_mat4(name, m),
                UniformValue::Texture(slot, texture_handle) => {
                    shader.set_uniform_i32(name, *slot as i32);
                    let texture = texture_manager
                        .get(*texture_handle)
                        .ok_or(MaterialError::FindTextureFail)?;

                    unsafe {
                        gl::ActiveTexture(gl::TEXTURE0 + *slot as u32);
                        gl::BindTexture(gl::TEXTURE_2D, texture.id);
                    }

                    Ok(())
                }
            }
            .map_err(|e| MaterialError::BindFail(e))?;
        }

        Ok(())
    }

    pub(crate) fn unbind(
        &self,
        material_handle: MaterialHandle,
        shader_manager: &ShaderManager,
    ) -> Result<(), MaterialError> {
        let material = self
            .get(material_handle)
            .ok_or(MaterialError::FindMatFail)?;
        let shader = shader_manager
            .get(material.shader_handle)
            .ok_or(MaterialError::FindShaderFail)?;
        shader.unbind();
        Ok(())
    }
}
