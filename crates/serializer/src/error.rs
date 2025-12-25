//! Comprehensive error types for dx-serializer
//!
//! This module provides a unified error handling system with detailed
//! location information for debugging and user-friendly error messages.

use std::fmt;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, DxError>;

/// Source location information for parse errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceLocation {
    /// Line number (1-indexed)
    pub line: usize,
    /// Column number (1-indexed)
    pub column: usize,
    /// Byte offset from start of input
    pub offset: usize,
}

impl SourceLocation {
    /// Create a new source location
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Self { line, column, offset }
    }

    /// Create a source location from a byte offset and input
    pub fn from_offset(input: &[u8], offset: usize) -> Self {
        let mut line = 1;
        let mut column = 1;
        
        for (i, &byte) in input.iter().enumerate() {
            if i >= offset {
                break;
            }
            if byte == b'\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }
        
        Self { line, column, offset }
    }
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}, column {}", self.line, self.column)
    }
}

/// Magic bytes for DX binary format
pub const DX_MAGIC: [u8; 2] = [0x5A, 0x44]; // "ZD" in ASCII

/// Current binary format version
pub const DX_VERSION: u8 = 1;

/// Maximum input size (100 MB) - prevents memory exhaustion attacks
pub const MAX_INPUT_SIZE: usize = 100 * 1024 * 1024;

/// Maximum recursion depth for nested structures - prevents stack overflow
pub const MAX_RECURSION_DEPTH: usize = 1000;

/// Maximum table row count - prevents memory exhaustion
pub const MAX_TABLE_ROWS: usize = 10_000_000;

/// Comprehensive error type for all DX serializer operations
#[derive(Error, Debug, Clone, PartialEq)]
pub enum DxError {
    // === Parse Errors ===
    
    /// Unexpected end of input during parsing
    #[error("Unexpected end of input at position {0}")]
    UnexpectedEof(usize),

    /// Parse error with location information
    #[error("Parse error at {location}: {message}")]
    ParseError {
        location: SourceLocation,
        message: String,
    },

    /// Invalid syntax at a specific position
    #[error("Invalid syntax at position {pos}: {msg}")]
    InvalidSyntax { pos: usize, msg: String },

    // === Schema Errors ===
    
    /// Schema validation error
    #[error("Schema error: {0}")]
    SchemaError(String),

    /// Type mismatch during parsing or conversion
    #[error("Type mismatch: expected {expected}, got {got}")]
    TypeMismatch { expected: String, got: String },

    // === Reference Errors ===
    
    /// Unknown alias reference
    #[error("Unknown alias: {0}")]
    UnknownAlias(String),

    /// Unknown anchor reference
    #[error("Unknown anchor: {0}")]
    UnknownAnchor(String),

    // === Type Errors ===
    
    /// Invalid type hint in schema
    #[error("Invalid type hint: {0}")]
    InvalidTypeHint(String),

    /// Invalid number format
    #[error("Invalid number format: {0}")]
    InvalidNumber(String),

    // === Encoding Errors ===
    
    /// Invalid UTF-8 sequence
    #[error("Invalid UTF-8 at byte offset {offset}")]
    Utf8Error { offset: usize },

    /// Invalid Base62 character
    #[error("Invalid Base62 character '{char}' at position {position}: {message}")]
    Base62Error {
        char: char,
        position: usize,
        message: String,
    },

    /// Legacy Base62 error (for compatibility)
    #[error("Invalid Base62 character '{char}': {msg}")]
    InvalidBase62 { char: char, msg: String },

    /// Integer overflow during encoding/decoding
    #[error("Integer overflow")]
    IntegerOverflow,

    // === Binary Format Errors ===
    
    /// Invalid magic bytes in binary header
    #[error("Invalid magic bytes: expected [0x5A, 0x44], got [{0:#04X}, {1:#04X}]")]
    InvalidMagic(u8, u8),

    /// Unsupported binary format version
    #[error("Unsupported version {found}, expected {expected}")]
    UnsupportedVersion { found: u8, expected: u8 },

    /// Buffer too small for operation
    #[error("Buffer too small: need {required} bytes, have {available}")]
    BufferTooSmall { required: usize, available: usize },

    // === Compression Errors ===
    
    /// Compression operation failed
    #[error("Compression error: {0}")]
    CompressionError(String),

    /// Decompression operation failed
    #[error("Decompression error: {0}")]
    DecompressionError(String),

    // === I/O Errors ===
    
    /// General I/O error
    #[error("IO error: {0}")]
    Io(String),

    /// Platform not supported for operation
    #[error("Unsupported platform: {0}")]
    UnsupportedPlatform(String),

    // === Conversion Errors ===
    
    /// Format conversion error
    #[error("Conversion error: {0}")]
    ConversionError(String),

    /// Ditto operator without previous value
    #[error("Ditto without previous value at position {0}")]
    DittoNoPrevious(usize),

    /// Prefix inheritance failed
    #[error("Prefix inheritance failed: {0}")]
    PrefixError(String),

    // === Resource Limit Errors ===
    
    /// Input size exceeds maximum allowed
    #[error("Input too large: {size} bytes exceeds maximum of {max} bytes")]
    InputTooLarge { size: usize, max: usize },

