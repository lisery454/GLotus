use crate::AppContext;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

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

// 组件存储：使用 Sparse Set 或简单的 Vec<Option<T>> TODO: Slot Map
pub struct ComponentManager<T: IComponent> {
    data: Vec<Option<T>>,
}

impl<T: IComponent> ComponentManager<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn add(&mut self, entity_id: EntityId, component: T) {
        if entity_id >= self.data.len() {
            self.data.resize_with(entity_id + 1, || None);
        }
        self.data[entity_id] = Some(component);
    }

    pub fn get(&self, entity_id: EntityId) -> Option<&T> {
        self.data.get(entity_id).and_then(|opt| opt.as_ref())
    }

    pub fn get_mut(&mut self, entity_id: EntityId) -> Option<&mut T> {
        self.data.get_mut(entity_id).and_then(|opt| opt.as_mut())
    }

    pub fn has(&self, entity_id: EntityId) -> bool {
        self.get(entity_id).is_some()
    }

    /// 遍历所有有效的组件，返回 (EntityId, &T)
    pub fn iter(&self) -> impl Iterator<Item = (EntityId, &T)> {
        self.data
            .iter()
            .enumerate()
            .filter_map(|(id, opt)| opt.as_ref().map(|comp| (id, comp)))
    }

    /// 可变遍历所有有效的组件，返回 (EntityId, &mut T)
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (EntityId, &mut T)> {
        self.data
            .iter_mut()
            .enumerate()
            .filter_map(|(id, opt)| opt.as_mut().map(|comp| (id, comp)))
    }

    /// 根据条件查找第一个匹配的组件
    /// 示例：manager.find(|cam| cam.is_main)
    pub fn find<F>(&self, mut predicate: F) -> Option<(EntityId, &T)>
    where
        F: FnMut(&T) -> bool,
    {
        self.iter().find(|(_, comp)| predicate(comp))
    }

    /// 根据条件查找第一个匹配的可变组件
    pub fn find_mut<F>(&mut self, mut predicate: F) -> Option<(EntityId, &mut T)>
    where
        F: FnMut(&T) -> bool,
    {
        self.iter_mut().find(|(_, comp)| predicate(comp))
    }
}

pub trait IComponentManager: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: IComponent + 'static> IComponentManager for ComponentManager<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
