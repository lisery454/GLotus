use std::{cell::RefCell, rc::Rc};

use crate::render::*;

/// 实体对象
pub struct Entity {
    pub transform: Transform,
    pub material: Rc<RefCell<Material>>,
    pub mesh: Rc<RefCell<MeshGPUWrapper>>,
}

impl Entity {
    /// 新建
    pub fn new(
        transform: Transform,
        material: Rc<RefCell<Material>>,
        mesh: Rc<RefCell<Mesh>>,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            transform,
            material,
            mesh: MeshGPUWrapper::from_mesh(mesh),
        }))
    }
}
