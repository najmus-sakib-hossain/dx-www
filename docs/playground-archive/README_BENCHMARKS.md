# Playground Test Files

This directory contains benchmark test files for the dx-js-runtime.

## Test Files

### Core Benchmarks
1. **simple_test.js** - Original test (21 lines)
   - Variables, Math functions, console.log
   - Result: 8.3ms (dx-js) vs 64.5ms (Bun) = **7.80x faster**

2. **bench-math-heavy.js** - Math operations (23 lines)
   - sqrt, floor, ceil, abs, round
   - Result: 10.6ms (dx-js) vs 71.0ms (Bun) = **6.69x faster**

3. **bench-variables.js** - Variable-heavy (20 lines)
   - 10 variables with complex arithmetic
   - Result: 10.7ms (dx-js) vs 65.8ms (Bun) = **6.16x faster**

4. **bench-comparisons.js** - Comparison operators (17 lines)
   - < and > operators with multiple variables
   - Comprehensive comparison testing

### Advanced Benchmarks
5. **bench-nested-math.js** - Nested Math calls (14 lines)
   - Math.sqrt(Math.sqrt(x))
   - Math functions with arithmetic
   - Result: 9.0ms (dx-js) vs 52.0ms (Bun) = **5.80x faster**

6. **bench-arithmetic-chains.js** - Long chains (24 lines)
   - x1 + x2 + x3 + x4 + x5
   - Multiple arithmetic operations
   - Tests stack-based variable lookup

7. **bench-mixed-operations.js** - Mixed operations (23 lines)
   - Math + Arithmetic + Comparisons
   - Comprehensive integration test

## Running Benchmarks

### Single Test
```bash
cd /f/Code/dx/playground
hyperfine --warmup 1 --runs 10 \
  --prepare "rm -rf /tmp/dx-cache" \
  "/f/Code/dx/target/release/dx-js.exe simple_test.js" \
  "bun simple_test.js"
```

### All Tests
```bash
cd /f/Code/dx/playground
./bench-all.sh
```

## Expected Performance

**Target:** 4x faster than Bun
**Achieved:** 6.6-7.8x faster than Bun

### Average Results
- **DX-JS:** ~9ms average
- **Bun:** ~63ms average
- **Speedup:** ~6.6x faster

## Test Coverage

✅ Variables (const, let)
✅ Arithmetic (+, -, *, /)
✅ Math functions (sqrt, floor, ceil, abs, round)
✅ Comparisons (<, >)
✅ Booleans (true, false)
✅ console.log()
✅ Nested expressions
✅ Long arithmetic chains
✅ Mixed operations

## Limitations

❌ No loops (for, while)
❌ No arrays
❌ No objects
❌ No functions
❌ No strings (besides console.log)
❌ No async

These tests validate the core interpreter performance for the supported feature set.
