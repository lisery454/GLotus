/// 深度函数
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum DepthFunc {
    Never,
    Less, // 默认
    Equal,
    LessEqual,
    Greater,
    NotEqual,
    GreaterEqual,
    Always,
}

impl Default for DepthFunc {
    fn default() -> Self {
        return Self::Less;
    }
}

impl DepthFunc {
    pub fn apply(&self) {
        unsafe {
            gl::DepthFunc(match self {
                DepthFunc::Never => gl::NEVER,
                DepthFunc::Less => gl::LESS,
                DepthFunc::Equal => gl::EQUAL,
                DepthFunc::LessEqual => gl::LEQUAL,
                DepthFunc::Greater => gl::GREATER,
                DepthFunc::NotEqual => gl::NOTEQUAL,
                DepthFunc::GreaterEqual => gl::GEQUAL,
                DepthFunc::Always => gl::ALWAYS,
            });
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct DepthMode {
    enable: bool,
    write: bool,
    func: DepthFunc,
}

impl Default for DepthMode {
    fn default() -> Self {
        Self {
            enable: false,
            write: true,
            func: Default::default(),
        }
    }
}

impl DepthMode {
    pub fn new(enable: bool, write: bool, func: DepthFunc) -> Self {
        Self {
            enable,
            write,
            func,
        }
    }

    pub fn apply(&self) {
        unsafe {
            if self.enable {
                gl::Enable(gl::DEPTH_TEST);
            } else {
                gl::Disable(gl::DEPTH_TEST);
            }
            gl::DepthMask(if self.write { gl::TRUE } else { gl::FALSE });
        }

        self.func.apply();
    }
}
