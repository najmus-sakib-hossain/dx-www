# DX Serializer WASM Module

This directory contains the WebAssembly build of the DX Serializer.

## Building

Run the build script from the repository root:

```bash
# Unix/macOS
./scripts/build-wasm.sh

# Windows PowerShell
.\scripts\build-wasm.ps1

# For optimized release build
./scripts/build-wasm.sh --release
.\scripts\build-wasm.ps1 -Release
```

## Prerequisites

1. Rust toolchain with wasm32-unknown-unknown target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

2. wasm-pack:
   ```bash
   cargo install wasm-pack
   ```

## Output Files

After building, this directory will contain:

- `dx_serializer.js` - JavaScript bindings
- `dx_serializer.d.ts` - TypeScript type definitions
- `dx_serializer_bg.wasm` - WebAssembly binary
- `dx_serializer_bg.wasm.d.ts` - WASM type definitions

## Usage

```typescript
import init, { DxSerializer } from './wasm/dx_serializer';

async function main() {
    await init();
    
    const serializer = new DxSerializer();
    
    // Transform dense to human format
    const result = serializer.toHuman('key:value');
    if (result.success) {
        console.log(result.content);
    }
    
    // Transform human to dense format
    const dense = serializer.toDense(humanContent);
    
    // Validate content
    const validation = serializer.validate(content);
    if (!validation.success) {
        console.log(`Error: ${validation.error}`);
        console.log(`Hint: ${validation.hint}`);
    }
}
```
