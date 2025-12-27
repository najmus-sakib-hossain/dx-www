//! Runtime Error Types
//!
//! This module defines all error types for the DX-Py runtime.
//! All public functions return Result types instead of panicking.

use thiserror::Error;

/// Runtime errors that can occur during execution
#[derive(Error, Debug, Clone)]
pub enum RuntimeError {
    /// Type mismatch error
    #[error("TypeError: expected {expected}, got {actual}")]
    TypeError {
        expected: String,
        actual: String,
    },

    /// Index out of bounds
    #[error("IndexError: index {index} out of range for length {length}")]
    IndexError {
        index: i64,
        length: usize,
    },

    /// Key not found in dictionary
    #[error("KeyError: {key}")]
    KeyError {
        key: String,
    },

    /// Division by zero
    #[error("ZeroDivisionError: division by zero")]
    ZeroDivisionError,

    /// Arithmetic overflow
    #[error("OverflowError: {operation} overflow")]
    OverflowError {
        operation: String,
    },

    /// Name not found in scope
    #[error("NameError: name '{name}' is not defined")]
    NameError {
        name: String,
    },

    /// Attribute not found on object
    #[error("AttributeError: '{type_name}' object has no attribute '{attr}'")]
    AttributeError {
        attr: String,
        type_name: String,
    },

    /// Module import failed
    #[error("ImportError: No module named '{module}'")]
    ImportError {
        module: String,
    },

    /// Value error (invalid value for operation)
    #[error("ValueError: {message}")]
    ValueError {
        message: String,
    },

    /// Runtime assertion failed
    #[error("AssertionError: {message}")]
    AssertionError {
        message: String,
    },

    /// Stop iteration (used internally)
    #[error("StopIteration")]
    StopIteration,

    /// Memory allocation failed
    #[error("MemoryError: {message}")]
    MemoryError {
        message: String,
    },

    /// Recursion limit exceeded
    #[error("RecursionError: maximum recursion depth exceeded")]
    RecursionError,

    /// I/O error
    #[error("IOError: {message}")]
    IoError {
        message: String,
    },

    /// OS error
    #[error("OSError: {message}")]
    OsError {
        message: String,
    },

    /// File not found
    #[error("FileNotFoundError: {path}")]
    FileNotFoundError {
        path: String,
    },

    /// Permission denied
    #[error("PermissionError: {path}")]
    PermissionError {
        path: String,
    },

    /// Unicode decode error
    #[error("UnicodeDecodeError: {message}")]
    UnicodeDecodeError {
        message: String,
    },

    /// Unicode encode error
    #[error("UnicodeEncodeError: {message}")]
    UnicodeEncodeError {
        message: String,
    },

    /// Syntax error (for parser)
    #[error("SyntaxError: {message} at line {line}, column {column}")]
    SyntaxError {
        message: String,
        line: usize,
        column: usize,
    },

    /// Internal runtime error
    #[error("InternalError: {message}")]
    InternalError {
        message: String,
    },

    /// Not implemented
    #[error("NotImplementedError: {feature}")]
    NotImplementedError {
        feature: String,
    },
}

/// Result type for runtime operations
pub type RuntimeResult<T> = Result<T, RuntimeError>;

impl RuntimeError {
    /// Create a type error
    pub fn type_error(expected: impl Into<String>, actual: impl Into<String>) -> Self {
        Self::TypeError {
            expected: expected.into(),
            actual: actual.into(),
        }
    }

    /// Create an index error
    pub fn index_error(index: i64, length: usize) -> Self {
        Self::IndexError { index, length }
    }

    /// Create a key error
    pub fn key_error(key: impl Into<String>) -> Self {
        Self::KeyError { key: key.into() }
    }

    /// Create a name error
    pub fn name_error(name: impl Into<String>) -> Self {
        Self::NameError { name: name.into() }
    }

    /// Create an attribute error
    pub fn attribute_error(type_name: impl Into<String>, attr: impl Into<String>) -> Self {
        Self::AttributeError {
            type_name: type_name.into(),
            attr: attr.into(),
        }
    }

    /// Create a value error
    pub fn value_error(message: impl Into<String>) -> Self {
        Self::ValueError {
            message: message.into(),
        }
    }

    /// Create an overflow error
    pub fn overflow_error(operation: impl Into<String>) -> Self {
        Self::OverflowError {
            operation: operation.into(),
        }
    }

    /// Create an internal error
    pub fn internal_error(message: impl Into<String>) -> Self {
        Self::InternalError {
            message: message.into(),
        }
    }

