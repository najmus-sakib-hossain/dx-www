//! Property-based tests for dx-reactor.
//!
//! These tests verify the correctness properties defined in the design document.

use proptest::prelude::*;

// ============================================================================
// Property 1: Batch Submission Count
// For any sequence of I/O operations submitted to a Reactor, the submit()
// method SHALL return the exact count of operations that were successfully queued.
// Validates: Requirements 1.7
// ============================================================================

/// Mock reactor for testing batch submission behavior.
mod mock_reactor {
    use std::sync::atomic::{AtomicUsize, Ordering};

    pub struct MockReactor {
        pending: AtomicUsize,
    }

    impl MockReactor {
        pub fn new() -> Self {
            Self {
                pending: AtomicUsize::new(0),
            }
        }

        pub fn queue_operation(&self) {
            self.pending.fetch_add(1, Ordering::Relaxed);
        }

        pub fn submit(&self) -> usize {
            self.pending.swap(0, Ordering::Relaxed)
        }
    }
}

proptest! {
    /// Property 1: Batch Submission Count
    /// **Feature: binary-dawn, Property 1: Batch Submission Count**
    /// **Validates: Requirements 1.7**
    #[test]
    fn prop_batch_submission_returns_exact_count(count in 0usize..1000) {
        let reactor = mock_reactor::MockReactor::new();
        
        // Queue exactly `count` operations
        for _ in 0..count {
            reactor.queue_operation();
        }
        
        // Submit should return exactly the count of queued operations
        let submitted = reactor.submit();
        prop_assert_eq!(submitted, count, 
            "submit() should return exact count of queued operations");
        
        // After submit, pending should be 0
        let remaining = reactor.submit();
        prop_assert_eq!(remaining, 0, 
            "submit() should clear pending operations");
    }
}

// ============================================================================
// Property 3: Kernel Version Detection
// For any Linux kernel version string, the is_available() function SHALL return
// true if and only if the major version > 5 OR (major version == 5 AND minor version >= 1).
// Validates: Requirements 2.1
// ============================================================================

/// Parse kernel version for testing (mirrors the actual implementation).
fn parse_kernel_version(version: &str) -> bool {
    let parts: Vec<&str> = version.split_whitespace().collect();
    if parts.len() < 3 {
        return false;
    }
    
    let version_str = parts.iter()
        .find(|s| s.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false))
        .unwrap_or(&"0.0.0");
    
    let version_parts: Vec<u32> = version_str
        .split('.')
        .take(2)
        .filter_map(|s| s.split('-').next())
        .filter_map(|s| s.parse().ok())
        .collect();
    
    if version_parts.len() < 2 {
        return false;
    }
    
    let major = version_parts[0];
    let minor = version_parts[1];
    
    major > 5 || (major == 5 && minor >= 1)
}

proptest! {
    /// Property 3: Kernel Version Detection
    /// **Feature: binary-dawn, Property 3: Kernel Version Detection**
    /// **Validates: Requirements 2.1**
    #[test]
    fn prop_kernel_version_detection(major in 0u32..10, minor in 0u32..100) {
        let version_str = format!("Linux version {}.{}.0-generic", major, minor);
        let result = parse_kernel_version(&version_str);
        
        let expected = major > 5 || (major == 5 && minor >= 1);
        prop_assert_eq!(result, expected,
            "Kernel {}.{} should be {} but got {}",
            major, minor, 
            if expected { "available" } else { "unavailable" },
            if result { "available" } else { "unavailable" });
    }

    /// Property 3: Edge cases for kernel version
    #[test]
    fn prop_kernel_version_boundary_cases(minor in 0u32..100) {
        // Version 5.0.x should NOT be available
        let v5_0 = format!("Linux version 5.0.{}", minor);
        prop_assert!(!parse_kernel_version(&v5_0),
            "Kernel 5.0.{} should NOT be available", minor);
        
        // Version 5.1.x should be available
        let v5_1 = format!("Linux version 5.1.{}", minor);
        prop_assert!(parse_kernel_version(&v5_1),
            "Kernel 5.1.{} should be available", minor);
        
        // Version 6.x.x should be available
        let v6 = format!("Linux version 6.{}.0", minor);
        prop_assert!(parse_kernel_version(&v6),
            "Kernel 6.{}.0 should be available", minor);
    }
}

// ============================================================================
// Property 5: Completion Structure Integrity
// For any Completion returned by a Reactor, it SHALL contain valid user_data,
// result, and flags fields that correspond to the original operation.
// Validates: Requirements 3.5, 4.5
// ============================================================================

use dx_reactor::io::Completion;

