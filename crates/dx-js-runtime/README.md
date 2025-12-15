# dx-js-runtime

**The fastest JavaScript/TypeScript runtime** - targeting 10x faster performance than Bun.

## Architecture

```
TypeScript/JavaScript Source
        ↓
OXC Parser (fastest parser in existence)
        ↓
Type Solver (full program analysis)
        ↓
Typed MIR (every value has exact type)
        ↓
Cranelift JIT → Native Machine Code
        ↓
Immortal Cache (compiled code persists)
        ↓
Zero-Alloc Executor (no GC pauses)
```

## Key Technologies

| Component | Technology | Benefit |
|-----------|------------|---------|
| Parser | OXC | 2-3x faster than swc |
| JIT Compiler | Cranelift | Native code, no bytecode |
| Memory | Arena Allocator | Zero GC pauses |
| Cache | Blake3 + mmap | Instant cold starts |
| Values | NaN-boxing | Efficient primitives |

## Performance Targets

| Metric | Bun | dx-js-runtime Target |
|--------|-----|----------------------|
| Cold start | 28ms | <3ms |
| Parse speed | 1x | 2-3x faster |
| JIT warmup | Required | Instant (cached) |
| Memory usage | 100MB+ | <50MB |
| GC pauses | Yes | None |

## Usage

```bash
# Run a JavaScript file
dx-js script.js

# Run a TypeScript file
dx-js app.ts

# Show version
dx-js --version

# Show help
dx-js --help
```

## Build

```bash
# Build the runtime
cargo build -p dx-js-runtime --release

# Run tests
cargo test -p dx-js-runtime

# Run benchmarks
cargo bench -p dx-js-runtime
```

## Benchmark vs Bun

```bash
# Run with dx-js-runtime
cargo run -p dx-js-runtime --release -- playground/test.ts

# Run with Bun
bun run playground/test.ts
```

## Project Structure

```
crates/dx-js-runtime/
├── Cargo.toml
├── src/
│   ├── lib.rs           # Main entry point
│   ├── error.rs         # Error types
│   ├── compiler/
│   │   ├── mod.rs       # Compiler orchestration
│   │   ├── parser.rs    # OXC integration
│   │   ├── type_solver.rs
│   │   ├── mir.rs       # Typed IR
│   │   ├── codegen.rs   # Cranelift JIT
│   │   └── optimize.rs
│   ├── runtime/
│   │   ├── mod.rs       # Execution context
│   │   ├── memory.rs    # Arena allocator
│   │   └── builtins.rs  # console, JSON, etc.
│   ├── value/
│   │   ├── mod.rs       # Value types
│   │   ├── object.rs
│   │   ├── string.rs    # Interned strings
│   │   └── tagged.rs    # NaN-boxed values
│   ├── snapshot/
│   │   ├── mod.rs
│   │   └── immortal.rs  # Persistent cache
│   └── bin/
│       └── main.rs      # CLI
└── benches/
    └── runtime.rs       # Criterion benchmarks
```

## License

MIT OR Apache-2.0
