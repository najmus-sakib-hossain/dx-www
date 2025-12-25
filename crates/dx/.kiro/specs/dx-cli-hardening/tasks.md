# Implementation Plan: DX CLI Hardening

## Overview

This implementation plan transforms the DX CLI from a functional prototype into a battle-tested, production-ready tool. Tasks are organized to build hardening components incrementally, with each component enhancing existing functionality. The implementation uses Rust and the existing project structure.

## Tasks

- [x] 1. Implement Enhanced Error Handler with Retry Logic
  - [x] 1.1 Create EnhancedError struct and ErrorContext
    - Add retry_count, max_retries, and context fields
    - Implement display_message() with full context
    - Implement should_retry() and next_retry_delay()
    - _Requirements: 1.1, 1.2, 1.7_

  - [x] 1.2 Write property test for retry exponential backoff
    - **Property 1: Retry with Exponential Backoff**
    - **Validates: Requirements 1.1, 3.1**

  - [x] 1.3 Implement with_retry async wrapper function
    - Accept operation name, max_retries, and async closure
    - Implement exponential backoff (1s, 2s, 4s)
    - Accumulate retry information in EnhancedError
    - _Requirements: 1.1, 3.1_

  - [x] 1.4 Write property test for error retryability classification
    - **Property 2: Error Retryability Classification**
    - **Validates: Requirements 1.7**

  - [x] 1.5 Enhance DxError with comprehensive hints
    - Add hints for PermissionDenied, TLS, DNS errors
    - Ensure all hinted errors return non-empty strings
    - _Requirements: 1.3, 3.4, 3.6, 11.5_

  - [x] 1.6 Write property test for error hints completeness
    - **Property 3: Error Hints Completeness**
    - **Validates: Requirements 1.3, 3.4, 3.6, 11.5**

- [x] 2. Checkpoint - Ensure error handling compiles and tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [x] 3. Implement Input Validator
  - [x] 3.1 Create InputValidator struct and ValidationError
    - Define ValidationError with field, value, expected, suggestion
    - Implement Display trait for user-friendly messages
    - _Requirements: 8.1_

  - [x] 3.2 Implement validate_port function
    - Validate port is in range 1-65535
    - Return ValidationError with expected range
    - _Requirements: 8.4_

  - [x] 3.3 Write property test for port validation
    - **Property 25: Port Validation Range**
    - **Validates: Requirements 8.4**

  - [x] 3.4 Implement validate_version function
    - Parse and validate semver format (X.Y.Z)
    - Return ValidationError with expected format
    - _Requirements: 8.5_

  - [x] 3.5 Write property test for version validation
    - **Property 26: Version Validation Format**
    - **Validates: Requirements 8.5**

  - [x] 3.6 Implement sanitize_for_shell function
    - Define SHELL_METACHARACTERS constant
    - Escape all metacharacters with backslash
    - _Requirements: 8.3_

  - [x] 3.7 Write property test for shell metacharacter escaping
    - **Property 27: Shell Metacharacter Escaping**
    - **Validates: Requirements 8.3**

  - [x] 3.8 Implement check_path_traversal function
    - Resolve path and check if within project root
    - Handle symlinks that escape project directory
    - Return SecurityWarning for violations
    - _Requirements: 8.2_

  - [x] 3.9 Write property test for path traversal detection
    - **Property 28: Path Traversal Detection**
    - **Validates: Requirements 8.2**

- [x] 4. Checkpoint - Ensure input validation works
  - Ensure all tests pass, ask the user if questions arise.

- [x] 5. Enhance Path Resolver
  - [x] 5.1 Add Unicode path support
    - Ensure resolve_path handles emoji, CJK, RTL scripts
    - Test with various Unicode characters
    - _Requirements: 2.5_

  - [x] 5.2 Write property test for Unicode path handling
    - **Property 9: Unicode Path Handling**
    - **Validates: Requirements 2.5**

  - [x] 5.3 Write property test for path separator normalization
    - **Property 5: Path Separator Normalization**
    - **Validates: Requirements 2.1**

  - [x] 5.4 Write property test for home directory expansion
    - **Property 6: Home Directory Expansion**
    - **Validates: Requirements 2.2**

  - [x] 5.5 Write property test for long path prefix (Windows)
    - **Property 7: Long Path Prefix on Windows**
    - **Validates: Requirements 2.3**

  - [x] 5.6 Write property test for symlink resolution depth
    - **Property 8: Symlink Resolution Depth Limit**
    - **Validates: Requirements 2.4**

  - [x] 5.7 Implement is_within_project function
    - Resolve both paths including symlinks
    - Check if resolved path starts with project root
    - _Requirements: 2.7_

  - [x] 5.8 Implement escape_for_shell for paths
    - Handle spaces and special characters in paths
    - _Requirements: 2.6_

  - [x] 5.9 Implement fallback_dir for non-writable home
    - Return current_dir/.dx as fallback
    - _Requirements: 11.3_

