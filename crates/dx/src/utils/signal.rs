//! Signal handling for the DX CLI
//!
//! Provides graceful shutdown handling for Unix and Windows.
//! - Requirement 11.5: Handle SIGINT/SIGTERM on Unix, Ctrl+C on Windows

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Global flag indicating if shutdown was requested
static SHUTDOWN_REQUESTED: AtomicBool = AtomicBool::new(false);

/// Check if shutdown has been requested
pub fn is_shutdown_requested() -> bool {
    SHUTDOWN_REQUESTED.load(Ordering::SeqCst)
}

/// Request shutdown (can be called from signal handler)
pub fn request_shutdown() {
    SHUTDOWN_REQUESTED.store(true, Ordering::SeqCst);
}

/// Reset shutdown flag (useful for tests)
pub fn reset_shutdown() {
    SHUTDOWN_REQUESTED.store(false, Ordering::SeqCst);
}

/// Signal handler callback type
pub type SignalCallback = Arc<dyn Fn() + Send + Sync>;

/// Setup signal handlers for graceful shutdown
///
/// Requirement 11.5: Handle SIGINT/SIGTERM on Unix, Ctrl+C on Windows
///
/// Returns Ok(()) if handlers were set up successfully.
pub fn setup_signal_handlers<F>(callback: F) -> Result<(), ctrlc::Error>
where
    F: Fn() + Send + Sync + 'static,
{
    ctrlc::set_handler(move || {
        request_shutdown();
        callback();
    })
}

/// Setup default signal handlers that just set the shutdown flag
pub fn setup_default_handlers() -> Result<(), ctrlc::Error> {
    setup_signal_handlers(|| {})
}

#[cfg(unix)]
mod unix {
    /// Check if running on Unix
    pub fn is_unix() -> bool {
        true
    }

    /// Get the signal name for display
    pub fn signal_name(sig: i32) -> &'static str {
        match sig {
            2 => "SIGINT",
            15 => "SIGTERM",
            _ => "UNKNOWN",
        }
    }
}

#[cfg(windows)]
mod windows {
    /// Check if running on Windows
    pub fn is_windows() -> bool {
        true
    }

    /// Get the signal name for display
    pub fn signal_name(_sig: i32) -> &'static str {
        "Ctrl+C"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shutdown_flag() {
        reset_shutdown();
        assert!(!is_shutdown_requested());

        request_shutdown();
        assert!(is_shutdown_requested());

        reset_shutdown();
        assert!(!is_shutdown_requested());
    }

    #[test]
    fn test_shutdown_atomic() {
        reset_shutdown();

        // Simulate multiple requests
        for _ in 0..10 {
            request_shutdown();
        }

        assert!(is_shutdown_requested());
        reset_shutdown();
    }
}
