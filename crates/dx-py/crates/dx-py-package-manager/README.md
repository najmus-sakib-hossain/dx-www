# DX-Py Package Manager

Ultra-fast Python package manager workspace containing 5 crates.

## Crates

| Crate | Description |
|-------|-------------|
| `dx-py-core` | Core types, binary formats (DPP/DPL), SIMD version comparison |
| `dx-py-package-manager` | Cache, installer, resolver, PyPI registry client |
| `dx-py-project-manager` | Python version, venv, and workspace management |
| `dx-py-compat` | pyproject.toml parser and binary conversion |
| `dx-py-cli` | Command-line interface |

## Building

```bash
# Debug build
cargo build --workspace

# Release build
cargo build --release --workspace

# Build CLI only
cargo build --release --package dx-py-cli
```

## Testing

```bash
# Run all tests (113 total)
cargo test --workspace

# Run benchmarks
cargo bench --package dx-py-cli
```

## Performance

Benchmark results (release build):

- **Version filtering**: 244M versions/sec (SIMD-accelerated)
- **Lock file read**: 9.6M packages/sec (memory-mapped)
- **Package lookup**: 1.4M lookups/sec (O(1) hash table)
- **Resolution**: 29K packages/sec (PubGrub + hint cache)

## Binary Formats

### DPP (Dx Python Package)
- 64-byte packed header
- Zero-copy access via memory mapping
- BLAKE3 integrity verification
- Sections: metadata, files, bytecode, native, dependencies

### DPL (Dx Python Lock)
- O(1) package lookup via FNV-1a hash table
- 128-byte fixed entries
- Content hash for integrity
- Memory-mapped for instant access

## License

MIT OR Apache-2.0
