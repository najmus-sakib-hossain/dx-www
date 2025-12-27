# DX Test Runner Benchmarks

This directory contains benchmark test suites for comparing DX Test Runner performance with Bun.

## Test Suites

- `math.test.js` - 10 arithmetic tests
- `strings.test.js` - 10 string manipulation tests
- `arrays.test.js` - 10 array operation tests
- `objects.test.js` - 10 object manipulation tests
- `booleans.test.js` - 10 boolean/truthiness tests

**Total: 50 tests**

## Running Benchmarks

### DX Test Runner
```bash
# Build the test runner
cargo build --release -p dx-test-cli

# Run tests
./target/release/dx-test

# With verbose output
./target/release/dx-test --verbose

# Show cache statistics
./target/release/dx-test cache

# Clear cache and re-run
./target/release/dx-test clear
./target/release/dx-test
```

### Bun Test Runner
```bash
cd tests
bun test
```

## Performance Targets

| Metric | Bun (Target) | DX (Goal) | Speedup |
|--------|--------------|-----------|---------|
| 50 tests | ~42ms | <1ms | 50x+ |
| Discovery | ~2ms | <0.01ms | 200x+ |
| Execution | ~40ms | <1ms | 40x+ |

## Architecture Advantages

1. **O(1) Layout Cache** - Pre-compiled test index (vs Bun's file parsing)
2. **Custom Bytecode VM** - Stack-based execution (vs V8 JIT overhead)
3. **Parallel Execution** - Work-stealing across all cores
4. **Zero-Copy Memory** - Memory-mapped binary formats
5. **Warm State** - Persistent runtime between runs

## Expected Results

For 50 simple tests:
- **Bun**: ~40-50ms (with JIT warmup)
- **DX**: ~0.5-1ms (cached) or ~2-3ms (cold)
- **Speedup**: **40-100x faster**

For 500 complex tests:
- **Bun**: ~400-500ms
- **DX**: ~5-10ms
- **Speedup**: **50-80x faster**
