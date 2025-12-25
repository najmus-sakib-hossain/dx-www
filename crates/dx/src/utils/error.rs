//! Error types for the DX CLI
//!
//! Provides a comprehensive error type with context-specific hints,
//! retryability classification, and retry logic with exponential backoff.

use std::path::PathBuf;
use std::time::Duration;
use thiserror::Error;

// ═══════════════════════════════════════════════════════════════════════════
//  ENHANCED ERROR HANDLING WITH RETRY SUPPORT
// ═══════════════════════════════════════════════════════════════════════════

/// Context information for enhanced error reporting
#[derive(Debug, Clone)]
pub struct ErrorContext {
    /// Name of the operation that failed
    pub operation: String,
    /// Additional context details
    pub details: Option<String>,
    /// File path if relevant
    pub path: Option<PathBuf>,
}

impl ErrorContext {
    /// Create a new error context
    pub fn new(operation: impl Into<String>) -> Self {
        Self {
            operation: operation.into(),
            details: None,
            path: None,
        }
    }

    /// Add details to the context
    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }

    /// Add a path to the context
    pub fn with_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.path = Some(path.into());
        self
    }
}

/// Enhanced error with retry information and context
#[derive(Debug)]
pub struct EnhancedError {
    /// The underlying error
    pub error: DxError,
    /// Number of retries attempted
    pub retry_count: u32,
    /// Maximum retries allowed
    pub max_retries: u32,
    /// Error context
    pub context: ErrorContext,
}

impl EnhancedError {
    /// Create a new enhanced error
    pub fn new(error: DxError, context: ErrorContext) -> Self {
        Self {
            error,
            retry_count: 0,
            max_retries: 0,
            context,
        }
    }

    /// Create with retry information
    pub fn with_retries(error: DxError, context: ErrorContext, retry_count: u32, max_retries: u32) -> Self {
        Self {
            error,
            retry_count,
            max_retries,
            context,
        }
    }

    /// Display a user-friendly message with full context
    pub fn display_message(&self) -> String {
        let mut msg = format!("Error during {}: {}", self.context.operation, self.error);
        
        if let Some(ref details) = self.context.details {
            msg.push_str(&format!("\n  Details: {}", details));
        }
        
        if let Some(ref path) = self.context.path {
            msg.push_str(&format!("\n  Path: {}", path.display()));
        }
        
        if self.retry_count > 0 {
            msg.push_str(&format!("\n  Retried {} of {} times", self.retry_count, self.max_retries));
        }
        
        if let Some(hint) = self.error.hint() {
            msg.push_str(&format!("\n  Hint: {}", hint));
        }
        
        msg
    }

    /// Check if the operation should be retried
    pub fn should_retry(&self) -> bool {
        self.retry_count < self.max_retries && self.error.is_retryable()
    }

    /// Calculate the next retry delay using exponential backoff
    /// Base delay is 1 second, doubling each retry: 1s, 2s, 4s, 8s...
    pub fn next_retry_delay(&self) -> Duration {
        let base_delay_ms = 1000u64;
        let multiplier = 1u64 << self.retry_count.min(10); // Cap at 2^10 to prevent overflow
        Duration::from_millis(base_delay_ms * multiplier)
    }
}

impl std::fmt::Display for EnhancedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_message())
    }
}

impl std::error::Error for EnhancedError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}

/// Execute an async operation with retry logic and exponential backoff
///
/// # Arguments
/// * `operation_name` - Name of the operation for error context
/// * `max_retries` - Maximum number of retry attempts
/// * `operation` - Async closure that returns Result<T, DxError>
///
/// # Returns
/// * `Ok(T)` - If the operation succeeds
/// * `Err(EnhancedError)` - If all retries are exhausted
///
/// Requirement 1.1, 3.1: Retry with exponential backoff (1s, 2s, 4s)
pub async fn with_retry<T, F, Fut>(
    operation_name: &str,
    max_retries: u32,
    mut operation: F,
) -> Result<T, EnhancedError>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, DxError>>,
{
    let context = ErrorContext::new(operation_name);
    let mut retry_count = 0;

    loop {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(error) => {
                let enhanced = EnhancedError::with_retries(
                    error,
                    context.clone(),
                    retry_count,
                    max_retries,
                );

                if enhanced.should_retry() {
                    let delay = enhanced.next_retry_delay();
                    tokio::time::sleep(delay).await;
                    retry_count += 1;
                } else {
                    return Err(EnhancedError::with_retries(
                        enhanced.error,
                        enhanced.context,
                        retry_count,
                        max_retries,
                    ));
                }
            }
        }
    }
}

