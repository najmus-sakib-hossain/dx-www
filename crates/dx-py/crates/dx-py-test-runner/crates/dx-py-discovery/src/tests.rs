//! Tests for dx-py-discovery

use super::*;
use proptest::prelude::*;
use std::io::Write;
use tempfile::TempDir;

// Property 1: Test Function Detection
// For any Python source with functions, only test_* or *_test or decorated functions are detected

fn arb_function_name() -> impl Strategy<Value = String> {
    prop_oneof![
        // Test functions (should be detected)
        "test_[a-z_]{1,15}".prop_map(|s| s),
        "[a-z_]{1,15}_test".prop_map(|s| s),
        // Non-test functions (should not be detected)
        "[a-z_]{1,15}".prop_filter("not a test", |s| {
            !s.starts_with("test_") && !s.ends_with("_test")
        }),
    ]
}

fn arb_class_name() -> impl Strategy<Value = String> {
    prop_oneof![
        // Test classes (should be scanned)
        "Test[A-Z][a-zA-Z]{0,15}".prop_map(|s| s),
        // Non-test classes (should not be scanned)
        "[A-Z][a-zA-Z]{0,15}".prop_filter("not a test class", |s| !s.starts_with("Test")),
    ]
}

fn generate_python_function(name: &str, has_pytest_mark: bool) -> String {
    if has_pytest_mark {
        format!(
            "@pytest.mark.unit\ndef {}():\n    pass\n",
            name
        )
    } else {
        format!("def {}():\n    pass\n", name)
    }
}

fn generate_python_class(class_name: &str, methods: &[String]) -> String {
    let mut code = format!("class {}:\n", class_name);
    for method in methods {
        code.push_str(&format!("    def {}(self):\n        pass\n", method));
    }
    code
}

proptest! {
    /// Feature: dx-py-test-runner, Property 1: Test Function Detection
    /// Validates: Requirements 1.2, 1.3, 1.4
    #[test]
    fn prop_test_function_detection(
        func_name in arb_function_name(),
        has_pytest_mark in any::<bool>()
    ) {
        let source = generate_python_function(&func_name, has_pytest_mark);
        let mut scanner = TestScanner::new().unwrap();
        let tests = scanner.scan_source(&source).unwrap();

        let is_test_name = func_name.starts_with("test_") || func_name.ends_with("_test");
        let should_be_detected = is_test_name || has_pytest_mark;

        if should_be_detected {
            prop_assert!(!tests.is_empty(), "Expected test to be detected: {}", func_name);
            prop_assert_eq!(tests[0].name, func_name);
        } else {
            prop_assert!(tests.is_empty(), "Expected no test to be detected: {}", func_name);
        }
    }

    /// Feature: dx-py-test-runner, Property 1: Test Function Detection
    /// Validates: Requirements 1.2, 1.3
    #[test]
    fn prop_test_class_detection(
        class_name in arb_class_name(),
        method_names in prop::collection::vec(arb_function_name(), 1..5)
    ) {
        let source = generate_python_class(&class_name, &method_names);
        let mut scanner = TestScanner::new().unwrap();
        let tests = scanner.scan_source(&source).unwrap();

        let is_test_class = class_name.starts_with("Test");

        if is_test_class {
            // Should find test methods in Test* classes
            let expected_test_count = method_names.iter()
                .filter(|m| m.starts_with("test_") || m.ends_with("_test"))
                .count();
            prop_assert_eq!(tests.len(), expected_test_count,
                "Expected {} tests in class {}, found {}",
                expected_test_count, class_name, tests.len());

            // All detected tests should have the class name
            for test in &tests {
                prop_assert_eq!(test.class_name.as_deref(), Some(class_name.as_str()));
            }
        } else {
            // Non-Test classes should not have their methods scanned
            prop_assert!(tests.is_empty(),
                "Expected no tests from non-Test class {}", class_name);
        }
    }
}

// Property 2: Test Index Round-Trip
// For any set of test cases, writing to index and reading back produces equivalent data

