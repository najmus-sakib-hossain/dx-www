# DX CLI

**The Binary-First Development Experience**

A modern, high-performance command-line interface that provides unified control over the entire DX development platform. Built with Rust for maximum performance and a clean, Vercel-inspired user experience.

```
  ▲  DX v0.1.0

  Build faster. Ship smaller. Zero compromise.
```

## Overview

DX CLI is the central orchestration tool for the DX ecosystem—a comprehensive suite of development tools designed around binary-first principles. It provides a single entry point to manage assets, infrastructure, and development workflows with consistent, beautiful terminal output.

## Hardening Features

DX CLI includes comprehensive production hardening for reliability and robustness:

### Error Handling & Recovery
- Automatic retry with exponential backoff (1s, 2s, 4s) for transient network failures
- Comprehensive error messages with actionable hints and remediation steps
- Crash reporting with diagnostic information saved to `~/.dx/crash-reports/`
- Graceful signal handling (Ctrl+C) with resource cleanup

### Cross-Platform Support
- Unicode path handling (emoji, CJK, RTL scripts)
- Windows long path support (>260 characters)
- Platform-native path separator normalization
- Symlink resolution with loop detection (40 levels max)

### Network Resilience
- Proxy support via HTTP_PROXY, HTTPS_PROXY, NO_PROXY environment variables
- Offline mode detection with graceful degradation
- Resumable downloads for large files (>1MB)
- TLS certificate error guidance

### Configuration Safety
- Atomic config saves with backup (.bak files)
- Config validation with detailed error reporting
- Global + local config merging with clear precedence
- Unknown field warnings for forward compatibility

### Resource Management
- Process limiting to prevent resource exhaustion (default: 4 concurrent)
- Automatic temp file cleanup on exit
- Graceful child process termination (SIGTERM → SIGKILL)
- Disk space warnings (<100MB)

### Logging & Diagnostics
- Verbose mode (`--verbose`) with timing information
- Quiet mode (`--quiet`) for scripts
- CI mode with JSON output (auto-detected via CI env var)
- Log rotation (10MB max, 5 rotations kept)
- Debug mode via DX_DEBUG=1 environment variable

## Installation

```bash
# From crates.io (when published)
cargo install dx

# From source
cargo build --release -p dx
```

## Architecture

```
crates/dx/
├── src/
│   ├── main.rs           # Entry point with async runtime
│   ├── cli.rs            # Command definitions and routing
│   ├── commands/         # 10 core tool implementations
│   │   ├── style.rs      # Binary CSS compiler
│   │   ├── media.rs      # Image/video optimization
│   │   ├── font.rs       # Font subsetting
│   │   ├── icon.rs       # SVG icon system
│   │   ├── forge.rs      # Package manager + orchestrator
│   │   ├── serializer.rs # World-record data format
│   │   ├── stack.rs      # Language-aware dev stack
│   │   ├── driven.rs     # AI agents control
│   │   ├── generator.rs  # Code generation tools
│   │   └── workspace.rs  # IDE + setup tools
│   ├── config/           # Configuration management
│   ├── prompts/          # Interactive CLI prompts
│   ├── templates/        # Project templates
│   ├── ui/               # Terminal UI components
│   │   ├── theme.rs      # Consistent styling
│   │   ├── spinner.rs    # Progress indicators
│   │   ├── progress.rs   # Progress bars
│   │   ├── table.rs      # Data tables
│   │   └── logger.rs     # Logging utilities
│   └── utils/            # Path and system utilities
└── Cargo.toml
```

## The 10 Core Tools

DX CLI organizes functionality into three categories:

### Asset Tools

| Command | Alias | Description |
|---------|-------|-------------|
| `dx style` | `css` | Binary CSS (B-CSS) compiler—98% smaller, 80x faster |
| `dx media` | `img` | Image/video optimization—WebP, AVIF, responsive srcsets |
| `dx font` | — | Font subsetting and WOFF2 optimization |
| `dx icon` | — | SVG icon system with binary encoding |

### Infrastructure

| Command | Alias | Description |
|---------|-------|-------------|
| `dx forge` | `f` | Package manager + orchestrator for all dx-* crates |
| `dx serializer` | `ser`, `data` | World-record data format (DX ∞)—73% smaller, 4x faster |
| `dx stack` | `js`, `ts` | Unified JS/TS development stack |

### Development

