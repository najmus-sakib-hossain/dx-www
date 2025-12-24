//! DX-Zero error types

use std::fmt;

use super::header::HeaderError;
use super::slot::SlotError;

/// DX-Zero error type
#[derive(Debug, Clone)]
pub enum DxZeroError {
    /// Header validation error
    Header(HeaderError),
    /// Slot operation error
    Slot(SlotError),
    /// Buffer too small
    BufferTooSmall { required: usize, actual: usize },
    /// Invalid UTF-8 in string data
    InvalidUtf8,
    /// Invalid alignment
    InvalidAlignment,
    /// Corrupted data
    CorruptedData { reason: String },
    /// Heap offset out of bounds
    HeapOutOfBounds {
        offset: u32,
        length: u32,
        heap_size: usize,
    },
    /// Invalid magic bytes
    InvalidMagic,
    /// Unsupported version
    UnsupportedVersion { found: u8, supported: u8 },
    /// Invalid data (generic)
    InvalidData(String),
}

impl fmt::Display for DxZeroError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Header(err) => write!(f, "Header error: {}", err),
            Self::Slot(err) => write!(f, "Slot error: {}", err),
            Self::BufferTooSmall { required, actual } => {
                write!(f, "Buffer too small: need {} bytes, have {} bytes", required, actual)
            }
            Self::InvalidUtf8 => write!(f, "Invalid UTF-8 in string data"),
            Self::InvalidAlignment => write!(f, "Invalid buffer alignment"),
            Self::CorruptedData { reason } => write!(f, "Corrupted data: {}", reason),
            Self::HeapOutOfBounds {
                offset,
                length,
                heap_size,
            } => write!(
                f,
                "Heap access out of bounds: offset {} + length {} exceeds heap size {}",
                offset, length, heap_size
            ),
            Self::InvalidMagic => write!(f, "Invalid DX-Zero magic bytes"),
            Self::UnsupportedVersion { found, supported } => {
                write!(f, "Unsupported version: found {}, supported {}", found, supported)
            }
            Self::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
        }
    }
}

impl std::error::Error for DxZeroError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Header(err) => Some(err),
            Self::Slot(err) => Some(err),
            _ => None,
        }
    }
}

impl From<HeaderError> for DxZeroError {
    fn from(err: HeaderError) -> Self {
        Self::Header(err)
    }
}

impl From<SlotError> for DxZeroError {
    fn from(err: SlotError) -> Self {
        Self::Slot(err)
    }
}

impl From<std::str::Utf8Error> for DxZeroError {
    fn from(_: std::str::Utf8Error) -> Self {
        Self::InvalidUtf8
    }
}

/// Result type for DX-Zero operations
pub type Result<T> = std::result::Result<T, DxZeroError>;
