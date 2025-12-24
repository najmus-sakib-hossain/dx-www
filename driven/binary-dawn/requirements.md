# Requirements Document

## Introduction

Binary Dawn is a revolutionary cross-platform architecture for dx-www that achieves unprecedented web framework performance through platform-specific I/O backends (io_uring, kqueue, IOCP), a binary transfer protocol (HBTP), zero-copy memory teleportation, and compiler-inlined middleware. The goal is to make dx-www the fastest web framework ever built while maintaining excellent developer experience.

## Glossary

- **DxReactor**: The core I/O reactor that manages cross-platform async operations and thread-per-core architecture
- **HBTP**: Hyper-Binary Transfer Protocol - a binary protocol replacing HTTP for dx-www client-server communication
- **HTIP**: Holographic Template Instruction Protocol - binary format for UI operations
- **Teleportation**: Zero-copy serialization where server and WASM client share identical memory layouts
- **CIM**: Compiler-Inlined Middleware - middleware that is inlined at compile time for zero runtime overhead
- **IoBackend**: Platform-specific I/O implementation (io_uring, kqueue, IOCP, epoll)
- **CoreState**: Per-CPU-core state for thread-per-core architecture
- **TeleportBuffer**: Buffer for zero-copy data transfer between server and WASM client

## Requirements

### Requirement 1: Cross-Platform I/O Abstraction

**User Story:** As a developer, I want dx-www to automatically use the best I/O backend for my platform, so that I get optimal performance without manual configuration.

#### Acceptance Criteria

1. THE DxReactor SHALL provide a unified `Reactor` trait that abstracts platform-specific I/O operations
2. WHEN running on Linux 5.1+, THE DxReactor SHALL use io_uring as the I/O backend
3. WHEN running on older Linux, THE DxReactor SHALL fall back to epoll
4. WHEN running on macOS or BSD, THE DxReactor SHALL use kqueue as the I/O backend
5. WHEN running on Windows, THE DxReactor SHALL use IOCP as the I/O backend
6. THE DxReactor SHALL support runtime detection of the best available backend via `best_available()` function
7. THE Reactor trait SHALL support batch submission of I/O operations via `submit()` method
8. THE Reactor trait SHALL support waiting for completions via `wait()` and `submit_and_wait()` methods

### Requirement 2: io_uring Backend (Linux)

**User Story:** As a Linux user, I want dx-www to leverage io_uring for maximum I/O performance with zero-syscall operations.

#### Acceptance Criteria

1. THE UringReactor SHALL detect io_uring availability by checking kernel version >= 5.1
2. WHEN io_uring is available, THE UringReactor SHALL enable kernel-side polling (SQPOLL) for zero-syscall I/O
3. THE UringReactor SHALL support zero-copy receive via `recv_multishot()` using registered buffers
4. THE UringReactor SHALL support zero-copy send via `send_zc()` using splice
5. THE UringReactor SHALL pre-register buffers for zero-copy I/O when `zero_copy` config is enabled
6. THE UringReactor SHALL support cooperative task running via `setup_coop_taskrun()`

### Requirement 3: kqueue Backend (macOS/BSD)

**User Story:** As a macOS or BSD user, I want dx-www to use kqueue for efficient event-driven I/O.

#### Acceptance Criteria

1. THE KqueueReactor SHALL create a kqueue file descriptor on initialization
2. THE KqueueReactor SHALL support registering read events via `register_read()`
3. THE KqueueReactor SHALL support registering write events via `register_write()`
4. THE KqueueReactor SHALL batch pending changes and submit them in a single `kevent()` call
5. THE KqueueReactor SHALL return completions with user_data, result, and flags

### Requirement 4: IOCP Backend (Windows)

**User Story:** As a Windows user, I want dx-www to use IOCP for native async I/O performance.

#### Acceptance Criteria

1. THE IocpReactor SHALL create an I/O completion port on initialization
2. THE IocpReactor SHALL support associating handles with the completion port via `associate()`
3. THE IocpReactor SHALL support async file reads via `read_file()` using OVERLAPPED
4. THE IocpReactor SHALL support async socket receives via `recv_socket()` using WSARecv
5. THE IocpReactor SHALL batch completions via `GetQueuedCompletionStatusEx()`

