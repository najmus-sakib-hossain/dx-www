# Development Guide

## Development Setup

### 1. Install Tools

```bash
# Rust toolchain (Edition 2024)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
rustup update

# WASM target
rustup target add wasm32-unknown-unknown

# wasm-bindgen CLI
cargo install wasm-bindgen-cli

# Optional: cargo-watch for auto-rebuild
cargo install cargo-watch

# Optional: wasm-pack for easier builds
cargo install wasm-pack
```

### 2. VS Code Extensions (Recommended)

- `rust-analyzer`: Rust language server
- `vadimcn.vscode-lldb`: Debugging
- `tamasfe.even-better-toml`: TOML syntax
- `serayuzgur.crates`: Dependency management

### 3. Clone and Build

```bash
git clone <repository-url>
cd dx-www-runtime
cargo build --workspace
```

## Project Structure

```
dx-www-runtime/
├── crates/
│   ├── dx-core/          # Memory management
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs
│   ├── dx-dom/           # DOM rendering
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs
│   ├── dx-morph/         # State patching
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs
│   └── dx-sched/         # Scheduling
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs
├── examples/
│   └── hello-world/      # Proof of concept
│       ├── Cargo.toml
│       ├── src/
│       │   └── lib.rs
│       ├── index.html
│       ├── build.sh
│       └── README.md
├── Cargo.toml            # Workspace manifest
└── README.md
```

## Development Workflow

### Building Individual Crates

```bash
# Build a specific crate
cargo build -p dx-core
cargo build -p dx-dom --release

# Check all crates
cargo check --workspace

# Run clippy
cargo clippy --workspace --all-targets

# Format code
cargo fmt --all
```

### Building WASM

```bash
# Build for WASM target
cargo build --target wasm32-unknown-unknown --release

# Generate bindings (manual)
wasm-bindgen --target web \
  --out-dir examples/hello-world/pkg \
  target/wasm32-unknown-unknown/release/hello_world.wasm

# Or use the build script
cd examples/hello-world
./build.sh  # Unix
build.bat   # Windows
```

### Watch Mode (Auto-rebuild)

```bash
# Watch for changes and rebuild
cargo watch -x "check --workspace"

# Watch and run tests
cargo watch -x "test --workspace"
```

### Testing

```bash
# Run all tests
cargo test --workspace

# Run tests for specific crate
cargo test -p dx-core

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_dirty_mask
```

### Benchmarking

```bash
# Run benchmarks (when implemented)
cargo bench --workspace

# Profile with flamegraph
cargo install flamegraph
cargo flamegraph --bench my_bench
```

## Debugging

### Debug WASM in Browser

1. Build with debug symbols:
   ```bash
   cargo build --target wasm32-unknown-unknown
   wasm-bindgen --target web --debug --keep-debug \
     --out-dir pkg target/wasm32-unknown-unknown/debug/hello_world.wasm
   ```

2. Open DevTools → Sources → WebAssembly code

### Debug Rust Code

```bash
# Run with debug output
RUST_LOG=debug cargo test

# Use rust-lldb (macOS/Linux)
rust-lldb target/debug/my_binary

# Use rust-gdb (Linux)
rust-gdb target/debug/my_binary
```

### Console Logging in WASM

```rust
use web_sys::console;

console::log_1(&"Hello from WASM".into());
console::warn_1(&format!("Value: {}", x).into());
console::error_1(&"Something went wrong".into());
```

## Code Guidelines

### Adding New Features

1. **Start with dx-core** if it involves memory layout
2. **Update dx-dom** if it involves rendering
3. **Update dx-morph** if it involves state management
4. **Update dx-sched** if it involves scheduling

### Performance Testing

Always measure before and after:

```rust
use web_sys::window;

let perf = window().unwrap().performance().unwrap();
let start = perf.now();

// Your code here

let elapsed = perf.now() - start;
console::log_1(&format!("Took: {}ms", elapsed).into());
```

### Memory Safety Checklist

For every `unsafe` block:
- [ ] Document safety invariant
- [ ] Verify pointer alignment
- [ ] Check buffer bounds
- [ ] Consider race conditions
- [ ] Add test coverage

Example:
```rust
/// # Safety
/// `ptr` must point to valid memory of at least `len` bytes
/// and must be properly aligned for type T.
pub unsafe fn cast_slice<T>(ptr: *const u8, len: usize) -> &[T] {
    let ptr = ptr as *const T;
    std::slice::from_raw_parts(ptr, len / std::mem::size_of::<T>())
}
```

## Common Tasks

### Add a New Dependency

```bash
# Add to workspace dependencies in root Cargo.toml
# Then use in crate Cargo.toml:
[dependencies]
my-crate.workspace = true
```

### Create a New Example

```bash
mkdir -p examples/my-example/src
cd examples/my-example

# Create Cargo.toml
cat > Cargo.toml << EOF
[package]
name = "my-example"
version.workspace = true
edition.workspace = true

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen.workspace = true
dx-core.workspace = true
dx-dom.workspace = true
dx-morph.workspace = true
dx-sched.workspace = true
EOF

# Add to workspace members in root Cargo.toml
```

### Update web-sys Features

In root [Cargo.toml](cci:1://file:///f:/Code/dx-www-runtime/Cargo.toml:0:0-0:0):
```toml
web-sys = { version = "0.3", features = [
    "Window",
    "Document",
    # Add new features here
    "YourNewFeature",
] }
```

## Troubleshooting

### "error: no override and no default toolchain set"
```bash
rustup default stable
```

### "wasm-bindgen command not found"
```bash
cargo install wasm-bindgen-cli
```

### "cannot find -lclang"
Install LLVM:
```bash
# Ubuntu/Debian
sudo apt install llvm-dev libclang-dev

# macOS
brew install llvm

# Windows
# Download from https://releases.llvm.org/
```

### WASM binary too large
```toml
# In Cargo.toml
[profile.release]
opt-level = "z"      # Optimize for size
lto = true           # Link-time optimization
strip = true         # Strip symbols
```

### Performance issues
1. Use `--release` build
2. Profile with browser DevTools
3. Check for allocations in hot paths
4. Verify dirty bits are being used correctly

## Resources

- [Rust WASM Book](https://rustwasm.github.io/book/)
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)
- [web-sys Documentation](https://rustwasm.github.io/wasm-bindgen/api/web_sys/)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)

## Getting Help

1. Check [ARCHITECTURE.md](ARCHITECTURE.md) for design decisions
2. Read [CONTRIBUTING.md](CONTRIBUTING.md) for code standards
3. Open GitHub Discussion for questions
4. File GitHub Issue for bugs

---

Happy coding! Remember: **Binary Everywhere. Zero Parse. Zero GC. Zero Hydration.** 🚀
