use std::{cell::RefCell, rc::Rc};

use crate::render::*;

/// 实体对象
pub struct Entity {
    pub transform: Transform,
    pub material_group: Rc<RefCell<MaterialGroup>>,
    pub mesh_wrapper: Rc<RefCell<MeshGPUWrapper>>,
}

impl Entity {
    /// 新建
    pub fn new(
        transform: Transform,
        material_group: Rc<RefCell<MaterialGroup>>,
        mesh: Rc<RefCell<Mesh>>,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            transform,
            material_group,
            mesh_wrapper: MeshGPUWrapper::from_mesh(mesh),
        }))
    }
}
