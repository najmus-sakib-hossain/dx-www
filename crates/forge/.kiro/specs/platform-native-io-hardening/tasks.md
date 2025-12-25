# Implementation Plan: Platform-Native I/O and Hardening

## Overview

This implementation plan transforms the design into actionable coding tasks. The implementation follows a bottom-up approach: first building the platform abstraction layer, then integrating it into existing components, and finally adding hardening features. Each task builds on previous work to ensure incremental, testable progress.

## Tasks

- [x] 1. Set up platform I/O module structure and core traits
  - Create `src/platform_io/mod.rs` with module structure
  - Define `PlatformIO` trait with async methods (read, write, batch_read, batch_write, watch)
  - Define `EventStream` trait and `FileEvent` types
  - Define `WriteOp` struct for batch operations
  - Add platform detection types (`Platform`, `IoBackend`, `PlatformInfo`)
  - _Requirements: 1.1, 1.6_

- [x] 2. Implement fallback backend using tokio
  - [x] 2.1 Create `src/platform_io/fallback.rs` with `FallbackBackend` struct
    - Implement `PlatformIO` trait using `tokio::fs` operations
    - Implement batch operations as sequential tokio operations
    - Implement file watching using existing notify integration
    - _Requirements: 1.5, 1.6, 1.7_

  - [x] 2.2 Write property test for fallback backend round-trip
    - **Property 4: Batch Operation Correctness**
    - **Validates: Requirements 1.7**

- [x] 3. Implement io_uring backend for Linux
  - [x] 3.1 Add `io-uring` crate dependency with conditional compilation
    - Update `Cargo.toml` with `[target.'cfg(target_os = "linux")'.dependencies]`
    - _Requirements: 1.2_

  - [x] 3.2 Create `src/platform_io/io_uring.rs` with `IoUringBackend`
    - Implement io_uring ring initialization with SQPOLL
    - Implement async read/write using submission queue entries
    - Implement batch operations using multi-SQE submission
    - Add kernel version detection for availability check
    - _Requirements: 1.2, 1.7, 2.1_

  - [x] 3.3 Write property test for io_uring batch operations
    - **Property 4: Batch Operation Correctness**
    - **Validates: Requirements 1.7**

- [x] 4. Implement kqueue backend for macOS
  - [x] 4.1 Create `src/platform_io/kqueue.rs` with `KqueueBackend`
    - Implement kqueue initialization and event registration
    - Implement file watching using EVFILT_VNODE
    - Implement read/write using standard async I/O with kqueue events
    - _Requirements: 1.3, 2.2_

  - [x] 4.2 Write unit tests for kqueue backend on macOS
    - Test event registration and notification
    - _Requirements: 1.3_

- [x] 5. Implement IOCP backend for Windows
  - [x] 5.1 Create `src/platform_io/iocp.rs` with `IocpBackend`
    - Implement I/O Completion Port creation
    - Implement async read/write using overlapped I/O
    - Implement directory watching using ReadDirectoryChangesW
    - Add worker thread pool for completion processing
    - _Requirements: 1.4, 2.3_

  - [x] 5.2 Write unit tests for IOCP backend on Windows
    - Test completion port operations
    - _Requirements: 1.4_

- [x] 6. Implement platform selector and backend factory
  - [x] 6.1 Create `src/platform_io/selector.rs`
    - Implement `create_platform_io()` factory function
    - Add platform detection logic with fallback chain
    - Add logging for selected backend
    - _Requirements: 1.1, 1.5_

  - [x] 6.2 Write property test for platform detection
    - **Property 1: Platform Detection Correctness**
    - **Validates: Requirements 1.1**

  - [x] 6.3 Write property test for fallback behavior
    - **Property 2: Fallback Behavior Guarantee**
    - **Validates: Requirements 1.5**

- [x] 7. Checkpoint - Ensure platform I/O layer compiles on all platforms
  - Ensure all tests pass, ask the user if questions arise.

- [x] 8. Integrate platform I/O into storage layer
  - [x] 8.1 Update `src/storage/blob.rs` to use `PlatformIO`
    - Replace `tokio::fs` calls with `PlatformIO` trait methods
    - Add memory-mapped I/O for large blobs (>1MB)
    - Add parallel compression for files >100KB
    - _Requirements: 3.1, 3.2, 3.4_

  - [x] 8.2 Add blob integrity verification
    - Verify SHA-256 checksum on blob read
    - Return descriptive error on integrity failure
    - _Requirements: 3.5, 3.6_

  - [x] 8.3 Write property test for blob integrity round-trip
    - **Property 8: Blob Integrity Round-Trip**
    - **Validates: Requirements 3.5**

  - [x] 8.4 Write property test for concurrent storage operations
    - **Property 7: Concurrent Storage Operations**
    - **Validates: Requirements 3.3**

