use gl::types::*;
use std::ptr;

use super::{CameraData, FrameData, ModelData};

pub struct GlobalUniform {
    frame_ubo: GLuint,
    camera_ubo: GLuint,
    model_ubo: GLuint,
}

impl Default for GlobalUniform {
    fn default() -> Self {
        Self::new()
    }
}

impl GlobalUniform {
    pub fn new() -> Self {
        Self {
            frame_ubo: 0,
            camera_ubo: 0,
            model_ubo: 0,
        }
    }

    pub fn init(&mut self) {
        unsafe {
            let mut ubos = [0u32; 3];
            gl::GenBuffers(3, ubos.as_mut_ptr());

            let frame_ubo = ubos[0];
            let camera_ubo = ubos[1];
            let model_ubo = ubos[2];

            // 初始化 FrameData UBO (binding point 0)
            gl::BindBuffer(gl::UNIFORM_BUFFER, frame_ubo);
            gl::BufferData(
                gl::UNIFORM_BUFFER,
                std::mem::size_of::<FrameData>() as GLsizeiptr,
                ptr::null(),
                gl::DYNAMIC_DRAW, // 每帧更新
            );
            gl::BindBufferBase(gl::UNIFORM_BUFFER, 0, frame_ubo);

            // 初始化 CameraData UBO (binding point 1)
            gl::BindBuffer(gl::UNIFORM_BUFFER, camera_ubo);
            gl::BufferData(
                gl::UNIFORM_BUFFER,
                std::mem::size_of::<CameraData>() as GLsizeiptr,
                ptr::null(),
                gl::DYNAMIC_DRAW, // 每个相机更新
            );
            gl::BindBufferBase(gl::UNIFORM_BUFFER, 1, camera_ubo);

            // 初始化 ModelData UBO (binding point 2)
            gl::BindBuffer(gl::UNIFORM_BUFFER, model_ubo);
            gl::BufferData(
                gl::UNIFORM_BUFFER,
                std::mem::size_of::<ModelData>() as GLsizeiptr,
                ptr::null(),
                gl::DYNAMIC_DRAW, // 每个模型更新
            );
            gl::BindBufferBase(gl::UNIFORM_BUFFER, 2, model_ubo);

            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);

            self.frame_ubo = frame_ubo;
            self.camera_ubo = camera_ubo;
            self.model_ubo = model_ubo;
        }
    }
}

impl GlobalUniform {
    /// 更新 FrameData（每帧调用一次）
    pub fn update_frame_data(&self, frame_data: &FrameData) {
        unsafe {
            gl::BindBuffer(gl::UNIFORM_BUFFER, self.frame_ubo);

            gl::BufferSubData(
                gl::UNIFORM_BUFFER,
                0,
                std::mem::size_of::<FrameData>() as GLsizeiptr,
                frame_data as *const _ as *const _,
            );

            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
        }
    }

    /// 更新 CameraData（相机变化时调用）
    pub fn update_camera_data(&self, camera_data: &CameraData) {
        unsafe {
            gl::BindBuffer(gl::UNIFORM_BUFFER, self.camera_ubo);

            gl::BufferSubData(
                gl::UNIFORM_BUFFER,
                0,
                std::mem::size_of::<CameraData>() as GLsizeiptr,
                camera_data as *const _ as *const _,
            );
            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
        }
    }

    /// 更新 ModelData（每个模型渲染前调用）
    pub fn update_model_data(&self, model_data: &ModelData) {
        unsafe {
            gl::BindBuffer(gl::UNIFORM_BUFFER, self.model_ubo);
            gl::BufferSubData(
                gl::UNIFORM_BUFFER,
                0,
                std::mem::size_of::<ModelData>() as GLsizeiptr,
                model_data as *const _ as *const _,
            );
            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
        }
    }
}
