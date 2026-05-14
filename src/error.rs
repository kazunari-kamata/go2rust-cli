use thiserror::Error;

pub type Result<T> = std::result::Result<T, Go2RustError>;

#[derive(Debug, Error)]
pub enum Go2RustError {
    #[error("failed to compile conversion regex: {0}")]
    Regex(#[from] regex::Error),
}
