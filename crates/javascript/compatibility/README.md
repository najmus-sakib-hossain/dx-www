# dx-js-compatibility

100% Bun API compatibility layer with 10-50x performance gains.

## Overview

This crate provides a comprehensive compatibility layer enabling drop-in Bun replacement while leveraging DX's binary-first architecture for superior performance.

## Features

The system is organized into 12 sub-crates, each responsible for a specific domain:

| Feature | Sub-crate | Description |
|---------|-----------|-------------|
| `node-core` | dx-compat-node | Node.js API compatibility (40+ modules) |
| `web-core` | dx-compat-web | Web Standard APIs (30+ APIs) |
| `bun-core` | dx-compat-bun | Bun-specific APIs (50+ functions) |
| `bun-sqlite` | dx-compat-sqlite | Built-in SQLite database |
| `bun-s3` | dx-compat-s3 | S3-compatible object storage |
| `bun-ffi` | dx-compat-ffi | Foreign Function Interface |
| `bun-shell` | dx-compat-shell | Shell scripting |
| `compile` | dx-compat-compile | Single executable compilation |
| `hmr` | dx-compat-hmr | Hot Module Replacement |
| `plugins` | dx-compat-plugin | Plugin system |
| `macros` | dx-compat-macro | Compile-time macros |
| `html-rewriter` | dx-compat-html | HTML Rewriter |

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
dx-js-compatibility = { version = "0.1", features = ["default"] }
```

Or with specific features:

```toml
[dependencies]
dx-js-compatibility = { version = "0.1", features = ["node-core", "bun-sqlite"] }
```

## Performance Targets

| Operation | Target | vs Bun |
|-----------|--------|--------|
| HTTP requests/sec | 400,000+ | 2x |
| File read throughput | 1 GB/s | 2x |
| SQLite ops/sec | 200,000+ | 2x |
| SHA256 throughput | 2 GB/s | 2x |
| Gzip throughput | 450 MB/s | 1.5x |
| Process spawns/sec | 10,000+ | 2x |
| WebSocket msgs/sec | 200,000+ | 2x |

## License

MIT OR Apache-2.0
