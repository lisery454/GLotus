use std::{cell::RefCell, rc::Rc};

use crate::{material::Material, mesh::Mesh, transform::Transform};

pub struct Entity {
    pub transform: Transform,
    pub material: Rc<RefCell<Material>>,
    pub mesh: Rc<RefCell<Mesh>>,
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
            mesh,
        }))
    }
}
