use std::cmp::Ordering;

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
    pub fn transparent_pass() -> PassId {
        PassId::named("transparent")
    }

    #[inline]
    pub fn outline_pass() -> PassId {
        PassId::named("outline")
    }

    #[inline]
    pub fn ui_pass() -> PassId {
        PassId::named("ui")
    }

    #[inline]
    pub fn skybox_pass() -> PassId {
        PassId::named("skybox")
    }

    #[inline]
    pub fn debug_pass() -> PassId {
        PassId::named("debug")
    }

    pub(crate) fn build_default_pipeline() -> Pipeline {
        let mut pipeline = Pipeline::new();
        pipeline.insert(
            Pass::new(
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
                    CullFaceMode::Back,
                    PolygonMode::default(),
                ),
                true,
            )
            .with_sort(|a, b| {
                a.get_depth()
                    .partial_cmp(&b.get_depth())
                    .unwrap_or(Ordering::Equal)
            }),
        );
        pipeline.insert(
            Pass::new(
                Self::transparent_pass(),
                5,
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
                    BlendMode::Alpha,
                    CullFaceMode::default(),
                    PolygonMode::default(),
                ),
                false,
            )
            .with_sort(|a, b| {
                b.get_depth()
                    .partial_cmp(&a.get_depth())
                    .unwrap_or(Ordering::Equal)
            }),
        );
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
            false,
        ));
        pipeline.insert(Pass::new(
            Self::debug_pass(),
            15,
            RenderState::new(
                // 开启深度测试但关闭深度写入，这样法线线段不会遮挡后续渲染
                DepthMode::new(true, false, DepthFunc::Less),
                StencilMode::new(
                    false,
                    StencilFunc::new(StencilFuncType::Always, 0, 0xFF),
                    StencilOp::new(
                        StencilOpType::Keep,
                        StencilOpType::Keep,
                        StencilOpType::Keep,
                    ),
                    0x00,
                ),
                BlendMode::default(),
                CullFaceMode::None, // 法线线段通常是线段，关闭背面剔除
                PolygonMode::default(),
            ),
            false,
        ));
        pipeline.insert(Pass::new(
            Self::skybox_pass(),
            20,
            RenderState::new(
                DepthMode::new(true, false, DepthFunc::LessEqual),
                StencilMode::new(
                    false,
                    StencilFunc::new(StencilFuncType::Always, 0, 0xFF),
                    StencilOp::new(
                        StencilOpType::Keep,
                        StencilOpType::Keep,
                        StencilOpType::Keep,
                    ),
                    0xFF,
                ),
                BlendMode::default(),
                CullFaceMode::None,
                PolygonMode::default(),
            ),
            false,
        ));
        pipeline.insert(Pass::new(
            Self::ui_pass(),
            30,
            RenderState::new(
                DepthMode::new(false, false, DepthFunc::Less),
                StencilMode::new(
                    true,                                              // 启用模板
                    StencilFunc::new(StencilFuncType::Equal, 1, 0xFF), // 只渲染模板值为1的区域
                    StencilOp::new(
                        StencilOpType::Keep,
                        StencilOpType::Keep,
                        StencilOpType::Keep, // 保持模板值不变
                    ),
                    0xFF,
                ),
                BlendMode::default(),
                CullFaceMode::None,
                PolygonMode::default(),
            ),
            false,
        ));
        pipeline
    }
}
