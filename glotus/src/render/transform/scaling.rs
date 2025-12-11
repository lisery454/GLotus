use cgmath::{Matrix4, Vector3};

/// 缩放
#[derive(Debug, Clone, Copy)]
pub struct Scaling {
    pub(crate) data: Vector3<f32>,
}

impl Default for Scaling {
    /// 默认是1，也就是不缩放
    fn default() -> Self {
        Scaling::one()
    }
}

impl Scaling {
    /// 从三维数据创建缩放
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            data: Vector3::new(x, y, z),
        }
    }

    /// 1缩放
    pub fn one() -> Self {
        Self {
            data: Vector3::new(1.0, 1.0, 1.0),
        }
    }

    /// 获取缩放矩阵
    pub(crate) fn get_scale_matrix(&self) -> Matrix4<f32> {
        Matrix4::from_nonuniform_scale(self.data.x, self.data.y, self.data.z)
    }

    /// 缩放一个另一个缩放
    pub fn scale(&mut self, delta: Scaling) {
        self.data.x *= delta.data.x;
        self.data.y *= delta.data.y;
        self.data.z *= delta.data.z;
    }
}
