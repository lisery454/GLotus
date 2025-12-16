use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::Material;

pub struct MaterialGroup {
    pub(crate) materials: HashMap<String, Rc<RefCell<Material>>>,
}

impl MaterialGroup {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            materials: HashMap::new(),
        }))
    }

    pub fn single(pass_name: &str, material: Rc<RefCell<Material>>) -> Rc<RefCell<Self>> {
        let m = Self::new();
        m.borrow_mut().insert(pass_name, material);
        m
    }

    pub fn insert(&mut self, name: &str, material: Rc<RefCell<Material>>) {
        self.materials.insert(name.to_string(), material);
    }
}
