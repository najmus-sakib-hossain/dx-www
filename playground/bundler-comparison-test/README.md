# DX JS Bundler vs Bun - Comparison Test

This directory contains a comprehensive comparison test between DX JS Bundler and Bun bundler.

## Test Setup

The test uses the `benchmark-simple` project which includes:
- TypeScript files with types, interfaces, classes
- ES6 imports/exports
- Generic types (e.g., `Record<string, Type>`)
- Complex type annotations

## Running the Test

```bash
# Make script executable
chmod +x test-bundlers.sh

# Run the comparison test
./test-bundlers.sh
```

## What It Tests

1. **Bundle Creation** - Both bundlers process the same TypeScript source
2. **Syntax Validation** - Verifies both outputs are valid JavaScript (`node -c`)
3. **Execution Test** - Runs both bundles and captures output
4. **Performance Comparison** - Measures bundle time for each bundler
5. **Size Comparison** - Compares output file sizes

## Expected Results

- ✅ Both bundles should pass syntax validation
- ✅ Both bundles should execute successfully
- ✅ DX Bundler should be ~2-3x faster than Bun
- ✅ Both should produce functionally correct output

## Output Location

Bundled files are saved to:
- `./output/dx-bundle.js` - DX Bundler output
- `./output/bun-bundle.js` - Bun bundler output

## Prerequisites

- Bun installed: https://bun.sh
- DX bundler built (script will build if needed)
- Node.js for validation and execution