| Command | Alias | Description |
|---------|-------|-------------|
| `dx driven` | `ai` | AI agents control—review, refactor, test generation |
| `dx generator` | `gen`, `g` | Code generation tools—components, APIs, forms, CRUD |
| `dx workspace` | `ws`, `ide` | Code editors + preinstall and setup |

## Command Reference

### dx style — Binary CSS Compiler

```bash
# Build binary CSS from source
dx style build --input styles.css --output styles.bcss

# Analyze CSS usage in project
dx style analyze ./src

# Show style statistics
dx style stats
```

**Performance**: 98% smaller output, 80x faster application than traditional CSS.

### dx media — Image/Video Optimization

```bash
# Optimize images to WebP/AVIF
dx media optimize ./images --format webp --quality 85

# Generate responsive image variants
dx media srcset hero.png --widths 320,640,960,1280,1920

# Generate blur placeholder (LQIP)
dx media placeholder hero.png --size 32

# Extract video preview frames
dx media preview video.mp4 --frames 5

# Analyze and suggest optimizations
dx media analyze ./public
```

### dx font — Font Optimization

```bash
# Subset fonts to used characters
dx font subset ./fonts --chars "used-chars.txt"

# Convert to WOFF2
dx font convert input.ttf --output output.woff2

# Analyze font usage
dx font analyze ./src
```

### dx icon — SVG Icon System

```bash
# Build icon sprite
dx icon build ./icons --output sprite.svg

# Optimize individual icons
dx icon optimize ./icons

# Generate icon component
dx icon component ./icons --framework react
```

### dx forge — Package Manager + Orchestrator

```bash
# Show status of all dx tools in project
dx forge status --verbose

# List all available dx tools
dx forge list --category assets

# Install a dx tool
dx forge install dx-media

# Update all tools
dx forge update --all

# Check compatibility
dx forge check --fix

# Orchestrate full build
dx forge build --config release --target vercel

# Show dependency graph
dx forge graph

# Analyze tool usage
dx forge analyze
```

**Available Tools**:
- **Runtime**: dx-core, dx-client, dx-binary, dx-state
- **Assets**: dx-style, dx-media, dx-font, dx-icon
- **Data**: dx-form, dx-query, dx-db, dx-serializer
- **Security**: dx-auth, dx-a11y
- **Network**: dx-sync, dx-offline, dx-server
- **JavaScript**: dx-js-runtime, dx-js-bundler, dx-js-test-runner, dx-js-package-manager

### dx serializer — World-Record Data Format

```bash
# Encode data to DX binary format
dx serializer encode data.json --output data.dxb --stats

# Decode DX binary to readable format
dx serializer decode data.dxb --format json

# Compare formats (JSON vs DX)
dx serializer compare data.json

# Benchmark performance
dx serializer bench data.json --iterations 1000

# Validate DX binary file
dx serializer validate data.dxb

# Generate schema from data
dx serializer schema data.json --output schema.dxs
```

**Performance**:
- 37.2% smaller than TOON (previous record holder)
- 73.4% smaller than JSON
- ~1.9µs parse speed (4-5x faster than JS parsers)
- Zero-copy deserialization

### dx stack — Language-Aware Development Stack

```bash
# Run a file
dx stack run app.ts --watch

# Start REPL
dx stack repl

# Bundle for production
dx stack bundle src/index.ts --output dist/bundle.js --minify --sourcemap

# Start development server
dx stack dev --port 3000

# Build for production
dx stack build --output dist

# Run tests
dx stack test --watch --coverage

# Package management
dx stack install
dx stack add react --dev
dx stack remove lodash
dx stack update

# Initialize new project
dx stack init --template default

# Monorepo commands
dx stack workspace init
dx stack workspace list
dx stack workspace run-all build
dx stack workspace graph
dx stack workspace add @app/new-package

# Check compatibility
dx stack compatibility es2022

# Show stack info
dx stack info
dx stack languages
```

**Supported Languages**:

| Language | Runtime | Package Manager | Bundler | Monorepo | Compat | Test |
|----------|---------|-----------------|---------|----------|--------|------|
| JavaScript/TS | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Python | ✓ | ✓ | — | ✓ | ✓ | ✓ |
| Rust | (cargo) | (cargo) | (cargo) | (cargo) | (cargo) | (cargo) |
| Go | (go) | (go) | (go) | (go) | (go) | (go) |

### dx driven — AI Agents Control

