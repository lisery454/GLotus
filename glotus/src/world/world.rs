use std::{cell::RefCell, rc::Rc};

use crate::{camera::Camera, entity::entity::Entity, light::Light};

pub struct World {
    lights: Vec<Box<dyn Light>>,
    entities: Vec<Rc<RefCell<Entity>>>,
    camera: Rc<RefCell<Camera>>,
}

impl World {
    pub fn new() -> Self {
        World {
            lights: Vec::new(),
            entities: Vec::new(),
            camera: Rc::new(RefCell::new(Camera::new())),
        }
    }

    pub fn get_camera(&self) -> Rc<RefCell<Camera>> {
        self.camera.clone()
    }

    pub fn get_entities(&self) -> &Vec<Rc<RefCell<Entity>>> {
        &self.entities
    }

    pub fn add_entity(&mut self, entity: Rc<RefCell<Entity>>) {
        self.entities.push(entity);
    }
}
