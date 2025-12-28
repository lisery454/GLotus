use thiserror::Error;

#[derive(Error, Debug)]
pub enum TextureError {
    #[error("Failed to read texture file: {0}")]
    FileReadError(String),
    #[error("Failed to read texture from byte data")]
    ByteReadError,
    #[error("Failed to get texture by handle")]
    InvalidHandle,
}
