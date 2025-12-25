# Requirements Document

## Introduction

This document specifies requirements for updating the VS Code DX Serializer extension to use the battle-hardened serializer Rust crate via WebAssembly (WASM). The extension currently uses a TypeScript implementation for parsing and formatting. This update will replace the TypeScript implementation with WASM bindings to the Rust crate, ensuring the extension benefits from the security improvements, validation limits, and performance optimizations implemented in the Rust crate.

**CRITICAL CONSTRAINT**: The extension's auto-save, document management, and file system provider logic is complex and fragile. This update MUST NOT modify any of those components. Only the parsing, validation, and formatting logic should be updated to use WASM.

**APPROACH**: Build the Rust serializer crate to WASM using wasm-pack, expose the necessary functions via wasm-bindgen, and update dxCore.ts to use the WASM module instead of the TypeScript fallback.

## Glossary

- **LLM_Format**: The token-efficient sigil-based format stored on disk (#c:, #:, #<letter>)
- **Human_Format_V3**: The human-readable TOML-like format displayed in the editor
- **Machine_Format**: The binary format stored in cache files
- **DxDocument**: The internal representation of a parsed DX file
- **WASM_Module**: The WebAssembly binary compiled from the Rust serializer crate
- **wasm-bindgen**: Rust library for generating JavaScript bindings for WASM
- **wasm-pack**: Tool for building Rust-generated WASM packages
- **DxCore**: The TypeScript interface that wraps WASM or fallback implementations

## Requirements

### Requirement 1: WASM Build Infrastructure

**User Story:** As a developer, I want the Rust serializer crate to compile to WASM, so that the VS Code extension can use the same battle-hardened code.

#### Acceptance Criteria

1. THE serializer crate SHALL have a `wasm` feature flag for WASM-specific code
2. THE serializer crate SHALL expose wasm-bindgen bindings for parsing, formatting, and validation
3. THE build process SHALL use wasm-pack to generate JavaScript bindings
4. THE WASM output SHALL be placed in `crates/vscode-dx-serializer/wasm/`
5. THE build script SHALL support both debug and release builds

### Requirement 2: WASM API Surface

**User Story:** As a developer, I want the WASM module to expose the necessary functions, so that the extension can perform all required operations.

#### Acceptance Criteria

1. THE WASM module SHALL expose a `parse_llm(input: string) -> ParseResult` function
2. THE WASM module SHALL expose a `format_to_human(doc: DxDocument) -> string` function
3. THE WASM module SHALL expose a `serialize_to_llm(doc: DxDocument) -> string` function
4. THE WASM module SHALL expose a `validate(input: string) -> ValidationResult` function
5. THE WASM module SHALL expose a `serialize_to_binary(doc: DxDocument) -> Uint8Array` function
6. ALL functions SHALL return structured results with success/error information

### Requirement 3: Input Size Validation (via WASM)

**User Story:** As a developer, I want the extension to reject excessively large files using the Rust validation, so that the editor remains responsive.

#### Acceptance Criteria

1. WHEN the WASM Parser receives input exceeding 100MB, THE Parser SHALL return an InputTooLarge error
2. THE error SHALL include the actual size and maximum allowed size
3. THE extension SHALL display the error message in the status bar

### Requirement 4: Recursion Depth Protection (via WASM)

**User Story:** As a developer, I want the extension to handle deeply nested structures safely using Rust validation.

#### Acceptance Criteria

1. WHEN the WASM Parser encounters structures nested deeper than 1000 levels, THE Parser SHALL return a RecursionLimitExceeded error
2. THE error SHALL include the current depth and maximum allowed depth

### Requirement 5: Table Row Limits (via WASM)

**User Story:** As a developer, I want the extension to limit table sizes using Rust validation.

#### Acceptance Criteria

1. WHEN the WASM Parser encounters a table with more than 10 million rows, THE Parser SHALL return a TableTooLarge error
2. THE error SHALL include the row count and maximum allowed rows

### Requirement 6: DxCore WASM Integration

**User Story:** As a developer, I want dxCore.ts to use the WASM module, so that all parsing uses the battle-hardened Rust code.

#### Acceptance Criteria

1. THE loadDxCore function SHALL attempt to load the WASM module first
2. IF WASM loading fails, THE function SHALL fall back to TypeScript implementation
3. THE WasmDxCore class SHALL implement the DxCore interface using WASM bindings
4. THE WASM module SHALL be loaded asynchronously on extension activation

### Requirement 7: TypeScript Fallback Preservation

**User Story:** As a developer, I want the TypeScript implementation to remain as a fallback, so that the extension works even if WASM fails to load.

#### Acceptance Criteria

1. THE FallbackDxCore class SHALL remain unchanged
2. THE TypeScript parsers (llmParser.ts, humanParserV3.ts) SHALL remain unchanged
3. THE TypeScript formatters (humanFormatterV3.ts) SHALL remain unchanged
4. IF WASM fails to load, THE extension SHALL log a warning and use TypeScript

### Requirement 8: README Documentation Update

**User Story:** As a developer, I want the extension README to document the WASM integration and security limits.

#### Acceptance Criteria

1. THE README SHALL document that the extension uses the Rust serializer via WASM
2. THE README SHALL document the security limits (100 MB, 1000 depth, 10M rows)
3. THE README SHALL document the fallback behavior when WASM is unavailable
4. THE README SHALL document how to rebuild the WASM module

### Requirement 9: Preserve Existing Functionality

**User Story:** As a developer, I want all existing extension features to continue working.

#### Acceptance Criteria

1. THE extension SHALL continue to support auto-save with grace period
2. THE extension SHALL continue to support the dxlens:// virtual file system
3. THE extension SHALL continue to support format detection (JSON, YAML, TOML, CSV, LLM)
4. THE extension SHALL continue to support section order preservation
5. THE extension SHALL continue to generate cache files (.human, .machine)
6. THE extension SHALL NOT modify dxDocumentManager.ts
7. THE extension SHALL NOT modify dxLensFileSystem.ts
8. THE extension SHALL NOT modify extension.ts activation logic (except WASM loading)
