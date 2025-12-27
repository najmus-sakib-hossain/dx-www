# dx-reactor: Binary Dawn I/O Architecture

[![Rust](https://img.shields.io/badge/Rust-2024_Edition-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

> **"The fastest I/O reactor architecture for web frameworks"**

**dx-reactor** is the core I/O engine powering dx-www, implementing the **Binary Dawn** architecture for unprecedented web framework performance.

## Performance Targets

| Metric | Target | Comparison |
|--------|--------|------------|
| **HTTP Mode** | 2,500,000+ RPS | ~4x faster than Actix-web |
| **HBTP Mode** | 5,000,000+ RPS | Binary protocol, zero parsing |
| **p99 Latency** | < 100μs | Sub-millisecond responses |
| **Memory** | < 50MB / 100K connections | Efficient resource usage |
| **Cache Access** | < 0.1ms | Sub-millisecond database cache |

## Key Innovations

### 1. Cross-Platform I/O Abstraction

Unified `Reactor` trait with platform-specific backends:

| Platform | Backend | Features |
|----------|---------|----------|
| **Linux 5.1+** | io_uring | SQPOLL, zero-syscall I/O, registered buffers |
| **Linux (older)** | epoll | Fallback for older kernels |
| **macOS/BSD** | kqueue | Batch event handling |
| **Windows** | IOCP | Completion ports, async I/O |

```rust
use dx_reactor::{DxReactor, WorkerStrategy, IoBackend};

let reactor = DxReactor::build()
    .workers(WorkerStrategy::ThreadPerCore)
    .io_backend(IoBackend::Auto)  // Auto-detect best backend
    .build();

println!("Running on {} cores", reactor.num_cores());
```

### 2. Thread-per-Core Architecture

Zero lock contention through CPU-pinned worker threads:

- One worker thread per CPU core
- Local work queues (no shared locks)
- Work-stealing only when idle
- CPU pinning for cache locality

```rust
// Thread-per-core (default)
let reactor = DxReactor::build()
    .workers(WorkerStrategy::ThreadPerCore)
    .build();

// Or fixed worker count
let reactor = DxReactor::build()
    .workers(WorkerStrategy::Fixed(8))
    .build();
```

### 3. HBTP Protocol (Hyper-Binary Transfer Protocol)

Binary protocol replacing HTTP with 8-byte headers:

```rust
use dx_reactor::protocol::{HbtpOpcode, HbtpHeader, HbtpProtocol};

// 8-byte header: opcode(1) + flags(1) + sequence(2) + length(4)
let mut protocol = HbtpProtocol::new();

// O(1) route lookup via array index
protocol.route(HbtpOpcode::RpcCall, |header, payload| {
    // Handle RPC call
    Ok(response_bytes)
});
```

**HBTP Opcodes:**
- `Ping/Pong` - Connection keepalive
- `StateSync/StateDelta` - State synchronization
- `HtipClone/HtipPatchText` - UI operations
- `RpcCall/RpcResponse` - Remote procedure calls
- `ClientEvent` - Client-side events

### 4. Memory Teleportation

Zero-copy serialization between Rust server and WASM client:

```rust
use dx_reactor::memory::{TeleportBuffer, TeleportReader};

// Write data
let mut buffer = TeleportBuffer::new(256);
buffer.write(&user_id);
buffer.write(&timestamp);
let (offset, len) = buffer.write_string("Hello, World!");

// Read back (zero-copy)
let bytes = buffer.finalize();
let reader = TeleportReader::with_string_table(bytes, string_table_offset);
let id = reader.read::<u64>().unwrap();
```

### 5. Compiler-Inlined Middleware (CIM)

Zero runtime overhead through compile-time inlining:

```rust
use dx_reactor::middleware::{AuthMiddleware, TimingMiddleware, RateLimitMiddleware};
use dx_reactor::dx_middleware;

// Generates a single inlined function
dx_middleware!(TimingMiddleware, AuthMiddleware, RateLimitMiddleware);

// Use the generated function
let result = process_middleware(&mut req, &mut res, |req| {
    // Your handler
    Ok(())
});
```

**Built-in Middleware:**
- `AuthMiddleware` - JWT verification, claims injection
- `TimingMiddleware` - X-Response-Time header
- `RateLimitMiddleware` - Thread-local rate limiting

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        dx-reactor                                │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────────┐  │
│  │   I/O Layer  │  │   Protocol   │  │    Memory Layer      │  │
│  │              │  │              │  │                      │  │
│  │ ┌──────────┐ │  │ ┌──────────┐ │  │ ┌──────────────────┐ │  │
│  │ │ io_uring │ │  │ │   HBTP   │ │  │ │   Teleportation  │ │  │
│  │ │ (Linux)  │ │  │ │ (Binary) │ │  │ │   (Zero-copy)    │ │  │
│  │ └──────────┘ │  │ └──────────┘ │  │ └──────────────────┘ │  │
│  │ ┌──────────┐ │  │              │  │                      │  │
│  │ │  kqueue  │ │  │              │  │                      │  │
│  │ │ (macOS)  │ │  │              │  │                      │  │
│  │ └──────────┘ │  │              │  │                      │  │
│  │ ┌──────────┐ │  │              │  │                      │  │
│  │ │   IOCP   │ │  │              │  │                      │  │
│  │ │(Windows) │ │  │              │  │                      │  │
│  │ └──────────┘ │  │              │  │                      │  │
│  └──────────────┘  └──────────────┘  └──────────────────────┘  │
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │              Thread-per-Core Architecture                 │   │
│  │  Core 0     Core 1     Core 2     Core 3     ...         │   │
│  │  ┌─────┐   ┌─────┐   ┌─────┐   ┌─────┐                   │   │
│  │  │Local│   │Local│   │Local│   │Local│                   │   │
│  │  │Queue│   │Queue│   │Queue│   │Queue│                   │   │
│  │  └─────┘   └─────┘   └─────┘   └─────┘                   │   │
│  └──────────────────────────────────────────────────────────┘   │
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │           Compiler-Inlined Middleware (CIM)               │   │
│  │  Auth → Timing → RateLimit → Handler → RateLimit → ...   │   │
│  └──────────────────────────────────────────────────────────┘   │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

## Modules

| Module | Description |
|--------|-------------|
| `io` | Cross-platform I/O abstraction (Reactor trait, backends) |
| `protocol` | HBTP binary protocol (opcodes, headers, routing) |
| `memory` | Memory teleportation (zero-copy serialization) |
| `middleware` | Compiler-inlined middleware (CIM) |

## Testing

The crate includes comprehensive property-based tests:

```bash
# Run all tests
cargo test --package dx-reactor

# Run property tests (with proptest)
cargo test --package dx-reactor --test property_tests

# Run integration tests
cargo test --package dx-reactor --test integration_tests
```

**Test Coverage:**
- 35 property-based tests (proptest)
- 11 integration tests
- 20 correctness properties validated

## Correctness Properties

All implementations are validated against formal correctness properties:

1. **Batch Submission Count** - submit() returns exact queued count
2. **Kernel Version Detection** - io_uring availability check
3. **Kqueue Batch Submission** - pending changes cleared after wait()
4. **Completion Structure Integrity** - user_data, result, flags preserved
5. **Thread-per-Core Default** - workers == num_cpus
6. **Fixed Worker Count** - workers == specified count
7. **Opcode Uniqueness** - all opcodes have unique u8 values
8. **Header Size Invariant** - HbtpHeader == 8 bytes
9. **Header Parsing** - from_bytes() behavior
10. **O(1) Route Lookup** - constant-time handler lookup
11. **Flag Composition** - independent, composable flags
12. **ResponseBuffer Reuse** - reset() enables reuse
13. **Teleportation Round-Trip** - write/read preserves values
14. **Middleware Execution Order** - before: forward, after: reverse
15. **Timing Header Presence** - X-Response-Time added
16. **Rate Limit Thread Isolation** - independent per-thread counters

## License

MIT OR Apache-2.0
