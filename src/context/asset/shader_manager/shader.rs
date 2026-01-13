use gl::types::*;
use log::warn;
use std::{ffi::CString, fs, path::PathBuf, ptr};

use super::shader_error::ShaderError;

#[derive(Debug)]
pub enum ShaderInput {
    Path(PathBuf),
    Source(String),
}

#[derive(Debug)]
pub struct ShaderConfig {
    vert_shader_input: ShaderInput,
    frag_shader_input: ShaderInput,
    gemo_shader_input: Option<ShaderInput>,
}

impl ShaderConfig {
    pub fn new_vert_frag(vert: ShaderInput, frag: ShaderInput) -> Self {
        Self {
            vert_shader_input: vert,
            frag_shader_input: frag,
            gemo_shader_input: None,
        }
    }

    pub fn new_vert_frag_gemo(vert: ShaderInput, frag: ShaderInput, gemo: ShaderInput) -> Self {
        Self {
            vert_shader_input: vert,
            frag_shader_input: frag,
            gemo_shader_input: Some(gemo),
        }
    }
}

/// shader类
#[derive(Debug)]
pub struct Shader {
    pub(crate) id: GLuint,
}

/// 预处理shader
fn pre_process_vert_shader(source: String) -> String {
    let s = format!("{}\n{}", include_str!("./preload_vert.glsl"), source);
    format!("{}\n{}", include_str!("./preload.glsl"), s)
}
fn pre_process_frag_shader(source: String) -> String {
    format!("{}\n{}", include_str!("./preload.glsl"), source)
}
fn pre_process_geom_shader(source: String) -> String {
    format!("{}\n{}", include_str!("./preload.glsl"), source)
}

// create
impl Shader {
    fn input_to_source(input: ShaderInput) -> Result<String, ShaderError> {
        let source = match input {
            ShaderInput::Path(path_buf) => fs::read_to_string(path_buf)
                .map_err(|e| ShaderError::FileReadError(e.to_string()))?,
            ShaderInput::Source(s) => s,
        };

        Ok(source)
    }
    pub fn new(config: ShaderConfig) -> Result<Self, ShaderError> {
        let vertex_source = Self::input_to_source(config.vert_shader_input)?;
        let vertex_shader_id = Self::compile_shader(
            pre_process_vert_shader(vertex_source).as_str(),
            gl::VERTEX_SHADER,
        )?;
        let fragment_source = Self::input_to_source(config.frag_shader_input)?;
        let fragment_shader_id = Self::compile_shader(
            pre_process_frag_shader(fragment_source).as_str(),
            gl::FRAGMENT_SHADER,
        )?;

        let geometry_source = match config.gemo_shader_input {
            Some(input) => Some(Self::input_to_source(input)?),
            _ => None,
        };

        let geomtry_shader_id = match geometry_source {
            Some(source) => Some(Self::compile_shader(
                pre_process_geom_shader(source).as_str(),
                gl::GEOMETRY_SHADER,
            )?),
            None => None,
        };

        let program_id =
            Self::link_program(vertex_shader_id, fragment_shader_id, geomtry_shader_id)?;

        unsafe {
            gl::DeleteShader(vertex_shader_id);
            gl::DeleteShader(fragment_shader_id);
            if let Some(id) = geomtry_shader_id {
                gl::DeleteShader(id);
            }
        }

        Ok(Self { id: program_id })
    }

    /// 编译shader
    fn compile_shader(source: &str, shader_type: GLenum) -> Result<GLuint, ShaderError> {
        let shader = unsafe { gl::CreateShader(shader_type) };
        let c_str =
            CString::new(source.as_bytes()).map_err(|_| ShaderError::TransformCStringFail)?;
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

    /// link shader program
    fn link_program(
        vertex_shader: GLuint,
        fragment_shader: GLuint,
        geometry_shader: Option<GLuint>,
    ) -> Result<GLuint, ShaderError> {
        unsafe {
            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            if let Some(geometry_id) = geometry_shader {
                gl::AttachShader(program, geometry_id);
            }
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
    /// 绑定shader程序
    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    /// 取消绑定
    pub fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }

    /// 获取某个uniform的位置
    fn get_location_of_uniform(&self, name: &str) -> Result<GLint, ShaderError> {
        let name = CString::new(name).map_err(|_| ShaderError::TransformCStringFail)?;
        unsafe { Ok(gl::GetUniformLocation(self.id, name.as_ptr())) }
    }

    /// 设置3*3矩阵uniform
    pub(crate) fn set_uniform_mat3(
        &self,
        name: &str,
        value: &[[f32; 3]; 3],
    ) -> Result<(), ShaderError> {
        unsafe {
            let location = self.get_location_of_uniform(name)?;
            if location != -1 {
                gl::UniformMatrix3fv(location, 1, gl::FALSE, value.as_ptr() as *const f32);
            } else {
                // return Err(ShaderError::SetShaderLocationFail(String::from(name)));
                warn!("set location fail {:?}", name);
            }
        }

        Ok(())
    }

    /// 设置4*4矩阵uniform
    pub(crate) fn set_uniform_mat4(
        &self,
        name: &str,
        value: &[[f32; 4]; 4],
    ) -> Result<(), ShaderError> {
        unsafe {
            let location = self.get_location_of_uniform(name)?;
            if location != -1 {
                gl::UniformMatrix4fv(location, 1, gl::FALSE, value.as_ptr() as *const f32);
            } else {
                // return Err(ShaderError::SetShaderLocationFail(String::from(name)));
                warn!("set location fail {:?}", name);
            }
        }

        Ok(())
    }

    /// 设置vec3矩阵uniform
    pub(crate) fn set_uniform_vec3(&self, name: &str, value: &[f32; 3]) -> Result<(), ShaderError> {
        unsafe {
            let location = self.get_location_of_uniform(name)?;
            if location != -1 {
                gl::Uniform3f(location, value[0], value[1], value[2]);
            } else {
                // return Err(ShaderError::SetShaderLocationFail(String::from(name)));
                warn!("set location fail {:?}", name);
            }
        }

        Ok(())
    }

    /// 设置vec4矩阵uniform
    pub(crate) fn set_uniform_vec4(&self, name: &str, value: &[f32; 4]) -> Result<(), ShaderError> {
        unsafe {
            let location = self.get_location_of_uniform(name)?;
            if location != -1 {
                gl::Uniform4f(location, value[0], value[1], value[2], value[3]);
            } else {
                // return Err(ShaderError::SetShaderLocationFail(String::from(name)));
                warn!("set location fail {:?}", name);
            }
        }

        Ok(())
    }

    /// 设置f32矩阵uniform
    pub(crate) fn set_uniform_f32(&self, name: &str, value: f32) -> Result<(), ShaderError> {
        unsafe {
            let location = self.get_location_of_uniform(name)?;
            if location != -1 {
                gl::Uniform1f(location, value);
            } else {
                // return Err(ShaderError::SetShaderLocationFail(String::from(name)));
                warn!("set location fail {:?}", name);
            }
        }

        Ok(())
    }

    /// 设置i32矩阵uniform
    pub(crate) fn set_uniform_i32(&self, name: &str, value: i32) -> Result<(), ShaderError> {
        unsafe {
            let location = self.get_location_of_uniform(name)?;
            if location != -1 {
                gl::Uniform1i(location, value);
            } else {
                // return Err(ShaderError::SetShaderLocationFail(String::from(name)));
                warn!("set location fail {:?}", name);
            }
        }

        Ok(())
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