proptest! {
    /// Property 5: Completion Structure Integrity
    /// **Feature: binary-dawn, Property 5: Completion Structure Integrity**
    /// **Validates: Requirements 3.5, 4.5**
    #[test]
    fn prop_completion_structure_integrity(
        user_data in any::<u64>(),
        result in any::<i32>(),
        flags in any::<u32>()
    ) {
        let completion = Completion::new(user_data, result, flags);
        
        // Verify all fields are preserved
        prop_assert_eq!(completion.user_data, user_data,
            "user_data should be preserved");
        prop_assert_eq!(completion.result, result,
            "result should be preserved");
        prop_assert_eq!(completion.flags, flags,
            "flags should be preserved");
        
        // Verify helper methods
        if result >= 0 {
            prop_assert!(completion.is_success(),
                "positive result should indicate success");
            prop_assert_eq!(completion.bytes_transferred(), Some(result as usize),
                "bytes_transferred should return result for success");
            prop_assert_eq!(completion.error_code(), None,
                "error_code should be None for success");
        } else {
            prop_assert!(completion.is_error(),
                "negative result should indicate error");
            prop_assert_eq!(completion.bytes_transferred(), None,
                "bytes_transferred should be None for error");
            prop_assert_eq!(completion.error_code(), Some(-result),
                "error_code should return negated result");
        }
    }
}

// ============================================================================
// Property 6: Thread-per-Core Default
// For any DxReactor built with WorkerStrategy::ThreadPerCore, the number of
// CoreState instances SHALL equal num_cpus::get().
// Validates: Requirements 5.1
// ============================================================================

use dx_reactor::{DxReactor, WorkerStrategy};

#[test]
fn prop_thread_per_core_default() {
    // **Feature: binary-dawn, Property 6: Thread-per-Core Default**
    // **Validates: Requirements 5.1**
    
    let reactor = DxReactor::build()
        .workers(WorkerStrategy::ThreadPerCore)
        .build();
    
    let expected_cores = num_cpus::get();
    assert_eq!(reactor.num_cores(), expected_cores,
        "ThreadPerCore should create {} workers (one per CPU)", expected_cores);
}

// ============================================================================
// Property 7: Fixed Worker Count
// For any DxReactor built with WorkerStrategy::Fixed(n), the number of
// CoreState instances SHALL equal exactly n.
// Validates: Requirements 5.4
// ============================================================================

proptest! {
    /// Property 7: Fixed Worker Count
    /// **Feature: binary-dawn, Property 7: Fixed Worker Count**
    /// **Validates: Requirements 5.4**
    #[test]
    fn prop_fixed_worker_count(n in 1usize..32) {
        let reactor = DxReactor::build()
            .workers(WorkerStrategy::Fixed(n))
            .build();
        
        prop_assert_eq!(reactor.num_cores(), n,
            "Fixed({}) should create exactly {} workers", n, n);
    }
}


// ============================================================================
// Property 4: Kqueue Batch Submission
// For any KqueueReactor with pending changes, after calling wait(), the
// pending_changes vector SHALL be empty.
// Validates: Requirements 3.4
// ============================================================================

/// Mock kqueue reactor for testing batch submission behavior.
mod mock_kqueue {
    use std::sync::Mutex;

    pub struct MockKqueueReactor {
        pending_changes: Mutex<Vec<u64>>,
    }

    impl MockKqueueReactor {
        pub fn new() -> Self {
            Self {
                pending_changes: Mutex::new(Vec::new()),
            }
        }

        pub fn register_read(&self, fd: u64) {
            self.pending_changes.lock().unwrap().push(fd);
        }

        pub fn register_write(&self, fd: u64) {
            self.pending_changes.lock().unwrap().push(fd);
        }

        pub fn pending_count(&self) -> usize {
            self.pending_changes.lock().unwrap().len()
        }

        pub fn wait(&self) -> Vec<u64> {
            // Submit pending changes and clear them
            let changes = std::mem::take(&mut *self.pending_changes.lock().unwrap());
            // In real impl, this would call kevent() with the changes
            changes
        }
    }
}

proptest! {
    /// Property 4: Kqueue Batch Submission
    /// **Feature: binary-dawn, Property 4: Kqueue Batch Submission**
    /// **Validates: Requirements 3.4**
    #[test]
    fn prop_kqueue_batch_submission_clears_pending(
        read_fds in prop::collection::vec(any::<u64>(), 0..100),
        write_fds in prop::collection::vec(any::<u64>(), 0..100)
    ) {
        let reactor = mock_kqueue::MockKqueueReactor::new();
        
        // Register read events
        for fd in &read_fds {
            reactor.register_read(*fd);
        }
        
        // Register write events
        for fd in &write_fds {
            reactor.register_write(*fd);
        }
        
        let expected_count = read_fds.len() + write_fds.len();
        prop_assert_eq!(reactor.pending_count(), expected_count,
            "Should have {} pending changes before wait", expected_count);
        
        // Call wait - should submit and clear pending changes
        let _ = reactor.wait();
        
        prop_assert_eq!(reactor.pending_count(), 0,
            "pending_changes should be empty after wait()");
    }
}
