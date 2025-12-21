use crate::{Color, IComponent};
use downcast_rs::{Downcast, impl_downcast};

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

pub struct LightComponent {
    pub light: Box<dyn ILight>,
}

impl IComponent for LightComponent {}

impl LightComponent {
    pub fn new<L>(light: L) -> Self
    where
        L: ILight + 'static,
    {
        Self {
            light: Box::new(light),
        }
    }
}
