use super::ComponentManager;
use super::IComponentManager;
use crate::IComponent;
use std::any::TypeId;
use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;
use std::collections::HashMap;

type EntityHandle = usize;

pub struct World {
    components: HashMap<TypeId, RefCell<Box<dyn IComponentManager>>>,
    pub(crate) entities_count: usize,
}

impl World {
    pub fn new_with_default_registry() -> Self {
        let mut result = Self {
            components: HashMap::new(),
            entities_count: 0,
        };

        result.register_component::<super::TransformComponent>();
        result.register_component::<super::LightComponent>();
        result.register_component::<super::CameraComponent>();
        result.register_component::<super::RenderableComponent>();
        result.register_component::<super::ScriptComponent>();

        result
    }

    /// 手动注册组件
    pub fn register_component<T: IComponent + 'static>(&mut self) {
        let type_id = TypeId::of::<T>();
        let manager = ComponentManager::<T>::new();
        self.components
            .insert(type_id, RefCell::new(Box::new(manager)));
    }

    /// 获取只读管理器
    pub fn get_manager<'a, T: IComponent + 'static>(&'a self) -> Ref<'a, ComponentManager<T>> {
        let type_id = TypeId::of::<T>();

        let cell_ref = self.components.get(&type_id).expect("组件未注册");

        // 使用 Ref::map 进行类型收紧
        Ref::map(cell_ref.borrow(), |manager_box| {
            manager_box
                .as_any()
                .downcast_ref::<ComponentManager<T>>()
                .expect("类型转换失败")
        })
    }

    /// 获取可变管理器
    pub fn get_manager_mut<'a, T: IComponent + 'static>(
        &'a self,
    ) -> RefMut<'a, ComponentManager<T>> {
        let type_id = TypeId::of::<T>();

        let cell_ref = self.components.get(&type_id).expect("组件未注册");

        // 使用 RefMut::map 将 Box<dyn IComponentManager> 映射为具体的 ComponentManager<T>
        RefMut::map(cell_ref.borrow_mut(), |manager_box| {
            manager_box
                .as_any_mut()
                .downcast_mut::<ComponentManager<T>>()
                .expect("类型转换失败")
        })
    }

    pub fn spawn_entity(&mut self) -> EntityHandle {
        let id = self.entities_count;
        self.entities_count += 1;
        id
    }
}
