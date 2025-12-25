use slotmap::SecondaryMap;

use crate::AppContext;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use super::EntityHandle;

// 实体 ID 定义
type EntityId = usize;

pub trait IComponent: Any {
    fn update(
        &mut self,
        _entity_id: EntityId,
        _app_context: Rc<RefCell<AppContext>>,
        _delta_dt: f64,
    ) {
    }

    fn fixed_update(
        &mut self,
        _entity_id: EntityId,
        _app_context: Rc<RefCell<AppContext>>,
        _delta_dt: f64,
    ) {
    }
}

pub struct ComponentManager<T: IComponent> {
    components: SecondaryMap<EntityHandle, T>,
}

impl<T: IComponent> ComponentManager<T> {
    pub fn new() -> Self {
        Self {
            components: SecondaryMap::new(),
        }
    }

    pub fn add(&mut self, entity: EntityHandle, component: T) {
        self.components.insert(entity, component);
    }

    pub fn remove(&mut self, entity: EntityHandle) -> Option<T> {
        self.components.remove(entity)
    }

    /// 获取组件的不可变引用
    pub fn get(&self, entity: EntityHandle) -> Option<&T> {
        self.components.get(entity)
    }

    /// 获取组件的可变引用
    pub fn get_mut(&mut self, entity: EntityHandle) -> Option<&mut T> {
        self.components.get_mut(entity)
    }

    /// 检查实体是否有该组件
    pub fn has(&self, entity: EntityHandle) -> bool {
        self.components.contains_key(entity)
    }

    /// 返回所有 (EntityHandle, &T) 的迭代器
    pub fn iter(&self) -> slotmap::secondary::Iter<'_, EntityHandle, T> {
        self.components.iter()
    }

    /// 返回所有 (EntityHandle, &mut T) 的迭代器
    pub fn iter_mut(&mut self) -> slotmap::secondary::IterMut<'_, EntityHandle, T> {
        self.components.iter_mut()
    }

    /// 查找第一个符合条件的组件，返回 (实体句柄, 组件引用)
    pub fn find<F>(&self, mut predicate: F) -> Option<(EntityHandle, &T)>
    where
        F: FnMut(&T) -> bool,
    {
        // iter() 返回的是 (EntityHandle, &T)
        self.components.iter().find(|(_, comp)| predicate(comp))
    }

    /// 查找第一个符合条件的组件，返回 (实体句柄, 组件可变引用)
    pub fn find_mut<F>(&mut self, mut predicate: F) -> Option<(EntityHandle, &mut T)>
    where
        F: FnMut(&T) -> bool,
    {
        // iter_mut() 返回的是 (EntityHandle, &mut T)
        self.components.iter_mut().find(|(_, comp)| predicate(comp))
    }
}

pub trait IComponentManager: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn remove_abstract(&mut self, entity: EntityHandle);
}

impl<T: IComponent + 'static> IComponentManager for ComponentManager<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn remove_abstract(&mut self, entity: EntityHandle) {
        self.remove(entity);
    }
}
