//! Error types for dx-js-runtime

use thiserror::Error;

/// Result type for dx-js-runtime operations
pub type DxResult<T> = Result<T, DxError>;

/// Errors that can occur in dx-js-runtime
#[derive(Error, Debug)]
pub enum DxError {
    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Type error: {0}")]
    TypeError(String),

    #[error("Compilation error: {0}")]
    CompileError(String),

    #[error("Runtime error: {0}")]
    RuntimeError(String),

    #[error("Cache error: {0}")]
    CacheError(String),

    #[error("IO error: {0}")]
    IoError(String),

    #[error("Module not found: {0}")]
    ModuleNotFound(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<anyhow::Error> for DxError {
    fn from(err: anyhow::Error) -> Self {
        DxError::Internal(err.to_string())
    }
}

impl From<std::io::Error> for DxError {
    fn from(err: std::io::Error) -> Self {
        DxError::IoError(err.to_string())
    }
}
