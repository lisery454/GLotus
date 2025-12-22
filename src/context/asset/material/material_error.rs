use thiserror::Error;

use super::super::ShaderError;

#[derive(Error, Debug)]
pub enum MaterialError {
    #[error("Failed to Bind {0}")]
    BindFail(ShaderError),
    #[error("Failed to find material")]
    FindMatFail,
    #[error("Failed to find shader")]
    FindShaderFail,
    #[error("Failed to find texture")]
    FindTextureFail,
}