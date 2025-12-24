# Implementation Plan: Binary Dawn Architecture

## Overview

This implementation plan breaks down the Binary Dawn architecture into discrete coding tasks. The implementation will be done in Rust, following the existing codebase patterns. We'll create a new `dx-reactor` crate and extend the existing `dx-www` ecosystem.

## Tasks

- [x] 1. Set up dx-reactor crate structure
  - Create `crates/dx-reactor/` directory structure
  - Create `Cargo.toml` with platform-specific dependencies
  - Create module structure: `io/`, `protocol/`, `memory/`, `middleware/`
  - _Requirements: 1.1_

- [x] 2. Implement core I/O abstractions
  - [x] 2.1 Define Reactor trait and common types
    - Create `src/io/mod.rs` with Reactor trait
    - Define ReactorConfig, Completion, Interest, IoHandle types
    - Implement platform type aliases (PlatformReactor)
    - _Requirements: 1.1, 1.7, 1.8_

  - [x] 2.2 Write property test for batch submission
    - **Property 1: Batch Submission Count**
    - **Validates: Requirements 1.7**

  - [x] 2.3 Implement io_uring backend (Linux)
    - Create `src/io/uring.rs`
    - Implement `is_available()` kernel version check
    - Implement UringReactor with SQPOLL support
    - Implement `recv_multishot()` and `send_zc()` for zero-copy I/O
    - _Requirements: 2.1, 2.2, 2.3, 2.4, 2.5, 2.6_

  - [x] 2.4 Write property test for kernel version detection
    - **Property 3: Kernel Version Detection**
    - **Validates: Requirements 2.1**

  - [x] 2.5 Implement epoll backend (Linux fallback)
    - Create `src/io/epoll.rs`
    - Implement EpollReactor as fallback for older Linux
    - _Requirements: 1.3_

  - [x] 2.6 Implement kqueue backend (macOS/BSD)
    - Create `src/io/kqueue.rs`
    - Implement KqueueReactor with batch event handling
    - Implement `register_read()` and `register_write()`
    - _Requirements: 3.1, 3.2, 3.3, 3.4, 3.5_

  - [x] 2.7 Write property test for kqueue batch submission
    - **Property 4: Kqueue Batch Submission**
    - **Validates: Requirements 3.4**

  - [x] 2.8 Implement IOCP backend (Windows)
    - Create `src/io/iocp.rs`
    - Implement IocpReactor with completion port
    - Implement `associate()`, `read_file()`, `recv_socket()`
    - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5_

  - [x] 2.9 Write property test for completion structure
    - **Property 5: Completion Structure Integrity**
    - **Validates: Requirements 3.5, 4.5**

- [x] 3. Checkpoint - Verify I/O backends compile on target platforms
  - Ensure all tests pass, ask the user if questions arise.

- [x] 4. Implement DxReactor and thread-per-core architecture
  - [x] 4.1 Implement ReactorBuilder and DxReactor
    - Create `src/lib.rs` with DxReactor struct
    - Implement ReactorBuilder with fluent API
    - Implement WorkerStrategy and IoBackend enums
    - _Requirements: 5.1, 5.4_

  - [x] 4.2 Write property test for thread-per-core default
    - **Property 6: Thread-per-Core Default**
    - **Validates: Requirements 5.1**

  - [x] 4.3 Write property test for fixed worker count
    - **Property 7: Fixed Worker Count**
    - **Validates: Requirements 5.4**

  - [x] 4.4 Implement CoreState and event loop
    - Create CoreState struct with local work queue
    - Implement `run_event_loop()` for each core
    - Implement CPU pinning via core_affinity
    - _Requirements: 5.2, 5.3_

  - [x] 4.5 Implement `best_available()` runtime detection
    - Create function to detect and return best reactor
    - _Requirements: 1.6_

- [x] 5. Checkpoint - Verify reactor builds and spawns workers
  - Ensure all tests pass, ask the user if questions arise.

- [x] 6. Implement HBTP Protocol
  - [x] 6.1 Define HbtpOpcode enum
    - Create `src/protocol/hbtp.rs`
    - Define all opcodes as u8 repr
    - _Requirements: 6.1_

  - [x] 6.2 Write property test for opcode uniqueness
    - **Property 8: Opcode Uniqueness**
    - **Validates: Requirements 6.1**

  - [x] 6.3 Implement HbtpHeader and HbtpFlags
    - Define packed 8-byte header struct
    - Implement `from_bytes()` zero-copy parsing
    - Implement `payload()` slice extraction
    - Define HbtpFlags bitflags
    - _Requirements: 6.2, 6.3, 6.5_

  - [x] 6.4 Write property test for header size invariant
    - **Property 9: Header Size Invariant**
    - **Validates: Requirements 6.2**

  - [x] 6.5 Write property test for header parsing
    - **Property 10: Header Parsing**
    - **Validates: Requirements 6.3**

  - [x] 6.6 Write property test for flag composition
    - **Property 12: Flag Composition**
    - **Validates: Requirements 6.5**

  - [x] 6.7 Implement HbtpProtocol handler
    - Create HbtpProtocol struct with route handlers
    - Implement `route()` for O(1) handler registration
    - Implement `process()` for message handling
    - _Requirements: 6.4_

  - [x] 6.8 Write property test for O(1) route lookup
    - **Property 11: O(1) Route Lookup**
    - **Validates: Requirements 6.4**

  - [x] 6.9 Implement ResponseBuffer
    - Create pre-allocated response buffer
    - Implement `write_pong()`, `write_rpc_response()`
    - Implement `reset()` for reuse
    - _Requirements: 6.6_

  - [x] 6.10 Write property test for ResponseBuffer reuse
    - **Property 13: ResponseBuffer Reuse**
    - **Validates: Requirements 6.6**

