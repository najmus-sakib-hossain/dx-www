# Dx Check

**Dx Check** is the cornerstone of the **dx** developer experience suite, engineered to define a new standard in code quality assurance. It is not merely a linter or a formatter; it is a high-performance, unified code intelligence engine designed to deliver lightning-fast feedback and automated corrections. By leveraging state-of-the-art technologies and a modern, Rust-based architecture, Dx Check ensures that your codebase remains robust, consistent, and maintainable, regardless of the scale or complexity of your project.

## Table of Contents

- [Introduction](#introduction)
- [Key Features](#key-features)
- [Architecture & Internals](#architecture--internals)
    - [The Power of Rust](#the-power-of-rust)
    - [Green Syntax Trees & generic_node](#green-syntax-trees--generic_node)
    - [Rowan: The Foundation](#rowan-the-foundation)
    - [Taplo: TOML Mastery](#taplo-toml-mastery)
- [Submodules & Integrations](#submodules--integrations)
    - [Biome](#biome)
    - [The Playground](#the-playground)
- [Supported Languages](#supported-languages)
- [Installation](#installation)
- [Usage Guide](#usage-guide)
- [Configuration](#configuration)
- [Troubleshooting](#troubleshooting)
- [Contributing](#contributing)
- [License](#license)

## Introduction

In the modern software development landscape, the friction caused by slow CI pipelines, inconsistent formatting, and obscure linting rules can drastically reduce developer velocity. **Dx Check** addresses these challenges head-on.

Designed to be invisible yet omnipresent, Dx Check integrates seamlessly into your workflow. Whether triggered on file save, via a pre-commit hook, or as part of a continuous integration pipeline, it operates with sub-millisecond latency. It goes beyond simple error reporting; it understands the structure of your code, allowing it to perform complex automated refactoring and "safe fixes" without manual intervention.

## Key Features

- **Blazing Fast Performance**: Built on Rust, maximizing concurrency and memory efficiency.
- **Unified Toolchain**: Combines linting, formatting, and analysis in a single binary.
- **LSP Support**: First-class Language Server Protocol integration for instant editor feedback.
- **Error Recovery**: Robust parsing that understands partially broken code, enabling checking while you type.
- **Safe Fixes**: Automatically applies safe architectural fixes and code style improvements.
- **Micro-Frontend Ready**: scalable architecture that supports monorepos and diverse tech stacks.
- **Zero Config**: Sane defaults that work out of the box, with deep configurability when needed.

## Architecture & Internals

Dx Check's exceptional performance and capability are strictly due to its underlying architectural choices. We have carefully selected and integrated the most advanced libraries in the Rust ecosystem.

### The Power of Rust

At the core of Dx Check is **Rust**. Chosen for its memory safety without garbage collection, Rust allows us to process massive codebases with minimal memory overhead and predictable performance. This ensures that Dx Check scales linearly with your project size, utilizing every available CPU core to parallelize work.

### Green Syntax Trees & generic_node

Unlike traditional compilers that use Abstract Syntax Trees (AST) which discard formatting and whitespace information, Dx Check utilizes **Green Syntax Trees** (GST) or Concrete Syntax Trees (CST).

- **Lossless Representation**: Every token, including whitespace and comments, is preserved.
- **Resilient Parsing**: The parser can recover from syntax errors, generating a valid tree even for invalid code. This is crucial for IDE support where code is often in a transient, broken state.
- **Incremental Reparsing**: Only changed sections of a file need to be re-parsed, enabling instant feedback loops.

### Rowan: The Foundation

We utilize **Rowan**, a library for lossless syntax trees inspired by the Swift compiler's `libsyntax`. Rowan provides:

- **Immutability**: Nodes are immutable, making them safe to share across threads.
- **Generic Nodes**: A unified interface for traversing and manipulating the syntax tree, regardless of the underlying language.
- **High Performance**: Optimized memory layout to reduce cache misses.

### Taplo: TOML Mastery

For TOML support, Dx Check integrates **Taplo**. Taplo represents the gold standard for TOML toolkit implementation in Rust.

- **Full TOML v1.0.0 Compliance**: Guarantees correct parsing of modern configuration files.
- **Semantic Validation**: Goes beyond syntax to check for structural correctness.
- **Reformatting**: Opinionated, consistent formatting for TOML files.

## Submodules & Integrations

Dx Check is a composition of powerful submodules, ensuring that for every language, we use the best tool available.

### Biome

One of our primary engines is **[Biome](https://biomejs.dev/)**. Biome is a spiritual successor to tools like Prettier and ESLint but rewritten in Rust for speed.

**Why Biome?**
- **Speed**: Biome is fast. Extremely fast. It can format and lint thousands of files in seconds.
- **Compatibility**: It maintains 97% compatibility with Prettier, making migration painless.
- **Rich Diagnostics**: Biome provides detailed, actionable error messages that teach you best practices.

Inside `crates/check/submodules/biome`, you will find the complete source code for this engine. We link directly against Biome's internal crates to provide a seamless experience without the overhead of spawning external processes.

### The Playground

Located at `crates/check/submodules/biome/playground`, the playground directory serves as a comprehensive test suite and showcase for Dx Check's capabilities. It contains representative files for all supported languages, covering edge cases and modern syntax features.

**Contents of the Playground:**

- **`sample.js`**: Demonstrates ES6+ features, including classes, async/await, and complex lexical scoping. Used to verify JavaScript linting rules.
- **`sample.ts`**: A robust TypeScript example featuring generics, interfaces, and type narrowing. Ensures type-aware linting acts correctly.
- **`sample.json`**: A complex JSON structure (mimicking `package.json`) to test JSON formatting and validation.
- **`sample.css`**: Modern CSS3 features including variables, flexbox, grid, and media queries, testing the CSS formatter.
- **`sample.graphql`**: Contains schema definitions, queries, and mutations to validate GraphQL support.
- **`sample.html`**: Semantic HTML5 markup test case.
- **`sample.toml`**: powered by Taplo, this file tests inline tables, arrays, and complex key-value pairs.

You can use this directory to instantly verify the tool's behavior:
```bash
# syntax to check the playground
dx check crates/check/submodules/biome/playground
```

## Supported Languages

Dx Check aims for universal support. Below is the current matrix of supported languages and the engines that power them.

| Language               | Linter Engine     | Formatter Engine | Status        |
|------------------------|-------------------|------------------|---------------|
| **HTML**               | Biome             | Biome            | ✅ Production |
| **CSS**                | Biome             | Biome            | ✅ Production |
| **JavaScript**         | Biome             | Biome            | ✅ Production |
| **TypeScript**         | Biome             | Biome            | ✅ Production |
| **JSX / TSX**          | Biome             | Biome            | ✅ Production |
| **JSON**               | Biome             | Biome            | ✅ Production |
| **GraphQL**            | Biome             | Biome            | ✅ Production |
| **Markdown**           | rumdl             | rumdl            | ✅ Production |
| **TOML**               | Taplo             | Taplo            | ✅ Production |
| **Python**             | Ruff              | Ruff             | ✅ Production |
| **PHP**                | Mago              | Mago             | ✅ Production |
| **Go**                 | Gold              | gofmt.rs         | ✅ Production |
| **Rust**               | rust-clippy       | rustfmt          | ✅ Production |
| **C / C++**            | cpp-linter-rs     | cpp-linter-rs    | ✅ Production |
| **Kotlin**             | ktlint            | ktlint           | ✅ Production |

**Note**: "Production" status indicates that the parser is feature-complete and stable for daily use.

## Installation

Dx Check is distributed as part of the `dx` CLI suite.

### Via Cargo
```bash
cargo install dx-check
```

### Pre-built Binaries
We provide pre-built binaries for Windows, macOS, and Linux. Please check the [Releases](https://github.com/dx/releases) page.

### From Source
To build from source, ensure you have the latest stable Rust toolchain installed:
```bash
git clone https://github.com/dx/dx
cd dx/crates/check
cargo build --release
```

## Usage Guide

Dx Check is designed to be intuitive.

### Basic Check
Run a check on the current directory. This will lint and check formatting.
```bash
dx check .
```

### Auto-Fix
Apply safe fixes (linting errors) and format files.
```bash
dx check --fix .
```
*Note: This will simplify your code and fix common issues automatically.*

### Watch Mode
Continuously watch for file changes and check them instantly.
```bash
dx check --watch
```

### CI Mode
Run in CI mode (fails on warnings, no interactive output).
```bash
dx check --ci
```

## Configuration

Dx Check requires no configuration to start, but can be customized via `dx.toml` or `biome.json` in your project root.

**Example `dx.toml`:**
```toml
[check]
ignore = ["dist", "node_modules"]

[check.javascript]
formatter.quoteStyle = "single"

[check.rust]
linter.level = "pedantic"
```

## Troubleshooting

### Common Issues

**Q: The linter is reporting errors on valid syntax.**
A: Ensure your language version matches the parser defaults. For TypeScript, ensure functionality is available in the target ECMAScript version.

**Q: VS Code is not showing diagnostics.**
A: Verify that the `dx-lsp` extension is installed and the binary is in your PATH.

### Debugging
Run with verbose logging enabled:
```bash
RUST_LOG=debug dx check .
```

## Contributing

We welcome contributions! Please see our [CONTRIBUTING.md](../../CONTRIBUTING.md) for details.

1. Fork the repository.
2. Create your feature branch (`git checkout -b feature/amazing-feature`).
3. Commit your changes (`git commit -m 'Add some amazing feature'`).
4. Push to the branch (`git push origin feature/amazing-feature`).
5. Open a Pull Request.

## License

Dx Check is licensed under the MIT License - see the [LICENSE](../../LICENSE) file for details.

