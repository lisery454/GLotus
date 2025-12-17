use crate::{Pass, Pipeline};

pub struct DefaultPipeline {}

impl DefaultPipeline {
    pub fn build_defalut_pipeline() -> Pipeline {
        let mut pipeline = Pipeline::new();
        pipeline.insert(Pass::new(Self::get_default_pass_name(), Default::default()));
        pipeline
    }

    pub fn get_default_pass_name() -> &'static str {
        "main"
    }
}
