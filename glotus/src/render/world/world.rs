use std::{cell::RefCell, rc::Rc};

use crate::render::*;

/// 世界
pub struct World {
    // lights: Vec<Rc<RefCell<dyn Light>>>,
    entities: Vec<EntityHandle>,
    camera: Camera,
}

impl World {
    /// 新建
    pub fn new() -> Self {
        World {
            // lights: Vec::new(),
            entities: Vec::new(),
            camera: Camera::new(),
        }
    }

    /// 获取光照的shader数据
    // pub fn get_light_shader_data(&self) -> Vec<LightShaderData> {
    //     self.get_lights()
    //         .iter()
    //         .map(|light| light.borrow().to_shader_data())
    //         .collect()
    // }

    /// 获取相机的shader数据
    // pub fn get_camera_shader_data(&self) -> CameraShaderData {
    //     self.get_camera().borrow().to_shader_data()
    // }

    /// 获取光源引用
    // pub fn get_lights(&self) -> &Vec<Rc<RefCell<dyn Light>>> {
    //     &self.lights
    // }

    /// 增加光源
    // pub fn add_light(&mut self, light: Rc<RefCell<dyn Light>>) {
    //     self.lights.push(light);
    // }

    /// 获取相机
    pub fn get_camera(&self) -> &Camera {
        &self.camera
    }

    /// 获取所有实体
    pub fn get_entities(&self) -> &Vec<EntityHandle> {
        &self.entities
    }

    /// 增加实体
    pub fn add_entity(&mut self, entity: Rc<RefCell<Entity>>) {
        self.entities.push(entity);
    }
}
