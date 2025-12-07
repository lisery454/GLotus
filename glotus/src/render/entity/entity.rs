use std::{cell::RefCell, rc::Rc};

use crate::render::{
    material::Material,
    mesh::{Mesh, MeshGPUWrapper},
    transform::Transform,
};

pub struct Entity {
    pub transform: Transform,
    pub material: Rc<RefCell<Material>>,
    pub mesh: Rc<RefCell<MeshGPUWrapper>>,
}

impl Entity {
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
