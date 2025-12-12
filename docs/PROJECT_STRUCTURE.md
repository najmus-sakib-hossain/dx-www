# Project Structure

This document outlines the organization of the dx-www-runtime project.

## Repository Structure

```
dx-www-runtime/
├── .github/              # GitHub configuration
│   └── copilot-instructions.md
├── .vscode/              # VS Code workspace settings
│   └── settings.json
├── benchmarks/           # Performance benchmarks
│   ├── index.html
│   ├── benchmark-results.json
│   └── README.md
├── crates/               # Rust workspace crates
│   ├── dx-binary/        # Binary protocol implementation
│   ├── dx-cache/         # Browser caching layer
│   ├── dx-client/        # Full WASM runtime (7.5 KB)
│   ├── dx-client-tiny/   # Minimal WASM runtime (338 bytes)
│   ├── dx-compiler/      # TSX → .dxb compiler
│   ├── dx-core/          # Core abstractions
│   ├── dx-dom/           # DOM manipulation
│   ├── dx-morph/         # State management
│   ├── dx-packet/        # Network protocol
│   ├── dx-sched/         # Scheduler
│   └── dx-server/        # Server-side rendering
├── docs/                 # Documentation
│   ├── README.md         # Documentation index
│   ├── QUICKSTART.md     # Getting started guide
│   ├── ARCHITECTURE.md   # Technical architecture
│   ├── VICTORY.md        # Achievement report
│   ├── 48_HOUR_PLAN.md   # Development milestone
│   ├── LAUNCH_SUMMARY.md # Launch preparation
│   └── ...
├── examples/             # Example applications
│   └── hello-world/      # Minimal example
├── scripts/              # Build and utility scripts
│   ├── optimize-wasm.sh
│   ├── optimize-wasm.bat
│   └── demo.sh
├── test-app/             # Test application
│   └── src/
├── .clippy.toml          # Clippy linter configuration
├── .gitignore            # Git ignore patterns
├── Cargo.toml            # Workspace manifest
├── CONTRIBUTING.md       # Contribution guidelines
├── README.md             # Project README
└── rustfmt.toml          # Rust formatter configuration
```

## Crate Organization

### Core Runtime Crates

**dx-core** - Core abstractions and types  
- Memory management primitives
- Capability system
- Base traits

**dx-dom** - DOM manipulation layer  
- HTIP (Hybrid Template Instantiation Protocol) engine
- Template caching
- Node operations

**dx-morph** - State management  
- Dirty-bit tracking
- State updates
- Component lifecycle

**dx-sched** - Scheduler  
- Frame budget management
- RAF loop
- Priority queue

### Binary Protocol

**dx-binary** - Binary protocol implementation  
- Serialization/deserialization
- Template encoding
- Opcode definitions

**dx-packet** - Network protocol  
- Packet structure
- Delta updates
- Streaming

### Client Runtimes

**dx-client** - Full WASM runtime (7.5 KB Brotli)  
- Complete feature set
- State management
- Event system
- For complex applications (10+ components)

**dx-client-tiny** - Minimal WASM runtime (338 bytes Brotli)  
- Core rendering only
- No state management
- No event system
- For simple sites (< 10 components)

### Developer Tools

**dx-compiler** - TSX to .dxb compiler  
- Parser (regex-based MVP)
- Splitter (template extraction)
- Packer (binary serialization)
- CLI (build/dev/new commands)

### Server-Side (Optional)

**dx-server** - Server-side rendering  
- SSR engine
- Delta streaming
- WebSocket support

**dx-cache** - Browser caching  
- IndexedDB integration
- Cache API wrapper
- Signature verification

## Documentation Structure

```
docs/
├── README.md                 # Documentation index
├── QUICKSTART.md             # Getting started (NEW USERS START HERE)
├── ARCHITECTURE.md           # Technical deep-dive
├── VICTORY.md                # Achievement report (338 bytes!)
├── 48_HOUR_PLAN.md          # Development milestone
├── LAUNCH_SUMMARY.md        # Launch preparation
├── DEVELOPMENT.md           # Development workflow
├── COMPILER.md              # Compiler architecture
├── COMPILER-BUILD-SUMMARY.md # Compiler build process
├── ACHIEVEMENTS.md          # Project achievements
├── CHANGELOG.md             # Version history
├── FRAMEWORKS.md            # Framework comparisons
├── PROJECT_SUMMARY.md       # Project overview
└── STACK_COMPLETE.md        # Technology stack
```

## Build Artifacts

```
target/
├── debug/                # Debug builds
├── release/              # Release builds
├── wasm32-unknown-unknown/ # WASM targets
├── pkg_optimized/        # Optimized WASM output
│   ├── dx_client_bg.wasm      (7.5 KB Brotli)
│   └── dx_client_optimized.wasm
└── pkg_tiny/            # Tiny WASM output
    └── dx_client_tiny_bg.wasm (338 bytes Brotli)
```

## File Naming Conventions

### Rust Files
- `lib.rs` - Library entry point
- `main.rs` - Binary entry point
- `mod.rs` - Module definitions
- `{feature}.rs` - Feature implementation

### Documentation
- `README.md` - Overview/index
- `{TOPIC}.md` - Specific documentation (UPPERCASE)

### Configuration
- `.{tool}.toml` - Tool configuration
- `Cargo.toml` - Rust package manifest

### Scripts
- `{action}.sh` - Unix shell scripts
- `{action}.bat` - Windows batch scripts

## Development Workflow

1. **Make changes** in `crates/*/src/`
2. **Format code**: `cargo fmt --all`
3. **Check lints**: `cargo clippy --all-targets`
4. **Run tests**: `cargo test --all`
5. **Build WASM**: `cargo build --release --target wasm32-unknown-unknown`
6. **Optimize**: `./scripts/optimize-wasm.sh`

## Best Practices

### Code Organization
- Keep crates small and focused
- Use workspace dependencies
- Avoid circular dependencies

### Documentation
- Every public API must have doc comments
- Update docs/ when changing architecture
- Keep CHANGELOG.md current

### Testing
- Unit tests in same file as code
- Integration tests in `tests/` directory
- Benchmarks in `benches/` directory

### Git Workflow
- Feature branches from `main`
- Descriptive commit messages
- Keep commits atomic

## Quick Navigation

- **New to dx-www?** Start with [docs/QUICKSTART.md](docs/QUICKSTART.md)
- **Building the runtime?** See [crates/dx-client/](crates/dx-client/)
- **Building the compiler?** See [crates/dx-compiler/](crates/dx-compiler/)
- **Adding examples?** See [examples/](examples/)
- **Writing docs?** See [docs/README.md](docs/README.md)

## Questions?

See [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines.
