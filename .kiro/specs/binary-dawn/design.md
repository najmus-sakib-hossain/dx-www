# Design Document: Binary Dawn Architecture

## Overview

Binary Dawn is a revolutionary cross-platform I/O and protocol architecture that makes dx-www the fastest web framework ever built. The design centers around four key innovations:

1. **Cross-Platform I/O Abstraction** - A unified `Reactor` trait with platform-specific implementations (io_uring, kqueue, IOCP, epoll)
2. **Thread-per-Core Architecture** - Zero lock contention through CPU-pinned worker threads with local queues
3. **HBTP Protocol** - Binary protocol replacing HTTP with 8-byte headers and O(1) routing
4. **Memory Teleportation** - Zero-copy serialization between Rust server and WASM client

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           dx-www Binary Dawn Stack                          │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                        dx-www (Compiler)                             │   │
│  │  TSX → Binary artifacts (.dxb) + Compiled routes + Inlined middleware│   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                    │                                        │
│  ┌─────────────────────────────────┴────────────────────────────────────┐  │
│  │                        dx-reactor (Core)                             │  │
│  │                                                                       │  │
│  │  ┌───────────────┐  ┌───────────────┐  ┌───────────────────────────┐│  │
│  │  │   I/O Layer   │  │   Protocol    │  │      Memory Layer         ││  │
│  │  │               │  │               │  │                           ││  │
│  │  │ ┌───────────┐ │  │ ┌───────────┐ │  │ ┌───────────────────────┐ ││  │
│  │  │ │ io_uring  │ │  │ │   HBTP    │ │  │ │    Teleportation      │ ││  │
│  │  │ │ (Linux)   │ │  │ │ (Binary   │ │  │ │    (Zero-copy         │ ││  │
│  │  │ └───────────┘ │  │ │ Protocol) │ │  │ │    serialization)     │ ││  │
│  │  │ ┌───────────┐ │  │ └───────────┘ │  │ └───────────────────────┘ ││  │
│  │  │ │  kqueue   │ │  │ ┌───────────┐ │  │ ┌───────────────────────┐ ││  │
│  │  │ │ (macOS)   │ │  │ │   HTIP    │ │  │ │   SharedArrayBuffer   │ ││  │
│  │  │ └───────────┘ │  │ │ (UI Ops)  │ │  │ │   (WASM shared mem)   │ ││  │
│  │  │ ┌───────────┐ │  │ └───────────┘ │  │ └───────────────────────┘ ││  │
│  │  │ │   IOCP    │ │  │               │  │                           ││  │
│  │  │ │ (Windows) │ │  │               │  │                           ││  │
│  │  │ └───────────┘ │  │               │  │                           ││  │
│  │  └───────────────┘  └───────────────┘  └───────────────────────────┘│  │
│  │                                                                       │  │
│  │  ┌───────────────────────────────────────────────────────────────┐  │  │
│  │  │                  Thread-per-Core Architecture                  │  │  │
│  │  │  Core 0     Core 1     Core 2     Core 3     ...     Core N    │  │  │
│  │  │  ┌─────┐   ┌─────┐   ┌─────┐   ┌─────┐            ┌─────┐     │  │  │
│  │  │  │Local│   │Local│   │Local│   │Local│            │Local│     │  │  │
│  │  │  │Queue│   │Queue│   │Queue│   │Queue│            │Queue│     │  │  │
│  │  │  └─────┘   └─────┘   └─────┘   └─────┘            └─────┘     │  │  │
│  │  └───────────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │                       dx-db-teleport (Cache)                          │  │
│  │  Postgres ──NOTIFY──▶ Pre-computed Binary ──▶ < 0.1ms Response       │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Components and Interfaces

### 1. Reactor Trait (I/O Abstraction)

The unified interface for all platform-specific I/O backends:

```rust
// crates/dx-reactor/src/io/mod.rs

/// Unified I/O reactor trait
pub trait Reactor: Send + Sync + 'static {
    type Handle: IoHandle;
    
    /// Create new reactor instance
    fn new(config: ReactorConfig) -> io::Result<Self> where Self: Sized;
    
    /// Register a file descriptor for events
    fn register(&self, fd: RawFd, interest: Interest) -> io::Result<Self::Handle>;
    
    /// Submit pending operations (batch)
    fn submit(&self) -> io::Result<usize>;
    
    /// Wait for completions
    fn wait(&self, timeout: Option<Duration>) -> io::Result<Vec<Completion>>;
    
    /// Submit and wait (optimized path)
    fn submit_and_wait(&self, min_complete: usize) -> io::Result<Vec<Completion>>;
}

/// Platform-specific reactor selection at compile time
#[cfg(all(target_os = "linux", feature = "io_uring"))]
pub type PlatformReactor = uring::UringReactor;

#[cfg(all(target_os = "linux", not(feature = "io_uring")))]
pub type PlatformReactor = epoll::EpollReactor;

#[cfg(any(target_os = "macos", target_os = "freebsd", target_os = "openbsd"))]
pub type PlatformReactor = kqueue::KqueueReactor;

#[cfg(target_os = "windows")]
pub type PlatformReactor = iocp::IocpReactor;
```

