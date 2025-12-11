use crate::{Color, render::transform::Transform};

/// 光源类型
#[derive(Clone, Copy)]
pub enum LightType {
    Directional,
    Point,
    Spot,
    Area,
    Custom(u32), // 扩展用
}

/// 光源shader数据
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LightShaderData {
    pub light_type: i32,
    pub color: [f32; 3],
    pub position: [f32; 3],
    pub direction: [f32; 3],
    pub intensity: f32,
    pub range: f32,
    pub inner_cone: f32,
    pub outer_cone: f32,
}

/// 光源的trait
pub trait Light {
    /// 世界空间位置
    fn transform(&self) -> &Transform;
    fn transform_mut(&mut self) -> &mut Transform;

    /// 光强信息
    fn color(&self) -> Color;
    fn intensity(&self) -> f32;

    /// 返回光源类型
    fn light_type(&self) -> LightType;

    /// 将光源数据打包成 shader 需要的 uniform 结构
    fn to_shader_data(&self) -> LightShaderData;
}
