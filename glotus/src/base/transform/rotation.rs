use cgmath::{Euler, InnerSpace, Matrix4, Quaternion, Rad, Vector3};

/// 旋转
#[derive(Debug, Clone, Copy)]
pub struct Rotation {
    data: Quaternion<f32>,
}

impl Default for Rotation {
    /// 默认是0
    fn default() -> Self {
        Rotation::zero()
    }
}

impl From<Quaternion<f32>> for Rotation {
    /// 从4元数生成
    fn from(value: Quaternion<f32>) -> Self {
        Self { data: value }
    }
}

impl Rotation {
    /// 从角度生成旋转，是角度值
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            data: Quaternion::from(Euler {
                x: Rad(x.to_radians()),
                y: Rad(y.to_radians()),
                z: Rad(z.to_radians()),
            }),
        }
    }

    /// 0旋转
    pub fn zero() -> Self {
        Rotation::new(0.0, 0.0, 0.0)
    }

    /// 获取旋转对应的矩阵
    pub(crate) fn get_rotation_matrix(&self) -> Matrix4<f32> {
        Matrix4::from(self.data)
    }

    /// 获取内部4元数数据
    pub(crate) fn get_data(&self) -> Quaternion<f32> {
        self.data
    }

    /// 旋转一个4元数的值
    pub(crate) fn rotate(&mut self, delta: Quaternion<f32>) {
        self.data = (delta * self.data).normalize();
    }

    /// 获取指向z负数方向轴的旋转分量
    pub(crate) fn forward(&self) -> Vector3<f32> {
        let forward = Vector3::new(0.0, 0.0, -1.0);
        (self.data * forward).normalize()
    }
}
