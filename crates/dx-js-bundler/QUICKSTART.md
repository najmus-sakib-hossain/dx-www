# DX Bundler - Quick Start Guide

## Build

```bash
cd crates/dx-js-bundler
cargo build --release -p dx-bundle-cli
```

## Run

```bash
./target/release/dx-bundle bundle src/index.js -o dist/bundle.js --verbose
```

## Test

```bash
./target/release/dx-bundle bundle examples/simple/src/index.js -o dist/test.js -v
node dist/test.js
```

## Benchmark vs Bun

```bash
cd benchmarks
chmod +x compare_with_bun.sh
./compare_with_bun.sh
```

## Project Structure

```
11 Crates:
✅ dx-bundle-core        - Binary formats (complete)
✅ dx-bundle-graph       - O(1) cache (partial)
✅ dx-bundle-resolve     - Resolution (stub)
✅ dx-bundle-parse       - AST cache (partial)
✅ dx-bundle-transform   - Transforms (stub)
✅ dx-bundle-tree-shake  - Tree shaking (stub)
✅ dx-bundle-concat      - Concatenation (complete)
✅ dx-bundle-minify      - Minification (stub)
✅ dx-bundle-sourcemap   - Source maps (partial)
✅ dx-bundle-cli         - CLI (complete)
✅ dx-js-bundler         - Main API (complete)
```

## Documentation

- `README.md` - User guide
- `docs/COMPLETE_SUMMARY.md` - Full technical docs
- `IMPLEMENTATION_COMPLETE.md` - Implementation notes
- `DX_JS_BUNDLER.md` - Original spec

## Next Steps

1. Implement stub crates (see TODOs)
2. Add comprehensive tests
3. Benchmark against Bun
4. Optimize hot paths

## Goal

**3x faster than Bun through Binary Dawn architecture!**
