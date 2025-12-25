//! Error types for the DX CLI
//!
//! Provides a comprehensive error type with context-specific hints
//! and retryability classification.

use std::path::PathBuf;
use thiserror::Error;

/// Main error type for the DX CLI
#[derive(Error, Debug)]
pub enum DxError {
    // ═══════════════════════════════════════════════════════════════════
    //  CONFIGURATION ERRORS
    // ═══════════════════════════════════════════════════════════════════
    /// Configuration file not found
    #[error("Configuration file not found: {path}")]
    ConfigNotFound { path: PathBuf },

    /// Invalid configuration file
    #[error("Invalid configuration: {message}\n  → at {path}:{line}")]
    ConfigInvalid {
        path: PathBuf,
        line: usize,
        message: String,
    },

    /// Missing required configuration field
    #[error("Missing required field '{field}' in configuration")]
    ConfigMissingField { field: String },

    // ═══════════════════════════════════════════════════════════════════
    //  FILE SYSTEM ERRORS
    // ═══════════════════════════════════════════════════════════════════
    /// File not found
    #[error("File not found: {path}")]
    FileNotFound { path: PathBuf },

    /// Directory not found
    #[error("Directory not found: {path}")]
    DirectoryNotFound { path: PathBuf },

    /// Permission denied
    #[error("Permission denied: {path}")]
    PermissionDenied { path: PathBuf },

    /// File already exists
    #[error("File already exists: {path}")]
    FileExists { path: PathBuf },

    /// I/O error
    #[error("I/O error: {message}")]
    Io { message: String },

    /// Too many symlink levels
    #[error("Too many levels of symbolic links (max 40): {path}")]
    SymlinkLoop { path: PathBuf },

    // ═══════════════════════════════════════════════════════════════════
    //  NETWORK ERRORS
    // ═══════════════════════════════════════════════════════════════════
    /// Network connection error
    #[error("Network error: {message}")]
    Network { message: String },

    /// Request timeout
    #[error("Request timed out after {timeout_secs}s")]
    Timeout { timeout_secs: u64 },

    /// TLS/SSL error
    #[error("TLS error: {message}")]
    Tls { message: String },

    /// HTTP error with status code
    #[error("HTTP error {status}: {message}")]
    Http { status: u16, message: String },

    // ═══════════════════════════════════════════════════════════════════
    //  TOOL ERRORS
    // ═══════════════════════════════════════════════════════════════════
    /// Tool not installed
    #[error("Tool not installed: {name}\n  → Run `dx forge install {name}` to install")]
    ToolNotInstalled { name: String },

    /// Tool version mismatch
    #[error("Tool version mismatch: {name} requires {required}, found {found}")]
    ToolVersionMismatch {
        name: String,
        required: String,
        found: String,
    },

    /// Tool execution failed
    #[error("Tool execution failed: {name}\n  → {message}")]
    ToolExecutionFailed { name: String, message: String },

    // ═══════════════════════════════════════════════════════════════════
    //  BUILD ERRORS
    // ═══════════════════════════════════════════════════════════════════
    /// Build failed
    #[error("Build failed: {message}")]
    BuildFailed { message: String },

    /// Compilation error with source location
    #[error("Compilation error at {file}:{line}:{column}\n  → {message}")]
    CompilationError {
        file: PathBuf,
        line: usize,
        column: usize,
        message: String,
    },

    // ═══════════════════════════════════════════════════════════════════
    //  UPDATE ERRORS
    // ═══════════════════════════════════════════════════════════════════
    /// Signature verification failed
    #[error("Signature verification failed")]
    SignatureInvalid,

    /// Checksum mismatch
    #[error("Checksum mismatch: expected {expected}, got {actual}")]
    ChecksumMismatch { expected: String, actual: String },

    /// Update download failed
    #[error("Failed to download update: {message}")]
    UpdateDownloadFailed { message: String },

    /// Delta patch failed
    #[error("Delta patch failed: {message}")]
    DeltaPatchFailed { message: String },

