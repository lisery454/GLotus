use glam::Mat4;

use crate::{FramebufferHandle, IComponent, MaterialHandle, Resolution};
// use cgmath::{Deg, Matrix4, Ortho, PerspectiveFov, Rad};

/// 投影类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProjectionType {
    /// 透视
    Perspective,
    /// 正交
    Orthographic,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RenderTarget {
    Screen,
    Framebuffer(FramebufferHandle),
}

impl Default for RenderTarget {
    fn default() -> Self {
        RenderTarget::Screen
    }
}

pub struct Camera {
    pub fov: f32,
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
            fov: 45.0_f32.to_radians(),
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

    pub fn with_far_plane(mut self, far_plane: f32) -> Self {
        self.far_plane = far_plane;
        self
    }

    pub fn with_near_plane(mut self, near_plane: f32) -> Self {
        self.near_plane = near_plane;
        self
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
        self.fov = angle.to_radians();
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
    pub fn set_aspect_ratio(&mut self, resolution: Resolution) {
        self.aspect_ratio = resolution.aspect_ratio();
    }

    /// 获取相机比例
    pub fn get_aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }

    /// 获取投影矩阵：视图空间到裁切空间
    pub(crate) fn get_projection_matrix(&self) -> Mat4 {
        let matrix = match self.projection_type {
            ProjectionType::Perspective => {
                // 使用 _rh_gl 确保深度范围映射到 [-1.0, 1.0]
                Mat4::perspective_rh_gl(
                    self.fov,
                    self.aspect_ratio,
                    self.near_plane,
                    self.far_plane,
                )
            }
            ProjectionType::Orthographic => {
                let half_height = self.fov / 2.0;
                let half_width = half_height * self.aspect_ratio;

                // 正交投影同样使用 _rh_gl
                Mat4::orthographic_rh_gl(
                    -half_width,  // left
                    half_width,   // right
                    -half_height, // bottom
                    half_height,  // top
                    self.near_plane,
                    self.far_plane,
                )
            }
        };

        matrix
    }
}
