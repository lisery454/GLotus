use std::{cell::RefCell, collections::HashMap, rc::Rc};

use cgmath::{Vector2, Vector3, Zero};
use gl::types::GLuint;

use super::Mesh;

use std::hash::{Hash, Hasher};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
struct Vertex {
    pub pos: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
}

impl Vertex {
    pub fn layout() -> Vec<VertexAttribute> {
        vec![
            VertexAttribute {
                index: 0,
                size: 3,
                offset: 0,
            },
            VertexAttribute {
                index: 1,
                size: 3,
                offset: std::mem::size_of::<[f32; 3]>(),
            },
            VertexAttribute {
                index: 2,
                size: 2,
                offset: std::mem::size_of::<[f32; 3]>() * 2,
            },
        ]
    }
}

impl Eq for Vertex {} // 必须加

impl Hash for Vertex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for v in self.pos {
            state.write_u32(v.to_bits());
        }
        for v in self.normal {
            state.write_u32(v.to_bits());
        }
        for v in self.uv {
            state.write_u32(v.to_bits());
        }
    }
}

struct VertexAttribute {
    pub index: u32,
    pub size: i32,
    pub offset: usize,
}

fn build_gpu_mesh(mesh: &Mesh) -> (Vec<Vertex>, Vec<u32>) {
    let mut unique_vertices: Vec<Vertex> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    let mut map: HashMap<Vertex, u32> = HashMap::new();

    for i in 0..mesh.count {
        let p = mesh.positions[mesh.position_indexs[i]];
        let n = if mesh.normal_indexs.len() == mesh.count {
            mesh.normals[mesh.normal_indexs[i]]
        } else {
            Vector3::zero()
        };
        let t = if mesh.texcoord_indexs.len() == mesh.count {
            mesh.texcoords[mesh.texcoord_indexs[i]]
        } else {
            Vector2::zero()
        };

        let v = Vertex {
            pos: [p.x, p.y, p.z],
            normal: [n.x, n.y, n.z],
            uv: [t.x, t.y],
        };

        if let Some(&idx) = map.get(&v) {
            // 顶点已出现，直接复用索引
            indices.push(idx);
        } else {
            // 新顶点
            let new_index = unique_vertices.len() as u32;
            unique_vertices.push(v);
            map.insert(v, new_index);
            indices.push(new_index);
        }
    }

    (unique_vertices, indices)
}

pub struct MeshGPUWrapper {
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,
    index_count: usize,
}

impl MeshGPUWrapper {
    pub fn from_mesh(mesh: Rc<RefCell<Mesh>>) -> Rc<RefCell<Self>> {
        let mesh = mesh.borrow();

        // Dedup + index buffer
        let (vertices, indices) = build_gpu_mesh(&mesh);

        let (mut vao, mut vbo, mut ebo) = (0, 0, 0);

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);

            // Upload VBO
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<Vertex>()) as isize,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // Upload EBO
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as isize,
                indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            for attr in Vertex::layout() {
                gl::EnableVertexAttribArray(attr.index);
                gl::VertexAttribPointer(
                    attr.index,
                    attr.size,
                    gl::FLOAT,
                    gl::FALSE,
                    std::mem::size_of::<Vertex>() as i32,
                    attr.offset as *const _,
                );
            }

            gl::BindVertexArray(0);
        }

        Rc::new(RefCell::new(Self {
            vao,
            vbo,
            ebo,
            index_count: indices.len(),
        }))
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElements(
                gl::TRIANGLES,
                self.index_count as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for MeshGPUWrapper {
    fn drop(&mut self) {
        unsafe {
            if self.vao != 0 {
                gl::DeleteVertexArrays(1, &self.vao);
            }
            if self.vbo != 0 {
                gl::DeleteBuffers(1, &self.vbo);
            }
            if self.ebo != 0 {
                gl::DeleteBuffers(1, &self.ebo);
            }
        }
    }
}
