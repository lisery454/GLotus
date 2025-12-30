use std::{cell::RefCell, rc::Rc};

use crate::{AppContext, ISystem, Scriptable};

#[derive(Default)]
pub struct ScriptSystem;

impl ISystem for ScriptSystem {
    fn name(&self) -> &str {
        "ScriptSystem"
    }

    fn update(&mut self, app_context: Rc<RefCell<AppContext>>, dt: f32) {
        let context = app_context.borrow();
        let world = context.world.borrow();

        let mut script_mgr = world.get_manager_mut::<Scriptable>();

        for (entity, script_comp) in script_mgr.iter_mut() {
            for behavior in script_comp.behaviors.iter_mut() {
                behavior.on_update(entity, app_context.clone(), dt);
            }
        }
    }

    fn fixed_update(&mut self, app_context: Rc<RefCell<AppContext>>, delta_dt: f32) {
        let context = app_context.borrow();
        let world = context.world.borrow();

        let mut script_mgr = world.get_manager_mut::<Scriptable>();

        for (entity, script_comp) in script_mgr.iter_mut() {
            for behavior in script_comp.behaviors.iter_mut() {
                behavior.on_fixed_update(entity, app_context.clone(), delta_dt);
            }
        }
    }
}
