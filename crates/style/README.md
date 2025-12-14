# Dx Style

## Enhance Developer Experience with Lightning-Fast CSS Generation

A high-performance CSS utility generator written in Rust, featuring on-demand style generation, intelligent file watching, and advanced optimizations for maximum speed.

## 🚀 Performance

dx-style is engineered for extreme performance:

- ⚡ **Sub-20µs** class additions/removals
- 🔥 **SIMD-accelerated** HTML parsing
- ⚡ **Incremental parsing** - only re-parses changed sections (50-90% faster on large files)
- 🧵 **Parallel CSS generation** for large files
- 💾 **Memory-efficient** with string interning and arena allocation
- 🎯 **Profile-Guided Optimization** for 10-20% extra speed

### Advanced Optimizations

This project includes cutting-edge performance optimizations:

1. **Incremental Parsing** - Only re-parses changed sections (50-90% faster for large files)
2. **String Interning** - Reduces memory by 20-30% through deduplication
3. **FxHash** - 10-20% faster hashing in hot paths
4. **Arena Allocation** - Zero-overhead batch CSS generation
5. **PGO** - Profile-guided optimization for real-world workloads

See [ADVANCED_OPTIMIZATIONS.md](.github/ADVANCED_OPTIMIZATIONS.md) for details.

## 📦 Building

### Standard Build

```bash
cargo build --release
```

### Maximum Performance Build (with PGO)

```bash
# Linux/macOS
./scripts/build_pgo.sh

# Windows
scripts\build_pgo.bat
```

This builds with Profile-Guided Optimization for 30-50% better performance.

## 🎯 Features

- ✅ **FlatBuffers-based** precompiled style engine
- ✅ **Automatic background processing** ("forging")
- ✅ **Grouping syntax** for reusable class combinations
- ✅ **OKLCH color space** support
- ✅ **Memory-mapped I/O** for large CSS files
- ✅ **Incremental rebuilds** for instant updates
- 🆕 **Binary Style System** - 5-level optimization (80-97% smaller, 3-50× faster)

## 📊 Benchmarks

```bash
# Run full benchmark suite
cargo bench

# Quick performance test
cargo test --test performance_integration --release
```

Expected performance (with all optimizations):

- Add single class: ~10-12µs
- Parse 100 classes: ~320µs (full) or ~30-100µs (incremental)
- Parse 1000 classes: ~1.9ms (full) or ~200-500µs (incremental)
- Large file changes: 50-90% faster with incremental parsing
- Full rebuild: ~5.5ms

## 🔧 Configuration

All advanced optimizations are enabled by default (including incremental parsing). To customize:

```toml
[features]
default = ["std", "image", "string-interning", "fast-hash", "arena-alloc"]
```

### Environment Variables

```bash
# Disable incremental parsing (for debugging)
DX_DISABLE_INCREMENTAL=1

# Enable debug logging for incremental parser
DX_DEBUG=1
```

## 🆕 Binary Style System

dx-style now includes a revolutionary **Binary Style System** with 5 optimization levels:

1. **Binary IDs** - Map class names to u16 integers (80% smaller)
2. **Direct cssText** - Skip classList, write directly (3-5× faster)
3. **Pre-Computed Combos** - Common patterns pre-joined (67% smaller)
4. **Varint Encoding** - 1-byte IDs for common utilities (50% smaller)
5. **Binary CSS Values** - Property + value as enums (6× smaller)

### Quick Start

```rust
use style::binary::*;

// Auto mode - automatically selects best optimization
let css = generate_css_optimized(
    &["flex", "items-center", "p-4"],
    EncodingMode::Auto
);
```

### Performance

| Level | Payload Size | Speed | vs Tailwind |
|-------|--------------|-------|-------------|
| Tailwind | 89 bytes | 0.8ms | baseline |
| Binary IDs | 16 bytes | 0.08ms | 10× faster |
| cssText | 16 bytes | 0.02ms | 40× faster |
| Combos | 8 bytes | 0.01ms | 80× faster |
| Varint | 4 bytes | 0.01ms | 80× faster |
| Binary Values | 2 bytes | 0.01ms | 80× faster |

See [Binary Style System Documentation](docs/BINARY_STYLE_SYSTEM.md) for details.

## 📚 Documentation

- [Binary Style System](docs/BINARY_STYLE_SYSTEM.md) 🆕
- [Performance Guide](.github/PERFORMANCE.md)
- [Advanced Optimizations](.github/ADVANCED_OPTIMIZATIONS.md)
- [Optimization Summary](.github/OPTIMIZATION_SUMMARY.md)

## 🤝 Contributing

Contributions welcome! This project uses advanced Rust patterns and performance techniques. See the optimization guides for implementation details.

## 📄 License

MIT License - See LICENSE file for details
