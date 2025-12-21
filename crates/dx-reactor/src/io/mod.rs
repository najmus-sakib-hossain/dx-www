//! Cross-platform I/O abstraction layer.
//!
//! This module provides a unified `Reactor` trait with platform-specific implementations.

mod config;
mod completion;
mod interest;

#[cfg(all(target_os = "linux", feature = "io_uring"))]
pub mod uring;

#[cfg(target_os = "linux")]
pub mod epoll;

#[cfg(any(target_os = "macos", target_os = "freebsd", target_os = "openbsd"))]
pub mod kqueue;

#[cfg(target_os = "windows")]
pub mod iocp;

pub use config::ReactorConfig;
pub use completion::Completion;
pub use interest::Interest;

use std::io;
use std::time::Duration;

/// Handle to a registered I/O resource.
pub trait IoHandle: Send + Sync + Clone + 'static {
    /// Get the user data associated with this handle.
    fn user_data(&self) -> u64;
}

/// Unified I/O reactor trait.
///
/// This trait abstracts platform-specific I/O operations, providing a consistent
/// interface across io_uring, kqueue, IOCP, and epoll backends.
pub trait Reactor: Send + Sync + 'static {
    /// Handle type for registered I/O resources.
    type Handle: IoHandle;

    /// Create a new reactor instance with the given configuration.
    fn new(config: ReactorConfig) -> io::Result<Self>
    where
        Self: Sized;

    /// Register a file descriptor for events.
    ///
    /// # Arguments
    /// * `fd` - The raw file descriptor (or handle on Windows)
    /// * `interest` - The events to monitor (read, write, or both)
    ///
    /// # Returns
    /// A handle that can be used to reference this registration.
    #[cfg(unix)]
    fn register(&self, fd: std::os::unix::io::RawFd, interest: Interest) -> io::Result<Self::Handle>;

    #[cfg(windows)]
    fn register(&self, handle: std::os::windows::io::RawHandle, interest: Interest) -> io::Result<Self::Handle>;

    /// Submit pending I/O operations.
    ///
    /// # Returns
    /// The number of operations successfully submitted.
    fn submit(&self) -> io::Result<usize>;

    /// Wait for I/O completions.
    ///
    /// # Arguments
    /// * `timeout` - Optional timeout duration. None means wait indefinitely.
    ///
    /// # Returns
    /// A vector of completed operations.
    fn wait(&self, timeout: Option<Duration>) -> io::Result<Vec<Completion>>;

    /// Submit pending operations and wait for completions (optimized path).
    ///
    /// This combines `submit()` and `wait()` into a single syscall where possible.
    ///
    /// # Arguments
    /// * `min_complete` - Minimum number of completions to wait for.
    ///
    /// # Returns
    /// A vector of completed operations.
    fn submit_and_wait(&self, min_complete: usize) -> io::Result<Vec<Completion>>;
}

/// Platform-specific reactor type alias.
///
/// This is automatically selected at compile time based on the target platform
/// and enabled features.
#[cfg(all(target_os = "linux", feature = "io_uring"))]
pub type PlatformReactor = uring::UringReactor;

#[cfg(all(target_os = "linux", not(feature = "io_uring")))]
pub type PlatformReactor = epoll::EpollReactor;

#[cfg(any(target_os = "macos", target_os = "freebsd", target_os = "openbsd"))]
pub type PlatformReactor = kqueue::KqueueReactor;

#[cfg(target_os = "windows")]
pub type PlatformReactor = iocp::IocpReactor;

/// Detect and return the best available I/O backend for the current platform.
pub fn best_available() -> &'static str {
    #[cfg(all(target_os = "linux", feature = "io_uring"))]
    {
        if uring::is_available() {
            return "io_uring";
        }
        return "epoll";
    }

    #[cfg(all(target_os = "linux", not(feature = "io_uring")))]
    {
        return "epoll";
    }

    #[cfg(any(target_os = "macos", target_os = "freebsd", target_os = "openbsd"))]
    {
        return "kqueue";
    }

    #[cfg(target_os = "windows")]
    {
        return "iocp";
    }

    #[cfg(not(any(
        target_os = "linux",
        target_os = "macos",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "windows"
    )))]
    {
        return "unsupported";
    }
}
