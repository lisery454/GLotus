use thiserror::Error;

#[derive(Error, Debug)]
pub enum MaterialError {
    #[error("Failed to Bind")]
    BindFail,
}