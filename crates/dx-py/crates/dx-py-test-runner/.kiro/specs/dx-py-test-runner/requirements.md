# Requirements Document

## Introduction

dx-py-test-runner is a high-performance Python test runner built with Rust, designed to achieve 10x+ speedup over traditional test runners (pytest, unittest). It leverages zero-import AST discovery, persistent daemon mode, binary IPC protocols, and smart change detection to dramatically reduce test execution time.

## Glossary

- **Test_Runner**: The main dx-py-test-runner system that orchestrates test discovery, execution, and reporting
- **Discovery_Engine**: Rust-based component that scans Python files for test functions without importing them
- **Daemon_Pool**: Pool of pre-warmed Python interpreters ready to execute tests
- **Binary_Protocol**: Zero-copy message format for communication between Rust orchestrator and Python workers
- **Dependency_Graph**: Import graph tracking which tests are affected by file changes
- **Work_Stealer**: Dynamic load-balancing executor that distributes tests across workers
- **Fixture_Cache**: Memory-mapped cache for expensive test fixtures
- **Snapshot_Index**: Hash-based index for O(1) snapshot comparisons
- **Test_Index**: Binary index file (.dxti) storing discovered test metadata

## Requirements

### Requirement 1: Rust-Powered Zero-Import Discovery

**User Story:** As a developer, I want test discovery to happen without importing Python files, so that discovery is 100x faster than traditional runners.

#### Acceptance Criteria

1. WHEN the Discovery_Engine scans a Python file, THE Discovery_Engine SHALL parse the AST using tree-sitter without invoking the Python interpreter
2. WHEN a function name starts with "test_" or ends with "_test", THE Discovery_Engine SHALL identify it as a test function
3. WHEN a class name starts with "Test", THE Discovery_Engine SHALL scan its methods for test functions
4. WHEN a function has a @pytest.mark decorator, THE Discovery_Engine SHALL identify it as a test function
5. WHEN discovery completes, THE Discovery_Engine SHALL write results to a binary Test_Index file (.dxti)
6. WHEN a Test_Index file exists and source files are unchanged, THE Discovery_Engine SHALL load from the index instead of re-scanning

### Requirement 2: Persistent Daemon Mode

**User Story:** As a developer, I want warm Python interpreters ready to execute tests, so that I avoid cold-start overhead on every test run.

#### Acceptance Criteria

1. WHEN the Test_Runner starts in daemon mode, THE Daemon_Pool SHALL spawn N pre-warmed Python interpreters
2. WHEN a warm interpreter is created, THE Daemon_Pool SHALL pre-import commonly used modules (django, sqlalchemy, numpy)
3. WHEN a test execution is requested, THE Daemon_Pool SHALL assign it to an available warm interpreter
4. WHEN a test completes, THE Daemon_Pool SHALL return the interpreter to the ready pool
5. IF no interpreters are available, THEN THE Daemon_Pool SHALL queue the test until one becomes available
6. WHEN the daemon receives a shutdown signal, THE Daemon_Pool SHALL gracefully terminate all interpreters

### Requirement 3: Binary Test Protocol

**User Story:** As a developer, I want zero-copy IPC between the Rust orchestrator and Python workers, so that communication overhead is minimized.

#### Acceptance Criteria

1. WHEN sending a test execution request, THE Binary_Protocol SHALL use a fixed-size binary header (32 bytes)
2. WHEN receiving test results, THE Binary_Protocol SHALL parse binary result messages without JSON deserialization
3. WHEN transferring test data, THE Binary_Protocol SHALL use shared memory for payloads larger than 1KB
4. THE Binary_Protocol SHALL serialize test messages using msgpack or bincode format
5. THE Binary_Protocol SHALL deserialize result messages back to structured data
6. WHEN a malformed message is received, THE Binary_Protocol SHALL return a descriptive error

### Requirement 4: Dependency Graph and Smart Change Detection

**User Story:** As a developer, I want to run only tests affected by my changes, so that I get faster feedback during development.

#### Acceptance Criteria

1. WHEN the Test_Runner initializes, THE Dependency_Graph SHALL build an import graph of all Python files
2. WHEN a file changes, THE Dependency_Graph SHALL identify all tests that transitively depend on that file
3. WHEN running in watch mode, THE Test_Runner SHALL only execute affected tests
4. WHEN the import graph is built, THE Dependency_Graph SHALL cache it to disk for subsequent runs
5. WHEN a cached graph exists and no structural changes occurred, THE Dependency_Graph SHALL load from cache
6. WHEN extracting imports, THE Dependency_Graph SHALL use tree-sitter parsing without Python execution

