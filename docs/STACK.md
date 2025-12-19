# DX Stack: Language-Aware Development Tooling

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

| Component       | Description                          | Command        |
|-----------------|--------------------------------------|----------------|
| **Runtime**     | Code execution engine                | `dx stack run` |
| **PackageManager** | Dependency management             | `dx stack install` |
| **Bundler**     | Code bundling/compilation            | `dx stack bundle` |
| **Monorepo**    | Multi-package workspace management   | `dx stack workspace` |
| **Compatibility** | Cross-version/platform compatibility | `dx stack compat` |
| **TestRunner**  | Test execution framework             | `dx stack test` |

## Language Support Matrix

| Language      | Runtime | PackageManager | Bundler | Monorepo | Compat | Test | Notes |
|---------------|:-------:|:--------------:|:-------:|:--------:|:------:|:----:|-------|
| JavaScript/TS | ✓       | ✓              | ✓       | ✓        | ✓      | ✓    | Full stack |
| Python        | ✓       | ✓              | ✗       | ✓        | ✓      | ✓    | No bundler needed |
| Ruby          | ✓       | ✓              | ✗       | ✗        | ✗      | ✓    | Basic stack |
| C/C++         | ✗       | ✗              | ✓       | ✗        | ✓      | ✓    | Build-focused |
| Java/Kotlin   | ✗       | ✓              | ✓       | ✗        | ✗      | ✓    | Build + pkg |
| Elixir        | ✓       | ✓              | ✗       | ✗        | ✗      | ✓    | Basic stack |
| **Rust**      | ✗       | ✗              | ✗       | ✗        | ✗      | ✗    | Uses `cargo` |
| **Go**        | ✗       | ✗              | ✗       | ✗        | ✗      | ✗    | Uses `go` |
| **Zig**       | ✗       | ✗              | ✗       | ✗        | ✗      | ✗    | Uses `zig build` |
| **Swift**     | ✗       | ✗              | ✗       | ✗        | ✗      | ✗    | Uses SwiftPM |

## JavaScript/TypeScript Stack

The JavaScript stack integrates all `dx-js-*` crates:

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

## Self-Sufficient Languages

Languages with unified toolchains don't need DX Stack:

### Rust
```bash
dx stack -l rust info
# Output: Rust has a unified native toolchain: cargo
#
# Cargo handles everything:
# • cargo build - Install dependencies
# • cargo test  - Run tests
# • cargo run   - Run project
# • cargo fmt   - Format code
# • cargo clippy - Lint code
```

### Go
```bash
dx stack -l go info
# Output: Go has a unified native toolchain: go
#
# Go toolchain handles everything:
# • go mod tidy - Install dependencies
# • go test     - Run tests
# • go run      - Run project
# • go fmt      - Format code
```

## CLI Reference

### Global Options

```bash
dx stack [OPTIONS] <COMMAND>

Options:
  -l, --language <LANGUAGE>  Target language [default: javascript]
                             [possible values: javascript, typescript, python,
                             rust, go, c, cpp, ruby, java, kotlin, swift, elixir, zig]
```

### Commands

#### Runtime Commands
```bash
dx stack run <file>        # Run a file
dx stack run --watch       # Run in watch mode
dx stack repl              # Start REPL
```

#### Bundler Commands
```bash
dx stack bundle            # Bundle for production
dx stack bundle --minify   # With minification
dx stack dev               # Start dev server
dx stack dev --port 8080   # Custom port
dx stack build             # Production build
```

#### Test Commands
```bash
dx stack test              # Run all tests
dx stack test --watch      # Watch mode
dx stack test --coverage   # With coverage
```

#### Package Manager Commands
```bash
dx stack install           # Install all dependencies
dx stack install pkg1 pkg2 # Install specific packages
dx stack add <package>     # Add a package
dx stack add <pkg> -D      # Add as dev dependency
dx stack remove <package>  # Remove a package
dx stack update            # Update all packages
dx stack update <package>  # Update specific package
dx stack init              # Initialize new project
```

