use std::mem;

use gl::types::*;
use glam::Mat4;

pub struct InstanceBuffer {
    vbo: GLuint,
    capacity: usize,
}

impl Default for InstanceBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl InstanceBuffer {
    pub fn new() -> Self {
        Self {
            vbo: 0,
            capacity: 0,
        }
    }

    pub fn init(&mut self) {
        let mut vbo = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }
        self.vbo = vbo;
    }

    pub fn upload(&mut self, matrices: &Vec<Mat4>) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            let size = (matrices.len() * mem::size_of::<Mat4>()) as GLsizeiptr;

            // 如果数据量超过当前容量，重新分配内存；否则只更新数据
            if matrices.len() > self.capacity {
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    size,
                    matrices.as_ptr() as *const _,
                    gl::DYNAMIC_DRAW,
                );
                self.capacity = matrices.len();
            } else {
                gl::BufferSubData(gl::ARRAY_BUFFER, 0, size, matrices.as_ptr() as *const _);
            }
        }
    }

    pub fn bind_to_vao(&self, start_location: u32) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            let stride = mem::size_of::<[[f32; 4]; 4]>() as i32;
            for i in 0..4 {
                let loc = start_location + i;
                gl::EnableVertexAttribArray(loc);
                gl::VertexAttribPointer(loc, 4, gl::FLOAT, gl::FALSE, stride, (i * 16) as *const _);
                gl::VertexAttribDivisor(loc, 1); // 开启实例化
            }
        }
    }

    pub fn unbind(&self, start_location: u32) {
        for i in 0..4 {
            unsafe {
                gl::VertexAttribDivisor(start_location + i, 0);
            }
        }
    }
}