- [x] 6. Implement File Lock Manager
  - [x] 6.1 Create FileLock struct and LockType enum
    - Define Shared and Exclusive lock types
    - Store path and lock file handle
    - _Requirements: 12.1, 12.2_

  - [x] 6.2 Implement acquire with timeout
    - Use platform-specific locking (flock on Unix, LockFile on Windows)
    - Implement blocking wait with timeout
    - _Requirements: 12.7_

  - [x] 6.3 Implement try_acquire non-blocking
    - Return immediately with Some(lock) or None
    - _Requirements: 12.7_

  - [x] 6.4 Write property test for blocking vs non-blocking
    - **Property 37: File Lock Blocking vs Non-Blocking**
    - **Validates: Requirements 12.7**

  - [x] 6.5 Implement Drop for automatic release
    - Ensure lock is released even on panic
    - _Requirements: 12.1_

- [x] 7. Checkpoint - Ensure file locking works
  - Ensure all tests pass, ask the user if questions arise.

- [x] 8. Implement Resilient Network Client
  - [x] 8.1 Create NetworkClient and ProxyConfig structs
    - Implement ProxyConfig::from_env()
    - Parse HTTP_PROXY, HTTPS_PROXY, NO_PROXY
    - _Requirements: 3.5_

  - [x] 8.2 Write property test for proxy configuration
    - **Property 10: Proxy Configuration from Environment**
    - **Validates: Requirements 3.5**

  - [x] 8.3 Implement get with retry
    - Use with_retry wrapper for automatic retries
    - Respect proxy configuration
    - _Requirements: 3.1_

  - [x] 8.4 Implement download_resumable
    - Use HTTP Range headers for files >1MB
    - Track download progress
    - _Requirements: 3.3_

  - [x] 8.5 Implement is_offline detection
    - Check network availability
    - _Requirements: 3.7, 11.4_

- [x] 9. Implement Resource Manager
  - [x] 9.1 Create ResourceManager struct
    - Track temp files and child processes
    - Implement process semaphore for limiting
    - _Requirements: 9.1, 9.7_

  - [x] 9.2 Implement register_temp_file and create_temp_file
    - Track all temp files for cleanup
    - _Requirements: 9.4, 9.7_

  - [x] 9.3 Implement spawn_limited
    - Acquire semaphore before spawning
    - Track child process for cleanup
    - _Requirements: 9.1_

  - [x] 9.4 Write property test for process limit enforcement
    - **Property 29: Process Limit Enforcement**
    - **Validates: Requirements 9.1**

  - [x] 9.5 Implement cleanup and Drop
    - Remove all temp files
    - Terminate all child processes
    - _Requirements: 9.4, 9.5_

  - [x] 9.6 Implement check_disk_space
    - Check available space on path's filesystem
    - Warn if below 100MB
    - _Requirements: 9.6_

  - [x] 9.7 Implement terminate_children with graceful shutdown
    - Send SIGTERM, wait timeout, then SIGKILL
    - _Requirements: 9.5_

- [x] 10. Checkpoint - Ensure resource management works
  - Ensure all tests pass, ask the user if questions arise.

- [x] 11. Implement Crash Reporter
  - [x] 11.1 Create CrashReport struct
    - Include all required fields (id, timestamp, version, os, arch, etc.)
    - Implement Serialize for JSON output
    - _Requirements: 10.5_

  - [x] 11.2 Implement CrashReporter::install
    - Set custom panic hook
    - Integrate with ResourceManager for cleanup
    - _Requirements: 1.5_

  - [x] 11.3 Implement generate_report
    - Capture panic message and location
    - Capture backtrace
    - Capture system info and recent commands
    - _Requirements: 1.5, 10.5_

  - [x] 11.4 Implement save_report
    - Save to ~/.dx/crash-reports/
    - Use timestamp in filename
    - _Requirements: 1.5_

  - [x] 11.5 Implement display_crash_message
    - Show user-friendly message with report location
    - _Requirements: 1.5_