    // ═══════════════════════════════════════════════════════════════════
    //  SHELL ERRORS
    // ═══════════════════════════════════════════════════════════════════
    /// Shell not detected
    #[error("Could not detect shell type")]
    ShellNotDetected,

    /// Shell integration already installed
    #[error("Shell integration already installed for {shell}")]
    ShellIntegrationExists { shell: String },

    // ═══════════════════════════════════════════════════════════════════
    //  GENERAL ERRORS
    // ═══════════════════════════════════════════════════════════════════
    /// Invalid argument
    #[error("Invalid argument: {message}")]
    InvalidArgument { message: String },

    /// Operation cancelled by user
    #[error("Operation cancelled")]
    Cancelled,

    /// Internal error
    #[error("Internal error: {message}")]
    Internal { message: String },
}

impl DxError {
    /// Returns a context-specific hint for the error
    ///
    /// Requirement 10.3: Provide context-specific suggestions
    pub fn hint(&self) -> Option<&'static str> {
        match self {
            DxError::ConfigNotFound { .. } => {
                Some("Run `dx init` to create a new project with dx.toml")
            }
            DxError::ConfigInvalid { .. } => {
                Some("Check the TOML syntax and ensure all required fields are present")
            }
            DxError::ConfigMissingField { .. } => {
                Some("Add the missing field to your dx.toml configuration")
            }
            DxError::FileNotFound { .. } => Some("Check that the file path is correct"),
            DxError::DirectoryNotFound { .. } => Some("Check that the directory path is correct"),
            DxError::PermissionDenied { .. } => {
                Some("Try running with elevated permissions or check file ownership")
            }
            DxError::FileExists { .. } => Some("Use --force to overwrite existing files"),
            DxError::Network { .. } => Some("Check your internet connection and try again"),
            DxError::Timeout { .. } => Some("Try again or increase the timeout with --timeout"),
            DxError::Tls { .. } => Some("Check your system's SSL certificates"),
            DxError::Http { status, .. } if *status == 401 || *status == 403 => {
                Some("Check your authentication credentials")
            }
            DxError::Http { status, .. } if *status == 404 => {
                Some("The requested resource was not found")
            }
            DxError::Http { status, .. } if *status >= 500 => {
                Some("The server is experiencing issues, try again later")
            }
            DxError::ToolNotInstalled { .. } => {
                Some("Install the tool with `dx forge install <tool>`")
            }
            DxError::ToolVersionMismatch { .. } => {
                Some("Update the tool with `dx forge update <tool>`")
            }
            DxError::SignatureInvalid => {
                Some("The download may be corrupted, try downloading again")
            }
            DxError::ChecksumMismatch { .. } => {
                Some("The download may be corrupted, try downloading again")
            }
            DxError::ShellNotDetected => {
                Some("Specify your shell with --shell (bash, zsh, fish, powershell)")
            }
            DxError::ShellIntegrationExists { .. } => {
                Some("Use --force to reinstall shell integration")
            }
            DxError::SymlinkLoop { .. } => Some("Check for circular symbolic links"),
            _ => None,
        }
    }

    /// Returns whether the error is retryable
    ///
    /// Requirement 10.4: Classify errors as retryable or not
    ///
    /// Network, Timeout, and Tls errors are considered retryable.
    /// All other errors are not retryable.
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            DxError::Network { .. } | DxError::Timeout { .. } | DxError::Tls { .. }
        )
    }

    /// Create a ConfigNotFound error
    pub fn config_not_found(path: impl Into<PathBuf>) -> Self {
        DxError::ConfigNotFound { path: path.into() }
    }

    /// Create a ConfigInvalid error
    pub fn config_invalid(path: impl Into<PathBuf>, line: usize, message: impl Into<String>) -> Self {
        DxError::ConfigInvalid {
            path: path.into(),
            line,
            message: message.into(),
        }
    }

    /// Create a FileNotFound error
    pub fn file_not_found(path: impl Into<PathBuf>) -> Self {
        DxError::FileNotFound { path: path.into() }
    }

    /// Create a PermissionDenied error
    pub fn permission_denied(path: impl Into<PathBuf>) -> Self {
        DxError::PermissionDenied { path: path.into() }
    }

    /// Create a Network error
    pub fn network(message: impl Into<String>) -> Self {
        DxError::Network {
            message: message.into(),
        }
    }

    /// Create a Timeout error
    pub fn timeout(timeout_secs: u64) -> Self {
        DxError::Timeout { timeout_secs }
    }

    /// Create a ToolNotInstalled error
    pub fn tool_not_installed(name: impl Into<String>) -> Self {
        DxError::ToolNotInstalled { name: name.into() }
    }
}

