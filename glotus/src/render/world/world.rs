use std::{cell::RefCell, rc::Rc};

use crate::render::{
    camera::Camera,
    entity::entity::Entity,
    light::{Light, LightShaderData},
};

pub struct World {
    lights: Vec<Rc<RefCell<dyn Light>>>,
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

    pub fn get_light_shader_data(&self) -> Vec<LightShaderData> {
        self.get_lights()
            .iter()
            .map(|light| light.borrow().to_shader_data())
            .collect()
    }

    pub fn get_lights(&self) -> &Vec<Rc<RefCell<dyn Light>>> {
        &self.lights
    }

    pub fn add_light(&mut self, light: Rc<RefCell<dyn Light>>) {
        self.lights.push(light);
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