```bash
# Start an AI agent session
dx driven start review --target ./src

# List running agents
dx driven list

# AI code review
dx driven review ./src --depth deep

# AI-powered refactoring
dx driven refactor ./src/utils.ts --goal "improve performance" --dry-run

# Generate tests with AI
dx driven test ./src/api.ts --framework vitest

# Generate documentation
dx driven docs ./src --format markdown

# Security audit
dx driven audit ./

# Chat with AI about codebase
dx driven chat "How does the authentication flow work?"

# Configure AI settings
dx driven config --model gpt-4o

# Show agent status
dx driven status
```

### dx generator — Code Generation Tools

```bash
# Generate component
dx generator component Button --kind functional --with-test --with-style

# Generate API endpoint
dx generator api users --methods get,post,put,delete --with-validation

# Generate database model
dx generator model User --fields "id:uuid,name:string,email:string" --with-migration

# Generate form from schema
dx generator form ContactForm --schema schema.json --with-validation

# Generate types from schema
dx generator types schema.json --output types/generated.ts --format json-schema

# Generate database migration
dx generator migration add_email_column --auto

# Generate CRUD operations
dx generator crud Product --full

# Generate from template
dx generator template react-component MyComponent

# List available generators
dx generator list
```

### dx workspace — IDE + Setup Tools

```bash
# Setup development environment
dx workspace setup

# Configure VS Code
dx workspace vscode

# Install recommended extensions
dx workspace extensions

# Initialize project configuration
dx workspace init
```

## Global Options

All commands support these global flags:

```bash
dx [command] --verbose    # Enable verbose output with timing
dx [command] --quiet      # Suppress all output except errors
dx [command] --no-color   # Disable colored output
```

### Environment Variables

| Variable | Description |
|----------|-------------|
| `NO_COLOR` | Disable colored output |
| `DX_DEBUG=1` | Enable debug mode with timing |
| `DX_OFFLINE=1` | Force offline mode |
| `CI` | Auto-detected for JSON log output |
| `HTTP_PROXY` | HTTP proxy URL |
| `HTTPS_PROXY` | HTTPS proxy URL |
| `NO_PROXY` | Comma-separated hosts to bypass proxy |

## Configuration

DX CLI reads configuration from `dx.toml` in your project root:

```toml
[project]
name = "my-dx-app"
version = "0.1.0"
edition = "2024"
target = "web"

[tools]
dx-core = "0.1.0"
dx-style = "0.1.0"
dx-client = "0.1.0"
dx-server = "0.1.0"

[style]
optimization = "release"
source_maps = false

[media]
format = "webp"
quality = 85
```

## Dependencies

### Core Dependencies

- **clap** (4.5) — CLI framework with derive macros
- **tokio** (1.40) — Async runtime with full features
- **serde** / **serde_json** / **toml** — Serialization

### Terminal UI

- **console** (0.15) — Terminal utilities
- **indicatif** (0.17) — Progress bars and spinners
- **dialoguer** (0.11) — Interactive prompts
- **owo-colors** (4.0) — Terminal colors

### File System

- **walkdir** (2.5) — Directory traversal
- **notify** (6.1) — File watching
- **ignore** (0.4) — Gitignore-aware file matching
- **glob** (0.3) — Glob pattern matching

### Utilities

- **anyhow** / **thiserror** — Error handling
- **tracing** — Structured logging
- **chrono** — Date/time handling
- **which** — Executable discovery

### Internal Crates

- **stack** — Language stack abstractions

## Development

```bash
# Build the CLI
cargo build -p dx

# Run in development
cargo run -p dx -- --help

# Run tests (includes property-based tests)
cargo test -p dx

# Run with verbose output
cargo run -p dx -- forge status --verbose

# Run with debug timing
DX_DEBUG=1 cargo run -p dx -- build
```

## Testing

DX CLI uses a dual testing approach:

- **Unit tests**: Verify specific examples and edge cases
- **Property-based tests**: Verify universal properties across all inputs using `proptest`

All property tests run with minimum 100 iterations and are tagged with requirement references for traceability.

## Design Principles

1. **Binary-First**: All tools prioritize compact binary formats over text
2. **Zero Compromise**: Maximum performance without sacrificing developer experience
3. **Unified Interface**: One CLI to rule all development tools
4. **Beautiful Output**: Consistent, Vercel-inspired terminal aesthetics
5. **Language Aware**: Respects native toolchains (Cargo, Go) where appropriate

## License

See workspace root for license information.
