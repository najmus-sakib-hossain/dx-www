//! Error types for dx-stack

use std::fmt;

/// Result type for stack operations
pub type StackResult<T> = Result<T, StackError>;

/// Errors that can occur in stack operations
#[derive(Debug)]
pub enum StackError {
    /// Language is not supported
    UnknownLanguage(String),

    /// Feature is not supported for this language
    NotSupported(String),

    /// No stack available for language (e.g., Rust uses cargo)
    NoStackRequired(crate::Language),

    /// Component not available
    ComponentNotAvailable(crate::StackCapability),

    /// IO error
    Io(std::io::Error),

    /// Configuration error
    Config(String),

    /// Process execution error
    Process {
        command: String,
        exit_code: Option<i32>,
        stderr: String,
    },

    /// Generic error
    Other(String),
}

impl fmt::Display for StackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StackError::UnknownLanguage(lang) => {
                write!(
                    f,
                    "Unknown language: '{}'. Use 'dx stack --help' to see supported languages.",
                    lang
                )
            }
            StackError::NotSupported(feature) => {
                write!(f, "Feature not supported: {}", feature)
            }
            StackError::NoStackRequired(lang) => {
                write!(
                    f,
                    "{} does not need a DX stack - use its native toolchain ({}) instead",
                    lang,
                    lang.native_toolchain()
                )
            }
            StackError::ComponentNotAvailable(cap) => {
                write!(f, "Component not available: {}", cap)
            }
            StackError::Io(e) => write!(f, "IO error: {}", e),
            StackError::Config(msg) => write!(f, "Configuration error: {}", msg),
            StackError::Process {
                command,
                exit_code,
                stderr,
            } => {
                write!(f, "Command '{}' failed", command)?;
                if let Some(code) = exit_code {
                    write!(f, " with exit code {}", code)?;
                }
                if !stderr.is_empty() {
                    write!(f, ": {}", stderr)?;
                }
                Ok(())
            }
            StackError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for StackError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            StackError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for StackError {
    fn from(e: std::io::Error) -> Self {
        StackError::Io(e)
    }
}

impl From<String> for StackError {
    fn from(s: String) -> Self {
        StackError::Other(s)
    }
}

impl From<&str> for StackError {
    fn from(s: &str) -> Self {
        StackError::Other(s.to_string())
    }
}