    /// Create a not implemented error
    pub fn not_implemented(feature: impl Into<String>) -> Self {
        Self::NotImplementedError {
            feature: feature.into(),
        }
    }

    /// Get the Python exception name for this error
    pub fn exception_name(&self) -> &'static str {
        match self {
            Self::TypeError { .. } => "TypeError",
            Self::IndexError { .. } => "IndexError",
            Self::KeyError { .. } => "KeyError",
            Self::ZeroDivisionError => "ZeroDivisionError",
            Self::OverflowError { .. } => "OverflowError",
            Self::NameError { .. } => "NameError",
            Self::AttributeError { .. } => "AttributeError",
            Self::ImportError { .. } => "ImportError",
            Self::ValueError { .. } => "ValueError",
            Self::AssertionError { .. } => "AssertionError",
            Self::StopIteration => "StopIteration",
            Self::MemoryError { .. } => "MemoryError",
            Self::RecursionError => "RecursionError",
            Self::IoError { .. } => "IOError",
            Self::OsError { .. } => "OSError",
            Self::FileNotFoundError { .. } => "FileNotFoundError",
            Self::PermissionError { .. } => "PermissionError",
            Self::UnicodeDecodeError { .. } => "UnicodeDecodeError",
            Self::UnicodeEncodeError { .. } => "UnicodeEncodeError",
            Self::SyntaxError { .. } => "SyntaxError",
            Self::InternalError { .. } => "RuntimeError",
            Self::NotImplementedError { .. } => "NotImplementedError",
        }
    }
}

impl From<std::io::Error> for RuntimeError {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::NotFound => Self::FileNotFoundError {
                path: err.to_string(),
            },
            std::io::ErrorKind::PermissionDenied => Self::PermissionError {
                path: err.to_string(),
            },
            _ => Self::IoError {
                message: err.to_string(),
            },
        }
    }
}

impl From<std::num::ParseIntError> for RuntimeError {
    fn from(err: std::num::ParseIntError) -> Self {
        Self::ValueError {
            message: format!("invalid literal for int(): {}", err),
        }
    }
}

impl From<std::num::ParseFloatError> for RuntimeError {
    fn from(err: std::num::ParseFloatError) -> Self {
        Self::ValueError {
            message: format!("could not convert string to float: {}", err),
        }
    }
}

impl From<std::str::Utf8Error> for RuntimeError {
    fn from(err: std::str::Utf8Error) -> Self {
        Self::UnicodeDecodeError {
            message: err.to_string(),
        }
    }
}

impl From<std::string::FromUtf8Error> for RuntimeError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Self::UnicodeDecodeError {
            message: err.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_error() {
        let err = RuntimeError::type_error("int", "str");
        assert_eq!(err.exception_name(), "TypeError");
        assert!(err.to_string().contains("expected int"));
        assert!(err.to_string().contains("got str"));
    }

    #[test]
    fn test_index_error() {
        let err = RuntimeError::index_error(10, 5);
        assert_eq!(err.exception_name(), "IndexError");
        assert!(err.to_string().contains("10"));
        assert!(err.to_string().contains("5"));
    }

    #[test]
    fn test_key_error() {
        let err = RuntimeError::key_error("missing_key");
        assert_eq!(err.exception_name(), "KeyError");
        assert!(err.to_string().contains("missing_key"));
    }

    #[test]
    fn test_name_error() {
        let err = RuntimeError::name_error("undefined_var");
        assert_eq!(err.exception_name(), "NameError");
        assert!(err.to_string().contains("undefined_var"));
    }

    #[test]
    fn test_attribute_error() {
        let err = RuntimeError::attribute_error("int", "foo");
        assert_eq!(err.exception_name(), "AttributeError");
        assert!(err.to_string().contains("int"));
        assert!(err.to_string().contains("foo"));
    }

    #[test]
    fn test_zero_division() {
        let err = RuntimeError::ZeroDivisionError;
        assert_eq!(err.exception_name(), "ZeroDivisionError");
    }

    #[test]
    fn test_from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err: RuntimeError = io_err.into();
        assert_eq!(err.exception_name(), "FileNotFoundError");
    }

    #[test]
    fn test_from_parse_int_error() {
        let parse_err: Result<i64, _> = "not_a_number".parse();
        let err: RuntimeError = parse_err.unwrap_err().into();
        assert_eq!(err.exception_name(), "ValueError");
    }
}
