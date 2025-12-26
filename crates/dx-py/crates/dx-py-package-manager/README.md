# DX-Py Package Manager

Ultra-fast Python package manager workspace containing 5 crates.

## Crates

| Crate | Description |
|-------|-------------|
| `dx-py-core` | Core types, PEP 440 versions, wheel tags, binary formats |
| `dx-py-package-manager` | Cache, installer, resolver, async PyPI client |
| `dx-py-project-manager` | Python version, venv, workspace, and tool management |
| `dx-py-compat` | pyproject.toml, environment markers, configuration |
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
# Run all unit tests
cargo test --workspace

# Run property-based tests
cargo test --workspace -- --include-ignored proptest

# Run integration tests (requires network)
cargo test --test integration_tests -- --ignored

# Run benchmarks
cargo bench --package dx-py-cli
```

## Key Features

### PEP 440 Version Support
- Full version parsing (epoch, release, pre/post/dev, local)
- Correct version ordering per PEP 440 spec
- Version constraint evaluation

### Environment Markers (PEP 508)
- Platform detection (os, arch, Python version)
- Marker expression parsing and evaluation
- Conditional dependency filtering

### Wheel Support
- Wheel tag parsing and compatibility checking
- Platform-specific wheel selection
- Priority scoring for best wheel match

### Workspace Support
- Cargo-style monorepo management
- Glob pattern matching for members
- Path dependency resolution
- Shared dependency management
- Topological sorting for build order

### Real PyPI Integration
- Async HTTP client with connection pooling
- Package metadata and version fetching
- Wheel download with SHA256 verification
- Retry with exponential backoff

### Virtual Environment Management
- Real venv creation using Python's venv module
- Activation script generation (bash, zsh, fish, PowerShell)
- pip/setuptools bootstrapping

### Python Version Management
- python-build-standalone integration
- Cross-platform support (Windows, macOS, Linux)
- Version pinning per project

### Tool Management
- Isolated tool installation (pipx replacement)
- Ephemeral tool execution
- Tool upgrade support

### Configuration System
- Environment variable support
- Global config (~/.config/dx-py/config.toml)
- Project config (pyproject.toml [tool.dx-py])
- Layered precedence (env > project > global > default)

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

## Property-Based Tests

Using proptest for comprehensive validation:

1. PEP 440 Version Round-Trip
2. PEP 440 Version Ordering
3. PEP 508 Dependency Parsing
4. Marker Evaluation Consistency
5. Wheel Tag Parsing
6. Wheel Selection Priority
7. SHA256 Verification
8. Cleanup on Failure
9. Configuration Layering
10. Workspace Member Enumeration
11. Activation Script Validity

## Integration Tests

Real-world tests against PyPI (run with `--ignored`):

- Package resolution (requests, flask, numpy)
- Wheel download and installation
- Virtual environment creation
- Package uninstallation

## License

MIT OR Apache-2.0