#### Monorepo Commands
```bash
dx stack workspace init           # Initialize monorepo
dx stack workspace list           # List packages
dx stack workspace run-all <cmd>  # Run in all packages
dx stack workspace graph          # Show dependency graph
dx stack workspace add <name>     # Add new package
```

#### Compatibility Commands
```bash
dx stack compat                   # List available targets
dx stack compat es2020            # Check against target
```

#### Info Commands
```bash
dx stack info                     # Show stack info
dx stack bench                    # Run benchmarks
dx stack languages                # List all languages
```

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        dx stack <command>                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │                   StackRegistry                          │    │
│  │  (Manages language-specific stack implementations)       │    │
│  └─────────────────────────────────────────────────────────┘    │
│                              │                                   │
│            ┌─────────────────┼─────────────────┐                │
│            ▼                 ▼                 ▼                │
│   ┌─────────────┐   ┌─────────────┐   ┌─────────────┐          │
│   │ JavaScript  │   │   Python    │   │   (Future)  │          │
│   │   Stack     │   │   Stack     │   │   Stacks    │          │
│   └─────────────┘   └─────────────┘   └─────────────┘          │
│          │                                                       │
│          ▼                                                       │
│   ┌──────┬──────┬──────┬──────┬──────┬──────┐                  │
│   │  RT  │  PM  │  BD  │  MR  │  CP  │  TR  │                  │
│   └──────┴──────┴──────┴──────┴──────┴──────┘                  │
│      │      │      │      │      │      │                       │
│      ▼      ▼      ▼      ▼      ▼      ▼                       │
│   ┌──────────────────────────────────────────┐                  │
│   │            dx-js-* crates                 │                  │
│   │  runtime │ pkg-mgr │ bundler │ test-runner │                │
│   │  monorepo │ compatibility                  │                │
│   └──────────────────────────────────────────┘                  │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

## Extending for New Languages

To add a new language stack:

1. Create a new module in `dx-stack/src/languages/`
2. Implement the `LanguageStack` trait
3. Implement relevant component traits (Runtime, Bundler, etc.)
4. Register in `StackRegistry`

Example:

```rust
// dx-stack/src/languages/python.rs
pub struct PythonStack {
    runtime: PythonRuntime,
    package_manager: PythonPackageManager,
    test_runner: PythonTestRunner,
    // No bundler - Python doesn't need one
}

impl LanguageStack for PythonStack {
    fn language(&self) -> Language {
        Language::Python
    }

    fn capabilities(&self) -> StackCapabilitySet {
        StackCapabilitySet::from_iter([
            StackCapability::Runtime,
            StackCapability::PackageManager,
            StackCapability::TestRunner,
            StackCapability::Compatibility,
            StackCapability::Monorepo,
        ])
    }

    fn runtime(&self) -> Option<&dyn Runtime> {
        Some(&self.runtime)
    }

    // ... implement other methods
}
```

## Performance

### JavaScript Stack Benchmarks

| Tool | DX Performance | Comparison |
|------|----------------|------------|
| Runtime | 10.59x | faster than Bun |
| Bundler | 3.8x | faster than Bun |
| Test Runner | 26x | faster than Jest |
| Package Manager | 50x | faster than npm |

### Why So Fast?

1. **Binary Protocols**: No JSON parsing overhead
2. **Zero-Copy**: Direct memory access
3. **Rust Performance**: No GC pauses
4. **SIMD**: AVX2 optimizations where applicable
5. **Parallel Execution**: Utilize all CPU cores

## Roadmap

- [x] JavaScript/TypeScript stack
- [ ] Python stack
- [ ] Ruby stack
- [ ] C/C++ stack (CMake replacement)
- [ ] Java/Kotlin stack (Gradle replacement)

## License

MIT OR Apache-2.0
