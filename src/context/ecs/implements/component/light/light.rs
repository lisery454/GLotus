use crate::{Color, IComponent};
use downcast_rs::{Downcast, impl_downcast};
use std::ops::{Deref, DerefMut};

/// 光源类型
#[derive(Clone, Copy)]
pub enum LightType {
    Directional,
    Point,
    Spot,
}

/// 光源的trait
pub trait ILight: Downcast {
    /// 光强信息
    fn color(&self) -> Color;
    fn intensity(&self) -> f32;

    /// 返回光源类型
    fn light_type(&self) -> LightType;
}
impl_downcast!(ILight);

pub struct Light(pub Box<dyn ILight>);

impl IComponent for Light {}

// 从 Box<dyn ILight> 转换
impl From<Box<dyn ILight>> for Light {
    fn from(light: Box<dyn ILight>) -> Self {
        Light(light)
    }
}

// 从具体类型转换（会自动装箱）
impl<T: ILight + 'static> From<T> for Light {
    fn from(light: T) -> Self {
        Light(Box::new(light))
    }
}

impl Deref for Light {
    type Target = Box<dyn ILight>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Light {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
