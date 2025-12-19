//! Error types for compile compatibility.

use thiserror::Error;

/// Compile error type.
#[derive(Debug, Error)]
pub enum CompileError {
    /// Bundling failed
    #[error("Bundling failed: {0}")]
    BundlingFailed(String),

    /// Target not supported
    #[error("Target not supported: {0}")]
    UnsupportedTarget(String),

    /// Asset embedding failed
    #[error("Asset embedding failed: {0}")]
    AssetEmbedding(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Result type for compile operations.
pub type CompileResult<T> = Result<T, CompileError>;
