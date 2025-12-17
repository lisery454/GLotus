/// 模板函数 (ref & mask)  vs  (stencil_value & mask)
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct StencilFunc {
    pub func: StencilFuncType, // gl::FUNC
    /// 与模板缓冲中的值进行比较的值
    pub ref_value: i32,
    /// 读取掩码，从模板缓冲区中读的值会先于这个掩码进行&，一共有八位可以进行掩码。
    pub mask: u32,
}

/// 模板函数类型
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
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

impl Default for StencilFunc {
    fn default() -> Self {
        StencilFunc {
            func: StencilFuncType::Always,
            ref_value: 0,
            mask: 0xFF,
        }
    }
}

impl StencilFunc {
    pub fn new(func: StencilFuncType, ref_value: i32, mask: u32) -> Self {
        Self {
            func,
            ref_value,
            mask,
        }
    }

    pub fn apply(&self) {
        unsafe {
            gl::StencilFunc(
                match self.func {
                    StencilFuncType::Always => gl::ALWAYS,
                    StencilFuncType::Never => gl::NEVER,
                    StencilFuncType::Equal => gl::EQUAL,
                    StencilFuncType::NotEqual => gl::NOTEQUAL,
                    StencilFuncType::Less => gl::LESS,
                    StencilFuncType::LessEqual => gl::LEQUAL,
                    StencilFuncType::Greater => gl::GREATER,
                    StencilFuncType::GreraterEqual => gl::GEQUAL,
                },
                self.ref_value,
                self.mask,
            );
        }
    }
}

/// 模板测试写操作的处理
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct StencilOp {
    /// 模板测试失败对模板缓冲的处理
    pub sfail: StencilOpType,
    /// 模板测试通过但是深度测试失败的对模板缓冲的处理
    pub dpfail: StencilOpType,
    /// 模板和深度都通过的对模板缓冲的处理
    pub dppass: StencilOpType,
}

/// 模板测试写操作的处理类型
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
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

impl StencilOpType {
    pub fn to_glenum(&self) -> gl::types::GLenum {
        match self {
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

impl StencilOp {
    pub fn new(sfail: StencilOpType, dpfail: StencilOpType, dppass: StencilOpType) -> Self {
        Self {
            sfail,
            dpfail,
            dppass,
        }
    }

    pub fn apply(&self) {
        unsafe {
            gl::StencilOp(
                self.sfail.to_glenum(),
                self.dpfail.to_glenum(),
                self.dppass.to_glenum(),
            );
        }
    }
}

/// 模板模式
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct StencilMode {
    enable: bool,
    func: StencilFunc,
    op: StencilOp,
    mask: u32,
}

impl Default for StencilMode {
    fn default() -> Self {
        Self {
            enable: false,
            func: Default::default(),
            op: Default::default(),
            mask: 0xFF,
        }
    }
}

impl StencilMode {
    pub fn new(enable: bool, func: StencilFunc, op: StencilOp, mask: u32) -> Self {
        Self {
            enable,
            func,
            op,
            mask,
        }
    }

    pub fn apply(&self) {
        unsafe {
            if self.enable {
                gl::Enable(gl::STENCIL_TEST);
            } else {
                gl::Disable(gl::STENCIL_TEST);
            }
        }

        self.func.apply();
        self.op.apply();

        unsafe {
            gl::StencilMask(self.mask);
        }
    }
}
