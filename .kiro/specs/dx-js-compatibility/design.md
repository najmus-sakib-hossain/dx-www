# Design Document: dx-js-compatibility

## Overview

The `dx-js-compatibility` crate provides a comprehensive compatibility layer enabling 100% Bun API compatibility while leveraging DX's binary-first architecture for 10-50x performance gains. The design follows a modular sub-crate architecture with feature flags for selective inclusion, allowing developers to include only the compatibility layers they need.

The system is organized into 12 sub-crates, each responsible for a specific domain of compatibility:
- **dx-compat-node**: Node.js API compatibility (40+ modules)
- **dx-compat-web**: Web Standard APIs (30+ APIs)
- **dx-compat-bun**: Bun-specific APIs (50+ functions)
- **dx-compat-sqlite**: Built-in SQLite database
- **dx-compat-s3**: S3-compatible object storage
- **dx-compat-ffi**: Foreign Function Interface
- **dx-compat-shell**: Shell scripting
- **dx-compat-compile**: Single executable compilation
- **dx-compat-hmr**: Hot Module Replacement
- **dx-compat-plugin**: Plugin system
- **dx-compat-macro**: Compile-time macros
- **dx-compat-html**: HTML Rewriter

## Architecture

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                     dx-js-compatibility                              │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                    Unified Re-exports (lib.rs)               │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                │                                     │
│  ┌─────────────┬───────────────┼───────────────┬─────────────┐     │
│  │             │               │               │             │     │
│  ▼             ▼               ▼               ▼             ▼     │
│ ┌───────┐ ┌───────┐ ┌───────┐ ┌───────┐ ┌───────┐ ┌───────┐       │
│ │ node  │ │  web  │ │  bun  │ │sqlite │ │  s3   │ │  ffi  │       │
│ └───────┘ └───────┘ └───────┘ └───────┘ └───────┘ └───────┘       │
│ ┌───────┐ ┌───────┐ ┌───────┐ ┌───────┐ ┌───────┐ ┌───────┐       │
│ │ shell │ │compile│ │  hmr  │ │plugin │ │ macro │ │ html  │       │
│ └───────┘ └───────┘ └───────┘ └───────┘ └───────┘ └───────┘       │
└─────────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────────┐
│                      DX Ecosystem Integration                        │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐                │
│  │dx-js-runtime │ │dx-js-bundler │ │dx-serializer │                │
│  └──────────────┘ └──────────────┘ └──────────────┘                │
└─────────────────────────────────────────────────────────────────────┘
```

### Directory Structure

```
crates/dx-js-compatibility/
├── Cargo.toml                    # Workspace manifest with feature flags
├── src/
│   └── lib.rs                    # Unified re-exports
│
├── crates/
│   ├── dx-compat-node/           # Node.js API compatibility
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── fs/               # node:fs module
│   │       ├── path/             # node:path module
│   │       ├── buffer/           # node:buffer module
│   │       ├── stream/           # node:stream module
│   │       ├── events/           # node:events module
│   │       ├── http/             # node:http module
│   │       ├── crypto/           # node:crypto module
│   │       ├── child_process/    # node:child_process module
│   │       └── ...
│   │
│   ├── dx-compat-web/            # Web Standard APIs
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── fetch/            # Fetch API
│   │       ├── streams/          # WHATWG Streams
│   │       ├── websocket/        # WebSocket API
│   │       ├── url/              # URL API
│   │       └── ...
│   │
│   ├── dx-compat-bun/            # Bun-specific APIs
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── serve/            # Bun.serve()
│   │       ├── file/             # Bun.file()
│   │       ├── spawn/            # Bun.spawn()
│   │       ├── hash/             # Bun.hash()
│   │       └── ...
│   │
│   └── ... (other sub-crates)
│
├── tests/                        # Compatibility test suite
└── benchmarks/                   # Performance comparison
```


## Components and Interfaces

### 1. dx-compat-node Component

The Node.js compatibility layer provides implementations for core Node.js modules.

#### File System Module (node:fs)

```rust
/// File system operations with memory-mapped I/O optimization
pub mod fs {
    use std::path::Path;
    use bytes::Bytes;
    use tokio::fs as async_fs;
    use memmap2::Mmap;

    /// Read file contents, using mmap for large files (>1MB)
    pub async fn read_file(path: impl AsRef<Path>) -> Result<Bytes, FsError> {
        let metadata = async_fs::metadata(path.as_ref()).await?;
        if metadata.len() > 1_048_576 {
            // Use memory-mapped I/O for large files
            read_file_mmap(path).await
        } else {
            // Use standard read for small files
            Ok(Bytes::from(async_fs::read(path).await?))
        }
    }

