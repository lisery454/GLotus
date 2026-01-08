use cgmath::{Vector2, Vector3};
use gl::types::*;
use std::mem;
use std::ptr;

/// 顶点属性标志位
#[derive(Debug, Clone, Copy)]
pub struct VertexAttributes {
    pub position: bool,  // 位置 (必需)
    pub normal: bool,    // 法线
    pub tangent: bool,   // 切线
    pub bitangent: bool, // 副切线
    pub uv: bool,        // 2D UV坐标
    pub uv3d: bool,      // 3D UV坐标
    pub color: bool,     // 顶点颜色
}

impl Default for VertexAttributes {
    fn default() -> Self {
        Self {
            position: true,
            normal: false,
            tangent: false,
            bitangent: false,
            uv: false,
            uv3d: false,
            color: false,
        }
    }
}

/// 顶点数据
#[derive(Debug, Clone)]
pub struct VertexData {
    pub positions: Vec<Vector3<f32>>,
    pub normals: Option<Vec<Vector3<f32>>>,
    pub tangents: Option<Vec<Vector3<f32>>>,
    pub bitangents: Option<Vec<Vector3<f32>>>,
    pub uvs: Option<Vec<Vector2<f32>>>,
    pub uvs_3d: Option<Vec<Vector3<f32>>>,
    pub colors: Option<Vec<Vector3<f32>>>,
}

impl VertexData {
    pub fn new(positions: Vec<Vector3<f32>>) -> Self {
        Self {
            positions,
            normals: None,
            tangents: None,
            bitangents: None,
            uvs: None,
            uvs_3d: None,
            colors: None,
        }
    }

    pub fn with_normals(mut self, normals: Vec<Vector3<f32>>) -> Self {
        self.normals = Some(normals);
        self
    }

    pub fn with_tangents(mut self, tangents: Vec<Vector3<f32>>) -> Self {
        self.tangents = Some(tangents);
        self
    }

    pub fn with_bitangents(mut self, bitangents: Vec<Vector3<f32>>) -> Self {
        self.bitangents = Some(bitangents);
        self
    }

    pub fn with_uvs(mut self, uvs: Vec<Vector2<f32>>) -> Self {
        self.uvs = Some(uvs);
        self
    }

    pub fn with_uvs_3d(mut self, uvs_3d: Vec<Vector3<f32>>) -> Self {
        self.uvs_3d = Some(uvs_3d);
        self
    }

    pub fn with_colors(mut self, colors: Vec<Vector3<f32>>) -> Self {
        self.colors = Some(colors);
        self
    }
}

/// Mesh类
pub struct Mesh {
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,
    index_count: i32,
    vertex_count: i32,
    attributes: VertexAttributes,
}

impl Mesh {
    /// 从顶点数据和索引创建Mesh
    pub fn from_data(vertex_data: VertexData, indices: Vec<u32>) -> Result<Self, String> {
        let vertex_count = vertex_data.positions.len() as i32;
        let index_count = indices.len() as i32;

        // 检测实际存在的属性
        let attributes = VertexAttributes {
            position: true,
            normal: vertex_data.normals.is_some(),
            tangent: vertex_data.tangents.is_some(),
            bitangent: vertex_data.bitangents.is_some(),
            uv: vertex_data.uvs.is_some(),
            uv3d: vertex_data.uvs_3d.is_some(),
            color: vertex_data.colors.is_some(),
        };

        // 交错顶点数据
        let interleaved = Self::interleave_vertices(&vertex_data, &attributes)?;

        let (vao, vbo, ebo) = unsafe {
            let mut vao = 0;
            let mut vbo = 0;
            let mut ebo = 0;

            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);

            // 顶点缓冲
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (interleaved.len() * mem::size_of::<f32>()) as GLsizeiptr,
                interleaved.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // 索引缓冲
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * mem::size_of::<u32>()) as GLsizeiptr,
                indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // 设置顶点属性
            Self::setup_vertex_attribs(&attributes);

            gl::BindVertexArray(0);

