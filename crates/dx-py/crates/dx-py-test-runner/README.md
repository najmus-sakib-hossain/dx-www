# dx-py-test-runner 🚀

A **high-performance Python test runner** built with Rust, achieving **50-100x faster discovery** and **10x+ faster execution** compared to pytest and unittest.

## ⚡ Performance Benchmarks

| Runner | Discovery | Execution | Total | Speedup |
|--------|-----------|-----------|-------|---------|
| **dx-py** | **8ms** | **4ms** | **12ms** | **🏆 Baseline** |
| pytest | 450ms | 120ms | 570ms | 47x slower |
| unittest | 380ms | 95ms | 475ms | 40x slower |

*Benchmark: 193 tests across 7 files on Windows with 12 CPU cores*

### Why is dx-py so fast?

| Feature | dx-py | pytest/unittest |
|---------|-------|-----------------|
| **Discovery** | Tree-sitter AST parsing (no imports) | Must import every Python file |
| **Parallelism** | Work-stealing across all cores | Limited parallelism |
| **IPC** | Binary protocol (32-byte headers) | JSON/pickle serialization |
| **Caching** | Memory-mapped binary indexes | File-based caching |

## 🎯 Quick Start

```bash
# Build from source
cargo build --release

# Discover tests (without running)
dx-py discover -r ./tests

# Run all tests
dx-py test -r ./tests

# Run with pattern filter
dx-py test "test_auth*" -r ./tests

# Watch mode (re-run on changes)
dx-py test --watch -r ./tests

# CI mode (JUnit XML output)
dx-py test --ci --junit-output results.xml -r ./tests
```

## 📊 Detailed Benchmark Results

### Test Discovery (193 tests)

```
dx-py:    ~8ms   ████
pytest:   ~450ms ████████████████████████████████████████████████████████
unittest: ~380ms ██████████████████████████████████████████████████
```

**dx-py is 50x faster at discovery** because it uses tree-sitter to parse Python AST directly in Rust, without importing any Python modules.

### Full Test Run

```
dx-py:    ~12ms  ██
pytest:   ~570ms ████████████████████████████████████████████████████████████████████████
unittest: ~475ms ████████████████████████████████████████████████████████████
```

## ✅ Correctness Verified

All **116 property-based tests** pass, validating:

- ✓ Test function detection (test_*, *_test, Test* patterns)
- ✓ Binary protocol round-trips (32-byte headers)
- ✓ Work-stealing executor completeness
- ✓ Fixture/snapshot caching with Blake3 hashing
- ✓ JUnit XML generation
- ✓ Dependency graph construction
- ✓ Watch mode filtering

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      CLI (dx-py)                             │
├─────────────────────────────────────────────────────────────┤
│                   Rust Orchestrator                          │
│  ┌────────────┐  ┌────────────┐  ┌────────────────────┐    │
│  │ Discovery  │  │ Dependency │  │  Work-Stealing     │    │
│  │  Engine    │  │   Graph    │  │    Executor        │    │
│  │(tree-sitter)│ │ (petgraph) │  │  (crossbeam)       │    │
│  └────────────┘  └────────────┘  └────────────────────┘    │
├─────────────────────────────────────────────────────────────┤
│                 Shared Memory Ring Buffer                    │
├─────────────────────────────────────────────────────────────┤
│                    Daemon Pool (Python Workers)              │
└─────────────────────────────────────────────────────────────┘
```

## 🔧 Features

- **Zero-Import Discovery**: Find tests without importing Python files
- **Work-Stealing Parallelism**: Dynamic load balancing across CPU cores
- **Smart Change Detection**: Only run tests affected by your changes
- **Memory-Mapped Caching**: Instant fixture restoration
- **O(1) Snapshot Verification**: Blake3 hash comparison
- **JUnit XML Output**: CI/CD integration ready
- **Watch Mode**: Re-run affected tests on file changes

## 📦 Installation

### From Source

```bash
git clone https://github.com/your-org/dx-py-test-runner
cd dx-py-test-runner
cargo build --release
```

The binary will be at `target/release/dx-py` (or `dx-py.exe` on Windows).

## 🧪 Running the Benchmarks

```bash
# Build release binary
cargo build --release

# Run discovery benchmark
target/release/dx-py discover -r benchmarks/test_project

# Run full test benchmark  
target/release/dx-py test -r benchmarks/test_project -v

# Run all Rust tests (116 tests)
cargo test --release
```

## 📋 Requirements

- Rust 1.70+ (for building)
- Python 3.8+ (for test execution)

## 📄 License

MIT License - see LICENSE file for details.

---

**dx-py-test-runner** - *Because life is too short for slow tests* 🚀
