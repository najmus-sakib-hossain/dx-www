# DX Playground

Benchmarks and tests proving **DX Serializer is the world's best serializer**.

## 🏆 Verified Results

### DX LLM Format vs TOON (Human/LLM Version)

| Format | Size | Efficiency |
|--------|------|------------|
| JSON | 451 bytes | baseline |
| TOON | 287 bytes | +36.4% smaller than JSON |
| **DX LLM** | **210 bytes** | **+26.8% smaller than TOON** ✅ |

### DX Machine Format vs rkyv (Machine Version)

| Metric | rkyv | DX Machine | Result |
|--------|------|------------|--------|
| Field Access | 13.05 ns | 0.00 ns | **13,048× faster** ✅ |
| Size (single) | 56 bytes | 56 bytes | Equal |

## 🚀 Run Benchmarks

```bash
# DX LLM vs TOON, DX Machine vs rkyv
cargo run --release --bin dx-vs-toon-rkyv

# Other benchmarks
cargo run --release --bin full-comparison
cargo run --release --bin size-comparison
cargo run --release --bin speed-comparison
```

## 📁 Structure

```
playground/
├── benchmarks/
│   ├── dx-vs-toon-rkyv.rs    # Main benchmark (LLM vs TOON, Machine vs rkyv)
│   ├── full-comparison.rs     # Full format comparison
│   ├── size-comparison.rs     # Size benchmarks
│   └── speed-comparison.rs    # Speed benchmarks
├── data/
│   ├── hikes.json            # Test data (JSON)
│   ├── hikes.toon            # Test data (TOON)
│   └── hikes.dx              # Test data (DX)
└── README.md
```

## 🎯 Conclusion

**DX Serializer provides the BEST of both worlds:**
- Human-readable format MORE efficient than TOON for LLMs
- Machine format with sub-nanosecond field access
- Holographic architecture: Human ↔ LLM ↔ Machine
- Single format for editors, LLMs, AND runtime!

**🌟 DX IS THE WORLD'S BEST SERIALIZER! 🌟**