- [x] 12. Enhance Config Loader
  - [x] 12.1 Add field validation
    - Validate types and ranges for all fields
    - Return ConfigInvalid with field name
    - _Requirements: 4.1_

  - [x] 12.2 Write property test for config field validation
    - **Property 11: Config Field Validation**
    - **Validates: Requirements 4.1**

  - [x] 12.3 Write property test for config error location
    - **Property 4: Config Error Location Reporting**
    - **Validates: Requirements 1.4**

  - [x] 12.4 Implement unknown field detection
    - Parse TOML and check for unknown keys
    - Log warnings but continue loading
    - _Requirements: 4.3_

  - [x] 12.5 Write property test for unknown fields
    - **Property 12: Unknown Config Fields Warning**
    - **Validates: Requirements 4.3**

  - [x] 12.6 Implement config merging
    - Load global config from ~/.dx/config.toml
    - Merge with local config (local overrides)
    - _Requirements: 4.5_

  - [x] 12.7 Write property test for config merge precedence
    - **Property 13: Config Merge Precedence**
    - **Validates: Requirements 4.5**

  - [x] 12.8 Implement atomic save with backup
    - Write to temp file, then atomic rename
    - Create .bak backup before overwriting
    - _Requirements: 4.6, 4.7_

  - [x] 12.9 Write property test for config backup
    - **Property 14: Config Backup on Save**
    - **Validates: Requirements 4.7**

  - [x] 12.10 Enhance cache invalidation
    - Check source mtime vs cache mtime
    - Reload from source if stale
    - _Requirements: 4.4, 12.5_

  - [x] 12.11 Write property test for cache invalidation
    - **Property 38: Cache Invalidation on Source Change**
    - **Validates: Requirements 12.5**

- [x] 13. Checkpoint - Ensure config loader enhancements work
  - Ensure all tests pass, ask the user if questions arise.

- [x] 14. Enhance Update System
  - [x] 14.1 Add signature verification
    - Verify Ed25519 signature before applying
    - Return SignatureInvalid on failure
    - _Requirements: 5.1, 5.2_

  - [x] 14.2 Write property test for signature verification
    - **Property 15: Signature Verification Gates Updates**
    - **Validates: Requirements 5.1, 5.2**

  - [x] 14.3 Write property test for delta preference
    - **Property 16: Delta Patch Preference**
    - **Validates: Requirements 5.5**

  - [x] 14.4 Write property test for version display
    - **Property 17: Update Version Display**
    - **Validates: Requirements 5.8**

  - [x] 14.5 Implement backup before update
    - Create backup of current binary
    - _Requirements: 5.3_

  - [x] 14.6 Implement restore on failure
    - Restore from backup if update fails
    - _Requirements: 5.4_

  - [x] 14.7 Implement atomic binary replacement
    - Write to temp, then atomic rename
    - _Requirements: 5.7, 12.4_

- [x] 15. Enhance Shell Integration
  - [x] 15.1 Write property test for duplicate detection
    - **Property 18: Shell Integration Duplicate Detection**
    - **Validates: Requirements 6.3**

  - [x] 15.2 Write property test for idempotence
    - **Property 19: Shell Integration Idempotence**
    - **Validates: Requirements 6.7**

  - [x] 15.3 Write property test for completion validity
    - **Property 20: Completion Script Validity**
    - **Validates: Requirements 6.6**

  - [x] 15.4 Implement config file creation with permissions
    - Create parent directories if needed
    - Set 0644 permissions on Unix
    - _Requirements: 6.2_

  - [x] 15.5 Implement clean uninstall
    - Remove only DX-related content
    - Preserve other configuration
    - _Requirements: 6.4_

