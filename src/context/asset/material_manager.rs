mod material;
mod material_error;
mod uniform_value;

pub use material::*;
pub use material_error::*;
pub use uniform_value::*;

use log::warn;
use slotmap::{SlotMap, new_key_type};

use crate::ShaderHandle;

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

    pub fn insert_uniform(&mut self, handle: MaterialHandle, name: &str, value: UniformValue) {
        let Some(material) = self.get_mut(handle) else {
            warn!("can not find material of handle: {:?} when insert uniform", handle);
            return;
        };

        material.insert_uniform(name, value);
    }

    pub fn remove(&mut self, handle: MaterialHandle) {
        self.materials.remove(handle);
    }
}
