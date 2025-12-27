use cgmath::{Matrix, Matrix3, Matrix4, One, SquareMatrix, Vector3};

use super::TransformError;
use super::rotation::Rotation;
use super::scaling::Scaling;
use super::translation::Translation;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransformSpace {
    World,  // 3D 世界空间，受相机 View/Projection 矩阵影响
    Screen, // 2D 屏幕空间 (NDC)，坐标范围 [-1, 1]，不受相机移动影响
}

/// 描述一个物体的位置旋转和缩放
#[derive(Debug)]
pub struct Transform {
    pub(crate) translation: Translation,
    pub(crate) rotation: Rotation,
    pub(crate) scaling: Scaling,
    pub(crate) space: TransformSpace,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: Default::default(),
            rotation: Default::default(),
            scaling: Default::default(),
            space: TransformSpace::World,
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
            space: TransformSpace::World,
        }
    }

    pub fn from_ui(x: f32, y: f32, scale_x: f32, scale_y: f32) -> Self {
        Self {
            translation: Translation::new(x, y, 0.0), // Z 设为 0
            scaling: Scaling::new(scale_x, scale_y, 1.0),
            space: TransformSpace::Screen,
            ..Default::default()
        }
    }

    /// 从位置的xyz生成，其他为默认
    pub fn from_position(x: f32, y: f32, z: f32) -> Self {
        Self {
            translation: Translation::new(x, y, z),
            rotation: Rotation::default(),
            scaling: Scaling::default(),
            space: TransformSpace::World,
        }
    }

    /// 获取变换矩阵
    fn get_matrix(&self) -> Matrix4<f32> {
        let scaling_matrix = self.scaling.get_scale_matrix();
        let rotation_matrix = self.rotation.get_rotation_matrix();
        let translation_matrix = self.translation.get_translation_matrix();
        let matrix = translation_matrix * rotation_matrix * scaling_matrix;
        matrix
    }

    /// 获取变换矩阵，用多维数组表示
    pub(crate) fn to_matrix(&self) -> [[f32; 4]; 4] {
        self.get_matrix().into()
    }

    /// 获取法线变化矩阵
    pub(crate) fn to_normal_matrix(&self) -> Result<[[f32; 3]; 3], TransformError> {
        let inverse_matrix = self
            .get_matrix()
            .invert()
            .ok_or(TransformError::InverseMatrixFail)?;

        let inverse_model_3x3 = Matrix3::new(
            inverse_matrix[0][0],
            inverse_matrix[0][1],
            inverse_matrix[0][2],
            inverse_matrix[1][0],
            inverse_matrix[1][1],
            inverse_matrix[1][2],
            inverse_matrix[2][0],
            inverse_matrix[2][1],
            inverse_matrix[2][2],
        );

        Ok(inverse_model_3x3.transpose().into())
    }

    pub(crate) fn get_view_matrix(&self) -> Matrix4<f32> {
        if self.space == TransformSpace::World {
            Matrix4::look_to_rh(
                self.get_translation().data,
                self.get_forward(),
                self.get_up(),
            )
        } else {
            Matrix4::one().into()
        }
    }

    /// 获取平移引用
    pub fn get_translation(&self) -> &Translation {
        &self.translation
    }

    /// 获取平移的可变引用
    pub fn get_translation_mut(&mut self) -> &mut Translation {
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
        &mut self.rotation
    }

    /// 设置旋转
    pub fn set_rotation(&mut self, rotation: Rotation) {
        self.rotation = rotation;
    }

    /// 获取前向的方向，也就是-z方向
    pub fn get_forward(&self) -> Vector3<f32> {
        self.get_rotation().get_data() * -Vector3::unit_z()
    }

    /// 获取右边的方向
    pub fn get_right(&self) -> Vector3<f32> {
        self.get_rotation().get_data() * Vector3::unit_x()
    }

    /// 获取向上的方向
    pub fn get_up(&self) -> Vector3<f32> {
        self.get_rotation().get_data() * Vector3::unit_y()
    }
}