    /// Write data to file
    pub async fn write_file(
        path: impl AsRef<Path>,
        data: impl AsRef<[u8]>,
    ) -> Result<(), FsError>;

    /// Read directory contents
    pub async fn read_dir(path: impl AsRef<Path>) -> Result<Vec<DirEntry>, FsError>;

    /// Get file/directory metadata
    pub async fn stat(path: impl AsRef<Path>) -> Result<Stats, FsError>;

    /// Watch for file changes
    pub fn watch(path: impl AsRef<Path>) -> Result<FsWatcher, FsError>;
}

/// File statistics matching Node.js fs.Stats
pub struct Stats {
    pub size: u64,
    pub mtime: SystemTime,
    pub atime: SystemTime,
    pub ctime: SystemTime,
    pub is_file: bool,
    pub is_directory: bool,
    pub is_symlink: bool,
    pub mode: u32,
}
```

#### Path Module (node:path)

```rust
/// Cross-platform path manipulation
pub mod path {
    use std::path::{Path, PathBuf, MAIN_SEPARATOR};

    /// Platform-specific path separator
    pub const SEP: char = MAIN_SEPARATOR;

    /// Join path segments
    pub fn join(paths: &[&str]) -> PathBuf;

    /// Resolve to absolute path
    pub fn resolve(paths: &[&str]) -> PathBuf;

    /// Get directory name
    pub fn dirname(path: &str) -> &str;

    /// Get base name
    pub fn basename(path: &str, ext: Option<&str>) -> &str;

    /// Get extension
    pub fn extname(path: &str) -> &str;

    /// Normalize path (resolve . and ..)
    pub fn normalize(path: &str) -> PathBuf;

    /// Check if path is absolute
    pub fn is_absolute(path: &str) -> bool;
}
```

#### Buffer Module (node:buffer)

```rust
/// Binary data handling with zero-copy optimization
pub mod buffer {
    use bytes::{Bytes, BytesMut};
    use zerocopy::AsBytes;

    /// Buffer implementation compatible with Node.js Buffer
    #[derive(Clone)]
    pub struct Buffer {
        inner: Bytes,
    }

    impl Buffer {
        /// Allocate zero-filled buffer
        pub fn alloc(size: usize) -> Self;

        /// Create from string with encoding
        pub fn from_string(s: &str, encoding: Encoding) -> Self;

        /// Create from byte slice (zero-copy when possible)
        pub fn from_slice(data: &[u8]) -> Self;

        /// Convert to string with encoding
        pub fn to_string(&self, encoding: Encoding) -> String;

        /// Concatenate buffers using arena allocation
        pub fn concat(buffers: &[Buffer]) -> Self;

        /// Get length
        pub fn len(&self) -> usize;

        /// Slice buffer (zero-copy)
        pub fn slice(&self, start: usize, end: usize) -> Self;
    }

    #[derive(Clone, Copy)]
    pub enum Encoding {
        Utf8,
        Ascii,
        Base64,
        Hex,
        Latin1,
    }
}
```

#### Stream Module (node:stream)

```rust
/// Streaming data with backpressure support
pub mod stream {
    use tokio::sync::mpsc;
    use futures::Stream;

    /// Readable stream trait
    pub trait Readable: Stream<Item = Result<Bytes, StreamError>> {
        fn pause(&mut self);
        fn resume(&mut self);
        fn is_paused(&self) -> bool;
    }

    /// Writable stream trait
    pub trait Writable {
        fn write(&mut self, chunk: Bytes) -> Result<bool, StreamError>;
        fn end(&mut self) -> Result<(), StreamError>;
    }

    /// Transform stream for data transformation
    pub struct Transform<F> {
        transform_fn: F,
        readable: Box<dyn Readable>,
        writable: Box<dyn Writable>,
    }

    /// Pipe streams with zero-copy transfer
    pub async fn pipe<R: Readable, W: Writable>(
        source: R,
        dest: W,
    ) -> Result<(), StreamError>;
}
```

#### Events Module (node:events)

```rust
/// Event emitter pattern
pub mod events {
    use std::collections::HashMap;
    use std::sync::Arc;
    use parking_lot::RwLock;

    type Listener = Box<dyn Fn(&[JsValue]) + Send + Sync>;

    /// EventEmitter implementation
    pub struct EventEmitter {
        listeners: Arc<RwLock<HashMap<String, Vec<Listener>>>>,
        max_listeners: usize,
    }

    impl EventEmitter {
        pub fn new() -> Self;

        /// Register event listener
        pub fn on(&self, event: &str, listener: Listener);

        /// Register one-time listener
        pub fn once(&self, event: &str, listener: Listener);

