use glam::{Mat4, Vec3, Vec3A};

/// 平移
#[derive(Debug, Clone, Copy)]
pub struct Translation {
    pub(crate) data: Vec3A,
}

impl Default for Translation {
    /// 默认在原点
    fn default() -> Self {
        Translation::zero()
    }
}

impl From<Translation> for [f32; 3] {
    /// 返回xyz数组
    fn from(value: Translation) -> Self {
        [value.data.x, value.data.y, value.data.z]
    }
}

impl Translation {
    /// 从xyz生成位置
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            data: Vec3A::new(x, y, z),
        }
    }

    pub(crate) fn from_vec(v: Vec3) -> Self {
        Self {
            data: Vec3A::new(v.x, v.y, v.z),
        }
    }

    /// 原点位置
    pub fn zero() -> Self {
        Self {
            data: Vec3A::new(0.0, 0.0, 0.0),
        }
    }

    /// 设置x的值
    pub fn set_x(&mut self, value: f32) {
        self.data.x = value;
    }

    /// 设置y的值
    pub fn set_y(&mut self, value: f32) {
        self.data.y = value;
    }

    /// 设置z的值
    pub fn set_z(&mut self, value: f32) {
        self.data.z = value;
    }

    /// 获取xyz的数组
    pub fn get_arr(&self) -> [f32; 3] {
        return [self.data.x, self.data.y, self.data.z];
    }

    /// 获取移动的矩阵
    pub(super) fn get_translation_matrix(&self) -> Mat4 {
        let mut m = Mat4::IDENTITY;
        m.w_axis = self.data.extend(1.0); // 将 Vec3A 扩展为 Vec4 并存入第四列
        m
    }

    /// 平移一个矢量
    pub fn translate(&mut self, delta: Translation) {
        self.data += delta.data;
    }
}
