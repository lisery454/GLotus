use glam::{EulerRot, Mat4, Quat, Vec3};

/// 旋转
#[derive(Debug, Clone, Copy)]
pub struct Rotation {
    pub(crate) data: Quat,
}

impl Default for Rotation {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl From<Quat> for Rotation {
    #[inline]
    fn from(value: Quat) -> Self {
        Self { data: value }
    }
}

impl Rotation {
    pub const IDENTITY: Self = Self {
        data: Quat::IDENTITY,
    };

    /// 从角度生成旋转，是角度值
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            // glam 使用 EulerRot 指定旋转顺序（通常是 YXZ 或 XYZ）
            data: Quat::from_euler(
                EulerRot::YXZ, // 常用顺序：偏航(Y)->俯仰(X)->翻滚(Z)
                y.to_radians(),
                x.to_radians(),
                z.to_radians(),
            ),
        }
    }

    /// 0旋转
    pub fn zero() -> Self {
        Self::IDENTITY
    }

    /// 获取旋转对应的矩阵
    pub(crate) fn get_rotation_matrix(&self) -> Mat4 {
        // 直接从四元数生成 4x4 矩阵
        Mat4::from_quat(self.data)
    }

    /// 旋转一个4元数的值
    pub(crate) fn rotate(&mut self, delta: Quat) {
        // glam 中四元数乘法顺序与 cgmath 一致：delta * self.data
        self.data = (delta * self.data).normalize();
    }

    /// 获取指向z负数方向轴的旋转分量
    pub(crate) fn forward(&self) -> Vec3 {
        // 直接乘法即可，glam 内部优化了四元数旋转向量的路径
        let forward = Vec3::new(0.0, 0.0, -1.0);
        (self.data * forward).normalize()
    }
}
