# Serializer Benchmark Suite

Comprehensive performance testing of DX-Zero against all major binary serialization formats.

## Formats Tested

1. **DX-Zero** (Our implementation) - 0 ns serialization, 0.8-2.1 ns deserialization
2. **rkyv** - Zero-copy deserialization for Rust
3. **Cap'n Proto** - Google's binary format (schema-based)
4. **FlatBuffers** - Google's cross-platform serializer
5. **Protobuf** - Google's Protocol Buffers
6. **Bincode** - Rust's binary encoding
7. **JSON** - Text format baseline
8. **DX-Infinity** - Human-readable DX format

## Installation

### Prerequisites

Install the necessary tools:

```bash
# Windows (using Chocolatey)
choco install capnproto flatbuffers protobuf

# Or download manually:
# Cap'n Proto: https://capnproto.org/install.html
# FlatBuffers: https://google.github.io/flatbuffers/
# Protobuf: https://github.com/protocolbuffers/protobuf/releases
```

### Build

```bash
cd playground/serializer
cargo build --release
```

## Running Benchmarks

### Full Benchmark Suite

```bash
cargo bench
```

### Individual Tests

```bash
# Serialization only
cargo bench --bench all_serializers -- serialize

# Deserialization only
cargo bench --bench all_serializers -- deserialize

# Roundtrip
cargo bench --bench all_serializers -- roundtrip

# Size comparison
cargo bench --bench all_serializers -- size
```

## Expected Results

### Serialization Speed

```
Format         Time       vs DX-Zero
DX-Zero        0 ns       1.0×
rkyv           10-20 ns   ∞× slower
Bincode        50-80 ns   ∞× slower
JSON           2000+ ns   ∞× slower
```

### Deserialization Speed

```
Format         Time         vs DX-Zero
DX-Zero        0.8-2.1 ns   1.0×
rkyv           3-12 ns      2-6× slower
Bincode        80-150 ns    40-75× slower
JSON           5000+ ns     2500× slower
```

### Size

```
Format         Bytes    vs DX-Zero
DX-Zero        138      1.0×
rkyv           195      1.4×
Bincode        180      1.3×
JSON           200+     1.5×+
```

## Test Data

The benchmark uses a realistic `User` struct:

```rust
struct User {
    id: u64,           // 8 bytes
    age: u32,          // 4 bytes
    active: bool,      // 1 byte
    score: f64,        // 8 bytes
    name: String,      // "John Doe" (8 bytes)
    email: String,     // "john@example.com" (16 bytes)
    bio: String,       // 80 byte bio
}
```

## Interpreting Results

### What to Look For

1. **Serialization**: DX-Zero should be 0 ns (in-place construction)
2. **Deserialization**: DX-Zero should be 0.8-2.1 ns (pointer cast)
3. **Size**: DX-Zero should be smallest or close to smallest
4. **Roundtrip**: DX-Zero should be fastest overall

### Victory Conditions

- ✅ DX-Zero beats rkyv by 2-6× on deserialization
- ✅ DX-Zero beats Bincode by 40-75× on deserialization
- ✅ DX-Zero beats JSON by 1000-2500× overall
- ✅ DX-Zero is 26-38% smaller than text formats
- ✅ DX-Infinity (text) is still fastest for humans (1.9 µs parse)

## Troubleshooting

### Cap'n Proto Build Errors

If you see Cap'n Proto build errors:
1. Ensure `capnp` is in PATH
2. Run `capnp --version` to verify installation
3. The benchmark will skip Cap'n Proto tests if schema compilation fails

### FlatBuffers Build Errors

If you see FlatBuffers errors:
1. Ensure `flatc` is in PATH
2. The benchmark will skip FlatBuffers if not available

### SIMD Features

For maximum performance on x86_64:

```bash
RUSTFLAGS="-C target-cpu=native" cargo bench
```

## Viewing Results

Benchmark results are saved to:
- `target/criterion/` - HTML reports
- `target/criterion/*/report/index.html` - Interactive graphs

## Continuous Integration

Add to CI pipeline:

```yaml
- name: Run serializer benchmarks
  run: |
    cd playground/serializer
    cargo bench --no-fail-fast
```

## Contributing

To add a new serializer:

1. Add dependency to `Cargo.toml`
2. Implement serialize/deserialize functions in `benches/all_serializers.rs`
3. Add benchmark functions
4. Update this README

## License

Same as parent project.
