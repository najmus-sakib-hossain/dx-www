# dx-javascript

[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

High-performance JavaScript/TypeScript tooling written in Rust, targeting 10-100x performance improvements over existing tools.

## Overview

The `javascript` workspace contains a suite of Rust-based tools for JavaScript and TypeScript development, including a runtime, bundler, package manager, test runner, and more.

## Subcrates

### Runtime

| Crate | Description |
|-------|-------------|
| [runtime](./runtime) | High-performance JS/TS runtime with JIT compilation |

### Bundler

| Crate | Description |
|-------|-------------|
| [bundler](./bundler) | Fast JavaScript bundler with parallel processing |

### Package Manager

| Crate | Description |
|-------|-------------|
| [package-manager](./package-manager) | Binary-first package manager with 30-100x faster installs |

### Test Runner

| Crate | Description |
|-------|-------------|
| [test-runner](./test-runner) | Parallel test execution with smart caching |

### Compatibility

| Crate | Description |
|-------|-------------|
| [compatibility](./compatibility) | Node.js and Bun API compatibility layers |

### Monorepo

| Crate | Description |
|-------|-------------|
| [monorepo](./monorepo) | Binary-first monorepo management system |

## Features

- 10x faster JavaScript runtime with Cranelift JIT
- Parallel bundling with incremental compilation
- Binary package format for instant installs
- Smart test caching and parallel execution
- Full Node.js and Bun API compatibility

## Installation

Each subcrate can be installed independently:

```bash
# Runtime
cargo install dx-js-runtime

# Package manager
cargo install dx-pkg-cli

# Test runner
cargo install dx-test-cli
```

## Usage

```bash
# Run JavaScript/TypeScript
dx-js run script.ts

# Install packages
dx-pkg install

# Run tests
dx-test run
```

## License

This project is dual-licensed under MIT OR Apache-2.0.