    /// Recursion depth exceeds maximum allowed
    #[error("Recursion limit exceeded: depth {depth} exceeds maximum of {max}")]
    RecursionLimitExceeded { depth: usize, max: usize },

    /// Table row count exceeds maximum allowed
    #[error("Table too large: {rows} rows exceeds maximum of {max} rows")]
    TableTooLarge { rows: usize, max: usize },
}

impl DxError {
    /// Create a parse error with location information
    pub fn parse_error(input: &[u8], offset: usize, message: impl Into<String>) -> Self {
        DxError::ParseError {
            location: SourceLocation::from_offset(input, offset),
            message: message.into(),
        }
    }

    /// Create a UTF-8 error at a specific offset
    pub fn utf8_error(offset: usize) -> Self {
        DxError::Utf8Error { offset }
    }

    /// Create a Base62 error with position
    pub fn base62_error(char: char, position: usize, message: impl Into<String>) -> Self {
        DxError::Base62Error {
            char,
            position,
            message: message.into(),
        }
    }

    /// Create an invalid magic error
    pub fn invalid_magic(byte0: u8, byte1: u8) -> Self {
        DxError::InvalidMagic(byte0, byte1)
    }

    /// Create an unsupported version error
    pub fn unsupported_version(found: u8) -> Self {
        DxError::UnsupportedVersion {
            found,
            expected: DX_VERSION,
        }
    }

    /// Create a buffer too small error
    pub fn buffer_too_small(required: usize, available: usize) -> Self {
        DxError::BufferTooSmall { required, available }
    }

    /// Create an input too large error
    pub fn input_too_large(size: usize) -> Self {
        DxError::InputTooLarge { size, max: MAX_INPUT_SIZE }
    }

    /// Create a recursion limit exceeded error
    pub fn recursion_limit_exceeded(depth: usize) -> Self {
        DxError::RecursionLimitExceeded { depth, max: MAX_RECURSION_DEPTH }
    }

    /// Create a table too large error
    pub fn table_too_large(rows: usize) -> Self {
        DxError::TableTooLarge { rows, max: MAX_TABLE_ROWS }
    }

    /// Get the byte offset if available
    pub fn offset(&self) -> Option<usize> {
        match self {
            DxError::UnexpectedEof(offset) => Some(*offset),
            DxError::ParseError { location, .. } => Some(location.offset),
            DxError::InvalidSyntax { pos, .. } => Some(*pos),
            DxError::Utf8Error { offset } => Some(*offset),
            DxError::Base62Error { position, .. } => Some(*position),
            DxError::DittoNoPrevious(pos) => Some(*pos),
            _ => None,
        }
    }

    /// Get the source location if available
    pub fn location(&self) -> Option<&SourceLocation> {
        match self {
            DxError::ParseError { location, .. } => Some(location),
            _ => None,
        }
    }

    /// Check if this is a recoverable error
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            DxError::UnknownAlias(_)
                | DxError::UnknownAnchor(_)
                | DxError::TypeMismatch { .. }
        )
    }
}

impl From<std::io::Error> for DxError {
    fn from(err: std::io::Error) -> Self {
        DxError::Io(err.to_string())
    }
}

impl From<std::str::Utf8Error> for DxError {
    fn from(err: std::str::Utf8Error) -> Self {
        DxError::Utf8Error {
            offset: err.valid_up_to(),
        }
    }
}

impl From<std::string::FromUtf8Error> for DxError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        DxError::Utf8Error {
            offset: err.utf8_error().valid_up_to(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_location_from_offset() {
        let input = b"line1\nline2\nline3";
        
        // First line
        let loc = SourceLocation::from_offset(input, 0);
        assert_eq!(loc.line, 1);
        assert_eq!(loc.column, 1);
        
        // Middle of first line
        let loc = SourceLocation::from_offset(input, 3);
        assert_eq!(loc.line, 1);
        assert_eq!(loc.column, 4);
        
        // Start of second line
        let loc = SourceLocation::from_offset(input, 6);
        assert_eq!(loc.line, 2);
        assert_eq!(loc.column, 1);
        
        // Middle of third line
        let loc = SourceLocation::from_offset(input, 14);
        assert_eq!(loc.line, 3);
        assert_eq!(loc.column, 3);
    }

    #[test]
    fn test_parse_error_with_location() {
        let input = b"key: value\nbad line here";
        let err = DxError::parse_error(input, 15, "unexpected token");
        
        if let DxError::ParseError { location, message } = &err {
            assert_eq!(location.line, 2);
            assert_eq!(location.column, 5);
            assert_eq!(message, "unexpected token");
        } else {
            panic!("Expected ParseError");
        }
    }

    #[test]
    fn test_invalid_magic() {
        let err = DxError::invalid_magic(0x00, 0x01);
        assert!(err.to_string().contains("0x00"));
        assert!(err.to_string().contains("0x01"));
    }

    #[test]
    fn test_buffer_too_small() {
        let err = DxError::buffer_too_small(100, 50);
        assert!(err.to_string().contains("100"));
        assert!(err.to_string().contains("50"));
    }

    #[test]
    fn test_error_offset() {
        assert_eq!(DxError::UnexpectedEof(42).offset(), Some(42));
        assert_eq!(DxError::utf8_error(10).offset(), Some(10));
        assert_eq!(DxError::SchemaError("test".into()).offset(), None);
    }
}
