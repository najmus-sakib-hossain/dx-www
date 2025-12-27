# dx-www

[![Crates.io](https://img.shields.io/crates/v/dx-www.svg)](https://crates.io/crates/dx-www)
[![Documentation](https://docs.rs/dx-www/badge.svg)](https://docs.rs/dx-www)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

A high-performance web framework with a transpiler-to-binary pipeline that converts TSX to optimized binary formats.

## Overview

`dx-www` is the core web framework for the DX ecosystem. It provides a complete solution for building modern web applications with a focus on performance through binary compilation and efficient reactivity.

## Subcrates

| Crate | Description |
|-------|-------------|
| [core](./core) | Core transpiler and compiler pipeline |
| [a11y](./a11y) | Accessibility utilities and ARIA support |
| [auth](./auth) | Authentication and authorization |
| [binary](./binary) | Binary format encoding/decoding |
| [cache](./cache) | Caching layer and strategies |
| [client](./client) | Client-side runtime |
| [client-tiny](./client-tiny) | Minimal client runtime (~2KB) |
| [db](./db) | Database abstractions |
| [db-teleport](./db-teleport) | Database state synchronization |
| [debug](./debug) | Development debugging tools |
| [dom](./dom) | Virtual DOM implementation |
| [error](./error) | Error handling utilities |
| [fallback](./fallback) | Fallback and error boundary support |
| [form](./form) | Form handling and validation |
| [framework-core](./framework-core) | Framework primitives |
| [guard](./guard) | Route guards and middleware |
| [interaction](./interaction) | User interaction handling |
| [morph](./morph) | DOM morphing/diffing |
| [offline](./offline) | Offline support and service workers |
| [packet](./packet) | Binary packet protocol |
| [print](./print) | Print stylesheet support |
| [query](./query) | Data fetching and caching |
| [reactor](./reactor) | Cross-platform I/O reactor |
| [rtl](./rtl) | Right-to-left language support |
| [sched](./sched) | Task scheduling |
| [server](./server) | Server-side rendering |
| [state](./state) | State management |
| [sync](./sync) | Real-time synchronization |

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
dx-www = "0.1.0"
```

## Usage

```rust
use dx_compiler::prelude::*;

fn main() -> anyhow::Result<()> {
    // Compile TSX to binary
    let compiler = Compiler::new();
    compiler.compile("./src/app.tsx", "./dist")?;
    
    Ok(())
}
```

## Features

- TSX to binary compilation
- Server-side rendering
- Reactive state management
- Form handling with validation
- Data fetching with caching
- Accessibility built-in
- Offline support

## License

This project is dual-licensed under MIT OR Apache-2.0.