        /// Emit event to all listeners
        pub fn emit(&self, event: &str, args: &[JsValue]) -> bool;

        /// Remove specific listener
        pub fn remove_listener(&self, event: &str, listener: &Listener);

        /// Remove all listeners for event
        pub fn remove_all_listeners(&self, event: Option<&str>);

        /// Set max listeners before warning
        pub fn set_max_listeners(&mut self, n: usize);
    }
}
```


### 2. dx-compat-web Component

The Web Standards compatibility layer implements WHATWG and W3C specifications.

#### Fetch API

```rust
/// Fetch API implementation
pub mod fetch {
    use reqwest::Client;
    use http::{Method, StatusCode};

    /// Global fetch function
    pub async fn fetch(
        input: RequestInput,
        init: Option<RequestInit>,
    ) -> Result<Response, FetchError>;

    /// Request object
    pub struct Request {
        pub method: Method,
        pub url: Url,
        pub headers: Headers,
        pub body: Option<Body>,
        pub signal: Option<AbortSignal>,
    }

    /// Response object
    pub struct Response {
        pub status: StatusCode,
        pub status_text: String,
        pub headers: Headers,
        body: Body,
    }

    impl Response {
        pub async fn text(&mut self) -> Result<String, FetchError>;
        pub async fn json<T: DeserializeOwned>(&mut self) -> Result<T, FetchError>;
        pub async fn array_buffer(&mut self) -> Result<Vec<u8>, FetchError>;
        pub async fn blob(&mut self) -> Result<Blob, FetchError>;
        pub fn body(&self) -> Option<ReadableStream>;
    }

    /// Headers with case-insensitive access
    pub struct Headers {
        inner: http::HeaderMap,
    }

    impl Headers {
        pub fn get(&self, name: &str) -> Option<&str>;
        pub fn set(&mut self, name: &str, value: &str);
        pub fn append(&mut self, name: &str, value: &str);
        pub fn delete(&mut self, name: &str);
        pub fn has(&self, name: &str) -> bool;
    }
}
```

#### WHATWG Streams

```rust
/// WHATWG Streams API
pub mod streams {
    /// ReadableStream implementation
    pub struct ReadableStream {
        source: Box<dyn UnderlyingSource>,
        state: StreamState,
    }

    impl ReadableStream {
        pub fn new(source: impl UnderlyingSource) -> Self;
        pub fn get_reader(&self) -> ReadableStreamReader;
        pub fn pipe_to(&self, dest: WritableStream) -> impl Future<Output = Result<(), Error>>;
        pub fn pipe_through(&self, transform: TransformStream) -> ReadableStream;
        pub fn tee(&self) -> (ReadableStream, ReadableStream);
    }

    /// WritableStream implementation
    pub struct WritableStream {
        sink: Box<dyn UnderlyingSink>,
        state: StreamState,
    }

    impl WritableStream {
        pub fn new(sink: impl UnderlyingSink) -> Self;
        pub fn get_writer(&self) -> WritableStreamWriter;
    }

    /// TransformStream for data transformation
    pub struct TransformStream {
        pub readable: ReadableStream,
        pub writable: WritableStream,
    }

    /// Compression stream
    pub struct CompressionStream {
        format: CompressionFormat,
        transform: TransformStream,
    }

    pub enum CompressionFormat {
        Gzip,
        Deflate,
        DeflateRaw,
    }
}
```

#### WebSocket API

```rust
/// WebSocket client implementation
pub mod websocket {
    use tokio_tungstenite::WebSocketStream;

    pub struct WebSocket {
        stream: WebSocketStream<TcpStream>,
        ready_state: ReadyState,
        on_open: Option<Box<dyn Fn()>>,
        on_message: Option<Box<dyn Fn(MessageEvent)>>,
        on_close: Option<Box<dyn Fn(CloseEvent)>>,
        on_error: Option<Box<dyn Fn(Error)>>,
    }

    impl WebSocket {
        pub async fn new(url: &str) -> Result<Self, WebSocketError>;
        pub async fn send(&mut self, data: Message) -> Result<(), WebSocketError>;
        pub async fn close(&mut self, code: Option<u16>, reason: Option<&str>);
        pub fn ready_state(&self) -> ReadyState;
    }

    pub enum ReadyState {
        Connecting = 0,
        Open = 1,
        Closing = 2,
        Closed = 3,
    }

    pub enum Message {
        Text(String),
        Binary(Vec<u8>),
    }
}
```

### 3. dx-compat-bun Component

The Bun-specific compatibility layer implements Bun's unique APIs.

#### Bun.serve() HTTP Server

```rust
/// High-performance HTTP server
pub mod serve {
    use hyper::{server::conn::http1, service::service_fn};
    use hyper_util::rt::TokioIo;

