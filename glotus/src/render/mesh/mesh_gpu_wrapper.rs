use std::{cell::RefCell, rc::Rc};

use gl::types::GLuint;

use super::{Mesh, Vertex};

pub struct MeshGPUWrapper {
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,
    index_count: usize,
}

impl MeshGPUWrapper {
    pub fn from_mesh(mesh: Rc<RefCell<Mesh>>) -> Rc<RefCell<Self>> {
        let (mut vao, mut vbo, mut ebo) = (0, 0, 0);

        let mesh = mesh.borrow();

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);

            // Upload vertex data
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (mesh.vertices.len() * std::mem::size_of::<Vertex>()) as isize,
                mesh.vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (mesh.indices.len() * std::mem::size_of::<u32>()) as isize,
                mesh.indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // Set vertex attributes
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
            index_count: mesh.indices.len(),
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
