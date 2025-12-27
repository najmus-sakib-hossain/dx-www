# Implementation Plan: Cross-Platform I/O Reactor

## Overview

This implementation plan creates the `dx-reactor` crate with cross-platform I/O backends (io_uring, kqueue, IOCP), HBTP binary protocol, memory teleportation, and compiler-inlined middleware. The implementation follows an incremental approach, building foundational components first and then integrating them.

## Tasks

- [ ] 1. Set up dx-reactor crate structure
  - Create `crates/dx-reactor/` directory structure
  - Create `Cargo.toml` with platform-specific dependencies
  - Create module structure: `io/`, `protocol/`, `memory/`, `middleware/`
  - Define feature flags for optional backends
  - _Requirements: 1.1, 1.6_

- [ ] 2. Implement core reactor traits and types
  - [ ] 2.1 Define Reactor trait and associated types
    - Create `src/io/mod.rs` with Reactor trait
    - Define `ReactorConfig`, `Completion`, `Interest`, `IoHandle` types
    - Implement platform type aliases using conditional compilation
    - _Requirements: 1.1, 1.6_
  - [ ] 2.2 Write property test for worker count configuration
    - **Property 8: Worker Count Configuration**
    - **Validates: Requirements 5.2**

- [ ] 3. Implement io_uring backend (Linux)
  - [ ] 3.1 Create UringReactor structure
    - Create `src/io/uring.rs` with UringReactor struct
    - Implement `is_available()` function for kernel version detection
    - Implement `new()` with SQPOLL and buffer registration support
    - _Requirements: 2.1, 2.2, 2.5, 2.7_
  - [ ] 3.2 Implement Reactor trait for UringReactor
    - Implement `register()`, `submit()`, `wait()`, `submit_and_wait()`
    - Implement multishot receive and zero-copy send helpers
    - _Requirements: 2.3, 2.4, 2.6_
  - [ ]* 3.3 Write unit tests for UringReactor
    - Test SQPOLL configuration
    - Test buffer registration
    - Test multishot receive
    - _Requirements: 2.1, 2.2, 2.3, 2.4_

- [ ] 4. Implement kqueue backend (macOS/BSD)
  - [ ] 4.1 Create KqueueReactor structure
    - Create `src/io/kqueue.rs` with KqueueReactor struct
    - Implement `new()` with configurable event capacity
    - _Requirements: 3.1, 3.3_
  - [ ] 4.2 Implement Reactor trait for KqueueReactor
    - Implement `register_read()`, `register_write()` helpers
    - Implement batch event submission and retrieval
    - Handle edge-triggered events correctly
    - _Requirements: 3.2, 3.4, 3.5_
  - [ ]* 4.3 Write unit tests for KqueueReactor
    - Test read/write event registration
    - Test batch operations
    - Test timeout configuration
    - _Requirements: 3.1, 3.2, 3.3_

- [ ] 5. Implement IOCP backend (Windows)
  - [ ] 5.1 Create IocpReactor structure
    - Create `src/io/iocp.rs` with IocpReactor struct
    - Implement `new()` with completion port creation
    - Implement `associate()` for handle association
    - _Requirements: 4.1_
  - [ ] 5.2 Implement Reactor trait for IocpReactor
    - Implement async file read with OVERLAPPED
    - Implement async socket recv with WSARecv
    - Implement batch completion retrieval
    - _Requirements: 4.2, 4.3, 4.4, 4.5, 4.6_
  - [ ]* 5.3 Write unit tests for IocpReactor
    - Test completion port creation
    - Test async file operations
    - Test async socket operations
    - _Requirements: 4.1, 4.2, 4.3_

- [ ] 6. Checkpoint - Ensure all reactor backends compile
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 7. Implement HBTP protocol
  - [ ] 7.1 Define HBTP opcodes and header
    - Create `src/protocol/hbtp.rs`
    - Define `HbtpOpcode` enum with all opcodes
    - Define `HbtpHeader` struct (8 bytes, packed)
    - Define `HbtpFlags` bitflags
    - _Requirements: 6.1, 6.2_
  - [ ]* 7.2 Write property test for HBTP header size invariant
    - **Property 1: HBTP Header Size Invariant**
    - **Validates: Requirements 6.2**
  - [ ] 7.3 Implement HBTP message serialization
    - Implement `HbtpHeader::from_bytes()` and `to_bytes()`
    - Implement compression support with zstd
    - Implement encryption support with ChaCha20
    - _Requirements: 6.3, 6.4_
  - [ ]* 7.4 Write property test for HBTP message round-trip
    - **Property 2: HBTP Message Round-Trip**
    - **Validates: Requirements 6.3, 6.4, 6.6**
  - [ ] 7.5 Implement HBTP protocol handler
    - Create `HbtpProtocol` struct with route handlers
    - Implement O(1) route lookup using array indexing
    - Implement `process()` method for message handling
    - _Requirements: 6.5_
  - [ ]* 7.6 Write property test for HBTP route lookup O(1)
    - **Property 3: HBTP Route Lookup O(1)**
    - **Validates: Requirements 6.5**