    /// Server configuration
    pub struct ServeOptions {
        pub port: u16,
        pub hostname: Option<String>,
        pub unix: Option<PathBuf>,
        pub tls: Option<TlsConfig>,
        pub websocket: Option<WebSocketHandler>,
        pub fetch: FetchHandler,
    }

    /// Create HTTP server
    pub async fn serve(options: ServeOptions) -> Result<Server, ServeError> {
        let listener = TcpListener::bind((
            options.hostname.as_deref().unwrap_or("0.0.0.0"),
            options.port,
        )).await?;

        // Use hyper for maximum performance
        loop {
            let (stream, _) = listener.accept().await?;
            let io = TokioIo::new(stream);
            
            tokio::spawn(async move {
                http1::Builder::new()
                    .serve_connection(io, service_fn(handle_request))
                    .await
            });
        }
    }

    /// Server handle
    pub struct Server {
        pub port: u16,
        pub hostname: String,
        shutdown_tx: oneshot::Sender<()>,
    }

    impl Server {
        pub async fn stop(&self);
        pub fn reload(&self, options: ServeOptions);
    }
}
```

#### Bun.file() File Handle

```rust
/// File handle with lazy loading
pub mod file {
    use memmap2::Mmap;

    /// BunFile handle
    pub struct BunFile {
        path: PathBuf,
        mmap: Option<Mmap>,
    }

    impl BunFile {
        pub fn new(path: impl AsRef<Path>) -> Self;

        /// Read as text
        pub async fn text(&self) -> Result<String, FileError>;

        /// Parse as JSON
        pub async fn json<T: DeserializeOwned>(&self) -> Result<T, FileError>;

        /// Read as ArrayBuffer
        pub async fn array_buffer(&self) -> Result<Vec<u8>, FileError>;

        /// Get readable stream
        pub fn stream(&self) -> ReadableStream;

        /// Get file size
        pub async fn size(&self) -> Result<u64, FileError>;

        /// Get MIME type
        pub fn type_(&self) -> &str;

        /// Slice file (lazy, zero-copy)
        pub fn slice(&self, start: u64, end: Option<u64>) -> BunFile;
    }

    /// Write data to file
    pub async fn write(
        path: impl AsRef<Path>,
        data: impl Into<WriteData>,
    ) -> Result<usize, FileError>;

    pub enum WriteData {
        String(String),
        Bytes(Vec<u8>),
        Stream(ReadableStream),
        File(BunFile),
    }
}
```


#### Bun.spawn() Process Spawning

```rust
/// Process spawning with high performance
pub mod spawn {
    use tokio::process::Command;

    /// Spawn options
    pub struct SpawnOptions {
        pub cwd: Option<PathBuf>,
        pub env: Option<HashMap<String, String>>,
        pub stdin: StdioConfig,
        pub stdout: StdioConfig,
        pub stderr: StdioConfig,
    }

    pub enum StdioConfig {
        Pipe,
        Inherit,
        Ignore,
    }

    /// Spawn async subprocess
    pub async fn spawn(
        cmd: &[&str],
        options: Option<SpawnOptions>,
    ) -> Result<Subprocess, SpawnError>;

    /// Spawn sync subprocess
    pub fn spawn_sync(
        cmd: &[&str],
        options: Option<SpawnOptions>,
    ) -> Result<SyncSubprocess, SpawnError>;

    /// Subprocess handle
    pub struct Subprocess {
        pub stdin: Option<WritableStream>,
        pub stdout: Option<ReadableStream>,
        pub stderr: Option<ReadableStream>,
        pub pid: u32,
        pub exited: impl Future<Output = ExitStatus>,
    }

    impl Subprocess {
        pub async fn kill(&self, signal: Option<i32>);
    }
}
```

#### Bun Hashing Functions

```rust
/// Fast hashing functions
pub mod hash {
    use xxhash_rust::xxh3::xxh3_64;
    use wyhash::wyhash;
    use crc32fast::Hasher as Crc32Hasher;

    /// Default fast hash (wyhash)
    pub fn hash(data: &[u8]) -> u64 {
        wyhash(data, 0)
    }

    /// WyHash
    pub fn wyhash(data: &[u8], seed: u64) -> u64;

    /// CRC-32
    pub fn crc32(data: &[u8]) -> u32 {
        let mut hasher = Crc32Hasher::new();
        hasher.update(data);
        hasher.finalize()
    }

    /// Adler-32
    pub fn adler32(data: &[u8]) -> u32;

    /// CityHash64
    pub fn city_hash_64(data: &[u8]) -> u64;

