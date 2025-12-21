use cgmath::{Deg, Matrix4, Ortho, PerspectiveFov, Rad};

use crate::IComponent;

use super::projection_type::ProjectionType;

pub struct CameraComponent {
    pub fov: Deg<f32>,
    pub aspect_ratio: f32,
    pub near_plane: f32,
    pub far_plane: f32,
    pub projection_type: ProjectionType,
    pub(crate) is_initialized: bool,
    pub(crate) is_active: bool, // 是否为主相机
}

impl IComponent for CameraComponent {}

impl CameraComponent {
    /// 创建一个默认的相机
    pub fn new(is_active: bool) -> Self {
        Self {
            fov: Deg(45.0),
            aspect_ratio: 16.0 / 9.0,
            near_plane: 0.1,
            far_plane: 100.0,
            projection_type: ProjectionType::Perspective,
            is_initialized: false,
            is_active,
        }
    }

    /// 设置相机比例
    pub fn set_aspect_ratio(&mut self, width: u32, height: u32) {
        self.aspect_ratio = width as f32 / height as f32;
    }

    /// 获取相机比例
    pub fn get_aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }

    /// 获取投影矩阵：视图空间到裁切空间
    pub(crate) fn get_projection_matrix(&self) -> [[f32; 4]; 4] {
        let matrix: Matrix4<f32> = match self.projection_type {
            ProjectionType::Perspective => PerspectiveFov {
                fovy: Rad::from(self.fov),
                aspect: self.aspect_ratio,
                near: self.near_plane,
                far: self.far_plane,
            }
            .into(),
            ProjectionType::Orthographic => {
                let half_height = self.fov.0 / 2.0;
                let half_width = half_height * self.aspect_ratio;
                Ortho {
                    left: -half_width,
                    right: half_width,
                    bottom: -half_height,
                    top: half_height,
                    near: self.near_plane,
                    far: self.far_plane,
                }
                .into()
            }
        };

        matrix.into()
    }
}
