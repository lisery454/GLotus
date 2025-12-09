use std::{cell::RefCell, rc::Rc};

use crate::Texture2D;

#[derive(Debug)]
pub enum UniformValue {
    Float(f32),
    Int(i32),
    Vector3([f32; 3]),
    Vector4([f32; 4]),
    Matrix3([[f32; 3]; 3]),
    Matrix4([[f32; 4]; 4]),
    Texture(usize, Rc<RefCell<Texture2D>>), // 纹理槽位
}
