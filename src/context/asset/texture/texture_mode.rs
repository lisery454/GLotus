/// 循环模式
#[derive(Debug)]
pub enum WrappingMode {
    /// 重复
    Repeat,
    /// 镜像重复
    MirroreroredRepeat,
    /// 边缘截至
    ClampToEdge,
    /// 边缘截至，外部是自定义的颜色
    ClampToBorder { color: [f32; 4] },
}

/// 过滤模式
#[derive(Debug)]
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