- [x] 9. Integrate platform I/O into watcher
  - [x] 9.1 Update `src/watcher.rs` to use platform-native watching
    - Replace notify-based watching with `PlatformIO::watch()`
    - Maintain fallback to notify when native watching unavailable
    - _Requirements: 2.1, 2.2, 2.3_

  - [x] 9.2 Implement enhanced debouncing and deduplication
    - Add configurable debounce window
    - Implement event deduplication within window
    - Handle directory deletion gracefully
    - _Requirements: 2.5, 2.6, 2.7_

  - [x] 9.3 Write property test for event debouncing
    - **Property 6: Event Debouncing and Deduplication**
    - **Validates: Requirements 2.5, 2.6**

  - [x] 9.4 Write property test for watcher scalability
    - **Property 5: Watcher Scalability**
    - **Validates: Requirements 2.4**

- [x] 10. Checkpoint - Ensure storage and watcher integration works
  - Ensure all tests pass, ask the user if questions arise.

- [x] 11. Implement resource manager
  - [x] 11.1 Create `src/resource_manager.rs`
    - Implement `ResourceManager` with semaphore-based handle limiting
    - Add `HandleGuard` RAII wrapper for automatic release
    - Implement operation queuing when at limit
    - Add shutdown with timeout
    - _Requirements: 6.1, 6.2, 6.3, 6.4_

  - [x] 11.2 Write property test for file handle limiting
    - **Property 12: File Handle Limiting**
    - **Validates: Requirements 6.1**

  - [x] 11.3 Write property test for handle queuing
    - **Property 13: Handle Queuing at Limit**
    - **Validates: Requirements 6.2**

- [-] 12. Enhance error handling
  - [x] 12.1 Update `src/error.rs` with enhanced error types
    - Add `ForgeError` struct with full context
    - Add `ErrorContext` with file, operation, timestamp, platform, backend
    - Implement `is_retryable()` method
    - Add `suggestions()` method for each category
    - _Requirements: 5.1, 5.3, 5.4_

  - [ ] 12.2 Implement exponential backoff retry
    - Update `with_retry()` to use proper exponential backoff
    - Add timing validation between attempts
    - _Requirements: 5.2_

  - [ ] 12.3 Add structured error logging
    - Log all errors with timestamp, category, context
    - Add platform-specific error handlers
    - _Requirements: 5.5_

  - [ ] 12.4 Write property test for error categorization
    - **Property 9: Error Categorization Completeness**
    - **Validates: Requirements 5.1**

  - [ ] 12.5 Write property test for exponential backoff
    - **Property 10: Exponential Backoff Retry**
    - **Validates: Requirements 5.2**

  - [ ] 12.6 Write property test for error context completeness
    - **Property 11: Error Context Completeness**
    - **Validates: Requirements 5.3, 5.4, 5.5**

- [ ] 13. Implement configuration validator
  - [ ] 13.1 Create `src/config/validator.rs`
    - Implement `ConfigValidator` struct
    - Add validation for required fields
    - Add range validation with error messages
    - Add path existence validation
    - Add network address validation
    - _Requirements: 7.1, 7.2, 7.3, 7.4, 7.5_

  - [ ] 13.2 Integrate validator into Forge startup
    - Call validator before initialization
    - Display all validation errors on failure
    - Exit with appropriate code
    - _Requirements: 7.6_

  - [ ] 13.3 Write property test for configuration validation
    - **Property 15: Configuration Validation Completeness**
    - **Validates: Requirements 7.1, 7.2, 7.3, 7.4, 7.5, 7.6**

- [ ] 14. Checkpoint - Ensure resource management and error handling work
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 15. Implement structured logging and metrics
  - [ ] 15.1 Update logging to use structured JSON format
    - Configure tracing-subscriber for JSON output
    - Add timing information for I/O operations in debug mode
    - Add slow operation warnings
    - _Requirements: 8.1, 8.3, 8.5_

  - [ ] 15.2 Create `src/metrics.rs` with metrics collector
    - Implement `MetricsCollector` with atomic counters
    - Add histogram for I/O latency
    - Implement `export_json()` method
    - _Requirements: 8.4_

  - [ ] 15.3 Implement log rotation
    - Configure tracing-appender for file rotation at 100MB
    - _Requirements: 8.6_

  - [ ] 15.4 Add configurable log levels
    - Support trace, debug, info, warn, error levels
    - Filter based on configured level
    - _Requirements: 8.2_

  - [ ] 15.5 Write property test for structured logging
    - **Property 16: Structured Logging Format**
    - **Validates: Requirements 8.1, 8.3**

  - [ ] 15.6 Write property test for log level filtering
    - **Property 17: Log Level Filtering**
    - **Validates: Requirements 8.2**

  - [ ] 15.7 Write property test for metrics availability
    - **Property 18: Metrics Availability**
    - **Validates: Requirements 8.4**