proptest! {
    /// Feature: dx-py-test-runner, Property 2: Test Index Round-Trip
    /// Validates: Requirements 1.5, 1.6
    #[test]
    fn prop_test_index_roundtrip(
        test_names in prop::collection::vec("test_[a-z_]{1,10}", 1..10)
    ) {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test_example.py");
        let index_file = temp_dir.path().join("test.dxti");

        // Create a Python file with tests
        let mut source = String::new();
        for name in &test_names {
            source.push_str(&format!("def {}():\n    pass\n\n", name));
        }
        std::fs::write(&test_file, &source).unwrap();

        // Scan and build index
        let mut scanner = TestScanner::new().unwrap();
        let tests = scanner.scan_file(&test_file).unwrap();

        let mut builder = TestIndexBuilder::new();
        builder.add_file(&test_file, tests.clone()).unwrap();
        let index = builder.build();

        // Save and reload
        index.save(&index_file).unwrap();
        let loaded = TestIndex::load(&index_file).unwrap();

        // Verify round-trip
        let original_tests = index.all_tests();
        let loaded_tests = loaded.all_tests();

        prop_assert_eq!(original_tests.len(), loaded_tests.len());
        for (orig, loaded) in original_tests.iter().zip(loaded_tests.iter()) {
            prop_assert_eq!(orig.name, loaded.name);
            prop_assert_eq!(orig.line_number, loaded.line_number);
            prop_assert_eq!(orig.class_name, loaded.class_name);
        }
    }
}

// Unit tests

#[test]
fn test_scan_simple_test_function() {
    let source = r#"
def test_example():
    assert True
"#;
    let mut scanner = TestScanner::new().unwrap();
    let tests = scanner.scan_source(source).unwrap();

    assert_eq!(tests.len(), 1);
    assert_eq!(tests[0].name, "test_example");
    assert!(tests[0].class_name.is_none());
}

#[test]
fn test_scan_test_class() {
    let source = r#"
class TestExample:
    def test_one(self):
        pass

    def test_two(self):
        pass

    def helper(self):
        pass
"#;
    let mut scanner = TestScanner::new().unwrap();
    let tests = scanner.scan_source(source).unwrap();

    assert_eq!(tests.len(), 2);
    assert_eq!(tests[0].name, "test_one");
    assert_eq!(tests[0].class_name.as_deref(), Some("TestExample"));
    assert_eq!(tests[1].name, "test_two");
    assert_eq!(tests[1].class_name.as_deref(), Some("TestExample"));
}

#[test]
fn test_scan_pytest_mark_decorator() {
    let source = r#"
import pytest

@pytest.mark.slow
def my_slow_function():
    pass
"#;
    let mut scanner = TestScanner::new().unwrap();
    let tests = scanner.scan_source(source).unwrap();

    assert_eq!(tests.len(), 1);
    assert_eq!(tests[0].name, "my_slow_function");
}

#[test]
fn test_scan_fixture() {
    let source = r#"
import pytest

@pytest.fixture
def my_fixture():
    return 42
"#;
    let mut scanner = TestScanner::new().unwrap();
    let tests = scanner.scan_source(source).unwrap();

    // Fixtures should be detected but filtered out from test list
    assert!(tests.is_empty() || tests.iter().all(|t| !t.is_fixture));
}

#[test]
fn test_scan_non_test_class() {
    let source = r#"
class Helper:
    def test_like_method(self):
        pass
"#;
    let mut scanner = TestScanner::new().unwrap();
    let tests = scanner.scan_source(source).unwrap();

    // Non-Test classes should not have their methods scanned
    assert!(tests.is_empty());
}

#[test]
fn test_index_needs_rescan() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test_example.py");
    let index_file = temp_dir.path().join("test.dxti");

    // Create initial file
    std::fs::write(&test_file, "def test_one():\n    pass\n").unwrap();

    // Build and save index
    let mut scanner = TestScanner::new().unwrap();
    let tests = scanner.scan_file(&test_file).unwrap();
    let mut builder = TestIndexBuilder::new();
    builder.add_file(&test_file, tests).unwrap();
    let index = builder.build();
    index.save(&index_file).unwrap();

    // Load index - should not need rescan
    let loaded = TestIndex::load(&index_file).unwrap();
    assert!(!loaded.needs_rescan(&test_file));

    // Modify file
    std::thread::sleep(std::time::Duration::from_millis(100));
    std::fs::write(&test_file, "def test_one():\n    pass\ndef test_two():\n    pass\n").unwrap();

    // Should need rescan now
    assert!(loaded.needs_rescan(&test_file));
}

#[test]
fn test_index_file_count_and_test_count() {
    let temp_dir = TempDir::new().unwrap();

    let mut builder = TestIndexBuilder::new();

    // Create two test files
    for i in 0..2 {
        let test_file = temp_dir.path().join(format!("test_{}.py", i));
        let source = format!("def test_a{}():\n    pass\ndef test_b{}():\n    pass\n", i, i);
        std::fs::write(&test_file, &source).unwrap();

        let mut scanner = TestScanner::new().unwrap();
        let tests = scanner.scan_file(&test_file).unwrap();
        builder.add_file(&test_file, tests).unwrap();
    }

    let index = builder.build();
    assert_eq!(index.file_count(), 2);
    assert_eq!(index.test_count(), 4);
}
