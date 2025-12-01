use gl::types::*;
use std::cell::RefCell;
use std::mem;
use std::ptr;
use std::rc::Rc;

use super::vertex::Vertex;

#[derive(Debug, Clone)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Rc<RefCell<Self>> {
        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0;

        let vertices_flat = vertices
            .iter()
            .flat_map(|v| {
                return vec![
                    v.position.x,
                    v.position.y,
                    v.position.z,
                    v.normal.x,
                    v.normal.y,
                    v.normal.z,
                    v.tex_coord.x,
                    v.tex_coord.y,
                ];
            })
            .collect::<Vec<f32>>();

        unsafe {
            // VBO
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices_flat.len() * mem::size_of::<f32>()) as isize,
                vertices_flat.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );

            // VAO
            gl::GenVertexArrays(1, &mut vao);

            gl::BindVertexArray(vao);

            // 设置顶点属性指针
            // 位置属性
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                8 * mem::size_of::<f32>() as GLsizei,
                (0 * mem::size_of::<f32>()) as *const GLvoid,
            );

            // 法线属性
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                8 * mem::size_of::<f32>() as GLsizei,
                (3 * mem::size_of::<f32>()) as *const GLvoid,
            );

            // 纹理坐标属性
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                8 * mem::size_of::<f32>() as GLsizei,
                (6 * mem::size_of::<f32>()) as *const GLvoid,
            );

            if indices.len() > 0 {
                // EBO
                gl::GenBuffers(1, &mut ebo);
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
                gl::BufferData(
                    gl::ELEMENT_ARRAY_BUFFER,
                    (indices.len() * mem::size_of::<u32>()) as isize,
                    indices.as_ptr() as *const GLvoid,
                    gl::STATIC_DRAW,
                );
            }

            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        Rc::new(RefCell::new(Self {
            vertices,
            indices,
            vao,
            vbo,
            ebo,
        }))
    }

    pub fn get_vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            if self.indices.len() > 0 {
                gl::DrawElements(
                    gl::TRIANGLES,
                    self.indices.len() as i32,
                    gl::UNSIGNED_INT,
                    ptr::null(),
                );
            } else {
                gl::DrawArrays(gl::TRIANGLES, 0, self.vertices.len() as i32);
            }
            gl::BindVertexArray(0);
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
