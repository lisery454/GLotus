use std::cell::Cell;

use glam::{Mat4, Vec3};

use super::TransformError;
use super::rotation::Rotation;
use super::scaling::Scaling;
use super::translation::Translation;

/// 描述一个物体的位置旋转和缩放
#[derive(Debug)]
pub struct Transform {
    translation: Translation,
    rotation: Rotation,
    scaling: Scaling,

    // 缓存相关
    is_dirty: Cell<bool>,
    cached_matrix: Cell<Mat4>,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: Default::default(),
            rotation: Default::default(),
            scaling: Default::default(),
            is_dirty: Cell::new(true),
            cached_matrix: Cell::new(Mat4::IDENTITY),
        }
    }
}

impl Transform {
    /// 从平移旋转和缩放新建
    pub fn new(translation: Translation, rotation: Rotation, scaling: Scaling) -> Self {
        Self {
            translation,
            rotation,
            scaling,
            is_dirty: Cell::new(true),
            cached_matrix: Cell::new(Mat4::IDENTITY),
        }
    }

    fn update_cache(&self) {
        let m = Mat4::from_scale_rotation_translation(
            self.scaling.data.into(),     // Vec3A -> Vec3
            self.rotation.data,           // Quat
            self.translation.data.into(), // Vec3A -> Vec3
        );
        self.cached_matrix.set(m);
        self.is_dirty.set(false);
    }

    /// 从位置的xyz生成，其他为默认
    pub fn from_position(x: f32, y: f32, z: f32) -> Self {
        Self {
            translation: Translation::new(x, y, z),
            rotation: Rotation::default(),
            scaling: Scaling::default(),
            is_dirty: Cell::new(true),
            cached_matrix: Cell::new(Mat4::IDENTITY),
        }
    }

    /// 获取变换矩阵，用多维数组表示
    pub(crate) fn to_matrix(&self) -> Mat4 {
        if self.is_dirty.get() {
            self.update_cache();
        }
        // glam 的 Mat4 转数组极快
        self.cached_matrix.get()
    }

    /// 获取法线变化矩阵
    pub(crate) fn to_normal_matrix(&self) -> Result<Mat4, TransformError> {
        // glam 的 Mat4 提供了更快的逆矩阵计算
        let model_mat = if self.is_dirty.get() {
            self.update_cache();
            self.cached_matrix.get()
        } else {
            self.cached_matrix.get()
        };

        // 法线矩阵：模型矩阵的逆转置
        // 注意：如果缩放是统一的，其实可以直接用旋转矩阵
        let inverse = model_mat.inverse();
        if inverse.is_nan() {
            return Err(TransformError::InverseMatrixFail);
        }

        // 只有 3x3 部分对法线有用
        Ok(inverse.transpose())
    }

    pub(crate) fn get_view_matrix(&self) -> Mat4 {
        // glam 使用的是 look_to_lh 或 look_to_rh
        Mat4::look_to_rh(
            self.translation.data.into(),
            self.get_forward(),
            self.get_up(),
        )
    }

    /// 获取平移引用
    pub fn get_translation(&self) -> &Translation {
        &self.translation
    }

    /// 获取平移的可变引用
    pub fn get_translation_mut(&mut self) -> &mut Translation {
        self.is_dirty.set(true);
        &mut self.translation
    }

    /// 设置平移
    pub fn set_translation(&mut self, translation: Translation) {
        self.translation = translation;
    }

    /// 获取缩放引用
    pub fn get_scaling(&self) -> &Scaling {
        &self.scaling
    }

    /// 获取缩放的可变引用
    pub fn get_scaling_mut(&mut self) -> &mut Scaling {
        self.is_dirty.set(true);
        &mut self.scaling
    }

    /// 设置缩放
    pub fn set_scaling(&mut self, scaling: Scaling) {
        self.scaling = scaling;
    }

    /// 获取旋转
    pub fn get_rotation(&self) -> &Rotation {
        &self.rotation
    }

    /// 获取旋转的可变引用
    pub fn get_rotation_mut(&mut self) -> &mut Rotation {
        self.is_dirty.set(true);
        &mut self.rotation
    }

    /// 设置旋转
    pub fn set_rotation(&mut self, rotation: Rotation) {
        self.rotation = rotation;
    }

    pub fn get_forward(&self) -> Vec3 {
        // Quat * Vec3 是优化过的 SIMD 路径
        (self.rotation.data * Vec3::NEG_Z).normalize()
    }

    pub fn get_right(&self) -> Vec3 {
        (self.rotation.data * Vec3::X).normalize()
    }

    pub fn get_up(&self) -> Vec3 {
        (self.rotation.data * Vec3::Y).normalize()
    }
}