/// Synchronous version of with_retry for non-async contexts
pub fn with_retry_sync<T, F>(
    operation_name: &str,
    max_retries: u32,
    mut operation: F,
) -> Result<T, EnhancedError>
where
    F: FnMut() -> Result<T, DxError>,
{
    let context = ErrorContext::new(operation_name);
    let mut retry_count = 0;

    loop {
        match operation() {
            Ok(result) => return Ok(result),
            Err(error) => {
                let enhanced = EnhancedError::with_retries(
                    error,
                    context.clone(),
                    retry_count,
                    max_retries,
                );

                if enhanced.should_retry() {
                    let delay = enhanced.next_retry_delay();
                    std::thread::sleep(delay);
                    retry_count += 1;
                } else {
                    return Err(EnhancedError::with_retries(
                        enhanced.error,
                        enhanced.context,
                        retry_count,
                        max_retries,
                    ));
                }
            }
        }
    }
}

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
    /// Requirement 1.3, 3.4, 3.6, 10.3, 11.5: Provide context-specific suggestions
    /// All hinted error types return non-empty strings
    pub fn hint(&self) -> Option<&'static str> {
        match self {
            // Configuration errors
            DxError::ConfigNotFound { .. } => {
                Some("Run `dx init` to create a new project with dx.toml")
            }
            DxError::ConfigInvalid { .. } => {
                Some("Check the TOML syntax and ensure all required fields are present")
            }
            DxError::ConfigMissingField { .. } => {
                Some("Add the missing field to your dx.toml configuration")
            }

            // File system errors with enhanced hints
            DxError::FileNotFound { .. } => Some("Check that the file path is correct and the file exists"),
            DxError::DirectoryNotFound { .. } => Some("Check that the directory path is correct and exists"),
            DxError::PermissionDenied { path } => {
                // Enhanced hint for permission denied (Requirement 3.4, 11.5)
                let path_str = path.to_string_lossy();
                if path_str.contains(".dx") || path_str.contains("dx.toml") {
                    Some("Check file ownership with `ls -la`. Try `chmod 644` for files or `chmod 755` for directories. On Windows, right-click → Properties → Security")
                } else {
                    Some("Try running with elevated permissions (sudo on Unix, Run as Administrator on Windows), or check file ownership with `ls -la`")
                }
            }
            DxError::FileExists { .. } => Some("Use --force to overwrite existing files"),
            DxError::Io { message } => {
                // Enhanced I/O error hints
                if message.contains("disk full") || message.contains("no space") {
                    Some("Free up disk space and try again. Check available space with `df -h` (Unix) or `dir` (Windows)")
                } else if message.contains("too many open files") {
                    Some("Close some applications or increase the file descriptor limit with `ulimit -n`")
                } else {
                    Some("Check disk space and file system permissions")
                }
            }
            DxError::SymlinkLoop { .. } => Some("Check for circular symbolic links. Use `ls -la` to inspect symlink targets"),

            // Network errors with enhanced hints (Requirement 3.4, 3.6)
            DxError::Network { message } => {
                // DNS resolution errors
                if message.contains("dns") || message.contains("DNS") || message.contains("resolve") || message.contains("getaddrinfo") {
                    Some("DNS resolution failed. Check your DNS settings, try `nslookup` to test, or use a different DNS server (e.g., 8.8.8.8)")
                } else if message.contains("connection refused") {
                    Some("Connection refused. Check if the server is running and the port is correct")
                } else if message.contains("connection reset") {
                    Some("Connection was reset. This may be a firewall issue or server problem. Try again later")
                } else {
                    Some("Check your internet connection. Try `ping google.com` to test connectivity")
                }
            }
            DxError::Timeout { .. } => Some("Request timed out. Check your network connection, try again, or increase timeout with --timeout"),
            
            // TLS/SSL errors with comprehensive hints (Requirement 3.4, 3.6)
            DxError::Tls { message } => {
                if message.contains("certificate") || message.contains("cert") {
                    Some("SSL certificate error. Update your CA certificates: `update-ca-certificates` (Linux), or check system date/time is correct")
                } else if message.contains("handshake") {
                    Some("TLS handshake failed. The server may not support your TLS version. Check firewall/proxy settings")
                } else if message.contains("expired") {
                    Some("Certificate has expired. Check your system date/time, or the server's certificate needs renewal")
                } else {
                    Some("TLS error. Update CA certificates, check system date/time, or try with --insecure (not recommended)")
                }
            }
            
            // HTTP errors
            DxError::Http { status, .. } if *status == 401 => {
                Some("Authentication required. Check your credentials or API token")
            }
            DxError::Http { status, .. } if *status == 403 => {
                Some("Access forbidden. Check your permissions or API token scope")
            }
            DxError::Http { status, .. } if *status == 404 => {
                Some("Resource not found. Check the URL or resource name")
            }
            DxError::Http { status, .. } if *status == 429 => {
                Some("Rate limited. Wait a moment and try again, or check API rate limits")
            }
            DxError::Http { status, .. } if *status >= 500 && *status < 600 => {
                Some("Server error. The service may be experiencing issues. Try again later")
            }
            DxError::Http { .. } => {
                Some("HTTP request failed. Check the URL and try again")
            }

            // Tool errors
            DxError::ToolNotInstalled { .. } => {
                Some("Install the tool with `dx forge install <tool>`")
            }
            DxError::ToolVersionMismatch { .. } => {
                Some("Update the tool with `dx forge update <tool>` or specify a compatible version")
            }
            DxError::ToolExecutionFailed { .. } => {
                Some("Tool execution failed. Check the tool's logs or run with --verbose for details")
            }

            // Build errors
            DxError::BuildFailed { .. } => {
                Some("Build failed. Check the error message above and fix the issues")
            }
            DxError::CompilationError { .. } => {
                Some("Compilation error. Fix the syntax error at the indicated location")
            }

            // Update errors
            DxError::SignatureInvalid => {
                Some("Signature verification failed. The download may be corrupted or tampered with. Try downloading again from the official source")
            }
            DxError::ChecksumMismatch { .. } => {
                Some("Checksum mismatch. The download may be corrupted. Try downloading again")
            }
            DxError::UpdateDownloadFailed { .. } => {
                Some("Update download failed. Check your internet connection and try again")
            }
            DxError::DeltaPatchFailed { .. } => {
                Some("Delta patch failed. Try a full download with --full-download")
            }

            // Shell errors
            DxError::ShellNotDetected => {
                Some("Could not detect your shell. Specify it with --shell (bash, zsh, fish, powershell)")
            }
            DxError::ShellIntegrationExists { .. } => {
                Some("Shell integration already installed. Use --force to reinstall")
            }

            // General errors
            DxError::InvalidArgument { .. } => {
                Some("Invalid argument provided. Check the command help with --help")
            }
            DxError::Cancelled => {
                Some("Operation was cancelled")
            }
            DxError::Internal { .. } => {
                Some("An internal error occurred. Please report this issue with the error details")
            }
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

    // ═══════════════════════════════════════════════════════════════════
    //  ENHANCED ERROR TESTS
    // ═══════════════════════════════════════════════════════════════════

    #[test]
    fn test_error_context() {
        let ctx = ErrorContext::new("download")
            .with_details("fetching package")
            .with_path("/tmp/pkg.tar.gz");
        
        assert_eq!(ctx.operation, "download");
        assert_eq!(ctx.details, Some("fetching package".to_string()));
        assert_eq!(ctx.path, Some(PathBuf::from("/tmp/pkg.tar.gz")));
    }

    #[test]
    fn test_enhanced_error_display() {
        let err = DxError::network("connection refused");
        let ctx = ErrorContext::new("download").with_details("fetching package");
        let enhanced = EnhancedError::with_retries(err, ctx, 2, 3);
        
        let msg = enhanced.display_message();
        assert!(msg.contains("download"));
        assert!(msg.contains("connection refused"));
        assert!(msg.contains("fetching package"));
        assert!(msg.contains("Retried 2 of 3"));
        assert!(msg.contains("Hint:"));
    }

    #[test]
    fn test_enhanced_error_should_retry() {
        let ctx = ErrorContext::new("test");
        
        // Retryable error with retries remaining
        let err = EnhancedError::with_retries(DxError::network("error"), ctx.clone(), 1, 3);
        assert!(err.should_retry());
        
        // Retryable error with no retries remaining
        let err = EnhancedError::with_retries(DxError::network("error"), ctx.clone(), 3, 3);
        assert!(!err.should_retry());
        
        // Non-retryable error
        let err = EnhancedError::with_retries(DxError::config_not_found("dx.toml"), ctx.clone(), 0, 3);
        assert!(!err.should_retry());
    }

    #[test]
    fn test_retry_delay_exponential() {
        let ctx = ErrorContext::new("test");
        
        // First retry: 1s
        let err = EnhancedError::with_retries(DxError::network("error"), ctx.clone(), 0, 3);
        assert_eq!(err.next_retry_delay(), Duration::from_secs(1));
        
        // Second retry: 2s
        let err = EnhancedError::with_retries(DxError::network("error"), ctx.clone(), 1, 3);
        assert_eq!(err.next_retry_delay(), Duration::from_secs(2));
        
        // Third retry: 4s
        let err = EnhancedError::with_retries(DxError::network("error"), ctx.clone(), 2, 3);
        assert_eq!(err.next_retry_delay(), Duration::from_secs(4));
        
        // Fourth retry: 8s
        let err = EnhancedError::with_retries(DxError::network("error"), ctx.clone(), 3, 5);
        assert_eq!(err.next_retry_delay(), Duration::from_secs(8));
    }

    // ═══════════════════════════════════════════════════════════════════
    //  PROPERTY TESTS
    // ═══════════════════════════════════════════════════════════════════

    // Feature: dx-cli, Property 1: Retry with Exponential Backoff
    // Validates: Requirements 1.1, 3.1
    //
    // For any retry_count n, the delay should be 2^n seconds (base 1s).
    // Delays should strictly increase with retry count.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_retry_exponential_backoff(retry_count in 0u32..10) {
            let ctx = ErrorContext::new("test");
            let err = EnhancedError::with_retries(DxError::network("error"), ctx, retry_count, 10);
            
            let delay = err.next_retry_delay();
            let expected_ms = 1000u64 * (1u64 << retry_count);
            
            prop_assert_eq!(
                delay.as_millis() as u64,
                expected_ms,
                "Delay for retry {} should be {}ms, got {}ms",
                retry_count,
                expected_ms,
                delay.as_millis()
            );
        }

        #[test]
        fn prop_retry_delays_increase(retry_count in 0u32..9) {
            let ctx = ErrorContext::new("test");
            let err1 = EnhancedError::with_retries(DxError::network("error"), ctx.clone(), retry_count, 10);
            let err2 = EnhancedError::with_retries(DxError::network("error"), ctx, retry_count + 1, 10);
            
            prop_assert!(
                err2.next_retry_delay() > err1.next_retry_delay(),
                "Delay should increase with retry count"
            );
        }

        #[test]
        fn prop_retry_delay_capped(retry_count in 10u32..100) {
            let ctx = ErrorContext::new("test");
            let err = EnhancedError::with_retries(DxError::network("error"), ctx, retry_count, 100);
            
            // Should be capped at 2^10 = 1024 seconds
            let delay = err.next_retry_delay();
            let max_delay = Duration::from_secs(1024);
            
            prop_assert!(
                delay <= max_delay,
                "Delay should be capped at 1024s, got {:?}",
                delay
            );
        }
    }

    // Feature: dx-cli, Property 2: Error Retryability Classification
    // Validates: Requirements 1.7
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

        #[test]
        fn prop_permission_errors_not_retryable(path in "[a-zA-Z0-9/._-]{1,50}") {
            let err = DxError::PermissionDenied { path: PathBuf::from(path) };
            prop_assert!(!err.is_retryable(), "Permission errors should not be retryable");
        }

        #[test]
        fn prop_build_errors_not_retryable(message in "[a-zA-Z0-9 ]{1,50}") {
            let err = DxError::BuildFailed { message };
            prop_assert!(!err.is_retryable(), "Build errors should not be retryable");
        }
    }

    // Feature: dx-cli, Property 3: Error Hints Completeness
    // Validates: Requirements 1.3, 3.4, 3.6, 11.5
    //
    // All error types should have a non-empty hint string.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_network_errors_have_hints(message in "[a-zA-Z0-9 ]{1,50}") {
            let err = DxError::Network { message };
            let hint = err.hint();
            prop_assert!(hint.is_some(), "Network errors should have hints");
            prop_assert!(!hint.unwrap().is_empty(), "Hint should not be empty");
        }

        #[test]
        fn prop_tls_errors_have_hints(message in "[a-zA-Z0-9 ]{1,50}") {
            let err = DxError::Tls { message };
            let hint = err.hint();
            prop_assert!(hint.is_some(), "TLS errors should have hints");
            prop_assert!(!hint.unwrap().is_empty(), "Hint should not be empty");
        }

        #[test]
        fn prop_permission_errors_have_hints(path in "[a-zA-Z0-9/._-]{1,50}") {
            let err = DxError::PermissionDenied { path: PathBuf::from(path) };
            let hint = err.hint();
            prop_assert!(hint.is_some(), "Permission errors should have hints");
            prop_assert!(!hint.unwrap().is_empty(), "Hint should not be empty");
        }

        #[test]
        fn prop_config_errors_have_hints(path in "[a-zA-Z0-9/._-]{1,50}") {
            let err = DxError::ConfigNotFound { path: PathBuf::from(path) };
            let hint = err.hint();
            prop_assert!(hint.is_some(), "Config errors should have hints");
            prop_assert!(!hint.unwrap().is_empty(), "Hint should not be empty");
        }

        #[test]
        fn prop_http_errors_have_hints(status in 100u16..600) {
            let err = DxError::Http { status, message: "error".to_string() };
            let hint = err.hint();
            prop_assert!(hint.is_some(), "HTTP errors should have hints for status {}", status);
            prop_assert!(!hint.unwrap().is_empty(), "Hint should not be empty");
        }

        #[test]
        fn prop_io_errors_have_hints(message in "[a-zA-Z0-9 ]{1,50}") {
            let err = DxError::Io { message };
            let hint = err.hint();
            prop_assert!(hint.is_some(), "IO errors should have hints");
            prop_assert!(!hint.unwrap().is_empty(), "Hint should not be empty");
        }

        #[test]
        fn prop_tool_errors_have_hints(name in "[a-zA-Z0-9-]{1,30}") {
            let err = DxError::ToolNotInstalled { name };
            let hint = err.hint();
            prop_assert!(hint.is_some(), "Tool errors should have hints");
            prop_assert!(!hint.unwrap().is_empty(), "Hint should not be empty");
        }
    }

    // Additional unit tests for specific hint content
    #[test]
    fn test_dns_error_hint() {
        let err = DxError::Network { message: "dns resolution failed".to_string() };
        let hint = err.hint().unwrap();
        assert!(hint.contains("DNS"), "DNS error hint should mention DNS");
    }

    #[test]
    fn test_tls_certificate_hint() {
        let err = DxError::Tls { message: "certificate expired".to_string() };
        let hint = err.hint().unwrap();
        assert!(hint.contains("expired") || hint.contains("certificate"), "TLS cert hint should be relevant");
    }

    #[test]
    fn test_rate_limit_hint() {
        let err = DxError::Http { status: 429, message: "too many requests".to_string() };
        let hint = err.hint().unwrap();
        assert!(hint.contains("Rate") || hint.contains("rate"), "429 hint should mention rate limiting");
    }
}
