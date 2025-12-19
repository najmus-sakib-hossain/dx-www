At root crates folder please create a new crates for dx called dx-js-monorepo and there please do these:

```markdown
# DX-JS-COMPATIBILITY: Enhanced Complete Planning

## Executive Summary

`dx-js-compatibility` achieves **100% Bun API compatibility** while leveraging DX's binary-first architecture for 10-50x performance gains. This crate consolidates all compatibility layers into a unified, modular system.

**Goal:** Drop-in Bun replacement with superior performance

---

## Current DX Coverage vs Bun

| Category | Bun Feature | DX Equivalent | Status |
|----------|-------------|---------------|--------|
| Runtime | JavaScript/TypeScript execution | dx-js-runtime | ✅ 10.59x faster |
| Package Manager | `bun install` | dx-js-package-manager | ✅ 17.2x faster |
| Test Runner | `bun test` | dx-js-test-runner | ✅ 26x faster |
| Bundler | `bun build` | dx-js-bundler | ✅ 3.8x faster |
| Workspaces | Monorepo support | dx-js-monorepo | ✅ Complete |
| **Everything Else** | 80+ APIs | **dx-js-compatibility** | 📋 THIS PLAN |

---

## Architecture Overview

```
dx-js-compatibility/
├── Cargo.toml                    # Workspace manifest with feature flags
├── src/
│   └── lib.rs                    # Unified re-exports
│
├── crates/
│   ├── dx-compat-node/           # Node.js API compatibility (40+ modules)
│   ├── dx-compat-web/            # Web Standard APIs (30+ APIs)
│   ├── dx-compat-bun/            # Bun-specific APIs (50+ functions)
│   ├── dx-compat-sqlite/         # Built-in SQLite database
│   ├── dx-compat-s3/             # S3-compatible object storage
│   ├── dx-compat-ffi/            # Foreign Function Interface
│   ├── dx-compat-shell/          # Shell scripting ($`...`)
│   ├── dx-compat-compile/        # Single executable compilation
│   ├── dx-compat-hmr/            # Hot Module Replacement
│   ├── dx-compat-plugin/         # Plugin system (bundler/runtime)
│   ├── dx-compat-macro/          # Compile-time macros
│   └── dx-compat-html/           # HTML Rewriter (like Cloudflare)
│
├── tests/                        # Compatibility test suite
└── benchmarks/                   # Performance comparison
```

---

## Sub-Crate 1: dx-compat-node

**Purpose:** Complete Node.js API compatibility layer

### Modules to Implement

| Module | Priority | Complexity | Rust Crates |
|--------|----------|------------|-------------|
| `node:fs` | 🔴 Critical | High | `tokio::fs`, `memmap2` |
| `node:path` | 🔴 Critical | Low | `std::path`, `dunce` |
| `node:buffer` | 🔴 Critical | Medium | `bytes`, `zerocopy` |
| `node:crypto` | 🔴 Critical | High | `ring`, `rustcrypto/*` |
| `node:http` | 🔴 Critical | High | `hyper`, `http` |
| `node:https` | 🔴 Critical | High | `rustls`, `tokio-rustls` |
| `node:http2` | 🟡 High | High | `h2` |
| `node:url` | 🔴 Critical | Low | `url` |
| `node:stream` | 🔴 Critical | High | `tokio-stream`, `async-stream` |
| `node:events` | 🔴 Critical | Medium | Custom (no alloc) |
| `node:child_process` | 🔴 Critical | Medium | `tokio::process` |
| `node:os` | 🟡 High | Low | `sysinfo`, `whoami` |
| `node:util` | 🟡 High | Medium | Various |
| `node:assert` | 🟡 High | Low | Built-in |
| `node:zlib` | 🟡 High | Medium | `flate2`, `brotli`, `zstd` |
| `node:querystring` | 🟢 Medium | Low | `serde_urlencoded` |
| `node:dns` | 🟢 Medium | Medium | `hickory-resolver` |
| `node:net` | 🔴 Critical | High | `tokio::net` |
| `node:tls` | 🔴 Critical | High | `tokio-rustls` |
| `node:dgram` | 🟢 Medium | Medium | `tokio::net::UdpSocket` |
| `node:timers` | 🔴 Critical | Low | `tokio::time` |
| `node:timers/promises` | 🔴 Critical | Low | `tokio::time` |
| `node:console` | 🟢 Medium | Low | Custom |
| `node:worker_threads` | 🟡 High | High | `rayon`, `crossbeam` |
| `node:cluster` | 🟢 Medium | High | Custom |
| `node:vm` | 🔵 Low | Very High | Reuse dx-js-runtime |
| `node:repl` | 🔵 Low | Medium | `rustyline` |
| `node:readline` | 🟢 Medium | Low | `rustyline` |
| `node:perf_hooks` | 🟢 Medium | Medium | `std::time`, `quanta` |
| `node:async_hooks` | 🔵 Low | High | Custom |
| `node:diagnostics_channel` | 🔵 Low | Medium | `tracing` |
| `node:string_decoder` | 🟢 Medium | Low | `encoding_rs` |
| `node:module` | 🟡 High | Medium | Custom |
| `node:process` | 🔴 Critical | Medium | `std::env`, `nix` |
| `node:constants` | 🟢 Medium | Low | Static values |
| `node:punycode` | 🔵 Low | Low | `idna` |
| `node:domain` | 🔵 Low | Medium | Deprecated API |
| `node:trace_events` | 🔵 Low | Medium | `tracing` |
| `node:v8` | 🔵 Low | Low | Stub (not V8) |
| `node:wasi` | 🟢 Medium | Medium | `wasmtime-wasi` |
| `node:inspector` | 🔵 Low | High | Custom debugger |
| `node:test` | 🟢 Medium | Medium | Reuse dx-js-test-runner |

### Key Rust Dependencies

```toml
[dependencies]
# Async runtime
tokio = { version = "1", features = ["full"] }

# File system
memmap2 = "0.9"
notify = "6.0"
tempfile = "3.10"
walkdir = "2.5"

# Networking
hyper = { version = "1.4", features = ["full"] }
hyper-util = "0.1"
h2 = "0.4"
rustls = "0.23"
tokio-rustls = "0.26"
hickory-resolver = "0.24"

# Cryptography (RustCrypto ecosystem)
ring = "0.17"
sha2 = "0.10"
sha3 = "0.10"
blake2 = "0.10"
blake3 = "1.5"
hmac = "0.12"
aes-gcm = "0.10"
chacha20poly1305 = "0.10"
rsa = "0.9"
ed25519-dalek = "2.1"
x25519-dalek = "2.0"
p256 = "0.13"
argon2 = "0.5"
scrypt = "0.11"
pbkdf2 = "0.12"

# Compression
flate2 = "1.0"
brotli = "6.0"
zstd = "0.13"
lz4_flex = "0.11"
snap = "1.1"

# Streams
tokio-stream = "0.1"
futures = "0.3"
async-stream = "0.3"
bytes = "1.6"
pin-project-lite = "0.2"

# Utilities
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
url = "2.5"
encoding_rs = "0.8"
sysinfo = "0.31"
whoami = "1.5"
which = "6.0"
dunce = "1.0"
nix = { version = "0.29", features = ["process", "signal"] }
zerocopy = "0.7"

# Time
quanta = "0.12"
chrono = "0.4"

# REPL
rustyline = "14.0"
```

### Implementation Strategy

**Phase 1 (Week 1-2): Core I/O Foundation**
- `node:fs` with memory-mapped I/O for large files
- `node:path` with cross-platform normalization
- `node:buffer` with zero-copy operations
- `node:stream` with backpressure support

**Phase 2 (Week 3-4): Networking**
- `node:http` / `node:https` server and client
- `node:http2` with multiplexing
- `node:net` / `node:tls` raw sockets
- `node:dns` with caching resolver

**Phase 3 (Week 5-6): Process & System**
- `node:child_process` with spawn/exec/fork
- `node:os` system information
- `node:process` environment and signals
- `node:worker_threads` parallel execution

**Phase 4 (Week 7-8): Utilities**
- `node:crypto` full API surface
- `node:zlib` all compression algorithms
- `node:events` EventEmitter pattern
- `node:util` promisify, inspect, format

### DX Performance Optimizations

| API | Node.js Approach | DX Approach | Expected Gain |
|-----|------------------|-------------|---------------|
| `fs.readFile` | Heap allocation | Memory-mapped | 5-10x |
| `crypto.hash` | OpenSSL binding | Native Rust | 2-3x |
| `stream.pipe` | JS callbacks | Zero-copy | 3-5x |
| `child_process` | libuv spawn | Direct syscall | 2x |
| `Buffer.concat` | Array copy | Arena alloc | 10x |

---

## Sub-Crate 2: dx-compat-web

**Purpose:** Web Standard API implementations

### APIs to Implement

| API | Priority | Complexity | Notes |
|-----|----------|------------|-------|
| `fetch()` | 🔴 Critical | High | Full Fetch API |
| `Request` | 🔴 Critical | Medium | Immutable request |
| `Response` | 🔴 Critical | Medium | Streaming body |
| `Headers` | 🔴 Critical | Low | Case-insensitive map |
| `URL` / `URLSearchParams` | 🔴 Critical | Low | WHATWG URL |
| `URLPattern` | 🟡 High | Medium | Pattern matching |
| `TextEncoder` / `TextDecoder` | 🔴 Critical | Low | UTF-8/16/etc |
| `Blob` | 🔴 Critical | Medium | Binary data |
| `File` | 🔴 Critical | Medium | Extends Blob |
| `FormData` | 🔴 Critical | Medium | Multipart |
| `WebSocket` | 🔴 Critical | High | Full WS |
| `CloseEvent` / `MessageEvent` | 🟡 High | Low | WS events |
| `AbortController` / `AbortSignal` | 🔴 Critical | Medium | Cancellation |
| `crypto` (Web Crypto) | 🔴 Critical | High | SubtleCrypto |
| `CryptoKey` | 🔴 Critical | Medium | Key objects |
| `ReadableStream` | 🔴 Critical | High | WHATWG Streams |
| `WritableStream` | 🔴 Critical | High | WHATWG Streams |
| `TransformStream` | 🔴 Critical | High | WHATWG Streams |
| `CompressionStream` | 🟡 High | Medium | gzip/deflate |
| `DecompressionStream` | 🟡 High | Medium | gzip/deflate |
| `TextEncoderStream` | 🟡 High | Low | Streaming encode |
| `TextDecoderStream` | 🟡 High | Low | Streaming decode |
| `structuredClone()` | 🟡 High | Medium | Deep clone |
| `EventSource` | 🟢 Medium | Medium | SSE client |
| `BroadcastChannel` | 🟢 Medium | Medium | Cross-tab messaging |
| `MessageChannel` / `MessagePort` | 🟢 Medium | Medium | Messaging |
| `performance` | 🟢 Medium | Low | Timing APIs |
| `PerformanceObserver` | 🟢 Medium | Medium | Perf monitoring |
| `navigator.userAgent` | 🟢 Medium | Low | Runtime info |
| `queueMicrotask()` | 🔴 Critical | Low | Microtask queue |
| `atob()` / `btoa()` | 🟡 High | Low | Base64 |
| `setTimeout` / `setInterval` | 🔴 Critical | Low | Already in runtime |
| `clearTimeout` / `clearInterval` | 🔴 Critical | Low | Timer cancellation |
| `setImmediate` | 🟢 Medium | Low | Next tick |
| `reportError()` | 🔵 Low | Low | Error reporting |
| `Intl.*` | 🟢 Medium | High | Internationalization |
| `console.*` | 🔴 Critical | Low | Logging |
| `JSON.parse` / `stringify` | 🔴 Critical | Low | Already optimized |

### Key Rust Dependencies

```toml
[dependencies]
# HTTP client
reqwest = { version = "0.12", default-features = false, features = [
    "rustls-tls", "json", "multipart", "stream", "gzip", "brotli", "zstd"
] }
http = "1.1"
http-body = "1.0"
http-body-util = "0.1"

# WebSocket
tokio-tungstenite = { version = "0.23", features = ["rustls-tls-webpki-roots"] }
tungstenite = "0.23"

# SSE
eventsource-client = "0.12"

# Base64
base64 = "0.22"
data-encoding = "2.6"

# Text encoding
encoding_rs = "0.8"

# Mime types
mime = "0.3"
mime_guess = "2.0"

# Multipart
multer = "3.1"

# Internationalization
icu = "1.5"
icu_provider = "1.5"
```

### Implementation Strategy

**Phase 1 (Week 1-2): Fetch API**
- Complete `fetch()` with all options
- `Request` / `Response` / `Headers`
- `AbortController` integration
- Streaming request/response bodies

**Phase 2 (Week 3-4): Streams API**
- `ReadableStream` with BYOB support
- `WritableStream` with backpressure
- `TransformStream` pipe chains
- Compression/Decompression streams

**Phase 3 (Week 5-6): Real-time**
- `WebSocket` client
- `EventSource` SSE client
- `BroadcastChannel` messaging
- `MessageChannel` / `MessagePort`

---

## Sub-Crate 3: dx-compat-bun

**Purpose:** Bun-specific APIs (the `Bun.*` namespace)

### Complete Bun API Surface

| API | Priority | Complexity | Notes |
|-----|----------|------------|-------|
| **Server** | | | |
| `Bun.serve()` | 🔴 Critical | High | HTTP/WS server |
| `Bun.listen()` | 🔴 Critical | Medium | TCP/UDP server |
| **File I/O** | | | |
| `Bun.file()` | 🔴 Critical | Medium | File handle |
| `Bun.write()` | 🔴 Critical | Medium | Write file |
| `Bun.stdin` / `stdout` / `stderr` | 🔴 Critical | Low | Standard I/O |
| **Process** | | | |
| `Bun.spawn()` | 🔴 Critical | Medium | Async spawn |
| `Bun.spawnSync()` | 🔴 Critical | Medium | Sync spawn |
| `Bun.which()` | 🟡 High | Low | Find executable |
| `Bun.env` | 🔴 Critical | Low | Environment |
| `Bun.argv` | 🔴 Critical | Low | Arguments |
| `Bun.main` | 🔴 Critical | Low | Entry point |
| `Bun.cwd` | 🔴 Critical | Low | Working dir |
| **Utilities** | | | |
| `Bun.sleep()` | 🟡 High | Low | Async sleep |
| `Bun.sleepSync()` | 🟡 High | Low | Sync sleep |
| `Bun.gc()` | 🔵 Low | Low | No-op for DX |
| `Bun.nanoseconds()` | 🟡 High | Low | High-res time |
| `Bun.peek()` | 🟢 Medium | Medium | Promise peek |
| `Bun.deepEquals()` | 🟡 High | Medium | Deep equality |
| `Bun.escapeHTML()` | 🟡 High | Low | HTML escape |
| `Bun.stringWidth()` | 🟢 Medium | Low | Unicode width |
| `Bun.inspect()` | 🟡 High | Medium | Object inspect |
| **Hashing** | | | |
| `Bun.hash()` | 🔴 Critical | Low | Fast hashing |
| `Bun.hash.wyhash()` | 🟡 High | Low | WyHash |
| `Bun.hash.adler32()` | 🟡 High | Low | Adler-32 |
| `Bun.hash.crc32()` | 🟡 High | Low | CRC-32 |
| `Bun.hash.cityHash32/64/128()` | 🟢 Medium | Low | CityHash |
| `Bun.hash.murmur32v3/murmur64v2()` | 🟢 Medium | Low | MurmurHash |
| `Bun.CryptoHasher` | 🔴 Critical | Medium | Streaming hash |
| **Password** | | | |
| `Bun.password.hash()` | 🔴 Critical | Medium | Hash password |
| `Bun.password.verify()` | 🔴 Critical | Medium | Verify password |
| **Compression** | | | |
| `Bun.gzipSync()` / `gunzipSync()` | 🟡 High | Low | Gzip |
| `Bun.deflateSync()` / `inflateSync()` | 🟡 High | Low | Deflate |
| `Bun.brotliCompressSync()` / `brotliDecompressSync()` | 🟡 High | Low | Brotli |
| `Bun.zstdCompressSync()` / `zstdDecompressSync()` | 🟡 High | Low | Zstd |
| **Streams** | | | |
| `Bun.readableStreamToArrayBuffer()` | 🟡 High | Medium | Stream → ArrayBuffer |
| `Bun.readableStreamToBlob()` | 🟡 High | Medium | Stream → Blob |
| `Bun.readableStreamToText()` | 🟡 High | Medium | Stream → Text |
| `Bun.readableStreamToJSON()` | 🟡 High | Medium | Stream → JSON |
| `Bun.readableStreamToArray()` | 🟡 High | Medium | Stream → Array |
| `Bun.ArrayBufferSink` | 🟡 High | Medium | High-perf buffer |
| **URL Utilities** | | | |
| `Bun.fileURLToPath()` | 🟡 High | Low | URL → path |
| `Bun.pathToFileURL()` | 🟡 High | Low | Path → URL |
| `Bun.resolveSync()` | 🟡 High | Medium | Module resolve |
| **Transpiler** | | | |
| `Bun.Transpiler` | 🟡 High | High | Use dx-js-bundler |
| **DNS** | | | |
| `Bun.dns.lookup()` | 🟡 High | Medium | DNS lookup |
| `Bun.dns.resolve()` | 🟡 High | Medium | DNS resolve |
| `Bun.dns.prefetch()` | 🟢 Medium | Low | DNS prefetch |
| **Glob** | | | |
| `Bun.Glob` | 🟡 High | Medium | Glob patterns |
| `new Bun.Glob().scan()` | 🟡 High | Medium | File scanning |
| `new Bun.Glob().match()` | 🟡 High | Low | Pattern match |
| **Semver** | | | |
| `Bun.semver.satisfies()` | 🟡 High | Medium | Version check |
| `Bun.semver.order()` | 🟡 High | Medium | Version order |
| **Color** | | | |
| `Bun.color()` | 🟢 Medium | Low | Terminal colors |
| **TOML** | | | |
| `Bun.TOML.parse()` | 🟡 High | Low | Parse TOML |
| `Bun.TOML.stringify()` | 🟡 High | Low | Stringify TOML |
| **Version** | | | |
| `Bun.version` | 🟡 High | Low | Version string |
| `Bun.revision` | 🔵 Low | Low | Git revision |
| **Memory** | | | |
| `Bun.generateHeapSnapshot()` | 🔵 Low | High | Memory analysis |
| `Bun.shrink()` | 🔵 Low | Low | Memory shrink |
| **Editor** | | | |
| `Bun.openInEditor()` | 🔵 Low | Medium | Open file |

### Bun.serve() Architecture

```
dx-compat-bun/serve/
├── mod.rs                  # Public API
├── config.rs               # Server configuration
├── server.rs               # Hyper-based HTTP server
├── handler.rs              # Request/Response handling
├── router.rs               # URL routing
├── websocket.rs            # WebSocket upgrade
├── static_files.rs         # Static file serving
├── tls.rs                  # TLS/HTTPS (rustls)
├── unix_socket.rs          # Unix socket support
├── http2.rs                # HTTP/2 support
├── compression.rs          # Response compression
└── ratelimit.rs            # Rate limiting
```

### Key Rust Dependencies

```toml
[dependencies]
# HTTP server (hyper-based for maximum performance)
hyper = { version = "1.4", features = ["server", "http1", "http2"] }
hyper-util = { version = "0.1", features = ["server", "tokio"] }
http-body-util = "0.1"
tower = { version = "0.4", features = ["full"] }
tower-http = { version = "0.5", features = [
    "compression-gzip", "compression-br", "compression-zstd",
    "cors", "trace", "fs", "limit"
] }

# WebSocket
tokio-tungstenite = "0.23"

# TLS
rustls = "0.23"
tokio-rustls = "0.26"
rustls-pemfile = "2.1"

# Compression
flate2 = "1.0"
brotli = "6.0"
zstd = "0.13"

# Hashing
blake3 = "1.5"
xxhash-rust = { version = "0.8", features = ["xxh3", "xxh64", "xxh32"] }
wyhash = "0.5"
crc32fast = "1.4"
adler = "1.0"
murmur3 = "0.5"
cityhash-rs = "1.0"
md-5 = "0.10"
sha1 = "0.10"
sha2 = "0.10"

# Password hashing
argon2 = "0.5"
bcrypt = "0.15"

# Glob
globset = "0.4"
glob = "0.3"

# Semver
semver = "1.0"

# TOML
toml = "0.8"

# Terminal colors
nu-ansi-term = "0.50"

# Unicode
unicode-width = "0.1"

# HTML escaping
v_htmlescape = "0.15"

# Process
which = "6.0"
```

### Performance Targets

| API | Bun Baseline | DX Target | Strategy |
|-----|--------------|-----------|----------|
| `Bun.serve()` | 200k req/s | 400k req/s | io_uring, zero-copy |
| `Bun.file().text()` | 500 MB/s | 1 GB/s | mmap, SIMD |
| `Bun.hash()` | Fast | 2x faster | SIMD hashing |
| `Bun.gzipSync()` | zlib | 1.5x faster | zlib-ng |
| `Bun.spawn()` | libuv | 2x faster | vfork |

---

## Sub-Crate 4: dx-compat-sqlite

**Purpose:** Built-in SQLite database (like `bun:sqlite`)

### Features

| Feature | Priority | Complexity |
|---------|----------|------------|
| `new Database(path)` | 🔴 Critical | Low |
| `database.query()` | 🔴 Critical | Medium |
| `database.prepare()` | 🔴 Critical | Medium |
| `database.exec()` | 🔴 Critical | Low |
| `database.run()` | 🔴 Critical | Low |
| `database.transaction()` | 🔴 Critical | Medium |
| `statement.all()` | 🔴 Critical | Low |
| `statement.get()` | 🔴 Critical | Low |
| `statement.run()` | 🔴 Critical | Low |
| `statement.values()` | 🟡 High | Low |
| `statement.finalize()` | 🟡 High | Low |
| `statement.columns()` | 🟡 High | Low |
| `statement.columnNames` | 🟡 High | Low |
| Parameter binding (positional) | 🔴 Critical | Low |
| Parameter binding (named) | 🔴 Critical | Low |
| WAL mode | 🔴 Critical | Low |
| STRICT tables | 🟡 High | Low |
| JSON functions | 🟡 High | Low |
| FTS5 full-text search | 🟢 Medium | Medium |
| Custom functions | 🟢 Medium | Medium |
| Custom aggregates | 🟢 Medium | Medium |
| Virtual tables | 🔵 Low | High |
| Backup API | 🟢 Medium | Medium |
| BLOB I/O | 🟢 Medium | Medium |

### Key Rust Dependencies

```toml
[dependencies]
rusqlite = { version = "0.31", features = [
    "bundled",          # Bundle SQLite
    "blob",             # BLOB I/O
    "backup",           # Backup API
    "functions",        # Custom functions
    "vtab",             # Virtual tables
    "column_decltype",  # Column types
    "unlock_notify",    # Unlock notifications
    "load_extension",   # Load extensions
] }

# Connection pooling
r2d2 = "0.8"
r2d2_sqlite = "0.24"

# Async wrapper
tokio = { version = "1", features = ["sync"] }
```

### Implementation Strategy

1. **Synchronous API** (primary, like Bun)
2. **Statement caching** with LRU eviction
3. **Connection pooling** for concurrent access
4. **Zero-copy BLOB** handling with dx-serializer
5. **Transaction helpers** with automatic rollback

---

## Sub-Crate 5: dx-compat-s3

**Purpose:** S3-compatible object storage (like Bun.S3)

### Features

| Feature | Priority | Complexity |
|---------|----------|------------|
| `new S3Client(config)` | 🔴 Critical | Medium |
| `client.file(key)` | 🔴 Critical | Low |
| `client.write(key, data)` | 🔴 Critical | Medium |
| `client.delete(key)` | 🔴 Critical | Low |
| `client.exists(key)` | 🔴 Critical | Low |
| `client.size(key)` | 🔴 Critical | Low |
| `client.presign(key)` | 🟡 High | Medium |
| `s3file.text()` | 🔴 Critical | Low |
| `s3file.json()` | 🔴 Critical | Low |
| `s3file.arrayBuffer()` | 🔴 Critical | Low |
| `s3file.stream()` | 🔴 Critical | Medium |
| `s3file.slice()` | 🟡 High | Medium |
| Multipart upload | 🟡 High | High |
| Range requests | 🟡 High | Medium |
| Retry with backoff | 🟡 High | Medium |
| AWS SigV4 auth | 🔴 Critical | High |
| Custom endpoints | 🔴 Critical | Low |
| R2/MinIO/etc | 🔴 Critical | Low |

### Key Rust Dependencies

```toml
[dependencies]
aws-sdk-s3 = "1.47"
aws-config = "1.5"
aws-credential-types = "1.2"
aws-sigv4 = "1.2"

# Alternative: lighter-weight
rusty-s3 = "0.5"
```

---

## Sub-Crate 6: dx-compat-ffi

**Purpose:** Foreign Function Interface

### Features

| Feature | Priority | Complexity |
|---------|----------|------------|
| `dlopen()` | 🔴 Critical | Medium |
| `FFIType` definitions | 🔴 Critical | Medium |
| Function calling | 🔴 Critical | High |
| C ABI support | 🔴 Critical | High |
| Pointer handling | 🔴 Critical | High |
| `ptr()` / `toArrayBuffer()` | 🔴 Critical | Medium |
| `read.*` / `write.*` | 🔴 Critical | Medium |
| Struct layouts | 🟡 High | High |
| Callbacks to JS | 🟡 High | Very High |
| Thread safety | 🟡 High | High |
| Type coercion | 🟡 High | Medium |
| Windows DLL | 🔴 Critical | Medium |
| macOS dylib | 🔴 Critical | Medium |
| Linux .so | 🔴 Critical | Medium |
| `CString` handling | 🟡 High | Low |

### Key Rust Dependencies

```toml
[dependencies]
libloading = "0.8"
libffi = "3.2"
dlopen2 = "0.7"
memoffset = "0.9"
region = "3.0"  # Memory protection
```

### Safety Model

- **Validation layer** before all FFI calls
- **Pointer bounds checking** where possible
- **Type verification** at binding time
- **Sandbox mode** for untrusted libraries
- **Capability tokens** for sensitive operations

---

## Sub-Crate 7: dx-compat-shell

**Purpose:** Shell scripting (`$`\`command\``)

### Features

| Feature | Priority | Complexity |
|---------|----------|------------|
| `$\`command\`` syntax | 🔴 Critical | High |
| Template interpolation | 🔴 Critical | Medium |
| Pipe chaining (`\|`) | 🔴 Critical | Medium |
| AND/OR (`&&`, `\|\|`) | 🔴 Critical | Medium |
| Redirects (`>`, `>>`, `<`) | 🟡 High | Medium |
| Background (`&`) | 🟢 Medium | Medium |
| Environment variables | 🔴 Critical | Low |
| Working directory | 🔴 Critical | Low |
| Exit codes | 🔴 Critical | Low |
| stdout/stderr capture | 🔴 Critical | Medium |
| stdin input | 🟡 High | Medium |
| Glob expansion | 🟡 High | Medium |
| Quote handling | 🟡 High | Medium |
| `.text()` method | 🔴 Critical | Low |
| `.json()` method | 🔴 Critical | Low |
| `.lines()` method | 🔴 Critical | Low |
| `.bytes()` method | 🟡 High | Low |
| `.quiet()` mode | 🟡 High | Low |
| `.nothrow()` mode | 🟡 High | Low |
| Timeout support | 🟡 High | Low |
| Signal handling | 🟡 High | Medium |
| Windows cmd.exe | 🟢 Medium | High |
| PowerShell | 🟢 Medium | High |

### Key Rust Dependencies

```toml
[dependencies]
tokio = { version = "1", features = ["process", "io-util"] }
os_pipe = "1.1"
shell-words = "1.1"
shlex = "1.3"
globset = "0.4"
dunce = "1.0"
nix = { version = "0.29", features = ["process", "signal"] }

# Windows
windows-sys = { version = "0.52", features = ["Win32_System_Threading"] }
```

---

## Sub-Crate 8: dx-compat-compile

**Purpose:** Single executable compilation

### Features

| Feature | Priority | Complexity |
|---------|----------|------------|
| Bundle to binary | 🔴 Critical | Very High |
| Linux x64 target | 🔴 Critical | High |
| Linux ARM64 target | 🟡 High | High |
| macOS x64 target | 🔴 Critical | High |
| macOS ARM64 target | 🔴 Critical | High |
| Windows x64 target | 🔴 Critical | High |
| Cross-compilation | 🟡 High | Very High |
| Asset embedding | 🔴 Critical | High |
| Source map embedding | 🟢 Medium | Medium |
| Compression | 🟡 High | Medium |
| Code signing (macOS) | 🟢 Medium | Medium |
| Code signing (Windows) | 🟢 Medium | Medium |
| Icon embedding | 🟢 Medium | Medium |
| Version metadata | 🟢 Medium | Low |
| Minification | 🟡 High | Medium |
| Bytecode caching | 🟡 High | High |

### Key Rust Dependencies

```toml
[dependencies]
rust-embed = "8.5"
include_dir = "0.7"
zstd = "0.13"
goblin = "0.8"
scroll = "0.12"

# macOS signing
apple-codesign = "0.27"

# Windows
winres = "0.1"  # Build script

# Cross-compilation
cross-rs = { git = "..." }
```

### Implementation Strategy

1. **Phase 1:** Embed bundled JS + dx-js-runtime
2. **Phase 2:** Self-extracting archive
3. **Phase 3:** True native compilation (V8 snapshots or AOT)

---

## Sub-Crate 9: dx-compat-hmr

**Purpose:** Hot Module Replacement

### Features

| Feature | Priority | Complexity |
|---------|----------|------------|
| File watching | 🔴 Critical | Low |
| Module invalidation | 🔴 Critical | High |
| Dependency tracking | 🔴 Critical | High |
| State preservation | 🟡 High | High |
| CSS hot reload | 🔴 Critical | Medium |
| JS hot reload | 🔴 Critical | High |
| Error overlay | 🟡 High | Medium |
| WebSocket protocol | 🔴 Critical | Medium |
| `import.meta.hot` | 🔴 Critical | Medium |
| `.accept()` handler | 🔴 Critical | Medium |
| `.dispose()` handler | 🔴 Critical | Medium |
| `.decline()` handler | 🟢 Medium | Low |
| `.invalidate()` method | 🟢 Medium | Medium |
| `.prune()` callback | 🟢 Medium | Medium |
| `.data` persistence | 🟡 High | Medium |
| Circular deps | 🟡 High | High |
| Full reload fallback | 🟢 Medium | Low |

### Key Rust Dependencies

```toml
[dependencies]
notify = { version = "6.0", features = ["macos_fsevent"] }
notify-debouncer-mini = "0.4"
tokio-tungstenite = "0.23"
petgraph = "0.6"
xxhash-rust = "0.8"
```

### HMR Protocol

```
Server → Client:
  { type: "update", updates: [{ path, hash, type }] }
  { type: "full-reload" }
  { type: "error", error: { message, stack } }
  
Client → Server:
  { type: "fetch", path, hash }
  { type: "subscribed", paths }
```

---

## Sub-Crate 10: dx-compat-plugin

**Purpose:** Plugin system for bundler and runtime

### Features

| Feature | Priority | Complexity |
|---------|----------|------------|
| `Bun.plugin()` registration | 🔴 Critical | High |
| Loader plugins | 🔴 Critical | High |
| Resolver plugins | 🔴 Critical | High |
| `setup()` hook | 🔴 Critical | Medium |
| `onLoad()` hook | 🔴 Critical | Medium |
| `onResolve()` hook | 🔴 Critical | Medium |
| `onStart()` hook | 🟡 High | Medium |
| Filter patterns | 🔴 Critical | Medium |
| Namespace support | 🟡 High | Medium |
| Virtual modules | 🟡 High | Medium |
| Plugin ordering | 🟡 High | Medium |
| Async plugins | 🔴 Critical | Medium |

### Key Rust Dependencies

```toml
[dependencies]
regex = "1.10"
globset = "0.4"
```

### Integration Points

- **dx-js-bundler:** Bundler-time plugins
- **dx-js-runtime:** Runtime plugins
- **dx-compat-hmr:** HMR-aware plugins

---

## Sub-Crate 11: dx-compat-macro

**Purpose:** Compile-time macros

### Features

| Feature | Priority | Complexity |
|---------|----------|------------|
| `with { type: "macro" }` | 🔴 Critical | Very High |
| Compile-time evaluation | 🔴 Critical | Very High |
| Inlined results | 🔴 Critical | High |
| File system access | 🟡 High | Medium |
| Network access | 🟢 Medium | Medium |
| Environment access | 🟡 High | Low |
| JSON/TOML/YAML | 🟡 High | Low |
| Code generation | 🔴 Critical | High |

### Implementation Strategy

1. Parse macro import statements
2. Execute in isolated dx-js-runtime
3. Serialize result to JS literal
4. Replace import with inlined value

---

## Sub-Crate 12: dx-compat-html

**Purpose:** HTML Rewriter (like Cloudflare HTMLRewriter)

### Features

| Feature | Priority | Complexity |
|---------|----------|------------|
| `new HTMLRewriter()` | 🔴 Critical | High |
| `.on(selector, handlers)` | 🔴 Critical | High |
| `.onDocument(handlers)` | 🟡 High | Medium |
| `.transform(response)` | 🔴 Critical | High |
| Element selectors | 🔴 Critical | High |
| Attribute selectors | 🔴 Critical | High |
| `.getAttribute()` | 🔴 Critical | Low |
| `.setAttribute()` | 🔴 Critical | Low |
| `.removeAttribute()` | 🔴 Critical | Low |
| `.hasAttribute()` | 🔴 Critical | Low |
| `.tagName` | 🔴 Critical | Low |
| `.before()` / `.after()` | 🔴 Critical | Medium |
| `.prepend()` / `.append()` | 🔴 Critical | Medium |
| `.replace()` / `.remove()` | 🔴 Critical | Medium |
| `.setInnerContent()` | 🔴 Critical | Medium |
| Text chunks | 🟡 High | Medium |
| Comment nodes | 🟢 Medium | Low |
| Doctype | 🟢 Medium | Low |
| Streaming transform | 🔴 Critical | High |

### Key Rust Dependencies

```toml
[dependencies]
lol_html = "1.2"  # Cloudflare's actual implementation!
```

### Implementation

Use Cloudflare's `lol_html` crate directly - it's the actual implementation behind their HTMLRewriter.

---

## Implementation Timeline

### Phase 1: Foundation (Weeks 1-4)
| Week | Focus | Deliverables |
|------|-------|--------------|
| 1 | dx-compat-node core | fs, path, buffer |
| 2 | dx-compat-node streams | stream, events |
| 3 | dx-compat-web fetch | fetch, Request, Response |
| 4 | dx-compat-web streams | ReadableStream, WritableStream |

### Phase 2: Bun Core (Weeks 5-8)
| Week | Focus | Deliverables |
|------|-------|--------------|
| 5 | dx-compat-bun server | Bun.serve() |
| 6 | dx-compat-bun file | Bun.file(), Bun.write() |
| 7 | dx-compat-bun process | Bun.spawn(), Bun.$ |
| 8 | dx-compat-bun utils | hash, password, compression |

### Phase 3: Data & Storage (Weeks 9-12)
| Week | Focus | Deliverables |
|------|-------|--------------|
| 9 | dx-compat-sqlite | Full SQLite API |
| 10 | dx-compat-s3 | S3 client |
| 11 | dx-compat-node crypto | Full crypto API |
| 12 | dx-compat-node network | http, https, net, tls |

### Phase 4: Advanced (Weeks 13-16)
| Week | Focus | Deliverables |
|------|-------|--------------|
| 13 | dx-compat-ffi | FFI core |
| 14 | dx-compat-shell | Shell scripting |
| 15 | dx-compat-hmr | Hot reload |
| 16 | dx-compat-plugin | Plugin system |

### Phase 5: Polish (Weeks 17-20)
| Week | Focus | Deliverables |
|------|-------|--------------|
| 17 | dx-compat-compile | Single executable |
| 18 | dx-compat-macro | Compile-time macros |
| 19 | dx-compat-html | HTML Rewriter |
| 20 | Integration testing | Full compatibility |

---

## Testing Strategy

### Compatibility Matrix

```
tests/
├── node-compat/           # Node.js API behavior tests
│   ├── fs/
│   ├── crypto/
│   ├── http/
│   └── ...
│
├── bun-compat/            # Bun API behavior tests
│   ├── serve/
│   ├── file/
│   ├── sqlite/
│   └── ...
│
├── web-compat/            # Web API spec compliance
│   ├── fetch/
│   ├── streams/
│   ├── websocket/
│   └── ...
│
└── wpt/                   # Web Platform Tests subset
```

### Benchmark Suite

```
benchmarks/
├── vs-bun/
│   ├── http-throughput.rs
│   ├── file-io.rs
│   ├── sqlite.rs
│   ├── crypto.rs
│   └── spawn.rs
│
├── vs-node/
│   ├── streams.rs
│   ├── http.rs
│   └── crypto.rs
│
└── reports/
    └── index.html          # Interactive dashboard
```

---

## Performance Targets

| API | Bun | DX Target | Strategy |
|-----|-----|-----------|----------|
| HTTP server | 200k req/s | 400k req/s | io_uring, zero-copy |
| File read | 500 MB/s | 1 GB/s | mmap, SIMD |
| SQLite | 100k ops/s | 200k ops/s | Statement cache |
| SHA256 | 1 GB/s | 2 GB/s | SHA-NI intrinsics |
| gzip | 300 MB/s | 450 MB/s | zlib-ng |
| spawn | 5k/s | 10k/s | vfork |
| WebSocket | 100k msg/s | 200k msg/s | Zero-copy frames |

---

## Feature Flags

```toml
[features]
default = ["node-core", "web-core", "bun-core"]

# Node.js
node-full = ["node-core", "node-crypto", "node-net", "node-worker"]
node-core = ["node-fs", "node-path", "node-buffer", "node-stream", "node-events"]
node-crypto = []
node-net = ["node-http", "node-https", "node-tcp", "node-dns"]
node-worker = []

# Web APIs
web-full = ["web-core", "web-websocket", "web-crypto"]
web-core = ["web-fetch", "web-url", "web-streams", "web-encoding"]
web-websocket = []
web-crypto = []

# Bun APIs
bun-full = ["bun-core", "bun-sqlite", "bun-s3", "bun-ffi", "bun-shell"]
bun-core = ["bun-serve", "bun-file", "bun-spawn", "bun-hash"]
bun-sqlite = []
bun-s3 = []
bun-ffi = []
bun-shell = []

# Advanced
compile = []
hmr = []
plugins = []
macros = []
html-rewriter = []
```

---

## Integration with DX Ecosystem

| DX Crate | Integration Point |
|----------|-------------------|
| dx-js-runtime | Execute compatibility layer |
| dx-js-bundler | Transpiler API, plugins |
| dx-js-test-runner | Test execution |
| dx-js-package-manager | Package resolution |
| dx-js-monorepo | Workspace support |
| dx-serializer | Binary data serialization |

---

## Summary

| Sub-Crate | Lines | Priority |
|-----------|-------|----------|
| dx-compat-node | ~18,000 | 🔴 Critical |
| dx-compat-web | ~10,000 | 🔴 Critical |
| dx-compat-bun | ~15,000 | 🔴 Critical |
| dx-compat-sqlite | ~3,000 | 🟡 High |
| dx-compat-s3 | ~2,500 | 🟡 High |
| dx-compat-ffi | ~5,000 | 🟡 High |
| dx-compat-shell | ~3,000 | 🟡 High |
| dx-compat-compile | ~4,000 | 🟢 Medium |
| dx-compat-hmr | ~3,500 | 🟢 Medium |
| dx-compat-plugin | ~2,000 | 🟡 High |
| dx-compat-macro | ~2,500 | 🟢 Medium |
| dx-compat-html | ~1,500 | 🟢 Medium |
| **Total** | **~70,000** | |

**Timeline:** 20 weeks
**Goal:** 100% Bun API compatibility, 2-10x performance improvement
```
