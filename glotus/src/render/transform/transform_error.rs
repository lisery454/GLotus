use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransformError {
    #[error("Model matrix must be invertible for normal transformation")]
    InverseMatrixFail,
}
