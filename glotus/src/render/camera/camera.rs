use cgmath::{Deg, Matrix4, Ortho, PerspectiveFov, Rad, Vector3};

use crate::Transform;

use super::projection_type::ProjectionType;

pub struct Camera {
    pub(crate) transform: Transform,
    pub(crate) fov: Deg<f32>,
    pub(crate) aspect_ratio: f32,
    pub(crate) near_plane: f32,
    pub(crate) far_plane: f32,
    pub(crate) projection_type: ProjectionType,
}

impl Camera {
    /// 创建一个默认的相机
    pub fn new() -> Self {
        Self {
            transform: Transform::default(),
            fov: Deg(45.0),
            aspect_ratio: 16.0 / 9.0,
            near_plane: 0.1,
            far_plane: 100.0,
            projection_type: ProjectionType::Perspective,
        }
    }

    /// 获取transform的引用
    pub fn get_transform(&self) -> &Transform {
        &self.transform
    }

    /// 获取transform的可变引用
    pub fn get_transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    /// 设置transform
    pub fn set_transform(&mut self, transform: Transform) {
        self.transform = transform
    }

    /// 设置相机比例
    pub fn set_aspect_ratio(&mut self, width: u32, height: u32) {
        self.aspect_ratio = width as f32 / height as f32;
    }

    /// 获取相机比例
    pub fn get_aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }

    /// 获取视口矩阵：世界空间到视图空间
    pub(crate) fn get_view_matrix(&self) -> [[f32; 4]; 4] {
        Matrix4::look_to_rh(
            self.transform.get_translation().data,
            self.get_forward(),
            self.get_up(),
        )
        .into()
    }

    /// 获取相机位置
    pub(crate) fn get_view_position(&self) -> [f32; 3] {
        self.get_transform().get_translation().get_arr()
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

    /// 获取前向的方向，也就是-z方向
    pub fn get_forward(&self) -> Vector3<f32> {
        self.transform.get_rotation().get_data() * -Vector3::unit_z()
    }

    /// 获取右边的方向
    pub fn get_right(&self) -> Vector3<f32> {
        self.transform.get_rotation().get_data() * Vector3::unit_x()
    }

    /// 获取向上的方向
    pub fn get_up(&self) -> Vector3<f32> {
        self.transform.get_rotation().get_data() * Vector3::unit_y()
    }
}


/// 相机的shader数据，用来传递给shader
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CameraShaderData {
    pub camera_type: i32,
    pub fov: f32,
    pub direction: [f32; 3],
    pub position: [f32; 3],
    pub aspect_ratio: f32,
    pub near_plane: f32,
    pub far_plane: f32,
}

impl Camera {
    /// 转换成shader数据
    pub fn to_shader_data(&self) -> CameraShaderData {
        CameraShaderData {
            camera_type: if self.projection_type == ProjectionType::Perspective {
                0
            } else {
                1
            },
            fov: self.fov.0, // Deg<f32> 解包成 f32
            position: self.transform.get_translation().get_arr().into(),
            direction: self.transform.get_rotation().forward().into(),
            aspect_ratio: self.aspect_ratio,
            near_plane: self.near_plane,
            far_plane: self.far_plane,
        }
    }
}
