mod shader;
mod shader_error;

pub use shader::Shader;
pub use shader_error::ShaderError;

use slotmap::{SlotMap, new_key_type};

new_key_type! {
    pub struct ShaderHandle;
}

pub struct ShaderManager {
    shaders: SlotMap<ShaderHandle, Shader>,
}

impl ShaderManager {
    pub fn new() -> Self {
        Self {
            shaders: SlotMap::with_key(),
        }
    }
    pub(crate) fn get(&self, handle: ShaderHandle) -> Option<&Shader> {
        self.shaders.get(handle)
    }

    pub(crate) fn get_mut(&mut self, handle: ShaderHandle) -> Option<&mut Shader> {
        self.shaders.get_mut(handle)
    }

    pub fn create_from_files_vf(
        &mut self,
        vertex_path: &str,
        fragment_path: &str,
    ) -> Result<ShaderHandle, ShaderError> {
        let shader = Shader::from_files_vf(vertex_path, fragment_path)?;
        Ok(self.shaders.insert(shader))
    }

    pub fn create_from_sources_vf(
        &mut self,
        vertex_source: &str,
        fragment_source: &str,
    ) -> Result<ShaderHandle, ShaderError> {
        let shader = Shader::from_sources_vf(vertex_source, fragment_source)?;
        Ok(self.shaders.insert(shader))
    }

    pub fn create_from_files_vfg(
        &mut self,
        vertex_path: &str,
        fragment_path: &str,
        geometry_path: &str,
    ) -> Result<ShaderHandle, ShaderError> {
        let shader = Shader::from_files_vfg(vertex_path, fragment_path, geometry_path)?;
        Ok(self.shaders.insert(shader))
    }

    pub fn create_from_sources_vfg(
        &mut self,
        vertex_source: &str,
        fragment_source: &str,
        geometry_source: &str,
    ) -> Result<ShaderHandle, ShaderError> {
        let shader = Shader::from_sources_vfg(vertex_source, fragment_source, geometry_source)?;
        Ok(self.shaders.insert(shader))
    }

    pub fn remove(&mut self, handle: ShaderHandle) {
        self.shaders.remove(handle);
    }
}