            (vao, vbo, ebo)
        };

        Ok(Self {
            vao,
            vbo,
            ebo,
            index_count,
            vertex_count,
            attributes,
        })
    }

    /// 从OBJ文件创建Mesh
    pub fn from_obj(path: &str) -> Result<Self, String> {
        let (models, _) = tobj::load_obj(
            path,
            &tobj::LoadOptions {
                single_index: true,
                triangulate: true,
                ..Default::default()
            },
        )
        .map_err(|e| format!("Failed to load OBJ: {}", e))?;

        if models.is_empty() {
            return Err("No models found in OBJ file".to_string());
        }

        // 使用第一个模型
        let mesh = &models[0].mesh;

        // 转换位置
        let positions: Vec<Vector3<f32>> = mesh
            .positions
            .chunks(3)
            .map(|p| Vector3::new(p[0], p[1], p[2]))
            .collect();

        let mut vertex_data = VertexData::new(positions);

        // 转换法线
        if !mesh.normals.is_empty() {
            let normals: Vec<Vector3<f32>> = mesh
                .normals
                .chunks(3)
                .map(|n| Vector3::new(n[0], n[1], n[2]))
                .collect();
            vertex_data = vertex_data.with_normals(normals);
        }

        // 转换UV
        if !mesh.texcoords.is_empty() {
            let uvs: Vec<Vector2<f32>> = mesh
                .texcoords
                .chunks(2)
                .map(|uv| Vector2::new(uv[0], uv[1]))
                .collect();
            vertex_data = vertex_data.with_uvs(uvs);
        }

        Self::from_data(vertex_data, mesh.indices.clone())
    }

    /// 绘制Mesh
    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElements(
                gl::TRIANGLES,
                self.index_count,
                gl::UNSIGNED_INT,
                ptr::null(),
            );
            gl::BindVertexArray(0);
        }
    }

    /// 获取顶点属性信息
    pub fn attributes(&self) -> &VertexAttributes {
        &self.attributes
    }

    /// 获取顶点数量
    pub fn vertex_count(&self) -> i32 {
        self.vertex_count
    }

    /// 获取索引数量
    pub fn index_count(&self) -> i32 {
        self.index_count
    }

    // 交错顶点数据
    fn interleave_vertices(
        data: &VertexData,
        attrs: &VertexAttributes,
    ) -> Result<Vec<f32>, String> {
        let count = data.positions.len();

        // 验证所有数据长度一致
        if let Some(ref normals) = data.normals {
            if normals.len() != count {
                return Err("Normals count mismatch".to_string());
            }
        }
        if let Some(ref uvs) = data.uvs {
            if uvs.len() != count {
                return Err("UVs count mismatch".to_string());
            }
        }

        let stride = Self::calculate_stride(attrs);
        let mut result = Vec::with_capacity(count * stride);

        for i in 0..count {
            // 位置 (必需)
            result.extend_from_slice(&[
                data.positions[i].x,
                data.positions[i].y,
                data.positions[i].z,
            ]);

            // 法线
            if attrs.normal {
                if let Some(ref normals) = data.normals {
                    result.extend_from_slice(&[normals[i].x, normals[i].y, normals[i].z]);
                }
            }

            // 切线
            if attrs.tangent {
                if let Some(ref tangents) = data.tangents {
                    result.extend_from_slice(&[tangents[i].x, tangents[i].y, tangents[i].z]);
                }
            }

            // 副切线
            if attrs.bitangent {
                if let Some(ref bitangents) = data.bitangents {
                    result.extend_from_slice(&[bitangents[i].x, bitangents[i].y, bitangents[i].z]);
                }
            }

            // 2D UV
            if attrs.uv {
                if let Some(ref uvs) = data.uvs {
                    result.extend_from_slice(&[uvs[i].x, uvs[i].y]);
                }
            }

            // 3D UV
            if attrs.uv3d {
                if let Some(ref uvs_3d) = data.uvs_3d {
                    result.extend_from_slice(&[uvs_3d[i].x, uvs_3d[i].y, uvs_3d[i].z]);
                }
            }

            // 颜色
            if attrs.color {
                if let Some(ref colors) = data.colors {
                    result.extend_from_slice(&[colors[i].x, colors[i].y, colors[i].z]);
                }
            }
        }

        Ok(result)
    }

    // 计算步长
    fn calculate_stride(attrs: &VertexAttributes) -> usize {
        let mut stride = 3; // 位置
        if attrs.normal {
            stride += 3;
        }
        if attrs.tangent {
            stride += 3;
        }
        if attrs.bitangent {
            stride += 3;
        }
        if attrs.uv {
            stride += 2;
        }
        if attrs.uv3d {
            stride += 3;
        }
        if attrs.color {
            stride += 3;
        }
        stride
    }

    // 设置顶点属性指针 - 固定location版本
    // 无论属性是否存在，location始终保持固定
    fn setup_vertex_attribs(attrs: &VertexAttributes) {
        unsafe {
            let stride = (Self::calculate_stride(attrs) * mem::size_of::<f32>()) as GLsizei;
            let mut offset = 0;

            // 位置 (固定 location 0)
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, offset as *const _);
            offset += 3 * mem::size_of::<f32>();

            // 法线 (固定 location 1)
            if attrs.normal {
                gl::EnableVertexAttribArray(1);
                gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, offset as *const _);
                offset += 3 * mem::size_of::<f32>();
            }

            // 切线 (固定 location 2)
            if attrs.tangent {
                gl::EnableVertexAttribArray(2);
                gl::VertexAttribPointer(2, 3, gl::FLOAT, gl::FALSE, stride, offset as *const _);
                offset += 3 * mem::size_of::<f32>();
            }

            // 副切线 (固定 location 3)
            if attrs.bitangent {
                gl::EnableVertexAttribArray(3);
                gl::VertexAttribPointer(3, 3, gl::FLOAT, gl::FALSE, stride, offset as *const _);
                offset += 3 * mem::size_of::<f32>();
            }

            // 2D UV (固定 location 4)
            if attrs.uv {
                gl::EnableVertexAttribArray(4);
                gl::VertexAttribPointer(4, 2, gl::FLOAT, gl::FALSE, stride, offset as *const _);
                offset += 2 * mem::size_of::<f32>();
            }

            // 3D UV (固定 location 5)
            if attrs.uv3d {
                gl::EnableVertexAttribArray(5);
                gl::VertexAttribPointer(5, 3, gl::FLOAT, gl::FALSE, stride, offset as *const _);
                offset += 3 * mem::size_of::<f32>();
            }

            // 颜色 (固定 location 6)
            if attrs.color {
                gl::EnableVertexAttribArray(6);
                gl::VertexAttribPointer(6, 3, gl::FLOAT, gl::FALSE, stride, offset as *const _);
            }
        }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
        }
    }
}

// 使用示例
#[allow(dead_code)]
fn example_usage() {
    // 方式1: 从数据创建
    let positions = vec![
        Vector3::new(-0.5, -0.5, 0.0),
        Vector3::new(0.5, -0.5, 0.0),
        Vector3::new(0.0, 0.5, 0.0),
    ];

    let normals = vec![
        Vector3::new(0.0, 0.0, 1.0),
        Vector3::new(0.0, 0.0, 1.0),
        Vector3::new(0.0, 0.0, 1.0),
    ];

    let uvs = vec![
        Vector2::new(0.0, 0.0),
        Vector2::new(1.0, 0.0),
        Vector2::new(0.5, 1.0),
    ];

    let indices = vec![0, 1, 2];

    let vertex_data = VertexData::new(positions)
        .with_normals(normals)
        .with_uvs(uvs);

    let mesh = Mesh::from_data(vertex_data, indices).unwrap();

    // 绘制
    mesh.draw();

    // 方式2: 从OBJ文件创建
    let mesh_from_file = Mesh::from_obj("model.obj").unwrap();
    mesh_from_file.draw();
}
