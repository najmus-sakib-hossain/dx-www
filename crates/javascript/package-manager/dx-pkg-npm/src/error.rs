//! Error types for npm client

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Package not found: {0}")]
    PackageNotFound(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Failed to parse response: {0}")]
    ParseError(String),

    #[error("Download failed: {0}")]
    DownloadFailed(String),

    #[error("Invalid version constraint: {0}")]
    InvalidConstraint(String),

    #[error("No matching version found")]
    NoVersionFound,

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
