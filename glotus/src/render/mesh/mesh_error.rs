use thiserror::Error;

#[derive(Error, Debug)]
pub enum MeshError {
    #[error("Failed to load .obj")]
    TObjLoadFail,
}
