#[derive(Clone, Debug)]
pub struct RenderState {
    /// 是否启用深度测试，也就是只有frag的深度小于当前位置的深度缓冲才会写入像素缓冲区
    pub depth_test: bool,
    /// 是否启用深度写入，也就是是否在深度测试判断时，会写入当前frag的深度到深度缓冲区
    pub depth_write: bool,
    /// 是否开启模板测试
    pub stencil_test: bool,
    /// 模板测试函数
    pub stencil_func: Option<StencilFunc>,
    /// 模板缓冲修改操作
    pub stencil_op: Option<StencilOp>,
    /// 混合模式
    pub blend: Option<BlendMode>,
    /// 剔除面模式
    pub cull_face: Option<CullFaceMode>,
    /// 多边形模式
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
                    gl::StencilFunc(
                        stencil_func.func.into(),
                        stencil_func.ref_value,
                        stencil_func.mask,
                    );
                }
                if let Some(stencil_op) = &self.stencil_op {
                    gl::StencilOp(
                        stencil_op.sfail.into(),
                        stencil_op.dpfail.into(),
                        stencil_op.dppass.into(),
                    );
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

#[derive(Clone, Debug, Default)]
pub struct PartialRenderState {
    pub depth_test: Option<bool>,
    pub depth_write: Option<bool>,
    pub stencil_test: Option<bool>,
    pub stencil_func: Option<StencilFunc>,
    pub stencil_op: Option<StencilOp>,
    pub blend: Option<BlendMode>,
    pub cull_face: Option<CullFaceMode>,
    pub polygon_mode: Option<PolygonMode>,
}

impl RenderState {
    /// 使用 partial 覆盖当前状态
    pub fn merge(&self, partial: &PartialRenderState) -> Self {
        Self {
            depth_test: partial.depth_test.unwrap_or(self.depth_test),
            depth_write: partial.depth_write.unwrap_or(self.depth_write),
            stencil_test: partial.stencil_test.unwrap_or(self.stencil_test),
            stencil_func: partial.stencil_func.or(self.stencil_func),
            stencil_op: partial.stencil_op.or(self.stencil_op),
            blend: partial.blend.or(self.blend),
            cull_face: partial.cull_face.or(self.cull_face),
            polygon_mode: partial.polygon_mode.unwrap_or(self.polygon_mode),
        }
    }

    /// 可变引用版本的 override
    pub fn override_by(&mut self, partial: PartialRenderState) {
        if let Some(depth_test) = partial.depth_test {
            self.depth_test = depth_test;
        }
        if let Some(depth_write) = partial.depth_write {
            self.depth_write = depth_write;
        }
        if let Some(stencil_test) = partial.stencil_test {
            self.stencil_test = stencil_test;
        }
        if let Some(stencil_func) = partial.stencil_func {
            self.stencil_func = Some(stencil_func);
        }
        if let Some(stencil_op) = partial.stencil_op {
            self.stencil_op = Some(stencil_op);
        }
        if let Some(blend) = partial.blend {
            self.blend = Some(blend);
        }
        if let Some(cull_face) = partial.cull_face {
            self.cull_face = Some(cull_face);
        }
        if let Some(polygon_mode) = partial.polygon_mode {
            self.polygon_mode = polygon_mode;
        }
    }
}

impl PartialRenderState {
    pub fn new() -> Self {
        Self::default()
    }
}

// 方便从完整状态创建部分状态
impl From<RenderState> for PartialRenderState {
    fn from(state: RenderState) -> Self {
        Self {
            depth_test: Some(state.depth_test),
            depth_write: Some(state.depth_write),
            stencil_test: Some(state.stencil_test),
            stencil_func: state.stencil_func,
            stencil_op: state.stencil_op,
            blend: state.blend,
            cull_face: state.cull_face,
            polygon_mode: Some(state.polygon_mode),
        }
    }
}

/// 模板函数
#[derive(Clone, Debug, Copy)]
pub struct StencilFunc {
    pub func: StencilFuncType, // gl::FUNC
    /// 与模板缓冲中的值进行比较的值
    pub ref_value: i32,
    /// 读取掩码，从模板缓冲区中读的值会先于这个掩码进行&，一共有八位可以进行掩码。
    pub mask: u32,
}

/// 模板函数类型
#[derive(Clone, Debug, Copy)]
pub enum StencilFuncType {
    /// 总是通过
    Always,
    /// 永远不通过
    Never,
    /// 参考值等于通过
    Equal,
    /// 参考值不等于读取值通过
    NotEqual,
    /// 参考值小于读取值通过
    Less,
    /// 参考值小于读取值等于通过
    LessEqual,
    /// 参考值大于读取值通过
    Greater,
    /// 参考值大于读取值等于通过
    GreraterEqual,
}

impl From<StencilFuncType> for gl::types::GLenum {
    fn from(value: StencilFuncType) -> Self {
        match value {
            StencilFuncType::Always => gl::ALWAYS,
            StencilFuncType::Never => gl::NEVER,
            StencilFuncType::Equal => gl::EQUAL,
            StencilFuncType::NotEqual => gl::NOTEQUAL,
            StencilFuncType::Less => gl::LESS,
            StencilFuncType::LessEqual => gl::LEQUAL,
            StencilFuncType::Greater => gl::GREATER,
            StencilFuncType::GreraterEqual => gl::GEQUAL,
        }
    }
}

impl Default for StencilFunc {
    fn default() -> Self {
        StencilFunc {
            func: StencilFuncType::Always,
            ref_value: 0,
            mask: 0xFF,
        }
    }
}

/// 模板测试写操作的处理
#[derive(Clone, Debug, Copy)]
pub struct StencilOp {
    /// 模板测试失败对模板缓冲的处理
    pub sfail: StencilOpType,
    /// 模板测试通过但是深度测试失败的对模板缓冲的处理
    pub dpfail: StencilOpType,
    /// 模板和深度都通过的对模板缓冲的处理
    pub dppass: StencilOpType,
}

/// 模板测试写操作的处理类型
#[derive(Clone, Debug, Copy)]
pub enum StencilOpType {
    /// 保持不变
    Keep,
    /// 置零
    Zero,
    /// 替换为模板方法中参考值
    Replace,
    /// 递增加1
    Increment,
    /// 递增加1，但是超过最大限制回到0（一共8位）
    IncrementWrap,
    /// 递减减1
    Decrement,
    /// 递减减1，但是超过最小限制回到最大值（一共8位）
    DecrementWrap,
    /// 反转所有位
    Invert,
}

impl From<StencilOpType> for gl::types::GLenum {
    fn from(value: StencilOpType) -> Self {
        match value {
            StencilOpType::Keep => gl::KEEP,
            StencilOpType::Zero => gl::ZERO,
            StencilOpType::Replace => gl::REPLACE,
            StencilOpType::Increment => gl::INCR,
            StencilOpType::IncrementWrap => gl::INCR_WRAP,
            StencilOpType::Decrement => gl::DECR,
            StencilOpType::DecrementWrap => gl::DECR_WRAP,
            StencilOpType::Invert => gl::INVERT,
        }
    }
}

impl Default for StencilOp {
    fn default() -> Self {
        StencilOp {
            sfail: StencilOpType::Keep,
            dpfail: StencilOpType::Keep,
            dppass: StencilOpType::Keep,
        }
    }
}

#[derive(Clone, Debug, Copy)]
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

#[derive(Clone, Debug, Copy)]
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

#[derive(Clone, Debug, Copy)]
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
