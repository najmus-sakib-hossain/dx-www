use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid magic number: expected {expected:?}, found {found:?}")]
    InvalidMagic { expected: [u8; 4], found: [u8; 4] },

    #[error("Unsupported version: {0}")]
    UnsupportedVersion(u16),

    #[error("Corrupted data: hash mismatch")]
    CorruptedData,

    #[error("Package not found: {0}")]
    PackageNotFound(String),

    #[error("File not found in package: {0}")]
    FileNotFound(String),

    #[error("Invalid version string: {0}")]
    InvalidVersion(String),

    #[error("Package too large: {size} bytes (max {max} bytes)")]
    PackageTooLarge { size: u64, max: u64 },

    #[error("Too many files: {count} (max {max})")]
    TooManyFiles { count: u32, max: u32 },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Compression error: {0}")]
    Compression(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Parse error: {0}")]
    Parse(String),
}

pub type Result<T> = std::result::Result<T, Error>;
