# DX-Py: Ultra-Fast Python Package Manager

**DX-Py** is a high-performance Python package manager written in Rust, designed to be 5-50x faster than existing tools like uv. It achieves this through innovative binary formats, SIMD-accelerated version comparison, and zero-copy memory-mapped file access.

## Performance Highlights

Based on our benchmark suite:

| Operation | dx-py | Performance |
|-----------|-------|-------------|
| Version filtering (1000 versions) | 3.6-4.6 µs | **244M versions/sec** |
| Lock file read (1000 packages) | 98-110 µs | **9.6M packages/sec** |
| Package lookup (O(1) hash table) | 0.7 ms/1000 | **1.4M lookups/sec** |
| Resolution (500 packages × 100 versions) | 15-18 ms | **29K packages/sec** |

### Why DX-Py is Fast

1. **Binary Formats**: Custom DPP (package) and DPL (lock file) formats with zero-copy access
2. **SIMD Acceleration**: AVX2-optimized version comparison processes 8 versions in parallel
3. **Memory Mapping**: Lock files and packages are memory-mapped for instant access
4. **O(1) Lookups**: FNV-1a hash tables enable constant-time package lookup
5. **Content-Addressable Cache**: BLAKE3-hashed deduplication eliminates redundant storage
6. **Resolution Hint Cache**: Delta resolution for similar dependency sets

## Installation

```bash
# Build from source
cargo build --release --package dx-py-cli

# The binary will be at target/release/dx-py
```

## Usage

```bash
# Initialize a new project
dx-py init --python 3.12

# Add dependencies
dx-py add requests numpy pandas
dx-py add --dev pytest black

# Install dependencies
dx-py install

# Run commands in the virtual environment
dx-py run python main.py
dx-py run pytest

# Python version management
dx-py python install 3.12.0
dx-py python pin 3.12.0
dx-py python list

# Global tool management (pipx replacement)
dx-py tool install black
dx-py tool run ruff check .

# Build and publish
dx-py build
dx-py publish --token $PYPI_TOKEN

# Generate shell completions
dx-py completions bash > ~/.bash_completion.d/dx-py
dx-py completions zsh > ~/.zfunc/_dx-py
dx-py completions fish > ~/.config/fish/completions/dx-py.fish
dx-py completions powershell > dx-py.ps1
```

## Architecture

DX-Py consists of 5 crates:

```
crates/dx-py-package-manager/
├── dx-py-core/           # Core types, binary formats, SIMD operations
├── dx-py-package-manager/ # Cache, installer, resolver, PyPI client
├── dx-py-project-manager/ # Python/venv/workspace management
├── dx-py-compat/         # pyproject.toml, markers, configuration
└── dx-py-cli/            # Command-line interface
```

### Workspace Support

DX-Py supports Cargo-style monorepo workspaces:

```toml
# pyproject.toml
[tool.dx-py.workspace]
members = ["packages/*", "libs/*"]
exclude = ["packages/deprecated"]
shared_dependencies = { requests = ">=2.28" }
```

Features:
- Glob pattern matching for workspace members
- Path dependency resolution between members
- Shared dependency management across workspace
- Topological sorting for build order
- Editable/development mode installation

### Path Dependencies

Reference local packages using PEP 508 syntax or tool.dx-py configuration:

```toml
# PEP 508 style
[project]
dependencies = ["my-lib @ file://../my-lib"]

# Or dx-py style
[tool.dx-py.dependencies]
my-lib = { path = "../my-lib", editable = true }
```

### Binary Formats

**DPP (Dx Python Package)** - 64-byte header:
- Magic number, protocol version, flags
- Section offsets: metadata, files, bytecode, native, deps
- BLAKE3 integrity hash

**DPL (Dx Python Lock)** - O(1) lookup lock file:
- Hash table for instant package lookup
- 128-byte fixed entries with name, version, source hash
- Content hash for integrity verification

### Key Features

- **PubGrub-based Resolver**: Conflict detection with clear error messages
- **Hard Link Installation**: Near-instant installs from cache
- **Workspace Support**: Cargo-style monorepo management
- **Cached Venv Skeletons**: Fast virtual environment creation
- **PyPI Integration**: Parallel downloads with connection pooling
- **Environment Markers**: Full PEP 508 marker evaluation
- **Path Dependencies**: Local package references with editable mode
- **Configuration Layering**: env > project > global > default precedence

## Configuration

DX-Py supports layered configuration:

```toml
# ~/.config/dx-py/config.toml (global)
# or pyproject.toml [tool.dx-py] (project)

[tool.dx-py]
python_version = "3.12"
index_url = "https://pypi.org/simple"
extra_index_urls = ["https://my-private-pypi.com/simple"]
cache_dir = "~/.cache/dx-py"
max_concurrent_downloads = 8
```

Environment variables override config files:
- `DX_PY_PYTHON_VERSION`
- `DX_PY_INDEX_URL`
- `DX_PY_CACHE_DIR`

## Testing

```bash
# Run all 113 tests
cargo test --workspace

# Run benchmarks
cargo bench --package dx-py-cli
```

### Property-Based Tests

DX-Py includes comprehensive property-based tests using proptest:

1. **PEP 440 Version Round-Trip** - Version parsing and formatting
2. **PEP 440 Version Ordering** - Correct version comparison
3. **PEP 508 Dependency Parsing** - Dependency spec round-trip
4. **Marker Evaluation Consistency** - Environment marker evaluation
5. **Wheel Tag Parsing** - Wheel filename parsing
6. **Wheel Selection Priority** - Platform-specific wheel selection
7. **SHA256 Verification** - Download integrity verification
8. **Cleanup on Failure** - Atomic operations and rollback
9. **Configuration Layering** - Config precedence (env > project > global)
10. **Workspace Member Enumeration** - Glob pattern matching
11. **Activation Script Validity** - Shell script generation

### Integration Tests

Integration tests verify real-world functionality (require network):

```bash
# Run integration tests
cargo test --test integration_tests -- --ignored
```

- PyPI package resolution (requests, flask, numpy)
- Wheel download and installation
- Virtual environment creation
- Package uninstallation

## Comparison with uv

| Feature | dx-py | uv |
|---------|-------|-----|
| Binary lock format | ✅ DPL (O(1) lookup) | ❌ TOML |
| Binary package format | ✅ DPP (zero-copy) | ❌ Wheel |
| SIMD version comparison | ✅ AVX2 | ❌ |
| Resolution hint cache | ✅ Delta resolution | ❌ |
| Memory-mapped files | ✅ | ✅ |
| Content-addressable cache | ✅ BLAKE3 | ✅ |

## License

MIT OR Apache-2.0

## Contributing

Contributions are welcome! Please ensure all tests pass before submitting PRs:

```bash
cargo test --workspace
cargo clippy --workspace
cargo fmt --check
```
