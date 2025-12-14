//! Error types for dx-serializer

use thiserror::Error;

pub type Result<T> = std::result::Result<T, DxError>;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum DxError {
    #[error("Unexpected end of input at position {0}")]
    UnexpectedEof(usize),

    #[error("Invalid syntax at position {pos}: {msg}")]
    InvalidSyntax { pos: usize, msg: String },

    #[error("Schema error: {0}")]
    SchemaError(String),

    #[error("Type mismatch: expected {expected}, got {got}")]
    TypeMismatch { expected: String, got: String },

    #[error("Unknown alias: {0}")]
    UnknownAlias(String),

    #[error("Unknown anchor: {0}")]
    UnknownAnchor(String),

    #[error("Invalid type hint: {0}")]
    InvalidTypeHint(String),

    #[error("Invalid number format: {0}")]
    InvalidNumber(String),

    #[error("Invalid UTF-8 at position {0}")]
    InvalidUtf8(usize),

    #[error("IO error: {0}")]
    Io(String),

    #[error("Ditto without previous value at position {0}")]
    DittoNoPrevious(usize),

    #[error("Prefix inheritance failed: {0}")]
    PrefixError(String),
}

impl From<std::io::Error> for DxError {
    fn from(err: std::io::Error) -> Self {
        DxError::Io(err.to_string())
    }
}

impl From<std::str::Utf8Error> for DxError {
    fn from(_: std::str::Utf8Error) -> Self {
        DxError::InvalidUtf8(0)
    }
}
