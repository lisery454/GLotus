use super::{
    BlendMode, CullFaceMode, DepthFunc, DepthMode, Pass, PassId, Pipeline, PolygonMode,
    RenderState, StencilFunc, StencilFuncType, StencilMode, StencilOp, StencilOpType,
};

pub struct DefaultPipeline;

impl DefaultPipeline {
    #[inline]
    pub fn main_pass() -> PassId {
        PassId::named("main")
    }

    #[inline]
    pub fn outline_pass() -> PassId {
        PassId::named("outline")
    }

    pub(crate) fn build_default_pipeline() -> Pipeline {
        let mut pipeline = Pipeline::new();
        pipeline.insert(Pass::new(
            Self::main_pass(),
            0,
            RenderState::new(
                DepthMode::new(true, true, DepthFunc::Less),
                StencilMode::new(
                    true,
                    StencilFunc::new(StencilFuncType::Always, 1, 0xFF),
                    StencilOp::new(
                        StencilOpType::Keep,
                        StencilOpType::Keep,
                        StencilOpType::Replace,
                    ),
                    0xFF,
                ),
                BlendMode::default(),
                CullFaceMode::default(),
                PolygonMode::default(),
            ),
        ));
        pipeline.insert(Pass::new(
            Self::outline_pass(),
            10,
            RenderState::new(
                DepthMode::new(true, false, DepthFunc::LessEqual),
                StencilMode::new(
                    true,
                    StencilFunc::new(StencilFuncType::NotEqual, 1, 0xFF),
                    StencilOp::new(
                        StencilOpType::Keep,
                        StencilOpType::Keep,
                        StencilOpType::Keep,
                    ),
                    0x00,
                ),
                BlendMode::default(),
                CullFaceMode::Front,
                PolygonMode::default(),
            ),
        ));
        pipeline
    }
}
