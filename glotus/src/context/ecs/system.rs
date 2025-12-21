use crate::AppContext;
use std::cell::RefCell;
use std::rc::Rc;

pub trait ISystem {
    fn name(&self) -> &str;
    fn init(&mut self, _app_context: Rc<RefCell<AppContext>>) {}
    fn update(&mut self, _app_context: Rc<RefCell<AppContext>>, _delta_dt: f32) {}
    fn fixed_update(&mut self, _app_context: Rc<RefCell<AppContext>>, _delta_dt: f32) {}
}

pub struct SystemDispatcher {
    pub(crate) systems: Vec<Box<dyn ISystem>>,
}

impl SystemDispatcher {
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
        }
    }

    // 注册系统
    pub fn add_system<S: ISystem + 'static>(&mut self, system: S) {
        self.systems.push(Box::new(system));
    }

    pub(crate) fn init_systems(&mut self, app_context: Rc<RefCell<AppContext>>) {
        for system in &mut self.systems {
            system.init(app_context.clone());
        }
    }

    pub(crate) fn run_systems(&mut self, app_context: Rc<RefCell<AppContext>>, delta_dt: f32) {
        for system in &mut self.systems {
            system.update(app_context.clone(), delta_dt);
        }
    }

    pub(crate) fn fixed_run_systems(
        &mut self,
        app_context: Rc<RefCell<AppContext>>,
        delta_dt: f32,
    ) {
        for system in &mut self.systems {
            system.fixed_update(app_context.clone(), delta_dt);
        }
    }
}
