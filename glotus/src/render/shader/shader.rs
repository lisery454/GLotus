use gl::types::*;
use log::{info, warn};
use std::{cell::RefCell, ffi::CString, fs, path::Path, ptr, rc::Rc};

use super::shader_error::ShaderError;
#[derive(Debug)]
pub struct Shader {
    id: GLuint,
}

fn pre_process_shader(source: &str) -> String {
    let replacement = include_str!("./glotus.glsl");

    let re = regex::Regex::new(r#"#include\s*"glotus\.glsl""#).unwrap();
    re.replace_all(source, replacement).into_owned()
}

// create
impl Shader {
    pub fn from_files(
        vertex_path: &str,
        fragment_path: &str,
    ) -> Result<Rc<RefCell<Self>>, ShaderError> {
        let vertex_source = fs::read_to_string(vertex_path)
            .map_err(|e| ShaderError::FileReadError(e.to_string()))?;
        let fragment_source = fs::read_to_string(fragment_path)
            .map_err(|e| ShaderError::FileReadError(e.to_string()))?;

        Self::from_sources(&vertex_source, &fragment_source)
    }

    pub fn from_sources(
        vertex_source: &str,
        fragment_source: &str,
    ) -> Result<Rc<RefCell<Self>>, ShaderError> {
        let vertex_shader_id = Self::compile_shader(
            pre_process_shader(vertex_source).as_str(),
            gl::VERTEX_SHADER,
        )?;
        let fragment_shader_id = Self::compile_shader(
            pre_process_shader(fragment_source).as_str(),
            gl::FRAGMENT_SHADER,
        )?;
        let program_id = Self::link_program(vertex_shader_id, fragment_shader_id)?;

        // 删除中间着色器对象
        unsafe {
            gl::DeleteShader(vertex_shader_id);
            gl::DeleteShader(fragment_shader_id);
        }

        Ok(Rc::new(RefCell::new(Self { id: program_id })))
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    fn compile_shader(source: &str, shader_type: GLenum) -> Result<GLuint, ShaderError> {
        let shader = unsafe { gl::CreateShader(shader_type) };
        let c_str = CString::new(source.as_bytes()).unwrap();
        unsafe {
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
            gl::CompileShader(shader);

            // 检查编译错误
            let mut success = 0;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buffer = vec![0; len as usize];
                gl::GetShaderInfoLog(
                    shader,
                    len,
                    ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut GLchar,
                );

                let error_msg = String::from_utf8_lossy(&buffer).to_string();
                return Err(ShaderError::CompileError(error_msg));
            }
        }

        Ok(shader)
    }

    fn link_program(vertex_shader: GLuint, fragment_shader: GLuint) -> Result<GLuint, ShaderError> {
        unsafe {
            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);

            // 检查链接错误
            let mut success = 0;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);

            if success == 0 {
                let mut log_len = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut log_len);

                let mut buffer = vec![0; log_len as usize];
                gl::GetProgramInfoLog(
                    program,
                    log_len,
                    &mut log_len,
                    buffer.as_mut_ptr() as *mut _,
                );

                let error_msg = String::from_utf8_lossy(&buffer).to_string();
                gl::DeleteProgram(program);

                return Err(ShaderError::LinkError(error_msg));
            }

            Ok(program)
        }
    }
}

// use && set uniform
impl Shader {
    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }

    fn get_location_of_uniform(&self, name: &str) -> GLint {
        unsafe { gl::GetUniformLocation(self.id, std::ffi::CString::new(name).unwrap().as_ptr()) }
    }

    pub fn set_uniform_mat3(&self, name: &str, value: &[[f32; 3]; 3]) {
        unsafe {
            let location = self.get_location_of_uniform(name);
            if location != -1 {
                gl::UniformMatrix3fv(location, 1, gl::FALSE, value.as_ptr() as *const f32);
            } else {
                warn!("set shader location failed: {}", name);
            }
        }
    }

    pub fn set_uniform_mat4(&self, name: &str, value: &[[f32; 4]; 4]) {
        unsafe {
            let location = self.get_location_of_uniform(name);
            if location != -1 {
                gl::UniformMatrix4fv(location, 1, gl::FALSE, value.as_ptr() as *const f32);
            } else {
                warn!("set shader location failed: {}", name);
            }
        }
    }

    pub fn set_uniform_vec3(&self, name: &str, value: &[f32; 3]) {
        unsafe {
            let location = self.get_location_of_uniform(name);
            if location != -1 {
                gl::Uniform3f(location, value[0], value[1], value[2]);
            } else {
                warn!("set shader location failed: {}", name);
            }
        }
    }

    pub fn set_uniform_vec4(&self, name: &str, value: &[f32; 4]) {
        unsafe {
            let location = self.get_location_of_uniform(name);
            if location != -1 {
                gl::Uniform4f(location, value[0], value[1], value[2], value[3]);
            } else {
                warn!("set shader location failed: {}", name);
            }
        }
    }

    pub fn set_uniform_f32(&self, name: &str, value: f32) {
        unsafe {
            let location = self.get_location_of_uniform(name);
            if location != -1 {
                gl::Uniform1f(location, value);
            } else {
                warn!("set shader location failed: {}", name);
            }
        }
    }

    pub fn set_uniform_i32(&self, name: &str, value: i32) {
        unsafe {
            let location = self.get_location_of_uniform(name);
            if location != -1 {
                gl::Uniform1i(location, value);
            } else {
                warn!("set shader location failed: {}", name);
            }
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