impl From<std::io::Error> for DxError {
    fn from(err: std::io::Error) -> Self {
        use std::io::ErrorKind;
        match err.kind() {
            ErrorKind::NotFound => DxError::FileNotFound {
                path: PathBuf::from("<unknown>"),
            },
            ErrorKind::PermissionDenied => DxError::PermissionDenied {
                path: PathBuf::from("<unknown>"),
            },
            _ => DxError::Io {
                message: err.to_string(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_error_display() {
        let err = DxError::config_not_found("dx.toml");
        assert!(err.to_string().contains("dx.toml"));

        let err = DxError::config_invalid("dx.toml", 10, "invalid syntax");
        assert!(err.to_string().contains("dx.toml"));
        assert!(err.to_string().contains("10"));
        assert!(err.to_string().contains("invalid syntax"));
    }

    #[test]
    fn test_error_hints() {
        let err = DxError::config_not_found("dx.toml");
        assert!(err.hint().is_some());
        assert!(err.hint().unwrap().contains("dx init"));

        let err = DxError::tool_not_installed("dx-media");
        assert!(err.hint().is_some());
        assert!(err.hint().unwrap().contains("dx forge install"));
    }

    #[test]
    fn test_retryable_errors() {
        // Network errors are retryable
        assert!(DxError::network("connection refused").is_retryable());
        assert!(DxError::timeout(30).is_retryable());
        assert!(DxError::Tls {
            message: "cert error".into()
        }
        .is_retryable());

        // Other errors are not retryable
        assert!(!DxError::config_not_found("dx.toml").is_retryable());
        assert!(!DxError::file_not_found("file.txt").is_retryable());
        assert!(!DxError::SignatureInvalid.is_retryable());
        assert!(!DxError::tool_not_installed("dx-media").is_retryable());
    }

    // Feature: dx-cli, Property 17: Error Retryability Classification
    // Validates: Requirements 10.3
    //
    // For any DxError of type Network, Timeout, or Tls, is_retryable() should
    // return true. For all other error types, is_retryable() should return false.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_network_errors_are_retryable(message in "[a-zA-Z0-9 ]{1,50}") {
            let err = DxError::Network { message };
            prop_assert!(err.is_retryable(), "Network errors should be retryable");
        }

        #[test]
        fn prop_timeout_errors_are_retryable(timeout_secs in 1u64..3600) {
            let err = DxError::Timeout { timeout_secs };
            prop_assert!(err.is_retryable(), "Timeout errors should be retryable");
        }

        #[test]
        fn prop_tls_errors_are_retryable(message in "[a-zA-Z0-9 ]{1,50}") {
            let err = DxError::Tls { message };
            prop_assert!(err.is_retryable(), "TLS errors should be retryable");
        }

        #[test]
        fn prop_config_errors_not_retryable(path in "[a-zA-Z0-9/._-]{1,50}") {
            let err = DxError::ConfigNotFound { path: PathBuf::from(path) };
            prop_assert!(!err.is_retryable(), "Config errors should not be retryable");
        }

        #[test]
        fn prop_file_errors_not_retryable(path in "[a-zA-Z0-9/._-]{1,50}") {
            let err = DxError::FileNotFound { path: PathBuf::from(path) };
            prop_assert!(!err.is_retryable(), "File errors should not be retryable");
        }

        #[test]
        fn prop_tool_errors_not_retryable(name in "[a-zA-Z0-9-]{1,30}") {
            let err = DxError::ToolNotInstalled { name };
            prop_assert!(!err.is_retryable(), "Tool errors should not be retryable");
        }
    }
}