- [ ] 8. Implement memory teleportation
  - [ ] 8.1 Define Teleportable trait and layout
    - Create `src/memory/teleport.rs`
    - Define `Teleportable` trait with SIZE, ALIGN constants
    - Define `TeleportLayout` struct for compile-time verification
    - _Requirements: 7.5_
  - [ ] 8.2 Implement TeleportBuffer
    - Implement `write<T>()` with correct alignment
    - Implement `write_slice<T>()` for arrays
    - Implement `write_string()` with string table
    - Implement `finalize()` to get final bytes
    - _Requirements: 7.1, 7.2_
  - [ ] 8.3 Implement TeleportReader
    - Implement `read<T>()` returning reference (zero-copy)
    - Implement `read_slice<T>()` for arrays
    - Implement `read_string()` using string table
    - _Requirements: 7.3_
  - [ ]* 8.4 Write property test for teleportation round-trip
    - **Property 4: Teleportation Round-Trip**
    - **Validates: Requirements 7.1, 7.2, 7.4**

- [ ] 9. Checkpoint - Ensure protocol and teleportation work
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 10. Implement compiler-inlined middleware
  - [ ] 10.1 Define Middleware trait
    - Create `src/middleware/cim.rs`
    - Define `Middleware` trait with `before()` and `after()` hooks
    - Define `MiddlewareResult` and `MiddlewareError` types
    - _Requirements: 8.1_
  - [ ] 10.2 Implement dx_middleware! macro
    - Create macro for compile-time middleware chain generation
    - Implement before hooks in order
    - Implement after hooks in reverse order
    - Support short-circuiting in before hooks
    - _Requirements: 8.2, 8.3, 8.4_
  - [ ]* 10.3 Write property test for middleware after hook order
    - **Property 5: Middleware After Hook Order**
    - **Validates: Requirements 8.3**
  - [ ] 10.4 Implement built-in middleware
    - Implement `AuthMiddleware` for JWT verification
    - Implement `TimingMiddleware` for request timing
    - Implement `RateLimitMiddleware` with per-core counters
    - Implement `CorsMiddleware` with compile-time origins
    - _Requirements: 8.1, 8.5_

- [ ] 11. Implement thread-per-core architecture
  - [ ] 11.1 Create DxReactor and ReactorBuilder
    - Create `src/lib.rs` with DxReactor struct
    - Implement ReactorBuilder with fluent API
    - Implement `build()` to create reactor
    - _Requirements: 5.1, 5.2_
  - [ ] 11.2 Implement CoreState and worker threads
    - Create `CoreState` struct with per-core reactor and queue
    - Implement thread spawning with CPU affinity
    - Implement `ignite()` to start all workers
    - _Requirements: 5.3, 5.4_
  - [ ] 11.3 Implement work-stealing
    - Implement `LocalQueue` with work-stealing support
    - Implement stealing logic for underloaded cores
    - _Requirements: 5.5_
  - [ ]* 11.4 Write unit tests for thread-per-core
    - Test worker count configuration
    - Test CPU affinity
    - Test work-stealing behavior
    - _Requirements: 5.1, 5.2, 5.3, 5.5_

- [ ] 12. Implement database teleport cache
  - [ ] 12.1 Create DbTeleport structure
    - Create `src/cache/db_teleport.rs` (or separate crate)
    - Define `DbTeleport`, `CacheEntry`, `CacheKey` structs
    - Implement connection pool integration
    - _Requirements: 9.1_
  - [ ] 12.2 Implement query registration and caching
    - Implement `register_query()` with table dependencies
    - Implement `execute_and_cache()` for cache population
    - Implement `get_cached()` for O(1) lookup
    - _Requirements: 9.2, 9.4_
  - [ ]* 12.3 Write property test for cache lookup O(1)
    - **Property 6: Cache Lookup O(1)**
    - **Validates: Requirements 9.4**
  - [ ] 12.4 Implement cache invalidation
    - Set up PostgreSQL LISTEN for table changes
    - Implement `process_notifications()` for invalidation
    - Implement cache size limits
    - _Requirements: 9.3, 9.5_

- [ ] 13. Checkpoint - Ensure all components integrate
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 14. Implement cross-platform testing
  - [ ] 14.1 Create cross-platform test suite
    - Create integration tests for binary format compatibility
    - Test that binary formats are identical across platforms
    - _Requirements: 10.4_
  - [ ]* 14.2 Write property test for binary format compatibility
    - **Property 7: Binary Format Cross-Platform Compatibility**
    - **Validates: Requirements 10.4**
  - [ ] 14.3 Create platform-specific CI configuration
    - Add GitHub Actions workflow for Linux, macOS, Windows
    - Configure test matrix for all platforms
    - Add code coverage reporting
    - _Requirements: 10.1, 10.2, 10.3, 10.5, 10.6_

- [ ] 15. Wire dx-reactor into dx-www
  - [ ] 15.1 Update dx-www Cargo.toml
    - Add dx-reactor dependency
    - Configure feature flags for backends
    - _Requirements: 1.1_
  - [ ] 15.2 Integrate reactor with dev server
    - Update `dev_server.rs` to use dx-reactor
    - Replace tokio I/O with reactor-based I/O
    - _Requirements: 1.2, 1.3, 1.4, 1.5_
  - [ ]* 15.3 Write integration tests for dx-www with reactor
    - Test compilation pipeline with reactor
    - Test hot reload with reactor
    - _Requirements: 10.3, 10.6_

- [ ] 16. Final checkpoint - Full test suite
  - Ensure all tests pass, ask the user if questions arise.

## Notes

- Tasks marked with `*` are optional and can be skipped for faster MVP
- Each task references specific requirements for traceability
- Checkpoints ensure incremental validation
- Property tests validate universal correctness properties
- Unit tests validate specific examples and edge cases
- Platform-specific code uses conditional compilation (`#[cfg(...)]`)
- The io_uring backend requires the `io-uring` crate
- The kqueue backend uses `libc` directly
- The IOCP backend uses `windows-sys` crate

