/// 投影类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProjectionType {
    /// 透视
    Perspective,
    /// 正交
    Orthographic,
}