### 2. ReactorConfig

Configuration for reactor initialization:

```rust
pub struct ReactorConfig {
    /// Number of submission queue entries
    pub entries: u32,
    /// Enable kernel-side polling (Linux io_uring)
    pub sqpoll: bool,
    /// SQPOLL idle timeout in milliseconds
    pub sqpoll_idle_ms: u32,
    /// CPU to pin SQPOLL thread to
    pub sqpoll_cpu: Option<u32>,
    /// Enable zero-copy I/O
    pub zero_copy: bool,
    /// Buffer size for registered buffers
    pub buffer_size: usize,
    /// Number of pre-registered buffers
    pub buffer_count: usize,
    /// Concurrency hint for IOCP
    pub concurrency_hint: usize,
}
```

### 3. Completion Structure

Unified completion result across all backends:

```rust
pub struct Completion {
    /// User-provided identifier
    pub user_data: u64,
    /// Operation result (bytes transferred or error code)
    pub result: i32,
    /// Backend-specific flags
    pub flags: u32,
}
```

### 4. DxReactor (Main Entry Point)

```rust
pub struct DxReactor {
    config: ReactorConfig,
    cores: Vec<CoreState>,
    protocol: Arc<HbtpProtocol>,
    router: CompiledRouter,
}

impl DxReactor {
    pub fn build() -> ReactorBuilder { ... }
    pub fn ignite(self) -> ! { ... }
}

pub struct ReactorBuilder {
    workers: WorkerStrategy,
    io_backend: Option<IoBackend>,
    teleport: bool,
    hbtp: bool,
    buffer_size: usize,
    buffer_count: usize,
}

pub enum WorkerStrategy {
    ThreadPerCore,
    Fixed(usize),
}

pub enum IoBackend {
    IoUring,
    Epoll,
    Kqueue,
    Iocp,
    Auto,
}
```

### 5. HBTP Protocol

```rust
/// HBTP OpCodes - 1 byte for common operations
#[repr(u8)]
pub enum HbtpOpcode {
    Ping = 0x00,
    Pong = 0x01,
    Close = 0x02,
    StateSync = 0x10,
    StateDelta = 0x11,
    HtipClone = 0x20,
    HtipPatchText = 0x21,
    RpcCall = 0x30,
    RpcResponse = 0x31,
    ClientEvent = 0x40,
    Extended = 0xFF,
}

/// 8-byte header for all messages
#[repr(C, packed)]
pub struct HbtpHeader {
    pub opcode: HbtpOpcode,
    pub flags: HbtpFlags,
    pub sequence: u16,
    pub length: u32,
}

bitflags! {
    pub struct HbtpFlags: u8 {
        const COMPRESSED = 0b0000_0001;
        const ENCRYPTED = 0b0000_0010;
        const EXPECTS_RESPONSE = 0b0000_0100;
        const FINAL = 0b0000_1000;
    }
}
```

### 6. Memory Teleportation

```rust
/// Marker trait for zero-copy transferable types
pub unsafe trait Teleportable: Copy + 'static {
    const LAYOUT: TeleportLayout;
}

pub struct TeleportLayout {
    pub size: usize,
    pub align: usize,
    pub checksum: u64,
}

pub struct TeleportBuffer {
    buffer: Vec<u8>,
    position: usize,
    string_table_start: usize,
    strings: Vec<u8>,
}

impl TeleportBuffer {
    pub fn new(capacity: usize) -> Self;
    pub fn write<T: Teleportable>(&mut self, value: &T);
    pub fn write_slice<T: Teleportable>(&mut self, values: &[T]);
    pub fn write_string(&mut self, s: &str) -> (u32, u32);
    pub fn finalize(&mut self) -> &[u8];
}

pub struct TeleportReader<'a> {
    buffer: &'a [u8],
    position: usize,
    string_table_offset: usize,
}

impl<'a> TeleportReader<'a> {
    pub fn read<T: Teleportable>(&mut self) -> &'a T;
    pub fn read_slice<T: Teleportable>(&mut self, count: usize) -> &'a [T];
    pub fn read_string(&self, offset: u32, len: u32) -> &'a str;
}
```

