//! Platform I/O backend selector.
//!
//! This module provides automatic selection of the most performant I/O backend
//! for the current platform.

use std::sync::Arc;
use tracing::{info, debug};

use super::{PlatformIO, FallbackBackend, IoBackend, Platform, PlatformInfo};

#[cfg(target_os = "linux")]
use super::IoUringBackend;

#[cfg(target_os = "macos")]
use super::KqueueBackend;

#[cfg(target_os = "windows")]
use super::IocpBackend;

/// Create the most performant platform I/O backend for the current system.
///
/// This function automatically detects the platform and selects the best
/// available I/O backend:
/// - Linux: io_uring (if kernel 5.1+ and available)
/// - macOS: kqueue
/// - Windows: IOCP
/// - Fallback: tokio async I/O (always available)
///
/// # Example
/// ```rust,ignore
/// use dx_forge::platform_io::create_platform_io;
///
/// let io = create_platform_io();
/// println!("Using backend: {}", io.backend_name());
/// ```
pub fn create_platform_io() -> Arc<dyn PlatformIO> {
    let platform = Platform::current();
    debug!("Detected platform: {:?}", platform);

    #[cfg(target_os = "linux")]
    {
        if IoUringBackend::is_available() {
            match IoUringBackend::new(256) {
                Ok(backend) => {
                    info!("Using io_uring backend");
                    return Arc::new(backend);
                }
                Err(e) => {
                    debug!("Failed to initialize io_uring backend: {}", e);
                }
            }
        } else {
            debug!("io_uring not available on this system");
        }
    }

    #[cfg(target_os = "macos")]
    {
        if KqueueBackend::is_available() {
            match KqueueBackend::new() {
                Ok(backend) => {
                    info!("Using kqueue backend");
                    return Arc::new(backend);
                }
                Err(e) => {
                    debug!("Failed to initialize kqueue backend: {}", e);
                }
            }
        } else {
            debug!("kqueue not available on this system");
        }
    }

    #[cfg(target_os = "windows")]
    {
        if IocpBackend::is_available() {
            match IocpBackend::new(num_cpus::get()) {
                Ok(backend) => {
                    info!("Using IOCP backend");
                    return Arc::new(backend);
                }
                Err(e) => {
                    debug!("Failed to initialize IOCP backend: {}", e);
                }
            }
        } else {
            debug!("IOCP not available on this system");
        }
    }

    info!("Using fallback (tokio) backend");
    Arc::new(FallbackBackend::new())
}

/// Get information about the current platform and I/O backend.
pub fn get_platform_info() -> PlatformInfo {
    let backend = detect_active_backend();
    PlatformInfo::current(backend)
}

/// Detect which backend would be active on this platform.
fn detect_active_backend() -> IoBackend {
    #[cfg(target_os = "linux")]
    {
        if IoUringBackend::is_available() {
            return IoBackend::IoUring;
        }
    }

    #[cfg(target_os = "macos")]
    {
        if KqueueBackend::is_available() {
            return IoBackend::Kqueue;
        }
    }

    #[cfg(target_os = "windows")]
    {
        if IocpBackend::is_available() {
            return IoBackend::Iocp;
        }
    }

    IoBackend::Fallback
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_platform_io() {
        let io = create_platform_io();
        // Should always succeed and return a valid backend
        let name = io.backend_name();
        assert!(!name.is_empty());
    }

    #[test]
    fn test_get_platform_info() {
        let info = get_platform_info();
        assert_eq!(info.platform, Platform::current());
    }

    #[test]
    fn test_detect_active_backend() {
        let backend = detect_active_backend();
        // On any platform, we should get a valid backend
        let name = backend.name();
        assert!(!name.is_empty());
    }
}


// ============================================================================
// Property-Based Tests
// ============================================================================

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    // Feature: platform-native-io-hardening, Property 1: Platform Detection Correctness
    // For any execution of the platform detection logic, the detected platform SHALL
    // match the actual operating system the code is running on.
    // **Validates: Requirements 1.1**
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_platform_detection_correctness(_dummy in 0..100u32) {
            // The platform detection should always return the correct platform
            let detected = Platform::current();

            #[cfg(target_os = "linux")]
            prop_assert_eq!(detected, Platform::Linux);

            #[cfg(target_os = "macos")]
            prop_assert_eq!(detected, Platform::MacOS);

            #[cfg(target_os = "windows")]
            prop_assert_eq!(detected, Platform::Windows);

            // Platform name should be consistent
            let name = detected.name();
            prop_assert!(!name.is_empty());

            #[cfg(target_os = "linux")]
            prop_assert_eq!(name, "linux");

            #[cfg(target_os = "macos")]
            prop_assert_eq!(name, "macos");

            #[cfg(target_os = "windows")]
            prop_assert_eq!(name, "windows");
        }

        #[test]
        fn prop_backend_detection_consistency(_dummy in 0..100u32) {
            // Backend detection should be consistent across calls
            let backend1 = detect_active_backend();
            let backend2 = detect_active_backend();

            prop_assert_eq!(backend1, backend2);

            // Backend name should be non-empty
            prop_assert!(!backend1.name().is_empty());
        }

        #[test]
        fn prop_platform_info_consistency(_dummy in 0..100u32) {
            // Platform info should be consistent
            let info1 = get_platform_info();
            let info2 = get_platform_info();

            prop_assert_eq!(info1.platform, info2.platform);
            prop_assert_eq!(info1.backend, info2.backend);
        }
    }

    // Feature: platform-native-io-hardening, Property 2: Fallback Behavior Guarantee
    // For any platform where the native I/O backend initialization fails or is unavailable,
    // the system SHALL successfully fall back to the tokio-based backend and all I/O
    // operations SHALL complete successfully.
    // **Validates: Requirements 1.5**
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_fallback_always_available(_dummy in 0..100u32) {
            // Fallback backend should always be available
            prop_assert!(FallbackBackend::is_available());
        }

        #[test]
        fn prop_create_platform_io_never_fails(_dummy in 0..100u32) {
            // create_platform_io should never fail - it always falls back to tokio
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let io = create_platform_io();
                // Should always return a valid backend
                let name = io.backend_name();
                prop_assert!(!name.is_empty());

                // The backend should be one of the known backends
                prop_assert!(
                    name == "io_uring" || name == "kqueue" || name == "iocp" || name == "fallback"
                );

                Ok(())
            })?;
        }
    }
}
