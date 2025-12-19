//! Error types for macro compatibility.

use thiserror::Error;

/// Macro error type.
#[derive(Debug, Error)]
pub enum MacroError {
    /// Execution failed
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),

    /// Serialization failed
    #[error("Serialization failed: {0}")]
    Serialization(String),

    /// File access error
    #[error("File access error: {0}")]
    FileAccess(String),
}

/// Result type for macro operations.
pub type MacroResult<T> = Result<T, MacroError>;