    /// MurmurHash3 32-bit
    pub fn murmur32v3(data: &[u8], seed: u32) -> u32;

    /// Streaming crypto hasher
    pub struct CryptoHasher {
        algorithm: HashAlgorithm,
        state: HasherState,
    }

    impl CryptoHasher {
        pub fn new(algorithm: HashAlgorithm) -> Self;
        pub fn update(&mut self, data: &[u8]);
        pub fn digest(&self) -> Vec<u8>;
    }

    pub enum HashAlgorithm {
        Md5,
        Sha1,
        Sha256,
        Sha384,
        Sha512,
        Blake2b256,
        Blake2b512,
        Blake3,
    }
}
```

#### Bun Compression

```rust
/// Compression functions
pub mod compression {
    use flate2::{Compression, read::GzDecoder, write::GzEncoder};
    use brotli::{CompressorReader, Decompressor};
    use zstd::{encode_all, decode_all};

    /// Gzip compress
    pub fn gzip_sync(data: &[u8], level: Option<u32>) -> Result<Vec<u8>, CompressionError> {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::new(level.unwrap_or(6)));
        encoder.write_all(data)?;
        Ok(encoder.finish()?)
    }

    /// Gzip decompress
    pub fn gunzip_sync(data: &[u8]) -> Result<Vec<u8>, CompressionError>;

    /// Deflate compress
    pub fn deflate_sync(data: &[u8], level: Option<u32>) -> Result<Vec<u8>, CompressionError>;

    /// Deflate decompress
    pub fn inflate_sync(data: &[u8]) -> Result<Vec<u8>, CompressionError>;

    /// Brotli compress
    pub fn brotli_compress_sync(data: &[u8], level: Option<u32>) -> Result<Vec<u8>, CompressionError>;

    /// Brotli decompress
    pub fn brotli_decompress_sync(data: &[u8]) -> Result<Vec<u8>, CompressionError>;

    /// Zstd compress
    pub fn zstd_compress_sync(data: &[u8], level: Option<i32>) -> Result<Vec<u8>, CompressionError>;

    /// Zstd decompress
    pub fn zstd_decompress_sync(data: &[u8]) -> Result<Vec<u8>, CompressionError>;
}
```

### 4. dx-compat-sqlite Component

```rust
/// Built-in SQLite database
pub mod sqlite {
    use rusqlite::{Connection, Statement, params};

    /// Database connection
    pub struct Database {
        conn: Connection,
        statement_cache: LruCache<String, Statement>,
    }

    impl Database {
        /// Open or create database
        pub fn new(path: impl AsRef<Path>) -> Result<Self, SqliteError> {
            let conn = Connection::open(path)?;
            // Enable WAL mode for performance
            conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA synchronous=NORMAL;")?;
            Ok(Self { conn, statement_cache: LruCache::new(100) })
        }

        /// Execute query and return all rows
        pub fn query<T: FromRow>(&self, sql: &str, params: &[Value]) -> Result<Vec<T>, SqliteError>;

        /// Prepare statement for reuse
        pub fn prepare(&mut self, sql: &str) -> Result<PreparedStatement, SqliteError>;

        /// Execute SQL without returning results
        pub fn exec(&self, sql: &str) -> Result<(), SqliteError>;

        /// Run in transaction
        pub fn transaction<F, T>(&mut self, f: F) -> Result<T, SqliteError>
        where
            F: FnOnce(&Transaction) -> Result<T, SqliteError>;
    }

    /// Prepared statement
    pub struct PreparedStatement<'a> {
        stmt: Statement<'a>,
    }

    impl PreparedStatement<'_> {
        pub fn all<T: FromRow>(&mut self, params: &[Value]) -> Result<Vec<T>, SqliteError>;
        pub fn get<T: FromRow>(&mut self, params: &[Value]) -> Result<Option<T>, SqliteError>;
        pub fn run(&mut self, params: &[Value]) -> Result<usize, SqliteError>;
    }
}
```

### 5. dx-compat-s3 Component

```rust
/// S3-compatible object storage
pub mod s3 {
    use aws_sdk_s3::Client;

    /// S3 client configuration
    pub struct S3Config {
        pub access_key_id: String,
        pub secret_access_key: String,
        pub endpoint: Option<String>,
        pub region: Option<String>,
        pub bucket: String,
    }

    /// S3 client
    pub struct S3Client {
        client: Client,
        bucket: String,
    }

    impl S3Client {
        pub async fn new(config: S3Config) -> Result<Self, S3Error>;

        /// Get file handle
        pub fn file(&self, key: &str) -> S3File;

        /// Write data to S3
        pub async fn write(&self, key: &str, data: impl Into<WriteData>) -> Result<(), S3Error>;

