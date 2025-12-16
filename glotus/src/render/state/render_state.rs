#[derive(Clone, Debug)]
pub struct RenderState {
    pub depth_test: bool,
    pub depth_write: bool,
    pub stencil_test: bool,
    pub stencil_func: Option<StencilFunc>,
    pub stencil_op: Option<StencilOp>,
    pub blend: Option<BlendMode>,
    pub cull_face: Option<CullFaceMode>,
    pub polygon_mode: PolygonMode,
}

impl Default for RenderState {
    fn default() -> Self {
        Self {
            depth_test: true,
            depth_write: true,
            stencil_test: false,
            stencil_func: Default::default(),
            stencil_op: Default::default(),
            blend: Default::default(),
            cull_face: Default::default(),
            polygon_mode: Default::default(),
        }
    }
}

impl RenderState {
    /// 将 RenderState 应用到 OpenGL
    pub fn apply(&self) {
        unsafe {
            // 深度测试
            if self.depth_test {
                gl::Enable(gl::DEPTH_TEST);
            } else {
                gl::Disable(gl::DEPTH_TEST);
            }
            gl::DepthMask(self.depth_write as u8);

            // 模板测试
            if self.stencil_test {
                gl::Enable(gl::STENCIL_TEST);
                if let Some(stencil_func) = &self.stencil_func {
                    gl::StencilFunc(stencil_func.func, stencil_func.ref_value, stencil_func.mask);
                }
                if let Some(stencil_op) = &self.stencil_op {
                    gl::StencilOp(stencil_op.sfail, stencil_op.dpfail, stencil_op.dppass);
                }
            } else {
                gl::Disable(gl::STENCIL_TEST);
            }

            // 混合模式
            match self.blend {
                Some(BlendMode::Alpha) => {
                    gl::Enable(gl::BLEND);
                    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
                }
                Some(BlendMode::Additive) => {
                    gl::Enable(gl::BLEND);
                    gl::BlendFunc(gl::ONE, gl::ONE);
                }
                _ => gl::Disable(gl::BLEND),
            }

            // 剔除模式
            match self.cull_face {
                Some(CullFaceMode::Back) => {
                    gl::Enable(gl::CULL_FACE);
                    gl::CullFace(gl::BACK);
                }
                Some(CullFaceMode::Front) => {
                    gl::Enable(gl::CULL_FACE);
                    gl::CullFace(gl::FRONT);
                }
                _ => gl::Disable(gl::CULL_FACE),
            }

            // 多边形模式
            match self.polygon_mode {
                PolygonMode::Fill => gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL),
                PolygonMode::Line => gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE),
                PolygonMode::Point => gl::PolygonMode(gl::FRONT_AND_BACK, gl::POINT),
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct StencilFunc {
    pub func: u32, // gl::FUNC
    pub ref_value: i32,
    pub mask: u32,
}

impl Default for StencilFunc {
    fn default() -> Self {
        StencilFunc {
            func: gl::ALWAYS, // 默认总是通过
            ref_value: 0,
            mask: 0xFF,
        }
    }
}

#[derive(Clone, Debug)]
pub struct StencilOp {
    pub sfail: u32,
    pub dpfail: u32,
    pub dppass: u32,
}

impl Default for StencilOp {
    fn default() -> Self {
        StencilOp {
            sfail: gl::KEEP,
            dpfail: gl::KEEP,
            dppass: gl::KEEP,
        }
    }
}

#[derive(Clone, Debug)]
pub enum BlendMode {
    Alpha,
    Additive,
    None,
}

impl Default for BlendMode {
    fn default() -> Self {
        BlendMode::None
    }
}

#[derive(Clone, Debug)]
pub enum CullFaceMode {
    Back,
    Front,
    None,
}

impl Default for CullFaceMode {
    fn default() -> Self {
        CullFaceMode::None
    }
}

#[derive(Clone, Debug)]
pub enum PolygonMode {
    Fill,
    Line,
    Point,
}

impl Default for PolygonMode {
    fn default() -> Self {
        PolygonMode::Fill
    }
}
