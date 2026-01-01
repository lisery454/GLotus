use crate::AppContext;
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

pub trait ISystem {
    fn name(&self) -> &str;
    fn init(&mut self, _app_context: Rc<RefCell<AppContext>>) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
    fn update(
        &mut self,
        _app_context: Rc<RefCell<AppContext>>,
        _delta_dt: f32,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
    fn fixed_update(
        &mut self,
        _app_context: Rc<RefCell<AppContext>>,
        _delta_dt: f32,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
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

    pub fn new_with_default_systems() -> Self {
        let mut sd = Self::new();

        sd.add_system(super::CameraSystem::default());
        sd.add_system(super::RenderSystem::default());
        sd.add_system(super::ScriptSystem::default());

        sd
    }

    // 注册系统
    pub fn add_system<S: ISystem + 'static>(&mut self, system: S) {
        self.systems.push(Box::new(system));
    }

    pub(crate) fn init_systems(
        &mut self,
        app_context: Rc<RefCell<AppContext>>,
    ) -> Result<(), Box<dyn Error>> {
        for system in &mut self.systems {
            system.init(app_context.clone())?;
        }
        Ok(())
    }

    pub(crate) fn run_systems(
        &mut self,
        app_context: Rc<RefCell<AppContext>>,
        delta_dt: f32,
    ) -> Result<(), Box<dyn Error>> {
        for system in &mut self.systems {
            system.update(app_context.clone(), delta_dt)?;
        }
        Ok(())
    }

    pub(crate) fn fixed_run_systems(
        &mut self,
        app_context: Rc<RefCell<AppContext>>,
        delta_dt: f32,
    ) -> Result<(), Box<dyn Error>> {
        for system in &mut self.systems {
            system.fixed_update(app_context.clone(), delta_dt)?;
        }
        Ok(())
    }
}
