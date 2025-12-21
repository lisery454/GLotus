use super::ComponentManager;
use super::IComponentManager;
use crate::IComponent;
use std::any::TypeId;
use std::cell::UnsafeCell;
use std::collections::HashMap;

pub struct World {
    components: UnsafeCell<HashMap<TypeId, Box<dyn IComponentManager>>>,
    pub(crate) entities_count: usize,
}

impl World {
    pub fn new() -> Self {
        Self {
            components: UnsafeCell::new(HashMap::new()),
            entities_count: 0,
        }
    }

    pub fn get_manager<T: IComponent + 'static>(&self) -> &ComponentManager<T> {
        let type_id = TypeId::of::<T>();

        unsafe {
            let components = &mut *self.components.get();

            // 自动注册
            components
                .entry(type_id)
                .or_insert_with(|| Box::new(ComponentManager::<T>::new()));

            components
                .get(&type_id)
                .unwrap()
                .as_any()
                .downcast_ref::<ComponentManager<T>>()
                .unwrap()
        }
    }

    pub fn get_manager_mut<T: IComponent + 'static>(&self) -> &mut ComponentManager<T> {
        let type_id = TypeId::of::<T>();

        unsafe {
            let components = &mut *self.components.get();
            components
                .entry(type_id)
                .or_insert_with(|| Box::new(ComponentManager::<T>::new()));

            components
                .get_mut(&type_id)
                .unwrap()
                .as_any_mut()
                .downcast_mut::<ComponentManager<T>>()
                .unwrap()
        }
    }

    // 创建 Entity 的方法
    pub fn spawn_entity(&mut self) -> usize {
        let id = self.entities_count;
        self.entities_count += 1;
        id
    }
}