### Requirement 5: Work-Stealing Parallel Executor

**User Story:** As a developer, I want tests distributed dynamically across workers, so that all CPU cores are utilized efficiently.

#### Acceptance Criteria

1. WHEN tests are submitted for execution, THE Work_Stealer SHALL distribute them across available workers
2. WHEN a worker finishes its local queue, THE Work_Stealer SHALL allow it to steal work from other workers
3. WHEN all tests complete, THE Work_Stealer SHALL aggregate results from all workers
4. THE Work_Stealer SHALL maintain near-linear scaling with the number of CPU cores
5. WHEN a worker encounters an error, THE Work_Stealer SHALL continue executing remaining tests on other workers

### Requirement 6: Memory-Mapped Fixture Cache

**User Story:** As a developer, I want expensive fixtures cached and restored instantly, so that test setup time is minimized.

#### Acceptance Criteria

1. WHEN a cacheable fixture is first created, THE Fixture_Cache SHALL serialize its state to disk
2. WHEN a cached fixture is requested, THE Fixture_Cache SHALL memory-map the serialized state
3. WHEN the fixture function changes, THE Fixture_Cache SHALL invalidate and recreate the cache
4. THE Fixture_Cache SHALL use Blake3 hashing to detect fixture function changes
5. WHEN deserializing a fixture, THE Fixture_Cache SHALL use zero-copy deserialization where possible

### Requirement 7: Hash-Based Snapshot Testing

**User Story:** As a developer, I want snapshot comparisons to be O(1) on match, so that snapshot tests don't slow down my test suite.

#### Acceptance Criteria

1. WHEN a snapshot is created, THE Snapshot_Index SHALL store a Blake3 hash of the content
2. WHEN verifying a snapshot, THE Snapshot_Index SHALL compare hashes before comparing content
3. WHEN hashes match, THE Snapshot_Index SHALL return success without loading snapshot content
4. WHEN hashes differ, THE Snapshot_Index SHALL load content and generate a diff
5. WHEN updating a snapshot, THE Snapshot_Index SHALL update both the hash and content

### Requirement 8: CLI Interface

**User Story:** As a developer, I want a simple CLI to run tests, so that I can easily integrate dx-py-test-runner into my workflow.

#### Acceptance Criteria

1. WHEN the user runs "dx-py test", THE Test_Runner SHALL discover and execute all tests
2. WHEN the user runs "dx-py test --watch", THE Test_Runner SHALL enter watch mode with smart change detection
3. WHEN the user runs "dx-py test <pattern>", THE Test_Runner SHALL filter tests matching the pattern
4. WHEN tests complete, THE Test_Runner SHALL display a summary with pass/fail counts and duration
5. WHEN a test fails, THE Test_Runner SHALL display the failure message and traceback
6. WHEN the user runs "dx-py test --update-snapshots", THE Test_Runner SHALL update all snapshot files

### Requirement 9: Python Bindings

**User Story:** As a developer, I want Python APIs to interact with the test runner, so that I can customize test behavior from Python code.

#### Acceptance Criteria

1. THE Test_Runner SHALL expose Python bindings via PyO3
2. WHEN importing dx_py_test_runner, THE Python module SHALL provide access to discovery, execution, and fixture APIs
3. WHEN a Python test uses @dx.fixture decorator, THE Fixture_Cache SHALL manage that fixture
4. WHEN a Python test uses dx.snapshot(), THE Snapshot_Index SHALL handle snapshot comparison

### Requirement 10: Test Result Reporting

**User Story:** As a developer, I want clear test output with timing information, so that I can identify slow tests and failures quickly.

#### Acceptance Criteria

1. WHEN a test passes, THE Test_Runner SHALL display a green checkmark with test name and duration
2. WHEN a test fails, THE Test_Runner SHALL display a red X with test name, duration, and failure details
3. WHEN all tests complete, THE Test_Runner SHALL display total duration and speedup compared to baseline
4. WHEN running in CI mode, THE Test_Runner SHALL output JUnit XML format
5. WHEN a test is skipped, THE Test_Runner SHALL display a yellow indicator with skip reason
