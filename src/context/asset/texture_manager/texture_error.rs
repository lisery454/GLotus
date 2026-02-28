use thiserror::Error;

#[derive(Error, Debug)]
pub enum TextureError {
    #[error("Failed to read texture file: {0}")]
    FileReadError(String),
    #[error("Failed to read texture from byte data")]
    ByteReadError,
    #[error("Failed to get texture by handle")]
    InvalidHandle,
    #[error("resolution is invalid")]
    InvalidResolution,
    #[error("not support resize")]
    NotSupportResize,
    #[error("not support bind")]
    NotSupportBind,
    #[error("texture config does not match")]
    ConfigNotMatch,
}
