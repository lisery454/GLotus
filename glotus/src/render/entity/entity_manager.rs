use std::collections::HashMap;

use slotmap::{SlotMap, new_key_type};

use crate::{MaterialHandle, MeshHandle, Transform};

use super::Entity;

new_key_type! {
    pub struct EntityHandle;
}

pub struct EntityManager {
    entities: SlotMap<EntityHandle, Entity>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            entities: SlotMap::with_key(),
        }
    }
    pub(crate) fn get(&self, handle: EntityHandle) -> Option<&Entity> {
        self.entities.get(handle)
    }

    pub(crate) fn get_mut(&mut self, handle: EntityHandle) -> Option<&mut Entity> {
        self.entities.get_mut(handle)
    }

    pub fn create(
        &mut self,
        material_handles: HashMap<String, MaterialHandle>,
        mesh_handle: MeshHandle,
    ) -> EntityHandle {
        let entity = Entity::new(material_handles, mesh_handle);
        self.entities.insert(entity)
    }

    pub fn remove(&mut self, handle: EntityHandle) {
        self.entities.remove(handle);
    }
}