- [x] 7. Checkpoint - Verify HBTP protocol works end-to-end
  - Ensure all tests pass, ask the user if questions arise.

- [x] 8. Implement Memory Teleportation
  - [x] 8.1 Define Teleportable trait and TeleportLayout
    - Create `src/memory/teleport.rs`
    - Define unsafe Teleportable trait
    - Define TeleportLayout struct
    - _Requirements: 7.1, 7.2_

  - [x] 8.2 Implement TeleportBuffer
    - Implement `new()`, `write()`, `write_slice()`
    - Implement `write_string()` with offset/length
    - Implement `finalize()` with string table
    - _Requirements: 7.3, 7.4_

  - [x] 8.3 Implement TeleportReader
    - Implement `new()`, `read()`, `read_slice()`
    - Implement `read_string()` by offset/length
    - _Requirements: 7.5_

  - [x] 8.4 Write property test for teleportation round-trip
    - **Property 14: Teleportation Round-Trip**
    - **Validates: Requirements 7.3, 7.4, 7.5**

  - [x] 8.5 Create example Teleportable types
    - Implement TeleportableUser as example
    - Add derive macro placeholder for future
    - _Requirements: 7.6_

- [x] 9. Checkpoint - Verify teleportation works correctly
  - Ensure all tests pass, ask the user if questions arise.

- [x] 10. Implement Compiler-Inlined Middleware (CIM)
  - [x] 10.1 Define Middleware trait
    - Create `src/middleware/mod.rs`
    - Define Middleware trait with before/after hooks
    - Define MiddlewareResult and MiddlewareError
    - _Requirements: 8.1_

  - [x] 10.2 Implement dx_middleware! macro
    - Create macro for compile-time middleware chaining
    - Implement forward before hooks, reverse after hooks
    - _Requirements: 8.2, 8.3_

  - [x] 10.3 Write property test for middleware execution order
    - **Property 15: Middleware Execution Order**
    - **Validates: Requirements 8.3**

  - [x] 10.4 Implement AuthMiddleware
    - Implement JWT verification in before hook
    - Inject claims into request extensions
    - _Requirements: 8.4_

  - [x] 10.5 Implement TimingMiddleware
    - Record start time in before hook
    - Add X-Response-Time header in after hook
    - _Requirements: 8.5_

  - [x] 10.6 Write property test for timing header presence
    - **Property 16: Timing Header Presence**
    - **Validates: Requirements 8.5**

  - [x] 10.7 Implement RateLimitMiddleware
    - Use thread_local! for per-thread counters
    - Implement rate limiting logic
    - _Requirements: 8.6_

  - [x] 10.8 Write property test for rate limit thread isolation
    - **Property 17: Rate Limit Thread Isolation**
    - **Validates: Requirements 8.6**

- [x] 11. Checkpoint - Verify middleware chain works correctly
  - Ensure all tests pass, ask the user if questions arise.

- [x] 12. Implement dx-db-teleport (Reactive Caching)
  - [x] 12.1 Create dx-db-teleport crate
    - Create `crates/dx-db-teleport/` structure
    - Add dependencies: deadpool-postgres, dashmap, tokio
    - _Requirements: 9.1_

  - [x] 12.2 Implement DbTeleport core
    - Create DbTeleport struct with cache and queries
    - Implement `new()` with connection pool setup
    - Set up Postgres NOTIFY listener
    - _Requirements: 9.1, 9.2_

  - [x] 12.3 Implement query registration and caching
    - Implement `register_query()` with table dependencies
    - Implement `get_cached()` for fast cache access
    - Implement `execute_and_cache()` for cache population
    - _Requirements: 9.2, 9.4, 9.5_

  - [x] 12.4 Write property test for cache consistency
    - **Property 18: Cache Consistency**
    - **Validates: Requirements 9.1, 9.5**

  - [x] 12.5 Implement cache invalidation
    - Implement `process_notifications()` for NOTIFY handling
    - Invalidate queries based on table dependencies
    - _Requirements: 9.3_

  - [x] 12.6 Write property test for cache invalidation
    - **Property 19: Cache Invalidation**
    - **Validates: Requirements 9.3**

  - [x] 12.7 Write property test for cache access latency
    - **Property 20: Cache Access Latency**
    - **Validates: Requirements 9.4**

- [x] 13. Final checkpoint - Full integration test
  - Ensure all tests pass, ask the user if questions arise.

- [x] 14. Integration and wiring
  - [x] 14.1 Wire dx-reactor into dx-www
    - Add dx-reactor as dependency to dx-www
    - Update dx-www to use DxReactor for I/O
    - _Requirements: All_

  - [x] 14.2 Add dx-db-teleport integration
    - Add dx-db-teleport as optional dependency
    - Create integration examples
    - _Requirements: 9.1-9.5_

  - [x] 14.3 Write integration tests
    - Test full request/response cycle
    - Test cross-platform I/O operations
    - _Requirements: All_

## Notes

- All tasks are required for comprehensive validation
- Each task references specific requirements for traceability
- Checkpoints ensure incremental validation
- Property tests validate universal correctness properties
- Unit tests validate specific examples and edge cases
- The implementation uses Rust with proptest for property-based testing
