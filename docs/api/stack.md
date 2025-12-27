# DX Stack API Reference

**The unified abstraction for language-specific development tools.**

## Overview

DX Stack provides a consistent interface for development tooling across multiple programming languages. Instead of treating JavaScript runtime, bundler, test runner, and package manager as separate tools, they're unified under a single "Stack" concept that adapts to each language's needs.

## Philosophy

> "Not every language needs the same tools. Rust has cargo. Go has go. JavaScript has... chaos."

DX Stack recognizes that:

1. **Some languages have unified toolchains** (Rust, Go, Zig) that handle everything
2. **Some languages have fragmented ecosystems** (JavaScript, Python) that need unified tooling
3. **Different languages need different components** based on their ecosystem

## Stack Components

| Component | Description | Command |
|-----------|-------------|---------|
| **Runtime** | Code execution engine | `dx stack run` |
| **PackageManager** | Dependency management | `dx stack install` |
| **Bundler** | Code bundling/compilation | `dx stack bundle` |
| **Monorepo** | Multi-package workspace management | `dx stack workspace` |
| **Compatibility** | Cross-version/platform compatibility | `dx stack compat` |
| **TestRunner** | Test execution framework | `dx stack test` |

## Language Support Matrix

| Language | Runtime | PackageManager | Bundler | Monorepo | Compat | Test |
|----------|:-------:|:--------------:|:-------:|:--------:|:------:|:----:|
| JavaScript/TS | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Python | ✓ | ✓ | ✗ | ✓ | ✓ | ✓ |
| Ruby | ✓ | ✓ | ✗ | ✗ | ✗ | ✓ |
| C/C++ | ✗ | ✗ | ✓ | ✗ | ✓ | ✓ |
| Java/Kotlin | ✗ | ✓ | ✓ | ✗ | ✗ | ✓ |
| **Rust** | ✗ | ✗ | ✗ | ✗ | ✗ | ✗ |
| **Go** | ✗ | ✗ | ✗ | ✗ | ✗ | ✗ |

*Rust, Go, Zig, and Swift use their native unified toolchains.*

## JavaScript/TypeScript Stack

### Components

| Component | Crate | Performance |
|-----------|-------|-------------|
| Runtime | `dx-js-runtime` | 10.59x faster than Bun |
| Package Manager | `dx-js-package-manager` | 50x faster than npm |
| Bundler | `dx-js-bundler` | 3.8x faster than Bun |
| Monorepo | `dx-js-monorepo` | Binary-first workspaces |
| Compatibility | `dx-js-compatibility` | ESNext → ES5 transforms |
| Test Runner | `dx-js-test-runner` | 26x faster than Jest |

### Usage

```bash
# Run a file
dx stack run index.ts

# Bundle for production
dx stack bundle --minify --sourcemap

# Run tests
dx stack test --coverage

# Install packages
dx stack install

# Add a package
dx stack add react

# Start dev server
dx stack dev --port 3000

# Monorepo commands
dx stack workspace list
dx stack workspace run-all build
dx stack workspace graph
```

## CLI Reference

### Global Options

```bash
dx stack [OPTIONS] <COMMAND>

Options:
  -l, --language <LANGUAGE>  Target language [default: javascript]
```

### Runtime Commands

```bash
dx stack run <file>        # Run a file
dx stack run --watch       # Run in watch mode
dx stack repl              # Start REPL
```

### Bundler Commands

```bash
dx stack bundle            # Bundle for production
dx stack bundle --minify   # With minification
dx stack dev               # Start dev server
dx stack dev --port 8080   # Custom port
dx stack build             # Production build
```

### Test Commands

```bash
dx stack test              # Run all tests
dx stack test --watch      # Watch mode
dx stack test --coverage   # With coverage
```

### Package Manager Commands

```bash
dx stack install           # Install all dependencies
dx stack add <package>     # Add a package
dx stack add <pkg> -D      # Add as dev dependency
dx stack remove <package>  # Remove a package
dx stack update            # Update all packages
dx stack init              # Initialize new project
```

### Monorepo Commands

```bash
dx stack workspace init           # Initialize monorepo
dx stack workspace list           # List packages
dx stack workspace run-all <cmd>  # Run in all packages
dx stack workspace graph          # Show dependency graph
dx stack workspace add <name>     # Add new package
```

## Related

- [CLI Reference](./cli.md)
- [DX WWW](./dx-www.md)
- [Architecture](../architecture/README.md)
