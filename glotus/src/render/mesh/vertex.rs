use cgmath::{Vector2, Vector3};
use gl::types::*;

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub tex_coord: Vector2<f32>,
    pub tangent: Vector3<f32>,
    pub bitangent: Vector3<f32>,
    pub color: Vector3<f32>,
}

impl Eq for Vertex {}

impl Vertex {
    pub fn layout() -> Vec<VertexAttribute> {
        let mut offset = 0;
        let mut attrs = Vec::new();

        // 位置
        attrs.push(VertexAttribute {
            index: 0,
            size: 3,
            offset,
        });
        offset += std::mem::size_of::<cgmath::Vector3<f32>>();

        // 法线
        attrs.push(VertexAttribute {
            index: 1,
            size: 3,
            offset,
        });
        offset += std::mem::size_of::<cgmath::Vector3<f32>>();

        // 纹理坐标
        attrs.push(VertexAttribute {
            index: 2,
            size: 2,
            offset,
        });
        offset += std::mem::size_of::<cgmath::Vector2<f32>>();

        // 可选：切线
        if std::mem::size_of::<Option<cgmath::Vector3<f32>>>() > 0 {
            attrs.push(VertexAttribute {
                index: 3,
                size: 3,
                offset,
            });
            offset += std::mem::size_of::<Option<cgmath::Vector3<f32>>>();
        }

        // 可选：副切线
        if std::mem::size_of::<Option<cgmath::Vector3<f32>>>() > 0 {
            attrs.push(VertexAttribute {
                index: 4,
                size: 3,
                offset,
            });
            offset += std::mem::size_of::<Option<cgmath::Vector3<f32>>>();
        }

        // 可选：颜色
        if std::mem::size_of::<Option<cgmath::Vector3<f32>>>() > 0 {
            attrs.push(VertexAttribute {
                index: 5,
                size: 3,
                offset,
            });
            // offset += std::mem::size_of::<Option<cgmath::Vector3<f32>>>();
        }

        attrs
    }
}

pub struct VertexAttribute {
    pub index: GLuint,
    pub size: GLint,
    pub offset: usize,
}

impl Vertex {
    pub fn from_position(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: Vector3::<f32>::new(x, y, z),
            normal: Vector3::<f32>::new(0.0, 0.0, 0.0),
            tex_coord: Vector2::<f32>::new(0.0, 0.0),
            tangent: Vector3::<f32>::new(0.0, 0.0, 0.0),
            bitangent: Vector3::<f32>::new(0.0, 0.0, 0.0),
            color: Vector3::<f32>::new(0.0, 0.0, 0.0),
        }
    }

    pub fn from_position_and_tex_coords(
        x: f32,
        y: f32,
        z: f32,
        tex_coord_x: f32,
        tex_coord_y: f32,
    ) -> Self {
        Self {
            position: Vector3::<f32>::new(x, y, z),
            normal: Vector3::<f32>::new(0.0, 0.0, 0.0),
            tex_coord: Vector2::<f32>::new(tex_coord_x, tex_coord_y),
            tangent: Vector3::<f32>::new(0.0, 0.0, 0.0),
            bitangent: Vector3::<f32>::new(0.0, 0.0, 0.0),
            color: Vector3::<f32>::new(0.0, 0.0, 0.0),
        }
    }

    pub fn from_position_and_normal_and_tex_coords(
        x: f32,
        y: f32,
        z: f32,
        normal_x: f32,
        normal_y: f32,
        normal_z: f32,
        tex_coord_x: f32,
        tex_coord_y: f32,
    ) -> Self {
        Self {
            position: Vector3::<f32>::new(x, y, z),
            normal: Vector3::<f32>::new(normal_x, normal_y, normal_z),
            tex_coord: Vector2::<f32>::new(tex_coord_x, tex_coord_y),
            tangent: Vector3::<f32>::new(0.0, 0.0, 0.0),
            bitangent: Vector3::<f32>::new(0.0, 0.0, 0.0),
            color: Vector3::<f32>::new(0.0, 0.0, 0.0),
        }
    }
}
