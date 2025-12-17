use slotmap::{SlotMap, new_key_type};

use super::{Mesh, MeshError, MeshGPUWrapper};

new_key_type! {
    pub struct MeshHandle;
}

pub struct MeshManager {
    meshes: SlotMap<MeshHandle, MeshGPUWrapper>,
}

impl MeshManager {
    pub fn new() -> Self {
        Self {
            meshes: SlotMap::with_key(),
        }
    }
    pub(crate) fn get(&self, handle: MeshHandle) -> Option<&MeshGPUWrapper> {
        self.meshes.get(handle)
    }

    pub(crate) fn get_mut(&mut self, handle: MeshHandle) -> Option<&mut MeshGPUWrapper> {
        self.meshes.get_mut(handle)
    }

    pub fn create_from_position(
        &mut self,
        indices: &Vec<usize>,
        positions: &Vec<f32>,
    ) -> Result<MeshHandle, MeshError> {
        let mesh = Mesh::from_position(indices, positions);
        let mesh_gpu_wrapper = MeshGPUWrapper::from_mesh(&mesh);
        Ok(self.meshes.insert(mesh_gpu_wrapper))
    }

    pub fn remove(&mut self, handle: MeshHandle) {
        self.meshes.remove(handle);
    }
}
