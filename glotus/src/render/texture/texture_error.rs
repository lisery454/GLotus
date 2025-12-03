use thiserror::Error;

#[derive(Error, Debug)]
pub enum TextureError {
    #[error("Failed to read texture file: {0}")]
    FileReadError(String),
}