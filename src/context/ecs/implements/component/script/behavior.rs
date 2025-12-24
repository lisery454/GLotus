use std::{cell::RefCell, rc::Rc};

use crate::{AppContext, EntityHandle};

pub trait IBehavior: 'static {
    fn on_update(&mut self, _entity: EntityHandle, _context: Rc<RefCell<AppContext>>, _dt: f32) {}
    fn on_fixed_update(&mut self, _entity: EntityHandle, _context: Rc<RefCell<AppContext>>, _dt: f32) {}
}