### Requirement 5: Thread-per-Core Architecture

**User Story:** As a performance-focused developer, I want dx-www to use a thread-per-core architecture to eliminate lock contention.

#### Acceptance Criteria

1. THE DxReactor SHALL spawn one worker thread per CPU core by default
2. THE DxReactor SHALL pin each worker thread to its corresponding CPU core
3. EACH CoreState SHALL have its own local work queue with no shared locks
4. THE DxReactor SHALL support configurable worker count via `WorkerStrategy::Fixed(n)`
5. THE DxReactor SHALL use work-stealing only when a core's queue is empty

### Requirement 6: HBTP Protocol

**User Story:** As a developer, I want dx-www to use a binary protocol for client-server communication to eliminate HTTP parsing overhead.

#### Acceptance Criteria

1. THE HbtpProtocol SHALL define opcodes as single bytes for common operations (Ping, Pong, Close, StateSync, etc.)
2. THE HbtpHeader SHALL be exactly 8 bytes: opcode (1), flags (1), sequence (2), length (4)
3. THE HbtpProtocol SHALL support zero-copy header parsing via `from_bytes()`
4. THE HbtpProtocol SHALL support O(1) route lookup via array index instead of radix tree
5. THE HbtpFlags SHALL support compression (COMPRESSED), encryption (ENCRYPTED), and response expectation (EXPECTS_RESPONSE)
6. THE ResponseBuffer SHALL be pre-allocated and reusable to avoid allocations

### Requirement 7: Memory Teleportation

**User Story:** As a developer, I want zero-copy serialization between server and WASM client to eliminate serialization overhead.

#### Acceptance Criteria

1. THE Teleportable trait SHALL mark types that can be safely transferred between server and WASM
2. THE TeleportLayout SHALL verify size, alignment, and checksum at compile time
3. THE TeleportBuffer SHALL write teleportable values with proper alignment
4. THE TeleportBuffer SHALL support string tables with offset/length pairs
5. THE TeleportReader SHALL read teleportable values as zero-copy references
6. ALL Teleportable types SHALL use `#[repr(C)]` for stable memory layout

### Requirement 8: Compiler-Inlined Middleware (CIM)

**User Story:** As a developer, I want middleware to have zero runtime overhead by being inlined at compile time.

#### Acceptance Criteria

1. THE Middleware trait SHALL define `before()` and `after()` hooks that can be inlined
2. THE `dx_middleware!` macro SHALL generate a single function with all middleware logic inlined
3. THE CIM system SHALL execute `before()` hooks in order and `after()` hooks in reverse order
4. THE AuthMiddleware SHALL verify JWT tokens and inject claims into request extensions
5. THE TimingMiddleware SHALL measure request duration and add X-Response-Time header
6. THE RateLimitMiddleware SHALL use thread-local counters to avoid lock contention

### Requirement 9: Reactive Database Caching (dx-db-teleport)

**User Story:** As a developer, I want frequently-read database queries to be pre-cached as binary responses for sub-millisecond access.

#### Acceptance Criteria

1. THE DbTeleport SHALL maintain a cache of pre-serialized binary responses
2. THE DbTeleport SHALL support registering queries with table dependencies
3. WHEN a table changes, THE DbTeleport SHALL invalidate all queries depending on that table via Postgres NOTIFY
4. THE DbTeleport SHALL return cached binary responses in < 0.1ms
5. IF cache miss occurs, THE DbTeleport SHALL execute the query, serialize to binary, and cache the result

### Requirement 10: Performance Targets

**User Story:** As a developer, I want dx-www to achieve specific performance targets that exceed all existing frameworks.

#### Acceptance Criteria

1. THE DxReactor SHALL achieve >= 2,500,000 RPS in HTTP mode
2. THE DxReactor SHALL achieve >= 5,000,000 RPS in HBTP mode
3. THE DxReactor SHALL maintain < 100μs p99 latency
4. THE DxReactor SHALL use < 50MB memory for 100K concurrent connections
5. THE Teleportation system SHALL have zero serialization overhead (just memcpy)
