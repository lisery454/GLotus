use thiserror::Error;

use crate::render::shader::ShaderError;

#[derive(Error, Debug)]
pub enum MaterialError {
    #[error("Failed to Bind {0}")]
    BindFail(ShaderError),
}