use crate::Color;

/// 循环模式
#[derive(Debug, Clone, Copy)]
pub enum WrappingMode {
    /// 重复
    Repeat,
    /// 镜像重复
    MirroreroredRepeat,
    /// 边缘截至
    ClampToEdge,
    /// 边缘截至，外部是自定义的颜色
    ClampToBorder { color: Color },
}

/// 过滤模式
#[derive(Debug, Clone, Copy)]
pub enum FilteringMode {
    /// 最近的像素
    Nearest,
    /// 线性组合值
    Linear,
    /// 最近的mipmap上最近的像素
    NearestMipmapNearest,
    /// 线性组合mipmap值上最近的像素
    LinearMipmapNearest,
    /// 最近的mipmap上线性组合
    NearestMipmapLinear,
    /// 线性组合mipmap值上线性组合
    LinearMipmapLinear,
}

/// 图片中的颜色存储格式
#[derive(Debug, Clone, Copy)]
pub enum FormatType {
    RGB,
    RGBA,
    SRGB,
    SRGBA,
}

impl FormatType {
    /// 映射到 OpenGL 的 internalformat (用于显存存储格式)
    pub fn as_gl_internal_format(&self) -> i32 {
        match self {
            FormatType::RGB => gl::RGB8 as i32,           // 线性 RGB
            FormatType::RGBA => gl::RGBA8 as i32,         // 线性 RGBA
            FormatType::SRGB => gl::SRGB8 as i32,         // sRGB 颜色空间
            FormatType::SRGBA => gl::SRGB8_ALPHA8 as i32, // sRGB 颜色空间 + Alpha
        }
    }

    /// 映射到 OpenGL 的 format (用于内存数据排列格式)
    pub fn as_gl_format(&self) -> u32 {
        match self {
            FormatType::RGB | FormatType::SRGB => gl::RGB,
            FormatType::RGBA | FormatType::SRGBA => gl::RGBA,
        }
    }
}
