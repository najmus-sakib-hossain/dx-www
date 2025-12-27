//! Tests for dx-py-executor

use super::*;
use proptest::prelude::*;

// Property 11: Test Distribution Completeness
// All submitted tests must be executed exactly once

proptest! {
    /// Feature: dx-py-test-runner, Property 11: Test Distribution Completeness
    /// Validates: Requirements 5.1
    #[test]
    fn prop_distribution_completeness(
        test_count in 1usize..=50usize,
        worker_count in 1usize..=4usize,
    ) {
        let config = ExecutorConfig::default().with_workers(worker_count);
        let executor = WorkStealingExecutor::new(config);

        // Create test cases
        let tests: Vec<TestCase> = (0..test_count)
            .map(|i| TestCase::new(format!("test_{}", i), "test.py", i as u32 + 1))
            .collect();

        let test_ids: std::collections::HashSet<TestId> = tests.iter()
            .map(|t| t.id)
            .collect();

        // Submit all tests
        executor.submit_all(tests).unwrap();

        // Execute
        let results = executor.execute();

        // Verify all tests were executed exactly once
        prop_assert_eq!(
            results.len(),
            test_count,
            "Expected {} results, got {}",
            test_count,
            results.len()
        );

        let result_ids: std::collections::HashSet<TestId> = results.iter()
            .map(|r| r.test_id)
            .collect();

        prop_assert_eq!(
            result_ids.len(),
            test_count,
            "Some tests were executed multiple times"
        );

        for id in &test_ids {
            prop_assert!(
                result_ids.contains(id),
                "Test {:?} was not executed",
                id
            );
        }
    }

    /// Feature: dx-py-test-runner, Property 12: Result Aggregation Completeness
    /// Validates: Requirements 5.3
    #[test]
    fn prop_result_aggregation(
        test_count in 1usize..=20usize,
    ) {
        let config = ExecutorConfig::default().with_workers(2);
        let executor = WorkStealingExecutor::new(config);

        let tests: Vec<TestCase> = (0..test_count)
            .map(|i| TestCase::new(format!("test_{}", i), "test.py", i as u32 + 1))
            .collect();

        executor.submit_all(tests).unwrap();
        let results = executor.execute();

        // All results should be retrievable
        for result in &results {
            let retrieved = executor.get_result(result.test_id);
            prop_assert!(retrieved.is_some());
            prop_assert_eq!(retrieved.unwrap().test_id, result.test_id);
        }

        // Completed count should match
        prop_assert_eq!(executor.completed(), test_count);
    }

    /// Feature: dx-py-test-runner, Property 13: Executor Fault Tolerance
    /// Validates: Requirements 5.5
    #[test]
    fn prop_fault_tolerance(
        test_count in 5usize..=20usize,
    ) {
        let config = ExecutorConfig::default()
            .with_workers(2)
            .with_fault_tolerance(true);
        let executor = WorkStealingExecutor::new(config);

        let tests: Vec<TestCase> = (0..test_count)
            .map(|i| TestCase::new(format!("test_{}", i), "test.py", i as u32 + 1))
            .collect();

        executor.submit_all(tests).unwrap();
        let results = executor.execute();

        // Even with fault tolerance enabled, normal execution should complete all tests
        prop_assert_eq!(results.len(), test_count);
    }
}

// Unit tests

#[test]
fn test_executor_creation() {
    let config = ExecutorConfig::default().with_workers(4);
    let executor = WorkStealingExecutor::new(config);

    assert_eq!(executor.pending(), 0);
    assert_eq!(executor.completed(), 0);
}

#[test]
fn test_submit_single() {
    let executor = WorkStealingExecutor::new(ExecutorConfig::default());
    let test = TestCase::new("test_one", "test.py", 1);

    executor.submit(test).unwrap();
    assert_eq!(executor.pending(), 1);
}

#[test]
fn test_submit_multiple() {
    let executor = WorkStealingExecutor::new(ExecutorConfig::default());
    let tests = vec![
        TestCase::new("test_one", "test.py", 1),
        TestCase::new("test_two", "test.py", 5),
        TestCase::new("test_three", "test.py", 10),
    ];

    executor.submit_all(tests).unwrap();
    assert_eq!(executor.pending(), 3);
}

#[test]
fn test_execute_single() {
    let config = ExecutorConfig::default().with_workers(1);
    let executor = WorkStealingExecutor::new(config);
    
    let test = TestCase::new("test_example", "test.py", 1);
    let test_id = test.id;

    executor.submit(test).unwrap();
    let results = executor.execute();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].test_id, test_id);
}

#[test]
fn test_execute_multiple_workers() {
    let config = ExecutorConfig::default().with_workers(4);
    let executor = WorkStealingExecutor::new(config);

    let tests: Vec<TestCase> = (0..10)
        .map(|i| TestCase::new(format!("test_{}", i), "test.py", i + 1))
        .collect();

    executor.submit_all(tests).unwrap();
    let results = executor.execute();

    assert_eq!(results.len(), 10);
    assert_eq!(executor.completed(), 10);
    assert_eq!(executor.pending(), 0);
}

#[test]
fn test_execution_summary() {
    let results = vec![
        TestResult::pass(TestId(1), Duration::from_millis(10)),
        TestResult::pass(TestId(2), Duration::from_millis(20)),
        TestResult::fail(TestId(3), Duration::from_millis(15), "assertion failed"),
        TestResult::skip(TestId(4), "not implemented"),
    ];

    let summary = ExecutionSummary::from_results(&results, 0);

    assert_eq!(summary.total, 4);
    assert_eq!(summary.passed, 2);
    assert_eq!(summary.failed, 1);
    assert_eq!(summary.skipped, 1);
    assert!(!summary.is_success());
}

#[test]
fn test_execution_summary_success() {
    let results = vec![
        TestResult::pass(TestId(1), Duration::from_millis(10)),
        TestResult::pass(TestId(2), Duration::from_millis(20)),
    ];

    let summary = ExecutionSummary::from_results(&results, 0);

    assert!(summary.is_success());
}

#[test]
fn test_shutdown() {
    let executor = WorkStealingExecutor::new(ExecutorConfig::default());
    
    executor.shutdown();
    
    let test = TestCase::new("test_after_shutdown", "test.py", 1);
    let result = executor.submit(test);
    
    assert!(result.is_err());
}

#[test]
fn test_config_builder() {
    let config = ExecutorConfig::default()
        .with_workers(8)
        .with_fault_tolerance(false);

    assert_eq!(config.num_workers, 8);
    assert!(!config.fault_tolerant);
}

#[test]
fn test_get_result() {
    let config = ExecutorConfig::default().with_workers(1);
    let executor = WorkStealingExecutor::new(config);
    
    let test = TestCase::new("test_get", "test.py", 1);
    let test_id = test.id;

    executor.submit(test).unwrap();
    executor.execute();

    let result = executor.get_result(test_id);
    assert!(result.is_some());
    assert_eq!(result.unwrap().test_id, test_id);

    // Non-existent test
    let missing = executor.get_result(TestId(99999));
    assert!(missing.is_none());
}
