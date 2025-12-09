use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShaderError {
    #[error("Failed to read shader file: {0}")]
    FileReadError(String),
    #[error("Failed to set shader location: {0}")]
    SetShaderLocationFail(String),
    #[error("Failed to transform String to CString")]
    TransformCStringFail,
    #[error("Failed to compile shader: {0}")]
    CompileError(String),
    #[error("Failed to link program: {0}")]
    LinkError(String),
}
