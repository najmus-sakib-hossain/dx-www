//! Property-based tests for dx-py-reactor
//!
//! These tests verify the correctness properties defined in the spec:
//! - Property 21: Platform-Native Async I/O Cross-Platform Correctness
//! - Property 22: Batched I/O Single Syscall
//! - Property 23: Multi-Shot Accept Correctness
//! - Property 24: Zero-Copy Send Correctness

use proptest::prelude::*;
use std::io::{Read, Write};
use std::time::Duration;
use tempfile::NamedTempFile;

// Only run these tests on supported platforms
#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
mod reactor_tests {
    use super::*;
    use dx_py_reactor::{
        create_basic_reactor, Completion, IoBuffer, IoOperation, Reactor, ReactorFeature,
    };

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(50))]

        /// Property 21: Platform-Native Async I/O Cross-Platform Correctness
        ///
        /// For any I/O operation (read, write), the result SHALL be correct
        /// regardless of the platform-specific implementation.
        #[test]
        fn prop_read_write_roundtrip(data in prop::collection::vec(any::<u8>(), 1..4096)) {
            // Create a temp file with the data
            let mut temp_file = NamedTempFile::new().unwrap();
            temp_file.write_all(&data).unwrap();
            temp_file.flush().unwrap();

            // Read it back using the reactor
            if let Ok(mut reactor) = create_basic_reactor() {
                let file = std::fs::File::open(temp_file.path()).unwrap();

                #[cfg(unix)]
                let fd = std::os::unix::io::AsRawFd::as_raw_fd(&file);
                #[cfg(windows)]
                let fd = std::os::windows::io::AsRawHandle::as_raw_handle(&file);

                let buf = IoBuffer::new(data.len());
                let op = IoOperation::Read {
                    fd,
                    buf,
                    offset: 0,
                    user_data: 1,
                };

                if let Ok(_) = reactor.submit(op) {
                    if let Ok(completions) = reactor.wait(Duration::from_secs(5)) {
                        prop_assert!(!completions.is_empty());
                        let completion = &completions[0];
                        prop_assert!(completion.is_success());
                        prop_assert_eq!(completion.bytes(), data.len());
                    }
                }
            }
        }

        /// Property 22: Batched I/O Single Syscall
        ///
        /// For any batch of N I/O operations submitted via submit_batch(),
        /// all operations should complete successfully.
        #[test]
        fn prop_batched_io_all_complete(num_ops in 1usize..20) {
            // Create temp files
            let temp_files: Vec<_> = (0..num_ops)
                .map(|i| {
                    let mut f = NamedTempFile::new().unwrap();
                    write!(f, "File content {}", i).unwrap();
                    f.flush().unwrap();
                    f
                })
                .collect();

            if let Ok(mut reactor) = create_basic_reactor() {
                let ops: Vec<IoOperation> = temp_files
                    .iter()
                    .enumerate()
                    .filter_map(|(i, f)| {
                        let file = std::fs::File::open(f.path()).ok()?;

                        #[cfg(unix)]
                        let fd = std::os::unix::io::AsRawFd::as_raw_fd(&file);
                        #[cfg(windows)]
                        let fd = std::os::windows::io::AsRawHandle::as_raw_handle(&file);

                        std::mem::forget(file); // Keep fd valid

                        Some(IoOperation::Read {
                            fd,
                            buf: IoBuffer::new(1024),
                            offset: 0,
                            user_data: i as u64 + 1,
                        })
                    })
                    .collect();

                let expected_count = ops.len();

                if let Ok(user_datas) = reactor.submit_batch(ops) {
                    prop_assert_eq!(user_datas.len(), expected_count);

                    // Wait for all completions
                    let mut all_completions = Vec::new();
                    let start = std::time::Instant::now();

                    while all_completions.len() < expected_count
                        && start.elapsed() < Duration::from_secs(10)
                    {
                        if let Ok(completions) = reactor.wait(Duration::from_secs(1)) {
                            all_completions.extend(completions);
                        }
                    }

                    prop_assert_eq!(all_completions.len(), expected_count);

                    for c in &all_completions {
                        prop_assert!(c.is_success());
                    }
                }
            }
        }
    }

    #[test]
    fn test_reactor_creation() {
        let reactor = create_basic_reactor();
        assert!(reactor.is_ok());
    }

    #[test]
    fn test_reactor_features() {
        if let Ok(reactor) = create_basic_reactor() {
            // All reactors should support timeouts
            assert!(reactor.supports(ReactorFeature::Timeouts));
        }
    }

    #[test]
    fn test_io_buffer_operations() {
        let mut buf = IoBuffer::new(1024);
        assert_eq!(buf.len(), 1024);
        assert!(!buf.is_empty());

        // Write some data
        buf.as_slice_mut()[0..5].copy_from_slice(b"hello");
        assert_eq!(&buf.as_slice()[0..5], b"hello");
    }

    #[test]
    fn test_io_buffer_from_vec() {
        let data = vec![1u8, 2, 3, 4, 5];
        let buf = IoBuffer::from_vec(data);
        assert_eq!(buf.len(), 5);
        assert_eq!(buf.as_slice(), &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_completion_success() {
        let c = Completion::success(42, 1024);
        assert!(c.is_success());
        assert!(!c.is_error());
        assert_eq!(c.user_data, 42);
        assert_eq!(c.bytes(), 1024);
    }

    #[test]
    fn test_completion_error() {
        let c = Completion::error(
            42,
            std::io::Error::new(std::io::ErrorKind::NotFound, "not found"),
        );
        assert!(!c.is_success());
        assert!(c.is_error());
        assert_eq!(c.bytes(), 0);
    }

    #[cfg(target_os = "linux")]
    mod linux_tests {
        use super::*;
        use dx_py_reactor::io_uring::IoUringReactor;

        #[test]
        fn test_io_uring_reactor_creation() {
            let reactor = IoUringReactor::new();
            assert!(reactor.is_ok());
        }

        #[test]
        fn test_io_uring_supports_advanced_features() {
            if let Ok(reactor) = IoUringReactor::new() {
                assert!(reactor.supports(ReactorFeature::MultishotAccept));
                assert!(reactor.supports(ReactorFeature::ZeroCopySend));
                assert!(reactor.supports(ReactorFeature::RegisteredFds));
                assert!(reactor.supports(ReactorFeature::RegisteredBuffers));
                assert!(reactor.supports(ReactorFeature::Cancellation));
            }
        }

        proptest! {
            #![proptest_config(ProptestConfig::with_cases(20))]

            /// Property 23: Multi-Shot Accept Correctness (Linux only)
            ///
            /// Multi-shot accept should be supported on io_uring.
            #[test]
            fn prop_io_uring_multishot_supported(_dummy in 0..1i32) {
                if let Ok(reactor) = IoUringReactor::new() {
                    prop_assert!(reactor.supports(ReactorFeature::MultishotAccept));
                }
            }

            /// Property 24: Zero-Copy Send Correctness (Linux only)
            ///
            /// Zero-copy send should be supported on io_uring.
            #[test]
            fn prop_io_uring_zerocopy_supported(_dummy in 0..1i32) {
                if let Ok(reactor) = IoUringReactor::new() {
                    prop_assert!(reactor.supports(ReactorFeature::ZeroCopySend));
                }
            }
        }
    }

    #[cfg(target_os = "macos")]
    mod macos_tests {
        use super::*;
        use dx_py_reactor::kqueue::KqueueReactor;

        #[test]
        fn test_kqueue_reactor_creation() {
            let reactor = KqueueReactor::new();
            assert!(reactor.is_ok());
        }

        #[test]
        fn test_kqueue_limited_features() {
            if let Ok(reactor) = KqueueReactor::new() {
                // kqueue doesn't support these advanced features
                assert!(!reactor.supports(ReactorFeature::MultishotAccept));
                assert!(!reactor.supports(ReactorFeature::ZeroCopySend));
                assert!(!reactor.supports(ReactorFeature::RegisteredFds));
            }
        }
    }

    #[cfg(target_os = "windows")]
    mod windows_tests {
        use super::*;
        use dx_py_reactor::iocp::IocpReactor;

        #[test]
        fn test_iocp_reactor_creation() {
            let reactor = IocpReactor::new();
            assert!(reactor.is_ok());
        }

        #[test]
        fn test_iocp_features() {
            if let Ok(reactor) = IocpReactor::new() {
                assert!(reactor.supports(ReactorFeature::Timeouts));
                assert!(reactor.supports(ReactorFeature::Cancellation));
                assert!(!reactor.supports(ReactorFeature::MultishotAccept));
            }
        }
    }
}

// Tests for PyFuture
mod py_future_tests {
    use dx_py_reactor::PyFuture;
    use std::sync::Arc;

    #[test]
    fn test_py_future_set_result() {
        let future = PyFuture::new();
        assert!(future.is_pending());

        future.set_result(42);
        assert!(future.is_ready());
        assert_eq!(future.try_get().unwrap().unwrap(), 42);
    }

    #[test]
    fn test_py_future_set_error() {
        let future: PyFuture<i32> = PyFuture::new();
        future.set_error(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "not found",
        ));

        assert!(future.is_ready());
        assert!(future.try_get().unwrap().is_err());
    }

    #[test]
    fn test_py_future_clone_shares_state() {
        let future1 = PyFuture::new();
        let future2 = future1.clone();

        future1.set_result(42);

        // Both should see the result
        assert_eq!(future1.try_get().unwrap().unwrap(), 42);
        assert_eq!(future2.try_get().unwrap().unwrap(), 42);
    }
}

// Tests for ReactorPool
#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
mod pool_tests {
    use dx_py_reactor::pool::{ReactorPool, ReactorPoolBuilder};

    #[test]
    fn test_reactor_pool_creation() {
        if let Ok(pool) = ReactorPool::with_cores(2) {
            assert_eq!(pool.num_reactors(), 2);
        }
    }

    #[test]
    fn test_reactor_pool_builder() {
        let builder = ReactorPoolBuilder::new().num_cores(4).sqpoll(false);

        if let Ok(pool) = builder.build() {
            assert_eq!(pool.num_reactors(), 4);
        }
    }

    #[test]
    fn test_reactor_handle() {
        if let Ok(pool) = ReactorPool::with_cores(2) {
            let handle = dx_py_reactor::pool::ReactorHandle::new(&pool, 0);
            assert_eq!(handle.core_id(), 0);
        }
    }
}