### 7. Compiler-Inlined Middleware

```rust
pub trait Middleware: Sized + 'static {
    fn before(req: &mut Request) -> MiddlewareResult<()>;
    fn after(req: &Request, res: &mut Response);
}

#[macro_export]
macro_rules! dx_middleware {
    ($($middleware:ty),* $(,)?) => { ... };
}

// Built-in middleware
pub struct AuthMiddleware;
pub struct TimingMiddleware;
pub struct RateLimitMiddleware;
pub struct CorsMiddleware<const ORIGINS: &'static [&'static str]>;
```

## Data Models

### ReactorConfig

| Field | Type | Description |
|-------|------|-------------|
| entries | u32 | Submission queue size (default: 16384) |
| sqpoll | bool | Enable kernel polling (Linux only) |
| sqpoll_idle_ms | u32 | SQPOLL idle timeout |
| sqpoll_cpu | Option<u32> | CPU affinity for SQPOLL |
| zero_copy | bool | Enable zero-copy I/O |
| buffer_size | usize | Per-buffer size |
| buffer_count | usize | Number of buffers |
| concurrency_hint | usize | Thread count hint |

### HbtpHeader (8 bytes)

| Offset | Size | Field | Description |
|--------|------|-------|-------------|
| 0 | 1 | opcode | Operation type |
| 1 | 1 | flags | Message flags |
| 2 | 2 | sequence | Request sequence number |
| 4 | 4 | length | Payload length |

### TeleportableUser (Example)

```rust
#[repr(C)]
pub struct TeleportableUser {
    pub id: u64,           // 8 bytes
    pub name_offset: u32,  // 4 bytes
    pub name_len: u32,     // 4 bytes
    pub age: u8,           // 1 byte
    pub active: u8,        // 1 byte
    pub _pad: [u8; 6],     // 6 bytes padding
}
// Total: 24 bytes, aligned to 8
```



## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system—essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

### Property 1: Batch Submission Count

*For any* sequence of I/O operations submitted to a Reactor, the `submit()` method SHALL return the exact count of operations that were successfully queued.

**Validates: Requirements 1.7**

### Property 2: Completion Delivery

*For any* submitted I/O operation, the `wait()` or `submit_and_wait()` method SHALL eventually return a Completion with the corresponding user_data.

**Validates: Requirements 1.8**

### Property 3: Kernel Version Detection

*For any* Linux kernel version string, the `is_available()` function SHALL return true if and only if the major version > 5 OR (major version == 5 AND minor version >= 1).

**Validates: Requirements 2.1**

### Property 4: Kqueue Batch Submission

*For any* KqueueReactor with pending changes, after calling `wait()`, the pending_changes vector SHALL be empty.

**Validates: Requirements 3.4**

### Property 5: Completion Structure Integrity

*For any* Completion returned by a Reactor, it SHALL contain valid user_data, result, and flags fields that correspond to the original operation.

**Validates: Requirements 3.5, 4.5**

### Property 6: Thread-per-Core Default

*For any* DxReactor built with `WorkerStrategy::ThreadPerCore`, the number of CoreState instances SHALL equal `num_cpus::get()`.

**Validates: Requirements 5.1**

### Property 7: Fixed Worker Count

*For any* DxReactor built with `WorkerStrategy::Fixed(n)`, the number of CoreState instances SHALL equal exactly n.

**Validates: Requirements 5.4**

### Property 8: Opcode Uniqueness

*For all* HbtpOpcode variants, each SHALL have a unique u8 value and fit within a single byte.

**Validates: Requirements 6.1**

### Property 9: Header Size Invariant

*For all* HbtpHeader instances, `size_of::<HbtpHeader>()` SHALL equal exactly 8 bytes.

**Validates: Requirements 6.2**

### Property 10: Header Parsing

*For any* byte slice of length >= 8, `HbtpHeader::from_bytes()` SHALL return Some. *For any* byte slice of length < 8, it SHALL return None.

**Validates: Requirements 6.3**

### Property 11: O(1) Route Lookup

*For any* HbtpProtocol with N registered routes, route lookup time SHALL be constant (O(1)) regardless of N.

**Validates: Requirements 6.4**

### Property 12: Flag Composition

*For any* combination of HbtpFlags, setting and checking individual flags SHALL be independent and composable.

**Validates: Requirements 6.5**