        /// Delete object
        pub async fn delete(&self, key: &str) -> Result<(), S3Error>;

        /// Check if object exists
        pub async fn exists(&self, key: &str) -> Result<bool, S3Error>;

        /// Get object size
        pub async fn size(&self, key: &str) -> Result<u64, S3Error>;

        /// Generate presigned URL
        pub async fn presign(&self, key: &str, expires_in: Duration) -> Result<String, S3Error>;
    }

    /// S3 file handle
    pub struct S3File {
        client: S3Client,
        key: String,
    }

    impl S3File {
        pub async fn text(&self) -> Result<String, S3Error>;
        pub async fn json<T: DeserializeOwned>(&self) -> Result<T, S3Error>;
        pub async fn array_buffer(&self) -> Result<Vec<u8>, S3Error>;
        pub fn stream(&self) -> ReadableStream;
    }
}
```


### 6. dx-compat-ffi Component

```rust
/// Foreign Function Interface
pub mod ffi {
    use libloading::{Library, Symbol};

    /// Load dynamic library
    pub fn dlopen(path: &str) -> Result<DynamicLibrary, FfiError>;

    /// Dynamic library handle
    pub struct DynamicLibrary {
        lib: Library,
    }

    impl DynamicLibrary {
        /// Get function symbol
        pub unsafe fn get<T>(&self, name: &str) -> Result<Symbol<T>, FfiError>;

        /// Close library
        pub fn close(self);
    }

    /// FFI type definitions
    pub enum FfiType {
        Void,
        Bool,
        I8, I16, I32, I64,
        U8, U16, U32, U64,
        F32, F64,
        Pointer,
        CString,
    }

    /// Pointer operations
    pub mod ptr {
        pub fn read<T>(ptr: *const T) -> T;
        pub fn write<T>(ptr: *mut T, value: T);
        pub fn to_array_buffer(ptr: *const u8, len: usize) -> Vec<u8>;
    }
}
```

### 7. dx-compat-shell Component

```rust
/// Shell scripting
pub mod shell {
    /// Shell command result
    pub struct ShellOutput {
        stdout: Vec<u8>,
        stderr: Vec<u8>,
        exit_code: i32,
    }

    impl ShellOutput {
        pub fn text(&self) -> Result<String, ShellError>;
        pub fn json<T: DeserializeOwned>(&self) -> Result<T, ShellError>;
        pub fn lines(&self) -> Vec<String>;
        pub fn bytes(&self) -> &[u8];
    }

    /// Execute shell command
    pub async fn shell(cmd: &str, args: &[&str]) -> Result<ShellOutput, ShellError>;

    /// Shell command builder
    pub struct ShellCommand {
        cmd: String,
        env: HashMap<String, String>,
        cwd: Option<PathBuf>,
        quiet: bool,
        nothrow: bool,
    }

    impl ShellCommand {
        pub fn new(cmd: &str) -> Self;
        pub fn env(mut self, key: &str, value: &str) -> Self;
        pub fn cwd(mut self, path: impl AsRef<Path>) -> Self;
        pub fn quiet(mut self) -> Self;
        pub fn nothrow(mut self) -> Self;
        pub async fn run(self) -> Result<ShellOutput, ShellError>;
    }
}
```

### 8. dx-compat-hmr Component

```rust
/// Hot Module Replacement
pub mod hmr {
    use notify::{Watcher, RecursiveMode};
    use petgraph::Graph;

    /// HMR server
    pub struct HmrServer {
        watcher: RecommendedWatcher,
        dependency_graph: Graph<ModuleId, ()>,
        clients: Vec<WebSocket>,
    }

    impl HmrServer {
        pub fn new(root: impl AsRef<Path>) -> Result<Self, HmrError>;

        /// Start watching for changes
        pub async fn start(&mut self);

        /// Handle file change
        async fn on_file_change(&mut self, path: &Path);

        /// Invalidate module and dependents
        fn invalidate(&mut self, module_id: ModuleId) -> Vec<ModuleId>;

        /// Send update to clients
        async fn send_update(&self, updates: Vec<HmrUpdate>);
    }

    /// HMR update message
    pub struct HmrUpdate {
        pub path: String,
        pub hash: String,
        pub update_type: UpdateType,
    }

    pub enum UpdateType {
        Js,
        Css,
        FullReload,
    }

    /// import.meta.hot API
    pub struct HotModule {
        pub data: HashMap<String, JsValue>,
    }

