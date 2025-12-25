# Design Document: VS Code DX Serializer WASM Update

## Overview

This design document specifies how to integrate the battle-hardened Rust serializer crate into the VS Code extension via WebAssembly (WASM). The integration is designed to be surgical and non-invasive, replacing only the core parsing/formatting logic while preserving the complex auto-save, virtual file system, and document management features that make this extension unique.

**CRITICAL PRESERVATION**: The following components are complex, battle-tested, and MUST NOT be modified:
- `dxDocumentManager.ts` - Document state, dirty tracking, save coordination
- `dxLensFileSystem.ts` - Virtual file system provider (dxlens://)
- `extension.ts` - Activation, file redirection, auto-save grace period
- `cacheManager.ts` - Cache file generation (.human, .machine)

These components implement the "show one file, write another" pattern that VS Code doesn't support out of the box. They are the result of significant engineering effort and are working correctly.

## Architecture

The WASM integration follows a clean layered architecture:

```
┌─────────────────────────────────────────────────────────────────┐
│                    VS Code Extension Layer                       │
│                    (PRESERVED - DO NOT MODIFY)                   │
├─────────────────────────────────────────────────────────────────┤
│  ┌──────────────────┐  ┌──────────────────┐  ┌──────────────┐  │
│  │ dxDocumentManager│  │ dxLensFileSystem │  │  extension   │  │
│  │ - dirty tracking │  │ - dxlens:// URI  │  │ - activation │  │
│  │ - save coord     │  │ - virtual FS     │  │ - grace period│  │
│  │ - state mgmt     │  │ - file redirect  │  │ - commands   │  │
│  └────────┬─────────┘  └────────┬─────────┘  └──────┬───────┘  │
│           │                     │                    │          │
│           └─────────────────────┼────────────────────┘          │
│                                 │                               │
│                                 ▼                               │
├─────────────────────────────────────────────────────────────────┤
│                      DxCore Interface Layer                      │
│                      (MODIFIED - WASM Integration)               │
├─────────────────────────────────────────────────────────────────┤
│  ┌──────────────────────────────────────────────────────────┐   │
│  │                      dxCore.ts                            │   │
│  │  ┌─────────────────┐    ┌─────────────────────────────┐  │   │
│  │  │   WasmDxCore    │◄───│  loadDxCore() - WASM first  │  │   │
│  │  │   (PRIMARY)     │    │  fallback to TypeScript     │  │   │
│  │  └────────┬────────┘    └─────────────────────────────┘  │   │
│  │           │                                               │   │
│  │           │ if WASM fails                                 │   │
│  │           ▼                                               │   │
│  │  ┌─────────────────┐                                      │   │
│  │  │ FallbackDxCore  │ (TypeScript - unchanged)             │   │
│  │  │   (BACKUP)      │                                      │   │
│  │  └─────────────────┘                                      │   │
│  └──────────────────────────────────────────────────────────┘   │
│                                 │                               │
│                                 ▼                               │
├─────────────────────────────────────────────────────────────────┤
│                        WASM Module Layer                         │
│                        (NEW - Rust Serializer)                   │
├─────────────────────────────────────────────────────────────────┤
│  ┌──────────────────────────────────────────────────────────┐   │
│  │                   wasm/dx_serializer.js                   │   │
│  │                   wasm/dx_serializer_bg.wasm              │   │
│  │                                                           │   │
│  │  Exports:                                                 │   │
│  │  - DxSerializer class                                     │   │
│  │    - parse_llm(input) -> ParseResult                      │   │
│  │    - format_to_human(doc) -> string                       │   │
│  │    - serialize_to_llm(doc) -> string                      │   │
│  │    - validate(input) -> ValidationResult                  │   │
│  │    - serialize_to_binary(doc) -> Uint8Array               │   │
│  └──────────────────────────────────────────────────────────┘   │
│                                 │                               │
│                                 ▼                               │
├─────────────────────────────────────────────────────────────────┤
│                     Rust Serializer Crate                        │
│                     (crates/serializer)                          │
├─────────────────────────────────────────────────────────────────┤
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  src/wasm.rs (NEW)                                        │   │
│  │  - #[wasm_bindgen] exports                                │   │
│  │  - DxSerializer struct                                    │   │
│  │  - Error type conversions                                 │   │
│  │                                                           │   │
│  │  Battle-Hardened Features:                                │   │
│  │  - MAX_INPUT_SIZE: 100 MB                                 │   │
│  │  - MAX_RECURSION_DEPTH: 1000                              │   │
│  │  - MAX_TABLE_ROWS: 10,000,000                             │   │
│  │  - 38 property-based tests                                │   │
│  └──────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
```

## Components and Interfaces

### 1. Rust WASM Bindings (NEW)

New file: `crates/serializer/src/wasm.rs`

```rust
//! WebAssembly bindings for the DX Serializer
//! 
//! This module exposes the serializer functionality to JavaScript
//! via wasm-bindgen.

use wasm_bindgen::prelude::*;
use crate::{parse, DxValue, DxDocument, DxError};
use crate::error::{MAX_INPUT_SIZE, MAX_RECURSION_DEPTH, MAX_TABLE_ROWS};

/// Result of a parse operation
#[wasm_bindgen]
pub struct ParseResult {
    success: bool,
    document_json: Option<String>,
    error: Option<String>,
    line: Option<u32>,
    column: Option<u32>,
    hint: Option<String>,
}

#[wasm_bindgen]
impl ParseResult {
    #[wasm_bindgen(getter)]
    pub fn success(&self) -> bool { self.success }
    
    #[wasm_bindgen(getter)]
    pub fn document_json(&self) -> Option<String> { self.document_json.clone() }
    
    #[wasm_bindgen(getter)]
    pub fn error(&self) -> Option<String> { self.error.clone() }
    
    #[wasm_bindgen(getter)]
    pub fn line(&self) -> Option<u32> { self.line }
    
    #[wasm_bindgen(getter)]
    pub fn column(&self) -> Option<u32> { self.column }
    
    #[wasm_bindgen(getter)]
    pub fn hint(&self) -> Option<String> { self.hint.clone() }
}

/// Result of a validation operation
#[wasm_bindgen]
pub struct ValidationResult {
    success: bool,
    error: Option<String>,
    line: Option<u32>,
    column: Option<u32>,
    hint: Option<String>,
}

/// Main serializer class exposed to JavaScript
#[wasm_bindgen]
pub struct DxSerializer {
    // Configuration options
}

#[wasm_bindgen]
impl DxSerializer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> DxSerializer {
        DxSerializer {}
    }
    
    /// Parse LLM format input into a document
    pub fn parse_llm(&self, input: &str) -> ParseResult {
        // Implementation calls crate::parse()
    }
    
    /// Format a document to Human V3 format
    pub fn format_to_human(&self, doc_json: &str) -> String {
        // Implementation calls crate::format_human()
    }
    
    /// Serialize a document to LLM format
    pub fn serialize_to_llm(&self, doc_json: &str) -> String {
        // Implementation calls crate::serialize_llm()
    }
    
    /// Validate input syntax
    pub fn validate(&self, input: &str) -> ValidationResult {
        // Implementation calls crate::validate()
    }
    
    /// Serialize a document to binary format
    pub fn serialize_to_binary(&self, doc_json: &str) -> Vec<u8> {
        // Implementation calls crate::serialize_binary()
    }
    
    /// Get the maximum input size limit
    pub fn max_input_size(&self) -> usize {
        MAX_INPUT_SIZE
    }
    
    /// Get the maximum recursion depth limit
    pub fn max_recursion_depth(&self) -> usize {
        MAX_RECURSION_DEPTH
    }
    
    /// Get the maximum table rows limit
    pub fn max_table_rows(&self) -> usize {
        MAX_TABLE_ROWS
    }
}
```

### 2. Updated dxCore.ts

The existing `dxCore.ts` already has the structure for WASM support. We need to:
1. Enable WASM loading (currently disabled)
2. Update the WASM interface to match our bindings
3. Keep the TypeScript fallback intact

```typescript
// Key changes to dxCore.ts

/**
 * Load the DxCore, attempting WASM first with TypeScript fallback
 */
export async function loadDxCore(
    extensionPath: string,
    indentSize: number = 2,
    keyPadding: number = 20
): Promise<DxCore> {
    if (cachedCore) {
        return cachedCore;
    }

    // Try to load WASM module
    try {
        const wasmPath = path.join(extensionPath, 'wasm', 'dx_serializer.js');
        if (fs.existsSync(wasmPath)) {
            const wasmModule = await import(wasmPath);
            await wasmModule.default(); // Initialize WASM
            const serializer = new wasmModule.DxSerializer();
            cachedCore = new WasmDxCore(serializer);
            console.log('DX Serializer: Using WASM core (battle-hardened)');
            return cachedCore;
        }
    } catch (error) {
        console.warn('DX Serializer: WASM load failed, using TypeScript fallback', error);
    }

    // Fallback to TypeScript implementation
    cachedCore = new FallbackDxCore(indentSize, keyPadding);
    console.log('DX Serializer: Using TypeScript core (fallback)');
    return cachedCore;
}
```

### 3. Build Script

New file: `scripts/build-wasm.ps1` (Windows) and `scripts/build-wasm.sh` (Unix)

```powershell
# scripts/build-wasm.ps1
param(
    [switch]$Release
)

$ErrorActionPreference = "Stop"

Write-Host "Building DX Serializer WASM module..."

# Navigate to serializer crate
Push-Location crates/serializer

# Build with wasm-pack
if ($Release) {
    wasm-pack build --target web --out-dir ../vscode-dx-serializer/wasm --release
} else {
    wasm-pack build --target web --out-dir ../vscode-dx-serializer/wasm
}

Pop-Location

Write-Host "WASM build complete!"
Write-Host "Output: crates/vscode-dx-serializer/wasm/"
```

## Data Models

### Document JSON Format

The WASM module communicates with JavaScript using JSON for complex types:

```typescript
// Document structure passed between JS and WASM
interface WasmDxDocument {
    context: Record<string, WasmDxValue>;
    refs: Record<string, string>;
    sections: Record<string, WasmDxSection>;
    sectionOrder?: string[];
}

interface WasmDxValue {
    type: 'string' | 'number' | 'bool' | 'null' | 'array' | 'ref';
    value: string | number | boolean | null | WasmDxValue[];
    refKey?: string;
}

interface WasmDxSection {
    id: string;
    schema: string[];
    rows: WasmDxValue[][];
}
```

### Error Types

```typescript
// Error types from WASM
interface WasmError {
    type: 'InputTooLarge' | 'RecursionLimitExceeded' | 'TableTooLarge' | 
          'ParseError' | 'Utf8Error' | 'InvalidSyntax';
    message: string;
    // For InputTooLarge
    size?: number;
    max?: number;
    // For RecursionLimitExceeded
    depth?: number;
    // For TableTooLarge
    rows?: number;
    // For ParseError
    line?: number;
    column?: number;
    offset?: number;
    hint?: string;
}
```

## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system—essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

*Note: The WASM module inherits all 38 property-based tests from the Rust serializer crate. The properties below focus on the integration layer.*

### Property 1: WASM API Structured Results

*For any* call to a WASM API function (parse_llm, format_to_human, serialize_to_llm, validate, serialize_to_binary), the function SHALL return a structured result object with success/error information, never throwing an unhandled exception.

**Validates: Requirements 2.6**

### Property 2: Section Order Preservation

*For any* valid DxDocument with a defined section order, formatting to Human V3 and parsing back SHALL preserve the exact section order.

**Validates: Requirements 9.4**

### Property 3: WASM-TypeScript Parse Equivalence

*For any* valid LLM format input, parsing with WASM and parsing with TypeScript fallback SHALL produce semantically equivalent DxDocument objects (same context, refs, sections, and section order).

**Validates: Requirements 6.3, 7.1-7.4**

## Error Handling

### WASM Loading Errors

If WASM fails to load, the extension gracefully falls back to TypeScript:

```typescript
try {
    // Load WASM
} catch (error) {
    console.warn('WASM load failed:', error);
    // Use TypeScript fallback - extension continues to work
}
```

### Validation Errors

Validation errors from WASM are converted to the existing ValidationResult format:

```typescript
interface ValidationResult {
    success: boolean;
    error?: string;
    line?: number;
    column?: number;
    hint?: string;
}
```

## Testing Strategy

### Unit Tests

1. **WASM Loading Test** - Verify WASM module loads correctly
2. **Fallback Test** - Verify TypeScript fallback works when WASM unavailable
3. **Parse Equivalence Test** - Verify WASM and TypeScript produce same results
4. **Error Mapping Test** - Verify WASM errors map to correct TypeScript types

### Integration Tests

1. **Extension Activation** - Verify extension activates with WASM
2. **File Open/Save** - Verify files open and save correctly with WASM
3. **Auto-Save** - Verify auto-save grace period works with WASM
4. **Cache Generation** - Verify .human and .machine files generated correctly

### Property Tests (Inherited from Rust)

The Rust crate has 38 property-based tests that validate:
- Input validation (size, recursion, table limits)
- Round-trip consistency
- Error handling
- Thread safety
- Compression integrity

## Files to Modify

### New Files

| File | Purpose |
|------|---------|
| `crates/serializer/src/wasm.rs` | WASM bindings for Rust serializer |
| `scripts/build-wasm.ps1` | Windows WASM build script |
| `scripts/build-wasm.sh` | Unix WASM build script |

### Modified Files

| File | Changes |
|------|---------|
| `crates/serializer/Cargo.toml` | Add wasm feature and wasm-bindgen dependency |
| `crates/serializer/src/lib.rs` | Add wasm module export |
| `crates/vscode-dx-serializer/src/dxCore.ts` | Enable WASM loading, update WasmDxCore |
| `crates/vscode-dx-serializer/README.md` | Document WASM integration |

### Preserved Files (DO NOT MODIFY)

| File | Reason |
|------|--------|
| `dxDocumentManager.ts` | Complex document state management |
| `dxLensFileSystem.ts` | Virtual file system provider |
| `extension.ts` | Activation and auto-save logic |
| `cacheManager.ts` | Cache file generation |
| `llmParser.ts` | TypeScript fallback |
| `humanFormatterV3.ts` | TypeScript fallback |
| `humanParserV3.ts` | TypeScript fallback |

## Build Process

### Prerequisites

```bash
# Install Rust WASM target
rustup target add wasm32-unknown-unknown

# Install wasm-pack
cargo install wasm-pack
```

### Build Commands

```bash
# Build WASM (debug)
./scripts/build-wasm.sh

# Build WASM (release)
./scripts/build-wasm.sh --release

# Build extension
cd crates/vscode-dx-serializer
npm install
npm run compile

# Package extension
npx vsce package --allow-missing-repository
```

### Output Files

After building, `crates/vscode-dx-serializer/wasm/` will contain:
- `dx_serializer.js` - JavaScript bindings
- `dx_serializer.d.ts` - TypeScript type definitions
- `dx_serializer_bg.wasm` - WebAssembly binary
- `dx_serializer_bg.wasm.d.ts` - WASM type definitions
