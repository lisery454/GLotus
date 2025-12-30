use cgmath::{Deg, Matrix4, Ortho, PerspectiveFov, Rad};

use crate::{FramebufferHandle, IComponent, MaterialHandle};

use super::{RenderTarget, projection_type::ProjectionType};

pub struct Camera {
    pub fov: Deg<f32>,
    pub aspect_ratio: f32,
    pub near_plane: f32,
    pub far_plane: f32,
    pub projection_type: ProjectionType,
    pub(crate) is_initialized: bool,
    pub(crate) is_active: bool, // 是否为主相机
    pub target: RenderTarget,
    pub order: i32, // 渲染顺序，较小的值先渲染
    pub postprocess_materials: Vec<MaterialHandle>,
}

impl IComponent for Camera {}

impl Camera {
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
            target: RenderTarget::Screen,
            order: 0,
            postprocess_materials: vec![],
        }
    }

    /// 添加后处理材质
    pub fn with_postprocess_material(mut self, material: MaterialHandle) -> Self {
        self.postprocess_materials.push(material);
        self
    }

    /// 添加多个后处理材质
    pub fn with_postprocess_materials(mut self, materials: Vec<MaterialHandle>) -> Self {
        self.postprocess_materials = materials;
        self
    }

    /// 检查是否有后处理
    pub fn has_postprocess(&self) -> bool {
        !self.postprocess_materials.is_empty()
    }

    /// 运行时添加后处理材质
    pub fn add_postprocess_material(&mut self, material: MaterialHandle) {
        self.postprocess_materials.push(material);
    }

    /// 清空后处理材质
    pub fn clear_postprocess_materials(&mut self) {
        self.postprocess_materials.clear();
    }

    pub fn with_fov(mut self, angle: f32) -> Self {
        self.fov = Deg(angle);
        self
    }

    pub fn with_projection_type(mut self, projection_type: ProjectionType) -> Self {
        self.projection_type = projection_type;
        self
    }

    /// 设置渲染目标为屏幕
    pub fn with_target_screen(mut self) -> Self {
        self.target = RenderTarget::Screen;
        self
    }

    /// 设置渲染目标为framebuffer
    pub fn with_target_framebuffer(mut self, framebuffer: FramebufferHandle) -> Self {
        self.target = RenderTarget::Framebuffer(framebuffer);
        self
    }

    /// 获取渲染目标
    pub fn get_target(&self) -> RenderTarget {
        self.target
    }

    /// 检查是否渲染到屏幕
    pub fn is_rendering_to_screen(&self) -> bool {
        matches!(self.target, RenderTarget::Screen)
    }

    /// 检查是否渲染到framebuffer
    pub fn is_rendering_to_framebuffer(&self) -> bool {
        matches!(self.target, RenderTarget::Framebuffer(_))
    }

    pub fn with_order(mut self, order: i32) -> Self {
        self.order = order;
        self
    }

    pub fn with_aspect_ratio(mut self, ratio: f32) -> Self {
        self.aspect_ratio = ratio;
        self
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