    impl HotModule {
        pub fn accept(&self, callback: impl Fn());
        pub fn dispose(&self, callback: impl Fn(&mut HashMap<String, JsValue>));
        pub fn decline(&self);
        pub fn invalidate(&self);
    }
}
```

### 9. dx-compat-plugin Component

```rust
/// Plugin system
pub mod plugin {
    /// Plugin registration
    pub fn register(plugin: Plugin);

    /// Plugin definition
    pub struct Plugin {
        pub name: String,
        pub setup: Box<dyn Fn(&mut PluginBuilder)>,
    }

    /// Plugin builder
    pub struct PluginBuilder {
        on_load_handlers: Vec<OnLoadHandler>,
        on_resolve_handlers: Vec<OnResolveHandler>,
    }

    impl PluginBuilder {
        /// Register load handler
        pub fn on_load(&mut self, filter: &str, namespace: Option<&str>, handler: OnLoadFn);

        /// Register resolve handler
        pub fn on_resolve(&mut self, filter: &str, namespace: Option<&str>, handler: OnResolveFn);
    }

    /// Load handler result
    pub struct OnLoadResult {
        pub contents: String,
        pub loader: Loader,
    }

    /// Resolve handler result
    pub struct OnResolveResult {
        pub path: String,
        pub namespace: Option<String>,
    }

    pub enum Loader {
        Js,
        Jsx,
        Ts,
        Tsx,
        Json,
        Text,
        Binary,
        Css,
    }
}
```

### 10. dx-compat-html Component

```rust
/// HTML Rewriter using lol_html
pub mod html {
    use lol_html::{HtmlRewriter, Settings, element, text};

    /// HTML Rewriter
    pub struct HTMLRewriter {
        handlers: Vec<ElementHandler>,
        document_handlers: Vec<DocumentHandler>,
    }

    impl HTMLRewriter {
        pub fn new() -> Self;

        /// Register element handler
        pub fn on(&mut self, selector: &str, handler: impl ElementContentHandler) -> &mut Self;

        /// Register document handler
        pub fn on_document(&mut self, handler: impl DocumentContentHandler) -> &mut Self;

        /// Transform response
        pub fn transform(&self, response: Response) -> Response;
    }

    /// Element content handler
    pub trait ElementContentHandler {
        fn element(&mut self, el: &mut Element);
        fn comments(&mut self, comment: &mut Comment) {}
        fn text(&mut self, text: &mut TextChunk) {}
    }

    /// Element manipulation
    pub struct Element {
        inner: lol_html::Element,
    }