- [x] 16. Enhance History Manager
  - [x] 16.1 Implement atomic history saves
    - Write to temp file, then rename
    - _Requirements: 7.1_

  - [x] 16.2 Implement corruption recovery
    - Detect unparseable history
    - Backup corrupted file and start fresh
    - _Requirements: 7.2_

  - [x] 16.3 Write property test for case-insensitive search
    - **Property 21: History Search Case Insensitivity**
    - **Validates: Requirements 7.3**

  - [x] 16.4 Write property test for FIFO eviction
    - **Property 22: History FIFO Eviction**
    - **Validates: Requirements 7.4**

  - [x] 16.5 Write property test for statistics accuracy
    - **Property 23: History Statistics Accuracy**
    - **Validates: Requirements 7.6**

  - [x] 16.6 Write property test for entry completeness
    - **Property 24: History Entry Completeness**
    - **Validates: Requirements 7.7**

  - [x] 16.7 Add file locking for concurrent access
    - Use FileLock for history file
    - _Requirements: 7.5, 12.2_

- [x] 17. Checkpoint - Ensure history and shell integration work
  - Ensure all tests pass, ask the user if questions arise.

- [x] 18. Implement Structured Logger
  - [x] 18.1 Create StructuredLogger struct
    - Support verbose, quiet, and debug modes
    - Support file and stderr output
    - _Requirements: 10.1, 10.2_

  - [x] 18.2 Write property test for verbose timing output
    - **Property 31: Verbose Output Contains Timing**
    - **Validates: Requirements 10.1, 10.6**

  - [x] 18.3 Write property test for quiet mode
    - **Property 32: Quiet Mode Suppresses Non-Errors**
    - **Validates: Requirements 10.2**

  - [x] 18.4 Implement CI mode JSON output
    - Detect CI environment
    - Output structured JSON logs
    - _Requirements: 10.4_

  - [x] 18.5 Write property test for CI JSON output
    - **Property 33: CI Mode JSON Output**
    - **Validates: Requirements 10.4**

  - [x] 18.6 Implement log rotation
    - Rotate when file exceeds 10MB
    - Keep last 5 rotations
    - _Requirements: 10.7_

  - [x] 18.7 Implement error logging to file
    - Log full error chain to ~/.dx/logs/
    - _Requirements: 10.3_

- [x] 19. Enhance Theme for Graceful Degradation
  - [x] 19.1 Write property test for color-disabled output
    - **Property 34: Color-Disabled Output Purity**
    - **Validates: Requirements 11.2**

  - [x] 19.2 Write property test for container detection
    - **Property 35: Container Detection**
    - **Validates: Requirements 11.6**

  - [x] 19.3 Write property test for terminal width fallback
    - **Property 36: Terminal Width Fallback**
    - **Validates: Requirements 11.7**

- [x] 20. Implement Event Debouncing
  - [x] 20.1 Create Debouncer struct
    - Coalesce events within 100ms window
    - _Requirements: 9.2, 12.6_

  - [x] 20.2 Write property test for event debouncing
    - **Property 30: Event Debouncing**
    - **Validates: Requirements 9.2, 12.6**

- [ ] 21. Checkpoint - Ensure logging and debouncing work
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 22. Integrate Hardening into Main Entry Point
  - [ ] 22.1 Install crash reporter at startup
    - Call CrashReporter::install() in main
    - _Requirements: 1.5_

  - [ ] 22.2 Initialize resource manager
    - Create global ResourceManager instance
    - _Requirements: 9.7_

  - [ ] 22.3 Enhance signal handler
    - Integrate with ResourceManager for cleanup
    - Terminate children gracefully on Ctrl+C
    - _Requirements: 1.6, 9.5_

  - [ ] 22.4 Add verbose/quiet/debug flag handling
    - Initialize StructuredLogger based on flags
    - _Requirements: 10.1, 10.2, 10.6_

  - [ ] 22.5 Add offline mode detection
    - Skip update checks when offline
    - _Requirements: 3.2, 3.7, 11.4_

- [ ] 23. Final Checkpoint - Full integration test
  - Ensure all tests pass, ask the user if questions arise.
  - Run `cargo test` to verify all property tests pass
  - Run `dx --help` and verify output
  - Test Ctrl+C handling and cleanup
  - Test with --verbose and --quiet flags

## Notes

- All tasks including property tests are required for comprehensive correctness guarantees
- Each task references specific requirements for traceability
- Checkpoints ensure incremental validation
- Property tests use proptest with minimum 100 iterations
- Unit tests validate specific examples and edge cases
- Platform-specific code uses cfg attributes for conditional compilation
