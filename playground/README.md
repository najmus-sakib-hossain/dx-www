# DX Serializer Playground

This playground demonstrates the efficiency gains of **DX Machine Format** over TOON, JSON, and YAML.

## Directory Structure

```
playground/
├── data/                    # Test datasets
│   ├── simple.json
│   ├── simple.toon
│   ├── simple.dx
│   ├── complex.json
│   ├── complex.toon
│   └── complex.dx
├── examples/                # Usage examples
│   ├── toon-example.js
│   ├── dx-machine-example.rs
│   └── dx-human-example.rs
├── benchmarks/              # Performance tests
│   ├── size-comparison.rs
│   └── speed-comparison.rs
└── results/                 # Benchmark outputs
    ├── size-results.md
    └── speed-results.md
```

## Quick Start

### Run Size Comparison
```bash
cd playground/benchmarks
cargo run --bin size-comparison --release
```

### Run Speed Benchmark
```bash
cd playground/benchmarks
cargo run --bin speed-comparison --release
```

## Target Goals

- **Size Efficiency:** 65%+ smaller than TOON
- **Parse Speed:** 65%+ faster than TOON  
- **Encode Speed:** 65%+ faster than TOON

## Key Innovations

1. **Vacuum Parsing:** No quotes needed for strings
2. **Vertical Compression:** Ditto (`_`) eliminates repetition
3. **Schema-Guided:** Type hints enable zero-copy parsing
4. **Alias System:** Compress repeated keys
5. **SIMD-Accelerated:** Uses `memchr` for fast tokenization
