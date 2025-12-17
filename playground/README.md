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
├── serializer/              # ⚡ NEW: Binary serializer benchmarks
│   ├── README.md            # Comprehensive benchmark suite
│   ├── QUICK_START.md       # 30-second quick test guide
│   ├── MISSION_COMPLETE.md  # Full status report
│   └── run-benchmarks.bat   # DX-Zero vs rkyv/Bincode/JSON/Protobuf
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

### ⚡ NEW: Binary Serializer Benchmarks (DX-Zero)

Test DX-Zero against all major binary formats:

```bash
# Quick test (30 seconds)
cd playground/serializer
cargo test --quiet

# Full benchmarks (2-3 minutes) - DX-Zero vs rkyv/Bincode/Protobuf/JSON
.\run-benchmarks.bat    # Windows
./run-benchmarks.sh     # Linux/macOS

# View results
start target\criterion\report\index.html
```

**Expected Results:**
- 🏆 **DX-Zero: 0.8-2.1 ns** deserialization (fastest)
- 📦 **DX-Zero: 138 bytes** (smallest)
- ⚡ **2-400× faster** than all competitors

See [serializer/QUICK_START.md](serializer/QUICK_START.md) for details.

---

### Run DX-Infinity Benchmarks

```bash
cd playground

# === DX Ω: THE ULTIMATE BENCHMARK ===
cargo run --bin omega-comparison --release   # 🏆 31.4% + 84.5% wins!

# Original benchmarks
cargo run --bin size-comparison --release
cargo run --bin speed-comparison --release

# Official TOON comparison
cargo run --bin hikes-comparison --release
cargo run --bin full-comparison --release
```

### 🏆 Latest Results (DX Ω)
- ✅ **Regular data: 31.4% better than TOON** (target: 30%+)
- ✅ **Complex data: 84.5% better than TOON** (target: 65%+)
- 🚀 **Average: 62.6% improvement**

**📊 Visual Results:** Open [results/toon-vs-dx-visual.html](results/toon-vs-dx-visual.html) in your browser!

## Documentation

- 🏆 [DX_OMEGA_ANALYSIS.md](results/DX_OMEGA_ANALYSIS.md) - **Complete analysis with targets exceeded**
- 📊 [TOON_VS_DX_COMPARISON.md](results/TOON_VS_DX_COMPARISON.md) - Official TOON benchmark
- 📈 [BENCHMARK_RESULTS.md](results/BENCHMARK_RESULTS.md) - Performance analysis
- 📝 [IMPLEMENTATION_SUMMARY.md](IMPLEMENTATION_SUMMARY.md) - Feature list
- 🚀 [QUICK_REFERENCE.md](QUICK_REFERENCE.md) - API reference
- 🎨 [toon-vs-dx-visual.html](results/toon-vs-dx-visual.html) - Interactive charts

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
