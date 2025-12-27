use thiserror::Error;

#[derive(Error, Debug)]
pub enum FramebufferError {
    #[error("Create fail")]
    CreationFailed(String),
    #[error("framebuffer incomplete")]
    IncompleteFramebuffer,
    #[error("InvalidHandle")]
    InvalidHandle,
    #[error("TextureManagerBorrowFail")]
    TextureManagerBorrowFail,
}
