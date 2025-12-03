use cgmath::{Matrix, Matrix3, Matrix4, SquareMatrix};

use super::position::Position;
use super::rotation::Rotation;
use super::scale::Scale;

#[derive(Debug)]
pub struct Transform {
    position: Position,
    rotation: Rotation,
    scale: Scale,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Default::default(),
            rotation: Default::default(),
            scale: Default::default(),
        }
    }
}

impl Transform {
    pub fn new(position: Position, rotation: Rotation, scale: Scale) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }

    pub fn from_position(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: Position::new(x, y, z),
            rotation: Rotation::default(),
            scale: Scale::default(),
        }
    }

    fn get_model_matrix(&self) -> Matrix4<f32> {
        // 1. 生成缩放矩阵
        let scale_matrix = self.scale.get_scale_matrix();

        // 2. 生成旋转矩阵（从四元数转换）
        let rotation_matrix = self.rotation.get_rotation_matrix();

        // 3. 生成平移矩阵
        let translation_matrix = self.position.get_translation_matrix();

        // 4. 组合变换：T * R * S（注意顺序！）
        let matrix = translation_matrix * rotation_matrix * scale_matrix;
        matrix
    }

    pub fn to_matrix(&self) -> [[f32; 4]; 4] {
        self.get_model_matrix().into()
    }

    pub fn to_normal_matrix(&self) -> [[f32; 3]; 3] {
        // 1. 计算模型矩阵的逆矩阵
        let inverse_model_matrix = self
            .get_model_matrix()
            .invert()
            .expect("Model matrix must be invertible for normal transformation");

        // 2. 取逆矩阵的左上 3x3 部分
        let inverse_model_3x3 = Matrix3::new(
            inverse_model_matrix[0][0],
            inverse_model_matrix[0][1],
            inverse_model_matrix[0][2],
            inverse_model_matrix[1][0],
            inverse_model_matrix[1][1],
            inverse_model_matrix[1][2],
            inverse_model_matrix[2][0],
            inverse_model_matrix[2][1],
            inverse_model_matrix[2][2],
        );

        // 3. 转置 3x3 矩阵得到法线矩阵
        inverse_model_3x3.transpose().into()
    }

    pub fn get_position(&self) -> &Position {
        &self.position
    }

    pub fn get_position_mut(&mut self) -> &mut Position {
        &mut self.position
    }

    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    pub fn get_scale(&self) -> &Scale {
        &self.scale
    }

    pub fn get_scale_mut(&mut self) -> &mut Scale {
        &mut self.scale
    }

    pub fn set_scale(&mut self, scale: Scale) {
        self.scale = scale;
    }

    pub fn get_rotation(&self) -> &Rotation {
        &self.rotation
    }

    pub fn get_rotation_mut(&mut self) -> &mut Rotation {
        &mut self.rotation
    }

    pub fn set_rotation(&mut self, rotation: Rotation) {
        self.rotation = rotation;
    }
}
