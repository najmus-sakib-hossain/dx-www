//! Tests for dx-py-daemon

use super::*;
use proptest::prelude::*;

// Property 3: Worker Pool Invariant
// available_workers + busy_workers == pool_size (always)

proptest! {
    /// Feature: dx-py-test-runner, Property 3: Worker Pool Invariant
    /// Validates: Requirements 2.3, 2.4, 2.5
    #[test]
    fn prop_worker_pool_invariant(
        pool_size in 1usize..=8usize,
        acquire_count in 0usize..=8usize,
    ) {
        let config = DaemonConfig::default().with_pool_size(pool_size);
        let pool = DaemonPool::new(config).unwrap();

        // Initial state: all workers available
        prop_assert_eq!(pool.available_workers(), pool_size);
        prop_assert_eq!(pool.busy_workers(), 0);
        prop_assert_eq!(pool.available_workers() + pool.busy_workers(), pool_size);

        // Acquire some workers
        let mut acquired = Vec::new();
        for _ in 0..acquire_count.min(pool_size) {
            if let Ok(id) = pool.acquire_worker() {
                acquired.push(id);
            }
        }

        // Invariant must hold
        prop_assert_eq!(
            pool.available_workers() + pool.busy_workers(),
            pool_size,
            "Invariant violated after acquiring {} workers",
            acquired.len()
        );

        // Release workers
        for id in acquired {
            pool.release_worker(id).unwrap();
        }

        // Invariant must still hold
        prop_assert_eq!(
            pool.available_workers() + pool.busy_workers(),
            pool_size,
            "Invariant violated after releasing workers"
        );
        prop_assert_eq!(pool.available_workers(), pool_size);
    }

    /// Feature: dx-py-test-runner, Property 3: Worker Pool Invariant
    /// Validates: Requirements 2.3, 2.4
    #[test]
    fn prop_acquire_release_cycle(
        pool_size in 1usize..=4usize,
        cycles in 1usize..=10usize,
    ) {
        let config = DaemonConfig::default().with_pool_size(pool_size);
        let pool = DaemonPool::new(config).unwrap();

        for _ in 0..cycles {
            // Acquire all workers
            let mut acquired = Vec::new();
            while let Ok(id) = pool.acquire_worker() {
                acquired.push(id);
            }
            prop_assert_eq!(acquired.len(), pool_size);
            prop_assert_eq!(pool.available_workers(), 0);

            // Release all workers
            for id in acquired {
                pool.release_worker(id).unwrap();
            }
            prop_assert_eq!(pool.available_workers(), pool_size);
        }
    }
}

// Unit tests

#[test]
fn test_pool_creation() {
    let config = DaemonConfig::default().with_pool_size(4);
    let pool = DaemonPool::new(config).unwrap();

    assert_eq!(pool.pool_size(), 4);
    assert_eq!(pool.available_workers(), 4);
    assert_eq!(pool.busy_workers(), 0);
}

#[test]
fn test_acquire_and_release() {
    let config = DaemonConfig::default().with_pool_size(2);
    let pool = DaemonPool::new(config).unwrap();

    let worker1 = pool.acquire_worker().unwrap();
    assert_eq!(pool.available_workers(), 1);

    let worker2 = pool.acquire_worker().unwrap();
    assert_eq!(pool.available_workers(), 0);

    // No more workers available
    assert!(pool.acquire_worker().is_err());

    pool.release_worker(worker1).unwrap();
    assert_eq!(pool.available_workers(), 1);

    pool.release_worker(worker2).unwrap();
    assert_eq!(pool.available_workers(), 2);
}

#[test]
fn test_test_queuing() {
    let config = DaemonConfig::default().with_pool_size(2);
    let pool = DaemonPool::new(config).unwrap();

    let test1 = TestCase::new("test_one", "test.py", 1);
    let test2 = TestCase::new("test_two", "test.py", 5);

    pool.queue_test(test1.clone()).unwrap();
    pool.queue_test(test2.clone()).unwrap();

    assert_eq!(pool.queued_tests(), 2);

    let next = pool.next_queued_test().unwrap();
    assert_eq!(next.name, "test_one");

    let next = pool.next_queued_test().unwrap();
    assert_eq!(next.name, "test_two");

    assert!(pool.next_queued_test().is_none());
}

#[test]
fn test_shutdown() {
    let config = DaemonConfig::default().with_pool_size(2);
    let pool = DaemonPool::new(config).unwrap();

    assert!(!pool.is_shutdown());
    pool.shutdown().unwrap();
    assert!(pool.is_shutdown());

    // Cannot acquire after shutdown
    assert!(pool.acquire_worker().is_err());
}

#[test]
fn test_config_builder() {
    let config = DaemonConfig::default()
        .with_pool_size(8)
        .with_python("/usr/bin/python3")
        .with_preload(vec!["django".into(), "numpy".into()])
        .with_timeout(Duration::from_secs(120));

    assert_eq!(config.pool_size, 8);
    assert_eq!(config.python_path, "/usr/bin/python3");
    assert_eq!(config.preload_modules, vec!["django", "numpy"]);
    assert_eq!(config.timeout, Duration::from_secs(120));
}
