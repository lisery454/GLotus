use thiserror::Error;

use crate::TextureError;

#[derive(Error, Debug)]
pub enum FramebufferError {
    #[error("texture error")]
    TextureError(#[from] TextureError),
    #[error("create fail: {0}")]
    CreationFailed(String),
    #[error("framebuffer incomplete")]
    IncompleteFramebuffer,
    #[error("InvalidHandle")]
    InvalidHandle,
    #[error("TextureManagerBorrowFail")]
    TextureManagerBorrowFail,
}