### Property 13: ResponseBuffer Reuse

*For any* ResponseBuffer, after calling `reset()`, the buffer SHALL be reusable without additional allocation, and `as_bytes()` SHALL return an empty or header-only slice.

**Validates: Requirements 6.6**

### Property 14: Teleportation Round-Trip

*For any* Teleportable value written to a TeleportBuffer, reading it back with TeleportReader SHALL produce a value equal to the original. *For any* string written via `write_string()`, reading it back via `read_string()` with the returned offset/length SHALL produce the original string.

**Validates: Requirements 7.3, 7.4, 7.5**

### Property 15: Middleware Execution Order

*For any* sequence of middleware types in `dx_middleware!`, the `before()` hooks SHALL execute in declaration order, and `after()` hooks SHALL execute in reverse declaration order.

**Validates: Requirements 8.3**

### Property 16: Timing Header Presence

*For any* request processed through TimingMiddleware, the response SHALL contain an "X-Response-Time" header with a valid duration value.

**Validates: Requirements 8.5**

### Property 17: Rate Limit Thread Isolation

*For any* RateLimitMiddleware, the rate counter SHALL be thread-local, meaning concurrent requests on different threads SHALL have independent counters.

**Validates: Requirements 8.6**

### Property 18: Cache Consistency

*For any* query executed via `execute_and_cache()`, subsequent calls to `get_cached()` with the same query_id and params_hash SHALL return the same binary data. After a cache miss, the result SHALL be cached for future calls.

**Validates: Requirements 9.1, 9.5**

### Property 19: Cache Invalidation

*For any* DbTeleport cache entry, when a Postgres NOTIFY is received for a table that the query depends on, the cache entry SHALL be removed.

**Validates: Requirements 9.3**

### Property 20: Cache Access Latency

*For any* cached query result, `get_cached()` SHALL return within 0.1ms (100 microseconds).

**Validates: Requirements 9.4**

## Error Handling

### I/O Errors

| Error | Handling |
|-------|----------|
| `io::Error` from reactor creation | Return `Err` to caller, log error |
| Submission queue full | Return error count from `submit()` |
| Completion timeout | Return empty Vec from `wait()` |
| Invalid file descriptor | Return `Err` from `register()` |

### Protocol Errors

| Error | Handling |
|-------|----------|
| `HbtpError::InvalidHeader` | Return error response, close connection |
| `HbtpError::InvalidPayload` | Return error response with opcode 0x32 |
| `HbtpError::RouteNotFound` | Return 404-equivalent error response |
| `HbtpError::UnknownOpcode` | Log warning, ignore message |

### Middleware Errors

| Error | Handling |
|-------|----------|
| `MiddlewareError::Unauthorized` | Short-circuit, return 401 response |
| `MiddlewareError::RateLimited` | Short-circuit, return 429 response |
| JWT verification failure | Return `Err` from `before()` |

### Database Errors

| Error | Handling |
|-------|----------|
| Connection pool exhausted | Return `Err`, retry with backoff |
| Query execution failure | Return `Err`, do not cache |
| Serialization failure | Return `Err`, log error |

## Testing Strategy

### Unit Tests

Unit tests will verify specific examples and edge cases:

1. **Reactor Creation**: Test that each platform-specific reactor can be created with default config
2. **Header Parsing**: Test parsing valid and invalid byte slices
3. **Opcode Values**: Test that all opcodes have expected values
4. **Middleware Hooks**: Test individual middleware before/after behavior
5. **Buffer Operations**: Test TeleportBuffer write/read for various types

### Property-Based Tests

Property-based tests will use the `proptest` crate to verify universal properties:

1. **Round-trip properties**: Teleportation write/read, header serialize/deserialize
2. **Invariant properties**: Header size, opcode uniqueness, worker count
3. **Ordering properties**: Middleware execution order
4. **Isolation properties**: Thread-local rate limiting

### Integration Tests

1. **Cross-platform I/O**: Test reactor operations on each supported platform
2. **End-to-end HBTP**: Test full request/response cycle
3. **Cache invalidation**: Test Postgres NOTIFY triggers cache removal

### Performance Benchmarks

1. **RPS benchmarks**: Measure requests per second in HTTP and HBTP modes
2. **Latency benchmarks**: Measure p50, p95, p99 latencies
3. **Memory benchmarks**: Measure memory usage under load
4. **Cache access benchmarks**: Measure get_cached() latency

### Test Configuration

- Property tests: Minimum 100 iterations per property
- Benchmarks: Warmup period + 10 second measurement window
- Integration tests: Use test containers for Postgres