- [ ] 16. Implement graceful shutdown
  - [ ] 16.1 Update daemon shutdown handling
    - Handle SIGTERM/SIGINT signals
    - Complete in-flight write operations
    - Flush pending log entries
    - Save state to disk
    - _Requirements: 9.1, 9.2, 9.3, 9.4_

  - [ ] 16.2 Add shutdown timeout with force termination
    - Force terminate after 30 seconds with warning
    - Return appropriate exit codes
    - _Requirements: 9.5, 9.6_

  - [ ] 16.3 Add panic handler for resource cleanup
    - Install panic hook for emergency cleanup
    - Attempt to release critical resources
    - _Requirements: 6.6_

  - [ ] 16.4 Write property test for graceful shutdown
    - **Property 21: Graceful Shutdown Completeness**
    - **Validates: Requirements 9.2, 9.3, 9.4**

  - [ ] 16.5 Write property test for exit codes
    - **Property 22: Exit Code Correctness**
    - **Validates: Requirements 9.6**

- [ ] 17. Implement thread safety improvements
  - [ ] 17.1 Update storage layer for concurrent access
    - Ensure concurrent reads work correctly
    - Serialize writes to same blob
    - _Requirements: 10.1, 10.2_

  - [ ] 17.2 Make watcher thread-safe
    - Ensure start/stop can be called from any thread
    - Add proper synchronization
    - _Requirements: 10.3_

  - [ ] 17.3 Update database connection pooling
    - Add configurable pool size
    - Ensure pool respects configured limit
    - _Requirements: 10.5_

  - [ ] 17.4 Add watcher handle cleanup verification
    - Ensure all handles released on stop
    - _Requirements: 6.5_

  - [ ] 17.5 Write property test for concurrent reads
    - **Property 23: Concurrent Read Support**
    - **Validates: Requirements 10.1**

  - [ ] 17.6 Write property test for write serialization
    - **Property 24: Write Serialization**
    - **Validates: Requirements 10.2**

  - [ ] 17.7 Write property test for thread-safe watcher
    - **Property 25: Thread-Safe Watcher Operations**
    - **Validates: Requirements 10.3**

  - [ ] 17.8 Write property test for watcher handle cleanup
    - **Property 14: Watcher Handle Cleanup**
    - **Validates: Requirements 6.5**

  - [ ] 17.9 Write property test for connection pool sizing
    - **Property 26: Connection Pool Sizing**
    - **Validates: Requirements 10.5**

- [ ] 18. Checkpoint - Ensure all hardening features work
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 19. Add cross-platform integration tests
  - [ ] 19.1 Create platform-specific test configurations
    - Add CI configuration for Linux, macOS, Windows
    - Configure test matrix for all platforms
    - _Requirements: 4.1, 4.2, 4.3, 4.4_

  - [ ] 19.2 Add stress tests
    - Test 1000+ concurrent file operations
    - Test 10,000+ watched files
    - _Requirements: 4.5, 2.4_

  - [ ] 19.3 Add graceful degradation tests
    - Test fallback when native I/O unavailable
    - _Requirements: 4.6_

- [ ] 20. Final integration and documentation
  - [ ] 20.1 Update public API exports in `src/lib.rs`
    - Export `PlatformIO` trait and types
    - Export `ResourceManager`
    - Export `MetricsCollector`
    - Export `ConfigValidator`

  - [ ] 20.2 Update README with platform-native I/O documentation
    - Document supported platforms and backends
    - Document configuration options
    - Document metrics and observability

  - [ ] 20.3 Add API documentation
    - Document all new public types and functions
    - Add examples for common use cases

- [ ] 21. Final checkpoint - Full test suite passes on all platforms
  - Ensure all tests pass, ask the user if questions arise.

## Notes

- All tasks including property-based tests are required for comprehensive validation
- Each task references specific requirements for traceability
- Checkpoints ensure incremental validation
- Property tests validate universal correctness properties (minimum 100 iterations each)
- Unit tests validate specific examples and edge cases
- Platform-specific code uses conditional compilation (`#[cfg(target_os = "...")]`)
