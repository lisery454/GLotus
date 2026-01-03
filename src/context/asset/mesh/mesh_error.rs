use thiserror::Error;

#[derive(Error, Debug)]
pub enum MeshError {
    #[error("Failed to load .obj {0}")]
    LoadError(String),
    #[error("invalid init data {0}")]
    InvalidData(String),
}
