use super::ComponentManager;
use super::IComponentManager;
use crate::IComponent;
use std::any::TypeId;
use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;
use std::collections::HashMap;

use slotmap::{SlotMap, new_key_type};

new_key_type! {
    pub struct EntityHandle;
}

pub struct World {
    components: HashMap<TypeId, RefCell<Box<dyn IComponentManager>>>,
    entities: SlotMap<EntityHandle, ()>,
}

impl World {
    pub fn new_with_default_registry() -> Self {
        let mut result = Self {
            components: HashMap::new(),
            entities: SlotMap::with_key(),
        };

        result.register_component::<crate::Transform>();
        result.register_component::<crate::Light>();
        result.register_component::<crate::Camera>();
        result.register_component::<crate::Renderable>();
        result.register_component::<crate::Scriptable>();

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
        self.entities.insert(())
    }

    pub fn despawn_entity(&mut self, entity: EntityHandle) {
        if self.entities.remove(entity).is_some() {
            // 同步删除所有组件
            for manager in self.components.values() {
                manager.borrow_mut().remove_abstract(entity);
            }
        }
    }

    pub fn add_component<T: IComponent>(&self, entity: EntityHandle, component: T) {
        self.get_manager_mut::<T>().add(entity, component);
    }

    pub fn spawn_entity_with<B: ComponentBundle>(&mut self, bundle: B) -> EntityHandle {
        let entity = self.spawn_entity();
        bundle.add_to_entity(self, entity);
        entity
    }

    pub fn remove_component<T: IComponent>(&self, entity: EntityHandle) -> Option<T> {
        self.get_manager_mut::<T>().remove(entity)
    }
}

pub trait ComponentBundle {
    fn add_to_entity(self, world: &World, entity: EntityHandle);
}

// 为单个组件实现
impl<T: IComponent> ComponentBundle for T {
    fn add_to_entity(self, world: &World, entity: EntityHandle) {
        world.add_component(entity, self);
    }
}

// 为元组实现
impl<T1, T2> ComponentBundle for (T1, T2)
where
    T1: IComponent,
    T2: IComponent,
{
    fn add_to_entity(self, world: &World, entity: EntityHandle) {
        world.add_component(entity, self.0);
        world.add_component(entity, self.1);
    }
}

impl<T1, T2, T3> ComponentBundle for (T1, T2, T3)
where
    T1: IComponent,
    T2: IComponent,
    T3: IComponent,
{
    fn add_to_entity(self, world: &World, entity: EntityHandle) {
        world.add_component(entity, self.0);
        world.add_component(entity, self.1);
        world.add_component(entity, self.2);
    }
}
