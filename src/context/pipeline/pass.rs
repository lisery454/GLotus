use std::cmp::Ordering;

use crate::{EntityHandle, MaterialHandle, MeshHandle};

use super::RenderState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PassId(u32);

const USER_BIT: u32 = 1 << 31;

impl PassId {
    /// 用户 / 命名 pass（稳定，可序列化）
    pub fn named(name: &str) -> Self {
        let hash = fnv1a_32(name.as_bytes());
        PassId(USER_BIT | (hash & !USER_BIT))
    }

    pub fn is_user(self) -> bool {
        (self.0 & USER_BIT) != 0
    }

    pub fn raw(self) -> u32 {
        self.0
    }
}

impl From<&str> for PassId {
    fn from(name: &str) -> Self {
        PassId::named(name)
    }
}

fn fnv1a_32(bytes: &[u8]) -> u32 {
    const FNV_OFFSET: u32 = 0x811C9DC5;
    const FNV_PRIME: u32 = 16777619;

    let mut hash = FNV_OFFSET;
    for b in bytes {
        hash ^= *b as u32;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

pub enum RenderJob {
    Single(SingleJob),
    Instanced(InstancedJob),
}

pub struct InstancedJob {
    mesh: MeshHandle,
    material: MaterialHandle,
    transforms: Vec<[[f32; 4]; 4]>,
}

impl InstancedJob {
    pub fn new(mesh: MeshHandle, material: MaterialHandle) -> Self {
        Self {
            mesh,
            material,
            transforms: Vec::new(),
        }
    }

    pub fn set_transforms(&mut self, transforms: Vec<[[f32; 4]; 4]>)  {
        self.transforms = transforms;
    }

    pub fn get_transforms(&self) -> &Vec<[[f32; 4]; 4]> {
        &self.transforms
    }

    pub fn get_mesh(&self) -> MeshHandle {
        self.mesh
    }

    pub fn get_material(&self) -> MaterialHandle {
        self.material
    }
}

pub struct SingleJob {
    entity: EntityHandle,
    mesh: MeshHandle,
    material: MaterialHandle,
    depth: f32, // 用于排序
}

impl SingleJob {
    pub fn new(
        entity: EntityHandle,
        mesh: MeshHandle,
        material: MaterialHandle,
        depth: f32,
    ) -> Self {
        Self {
            entity,
            mesh,
            material,
            depth,
        }
    }

    pub fn get_entity(&self) -> EntityHandle {
        self.entity
    }

    pub fn get_mesh(&self) -> MeshHandle {
        self.mesh
    }

    pub fn get_material(&self) -> MaterialHandle {
        self.material
    }

    pub fn get_depth(&self) -> f32 {
        self.depth
    }
}

pub struct Pass {
    pub id: PassId,
    pub priority: i32,
    pub is_opaque: bool,
    pub sort_func: Option<Box<dyn Fn(&SingleJob, &SingleJob) -> Ordering>>,
    pub default_state: RenderState,
}

impl Pass {
    pub fn new(id: PassId, priority: i32, state: RenderState, is_opaque: bool) -> Self {
        Self {
            id,
            priority,
            default_state: state,
            sort_func: None,
            is_opaque: is_opaque,
        }
    }

    pub fn with_sort<F>(mut self, sort_func: F) -> Self
    where
        F: Fn(&SingleJob, &SingleJob) -> Ordering + 'static,
    {
        self.sort_func = Some(Box::new(sort_func));
        self
    }
}
