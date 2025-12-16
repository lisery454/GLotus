use std::{cell::RefCell, rc::Rc};

use crate::{Material, MaterialGroup, Pass, Pipeline, Shader};

pub struct DefaultPipeline {}

impl DefaultPipeline {
    pub fn build_defalut_pipeline() -> Rc<RefCell<Pipeline>> {
        let pipeline = Rc::new(RefCell::new(Pipeline::new()));
        pipeline.borrow_mut().insert(Pass::new(
            Self::get_default_pass_name(),
            Default::default(),
        ));
        pipeline
    }

    pub fn get_default_pass_name() -> &'static str {
        "main"
    }

    pub fn build_default_pipeline_material(
        shader: Rc<RefCell<Shader>>,
    ) -> Rc<RefCell<MaterialGroup>> {
        let material = Material::new(shader);
        let pass_name = Self::get_default_pass_name();
        let material_group = MaterialGroup::single(pass_name, material);
        material_group
    }
}
