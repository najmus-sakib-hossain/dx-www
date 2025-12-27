//! Parser error types

use thiserror::Error;

/// Location in source code
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Location {
    /// Line number (1-indexed)
    pub line: usize,
    /// Column number (1-indexed)
    pub column: usize,
    /// Byte offset in source
    pub offset: usize,
}

impl Location {
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Self { line, column, offset }
    }
}

impl Default for Location {
    fn default() -> Self {
        Self { line: 1, column: 1, offset: 0 }
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line {}, column {}", self.line, self.column)
    }
}

/// Parse error with location information
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unexpected token at {location}: expected {expected}, got {actual}")]
    UnexpectedToken {
        location: Location,
        expected: String,
        actual: String,
    },

    #[error("Unexpected end of file at {location}: {message}")]
    UnexpectedEof {
        location: Location,
        message: String,
    },

    #[error("Invalid syntax at {location}: {message}")]
    InvalidSyntax {
        location: Location,
        message: String,
    },

    #[error("Indentation error at {location}: {message}")]
    IndentationError {
        location: Location,
        message: String,
    },

    #[error("Invalid string literal at {location}: {message}")]
    InvalidString {
        location: Location,
        message: String,
    },

    #[error("Invalid number literal at {location}: {message}")]
    InvalidNumber {
        location: Location,
        message: String,
    },

    #[error("Invalid identifier at {location}: {message}")]
    InvalidIdentifier {
        location: Location,
        message: String,
    },
}

impl ParseError {
    pub fn unexpected_token(location: Location, expected: &str, actual: &str) -> Self {
        Self::UnexpectedToken {
            location,
            expected: expected.to_string(),
            actual: actual.to_string(),
        }
    }

    pub fn unexpected_eof(location: Location, message: &str) -> Self {
        Self::UnexpectedEof {
            location,
            message: message.to_string(),
        }
    }

    pub fn invalid_syntax(location: Location, message: &str) -> Self {
        Self::InvalidSyntax {
            location,
            message: message.to_string(),
        }
    }

    pub fn indentation_error(location: Location, message: &str) -> Self {
        Self::IndentationError {
            location,
            message: message.to_string(),
        }
    }

    /// Get the location of the error
    pub fn location(&self) -> Location {
        match self {
            Self::UnexpectedToken { location, .. } => *location,
            Self::UnexpectedEof { location, .. } => *location,
            Self::InvalidSyntax { location, .. } => *location,
            Self::IndentationError { location, .. } => *location,
            Self::InvalidString { location, .. } => *location,
            Self::InvalidNumber { location, .. } => *location,
            Self::InvalidIdentifier { location, .. } => *location,
        }
    }
}

/// Result type for parser operations
pub type ParseResult<T> = Result<T, ParseError>;