    impl Element {
        pub fn get_attribute(&self, name: &str) -> Option<String>;
        pub fn set_attribute(&mut self, name: &str, value: &str);
        pub fn remove_attribute(&mut self, name: &str);
        pub fn has_attribute(&self, name: &str) -> bool;
        pub fn tag_name(&self) -> &str;
        pub fn before(&mut self, content: &str, content_type: ContentType);
        pub fn after(&mut self, content: &str, content_type: ContentType);
        pub fn prepend(&mut self, content: &str, content_type: ContentType);
        pub fn append(&mut self, content: &str, content_type: ContentType);
        pub fn replace(&mut self, content: &str, content_type: ContentType);
        pub fn remove(&mut self);
        pub fn set_inner_content(&mut self, content: &str, content_type: ContentType);
    }
}
```

## Data Models

### Error Types

```rust
/// Unified error type for compatibility layer
#[derive(Debug, thiserror::Error)]
pub enum CompatError {
    #[error("File system error: {0}")]
    Fs(#[from] FsError),

    #[error("Network error: {0}")]
    Network(#[from] NetworkError),

    #[error("SQLite error: {0}")]
    Sqlite(#[from] SqliteError),

    #[error("S3 error: {0}")]
    S3(#[from] S3Error),

    #[error("FFI error: {0}")]
    Ffi(#[from] FfiError),

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),
}

/// Node.js compatible error codes
pub enum ErrorCode {
    ENOENT,      // No such file or directory
    EACCES,      // Permission denied
    EEXIST,      // File exists
    EISDIR,      // Is a directory
    ENOTDIR,     // Not a directory
    ENOTEMPTY,   // Directory not empty
    ETIMEDOUT,   // Operation timed out
    ECONNREFUSED, // Connection refused
}
```

### Configuration Types

```rust
/// Feature flags configuration
pub struct CompatConfig {
    /// Enable Node.js compatibility
    pub node: NodeConfig,
    /// Enable Web API compatibility
    pub web: WebConfig,
    /// Enable Bun API compatibility
    pub bun: BunConfig,
}

pub struct NodeConfig {
    pub fs: bool,
    pub path: bool,
    pub buffer: bool,
    pub stream: bool,
    pub events: bool,
    pub http: bool,
    pub crypto: bool,
    pub child_process: bool,
}

pub struct WebConfig {
    pub fetch: bool,
    pub streams: bool,
    pub websocket: bool,
    pub url: bool,
    pub crypto: bool,
}

pub struct BunConfig {
    pub serve: bool,
    pub file: bool,
    pub spawn: bool,
    pub hash: bool,
    pub sqlite: bool,
    pub s3: bool,
    pub ffi: bool,
    pub shell: bool,
}
```


## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system—essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

Based on the prework analysis, the following correctness properties have been identified for property-based testing:

### Property 1: File System Read/Write Round-Trip

*For any* valid file path and any byte sequence, writing the data with `fs.writeFile()` and then reading it with `fs.readFile()` SHALL produce the original byte sequence.

**Validates: Requirements 2.1, 2.2**

### Property 2: Bun.file() Read/Write Round-Trip

*For any* valid file path and any string content, writing with `Bun.write()` and then reading with `Bun.file().text()` SHALL produce the original string content.

**Validates: Requirements 14.1, 14.2, 14.6**

### Property 3: S3 Object Read/Write Round-Trip

*For any* valid S3 key and any byte sequence, writing with `client.write()` and then reading with `s3file.arrayBuffer()` SHALL produce the original byte sequence.

**Validates: Requirements 20.3, 20.5**

### Property 4: Buffer Encoding Round-Trip

*For any* string and any supported encoding (utf8, ascii, base64, hex, latin1), creating a Buffer with `Buffer.from(string, encoding)` and then calling `buffer.toString(encoding)` SHALL produce the original string.

**Validates: Requirements 4.2, 4.7**

### Property 5: Compression Round-Trip

*For any* byte sequence and any compression algorithm (gzip, deflate, brotli, zstd), compressing and then decompressing SHALL produce the original byte sequence.

**Validates: Requirements 18.1, 18.2, 18.3, 18.4, 18.5, 18.6**

### Property 6: Password Hash/Verify Round-Trip

*For any* password string and any supported algorithm (argon2id, bcrypt), hashing with `Bun.password.hash()` and then verifying with `Bun.password.verify()` using the same password SHALL return true.

**Validates: Requirements 17.1, 17.2**

### Property 7: Hash Consistency

*For any* byte sequence and any hash algorithm, calling the hash function multiple times with the same input SHALL produce the same output.

**Validates: Requirements 16.1, 16.2, 16.3, 16.4, 16.5, 16.6**

### Property 8: Node Stream Pipe Completeness

*For any* readable stream with N chunks, piping to a writable stream SHALL transfer all N chunks in order, and the total bytes written SHALL equal the total bytes read.

**Validates: Requirements 5.1, 5.3**

### Property 9: Web Stream Pipe Completeness

*For any* ReadableStream with N chunks, calling `pipeTo()` on a WritableStream SHALL transfer all N chunks in order, and the total bytes written SHALL equal the total bytes read.

**Validates: Requirements 11.1, 11.4**

### Property 10: Event Emitter Listener Invocation

*For any* EventEmitter with N listeners registered for event E, emitting event E SHALL invoke all N listeners exactly once, in registration order.

**Validates: Requirements 6.1, 6.2, 6.3**

### Property 11: Path Operations Correctness

*For any* array of path segments, `path.join()` followed by `path.normalize()` SHALL produce a valid path, and `path.isAbsolute(path.resolve(segments))` SHALL return true.

**Validates: Requirements 3.1, 3.2, 3.6, 3.7**

### Property 12: SQLite Query Correctness

*For any* valid SQL INSERT followed by a SELECT with matching WHERE clause, the SELECT SHALL return the inserted row with all column values preserved.

**Validates: Requirements 19.2, 19.6, 19.7, 19.8**

### Property 13: SQLite Transaction Atomicity

*For any* transaction containing N operations where operation K fails (1 ≤ K ≤ N), all operations 1 through K-1 SHALL be rolled back, and the database state SHALL be unchanged.

**Validates: Requirements 19.5**

### Property 14: WebSocket Message Round-Trip

*For any* message (text or binary) sent via `ws.send()`, the server SHALL receive the exact same message content and type.

**Validates: Requirements 12.2, 12.6**

### Property 15: Fetch Response Body Consistency

*For any* HTTP response with body content, calling `response.text()`, `response.json()`, or `response.arrayBuffer()` SHALL return content consistent with the Content-Type header.

**Validates: Requirements 10.1, 10.2, 10.3**

### Property 16: HMR Dependency Invalidation

*For any* module M with dependents D1, D2, ..., Dn, when M is updated, all modules in the transitive closure of dependents SHALL be invalidated.

**Validates: Requirements 24.1, 24.2**

### Property 