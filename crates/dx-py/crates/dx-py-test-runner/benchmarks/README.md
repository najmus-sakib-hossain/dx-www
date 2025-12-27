# dx-py-test-runner Benchmarks

This directory contains benchmarks comparing dx-py-test-runner against pytest and unittest.

## Running Benchmarks

```bash
# Build dx-py first
cargo build --release

# Run the benchmark
python benchmarks/run_benchmark.py
```

## Test Suite Structure

- `test_project/` - Sample Python project with various test patterns
- `run_benchmark.py` - Benchmark runner script
- `results/` - Benchmark results (generated)
