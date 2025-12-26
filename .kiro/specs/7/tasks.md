# Implementation Plan: VS Code DX Serializer WASM Update

## Overview

This implementation plan integrates the battle-hardened Rust serializer crate into the VS Code extension via WebAssembly. The approach is surgical - only modifying the core parsing/formatting layer while preserving the complex auto-save, virtual file system, and document management features.

**CRITICAL**: Do NOT modify dxDocumentManager.ts, dxLensFileSystem.ts, extension.ts (except WASM loading), or cacheManager.ts.

## Tasks

- [x] 1. Add WASM support to Rust serializer crate
  - [x] 1.1 Add wasm-bindgen dependency and wasm feature to Cargo.toml
    - Add `wasm-bindgen = "0.2"` to dependencies
    - Add `[features] wasm = ["wasm-bindgen"]`
    - Add `crate-type = ["cdylib", "rlib"]` to lib section
    - _Requirements: 1.1_
  - [x] 1.2 Create src/wasm.rs with WASM bindings
    - Create DxSerializer struct with #[wasm_bindgen]
    - Implement parse_llm() returning ParseResult
    - Implement format_to_human() returning String
    - Implement serialize_to_llm() returning String
    - Implement validate() returning ValidationResult
    - Implement serialize_to_binary() returning Vec<u8>
    - Add getter methods for limits (max_input_size, max_recursion_depth, max_table_rows)
    - _Requirements: 1.2, 2.1-2.6_
  - [x] 1.3 Export wasm module from lib.rs
    - Add `#[cfg(feature = "wasm")] pub mod wasm;`
    - _Requirements: 1.1_

- [x] 2. Create WASM build scripts
  - [x] 2.1 Create scripts/build-wasm.ps1 for Windows
    - Accept -Release flag for optimized builds
    - Run wasm-pack build with --target web
    - Output to crates/vscode-dx-serializer/wasm/
    - _Requirements: 1.3, 1.4, 1.5_
  - [x] 2.2 Create scripts/build-wasm.sh for Unix
    - Accept --release flag for optimized builds
    - Run wasm-pack build with --target web
    - Output to crates/vscode-dx-serializer/wasm/
    - _Requirements: 1.3, 1.4, 1.5_

- [x] 3. Checkpoint - Build and verify WASM module
  - Run build script and verify output files exist
  - Verify dx_serializer.js, dx_serializer_bg.wasm generated
  - Ask the user if questions arise

- [x] 4. Update dxCore.ts for WASM integration
  - [x] 4.1 Update WasmDxCore class to use new WASM interface
    - Update toHuman() to call serializer.format_to_human()
    - Update toDense() to call serializer.serialize_to_llm()
    - Update validate() to call serializer.validate()
    - Map WASM errors to ValidationResult format
    - _Requirements: 6.3_
  - [x] 4.2 Enable WASM loading in loadDxCore()
    - Remove "WASM disabled" comment
    - Add try/catch for WASM loading
    - Log success/failure messages
    - Fall back to FallbackDxCore on failure
    - _Requirements: 6.1, 6.2, 6.4, 7.4_

- [x] 5. Checkpoint - Test WASM integration
  - Verify extension loads with WASM
  - Verify fallback works when WASM unavailable
  - Test file open/save with WASM
  - Ask the user if questions arise

- [x] 6. Add integration tests
  - [x] 6.1 Create WASM loading test
    - Test that WASM module loads successfully
    - Test that DxSerializer can be instantiated
    - _Requirements: 6.1, 6.4_
  - [x] 6.2 Create parse equivalence test
    - Parse same input with WASM and TypeScript
    - Verify documents are equivalent
    - _Requirements: Property 3_
  - [x] 6.3 Create error handling test
    - Test InputTooLarge error for large input
    - Test RecursionLimitExceeded for deep nesting
    - Test TableTooLarge for large tables
    - _Requirements: 3.1-3.2, 4.1-4.2, 5.1-5.2_

- [x] 7. Update README documentation
  - [x] 7.1 Add WASM integration section
    - Document that extension uses Rust serializer via WASM
    - Document TypeScript fallback behavior
    - _Requirements: 8.1, 8.3_
  - [x] 7.2 Add security limits section
    - Document MAX_INPUT_SIZE (100 MB)
    - Document MAX_RECURSION_DEPTH (1000 levels)
    - Document MAX_TABLE_ROWS (10 million rows)
    - _Requirements: 8.2_
  - [x] 7.3 Add build instructions
    - Document prerequisites (rustup, wasm-pack)
    - Document build commands
    - _Requirements: 8.4_

- [x] 8. Final checkpoint - Verify all functionality preserved
  - Test auto-save with grace period works
  - Test dxlens:// virtual file system works
  - Test format detection (JSON, YAML, TOML, CSV, LLM) works
  - Test section order preservation works
  - Test cache file generation (.human, .machine) works
  - Run existing extension tests
  - Ask the user if questions arise

## Notes

- The TypeScript fallback (FallbackDxCore, llmParser.ts, humanFormatterV3.ts, humanParserV3.ts) is preserved unchanged
- WASM loading is async and happens on extension activation
- If WASM fails to load, the extension continues to work with TypeScript
- The complex auto-save and virtual file system logic is NOT modified
- All 38 property-based tests from the Rust crate are inherited by the WASM module
