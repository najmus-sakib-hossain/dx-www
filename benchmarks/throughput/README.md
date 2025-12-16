# dx-server Throughput Benchmark Results

**Target:** Beat Actix Web's ~1,200,000 RPS

## Benchmark Servers

### 1. Axum Plaintext Server (`plaintext_server`)
- Based on Axum framework (same foundation as dx-server)
- Single `/plaintext` route returning "Hello, World!"
- Uses async handlers with Tokio runtime

### 2. Raw Hyper Server (`hyper_raw_server`)
- Bypasses Axum routing for maximum throughput
- Zero-allocation response path
- Pre-computed response headers

## Running Benchmarks

```bash
# Linux/macOS (recommended)
chmod +x run_benchmark.sh
./run_benchmark.sh

# Windows (WSL recommended for accurate results)
wsl bash run_benchmark.sh
```

## Required Tools

Install at least one of:
- **wrk**: `apt install wrk` (Linux) / `brew install wrk` (macOS)
- **rewrk**: `cargo install rewrk` (Rust-based, cross-platform)

## Expected Results

| Server Type | Expected RPS | Comparison |
|-------------|--------------|------------|
| Axum Plaintext | ~800K-1M | Good for real apps with routing |
| Raw Hyper | ~1.2M+ | Maximum baseline |
| Actix Web | ~1.2M | Industry benchmark |

## Results (to be filled)

See `results/` directory for timestamped benchmark results.

## Notes

1. **Linux recommended** for accurate results (Windows/WSL may show lower numbers)
2. **Warmup phase** included to reach steady state
3. **Native CPU optimizations** enabled via `RUSTFLAGS="-C target-cpu=native"`
