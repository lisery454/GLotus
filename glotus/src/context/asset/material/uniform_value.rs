use crate::TextureHandle;

/// uniform的值
#[derive(Debug)]
pub enum UniformValue {
    /// 浮点数
    Float(f32),
    /// 整数
    Int(i32),
    /// vec3
    Vector3([f32; 3]),
    /// vec4
    Vector4([f32; 4]),
    /// mat3*3
    Matrix3([[f32; 3]; 3]),
    /// mat4*4
    Matrix4([[f32; 4]; 4]),
    /// tex的slot，以及贴图数据
    Texture(usize, TextureHandle), // 纹理槽位
}
