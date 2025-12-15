//! Error types for DX Media.
//!
//! This module provides a comprehensive error hierarchy for all operations.

use std::path::PathBuf;
use thiserror::Error;

/// Result type alias using [`DxError`].
pub type Result<T> = std::result::Result<T, DxError>;

/// Main error type for DX Media operations.
#[derive(Error, Debug)]
pub enum DxError {
    // ─────────────────────────────────────────────────────────────
    // Configuration Errors
    // ─────────────────────────────────────────────────────────────
    /// Configuration file not found or invalid.
    #[error("Configuration error: {message}")]
    Config {
        /// Error description.
        message: String,
        /// Underlying cause.
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Missing required API key.
    #[error("Missing API key for provider '{provider}'. Add {env_var} to your .env file")]
    MissingApiKey {
        /// Provider name.
        provider: String,
        /// Environment variable name.
        env_var: String,
    },

    // ─────────────────────────────────────────────────────────────
    // API & Network Errors
    // ─────────────────────────────────────────────────────────────
    /// HTTP request failed.
    #[error("HTTP request failed: {message}")]
    Http {
        /// Error description.
        message: String,
        /// HTTP status code if available.
        status_code: Option<u16>,
        /// Underlying cause.
        #[source]
        source: Option<reqwest::Error>,
    },

    /// Rate limit exceeded for a provider.
    #[error(
        "Rate limit exceeded for provider '{provider}'. Retry after {retry_after_secs} seconds"
    )]
    RateLimited {
        /// Provider name.
        provider: String,
        /// Seconds to wait before retry.
        retry_after_secs: u64,
    },

    /// Provider API returned an error.
    #[error("Provider '{provider}' returned error: {message}")]
    ProviderApi {
        /// Provider name.
        provider: String,
        /// Error message from provider.
        message: String,
        /// HTTP status code.
        status_code: u16,
    },

    /// Provider is not available (disabled or missing API key).
    #[error("Provider '{provider}' is not available: {reason}")]
    ProviderUnavailable {
        /// Provider name.
        provider: String,
        /// Reason for unavailability.
        reason: String,
    },

    // ─────────────────────────────────────────────────────────────
    // Download & File Errors
    // ─────────────────────────────────────────────────────────────
    /// Download failed.
    #[error("Failed to download '{url}': {message}")]
    Download {
        /// URL that failed.
        url: String,
        /// Error description.
        message: String,
    },

    /// File I/O error.
    #[error("File operation failed for '{path}': {message}")]
    FileIo {
        /// File path.
        path: PathBuf,
        /// Error description.
        message: String,
        /// Underlying cause.
        #[source]
        source: Option<std::io::Error>,
    },

    // ─────────────────────────────────────────────────────────────
    // Search Errors
    // ─────────────────────────────────────────────────────────────
    /// No results found.
    #[error("No results found for query '{query}'")]
    NoResults {
        /// Search query.
        query: String,
    },

    /// Invalid search query.
    #[error("Invalid search query: {message}")]
    InvalidQuery {
        /// Error description.
        message: String,
    },

    // ─────────────────────────────────────────────────────────────
    // Parse Errors
    // ─────────────────────────────────────────────────────────────
    /// JSON parsing failed.
    #[error("Failed to parse JSON response: {message}")]
    JsonParse {
        /// Error description.
        message: String,
        /// Underlying cause.
        #[source]
        source: Option<serde_json::Error>,
    },

    /// Invalid media type.
    #[error("Invalid media type: '{value}'")]
    InvalidMediaType {
        /// Invalid value provided.
        value: String,
    },

    // ─────────────────────────────────────────────────────────────
    // Internal Errors
    // ─────────────────────────────────────────────────────────────
    /// Internal error (should not happen).
    #[error("Internal error: {message}")]
    Internal {
        /// Error description.
        message: String,
    },
}

impl DxError {
    /// Create a configuration error.
    pub fn config(message: impl Into<String>) -> Self {
        Self::Config {
            message: message.into(),
            source: None,
        }
    }

    /// Create an HTTP error.
    pub fn http(message: impl Into<String>) -> Self {
        Self::Http {
            message: message.into(),
            status_code: None,
            source: None,
        }
    }

    /// Create a download error.
    pub fn download(url: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Download {
            url: url.into(),
            message: message.into(),
        }
    }

    /// Create a file I/O error.
    pub fn file_io(path: impl Into<PathBuf>, message: impl Into<String>) -> Self {
        Self::FileIo {
            path: path.into(),
            message: message.into(),
            source: None,
        }
    }

    /// Check if this error is retryable.
    #[must_use]
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::RateLimited { .. }
                | Self::Http {
                    status_code: Some(500..=599),
                    ..
                }
        )
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// FROM IMPLEMENTATIONS
// ═══════════════════════════════════════════════════════════════════════════════

impl From<reqwest::Error> for DxError {
    fn from(err: reqwest::Error) -> Self {
        let status_code = err.status().map(|s| s.as_u16());
        Self::Http {
            message: err.to_string(),
            status_code,
            source: Some(err),
        }
    }
}

impl From<std::io::Error> for DxError {
    fn from(err: std::io::Error) -> Self {
        Self::FileIo {
            path: PathBuf::new(),
            message: err.to_string(),
            source: Some(err),
        }
    }
}

impl From<serde_json::Error> for DxError {
    fn from(err: serde_json::Error) -> Self {
        Self::JsonParse {
            message: err.to_string(),
            source: Some(err),
        }
    }
}

impl From<url::ParseError> for DxError {
    fn from(err: url::ParseError) -> Self {
        Self::InvalidQuery {
            message: format!("Invalid URL: {err}"),
        }
    }
}
