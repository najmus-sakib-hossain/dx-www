# Implementation Plan: DX Serializer V3

## Overview

Implement the new Human Format V3 with vertical key-value layout, multi-format input support, and automatic cache generation for the VS Code extension.

## Tasks

- [x] 1. Implement Human Format V3 Formatter
  - [x] 1.1 Create `humanFormatterV3.ts` with vertical key-value layout
    - Implement `formatDocumentV3()` function
    - Config values without section header
    - Data sections with `[section]` headers
    - Key padding to 20 characters (or longest key + 1)
    - Use `|` as array separator
    - Quote strings with spaces
    - _Requirements: 2.1, 2.2, 2.3, 2.4, 2.5, 2.6, 2.7_

  - [x] 1.2 Write property tests for Human Format V3
    - **Property 4: Key Alignment Consistency**
    - **Property 5: Array Separator Consistency**
    - **Property 6: No Table Formatting**
    - **Property 7: String Quoting**
    - **Validates: Requirements 2.3, 2.4, 2.6, 2.7**

- [x] 2. Implement Human Format V3 Parser
  - [x] 2.1 Create `humanParserV3.ts` to parse vertical key-value format
    - Parse config key-value pairs (no section header)
    - Parse `[section]` headers with optional schema
    - Parse `|` separated arrays
    - Handle quoted strings
    - _Requirements: 3.1_

  - [x] 2.2 Write property test for round-trip consistency
    - **Property 3: Human Format V3 Round-Trip**
    - **Validates: Requirements 2.1-2.7, 3.1**

- [x] 3. Implement Format Detector
  - [x] 3.1 Create `formatDetector.ts` with format detection logic
    - Detect JSON (starts with `{` or `[`)
    - Detect YAML (`:` patterns, `---`, `-` at line start)
    - Detect TOML (`[section]` with `key = value`)
    - Detect CSV (comma-separated with consistent columns)
    - Detect LLM format (`#c:`, `#:`, `#<letter>(`)
    - Detect Human V3 format (key = value patterns)
    - _Requirements: 5.1, 5.2, 5.3, 5.4, 5.5, 5.6_

  - [x] 3.2 Write property test for format detection
    - **Property 1: Format Detection Accuracy**
    - **Validates: Requirements 5.1-5.6**

- [x] 4. Implement Format Converters
  - [x] 4.1 Create `converters/jsonConverter.ts`
    - Convert JSON objects to DxDocument context
    - Convert JSON arrays to DxDocument sections
    - _Requirements: 1.1_

  - [x] 4.2 Create `converters/yamlConverter.ts`
    - Parse YAML using simple parser
    - Convert to DxDocument structure
    - _Requirements: 1.2_

  - [x] 4.3 Create `converters/tomlConverter.ts`
    - Parse TOML sections and key-value pairs
    - Convert to DxDocument structure
    - _Requirements: 1.3_

  - [x] 4.4 Create `converters/csvConverter.ts`
    - Parse CSV headers as schema
    - Parse CSV rows as data
    - _Requirements: 1.4_

  - [x] 4.5 Write property test for format conversion
    - **Property 2: Format Conversion Preserves Data**
    - **Validates: Requirements 1.1-1.4**

- [x] 5. Implement Cache Manager
  - [x] 5.1 Create `cacheManager.ts` with cache file operations
    - Create `.dx/cache` directory if needed
    - Write human cache as `{filename}.human`
    - Write machine cache as `{filename}.machine`
    - Preserve subdirectory structure
    - Delete cache files when source deleted
    - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5_

  - [x] 5.2 Write property test for cache path preservation
    - **Property 8: Cache Path Preservation**
    - **Validates: Requirements 4.4**

- [x] 6. Implement Machine Format
  - [x] 6.1 Create `machineFormat.ts` with JSON serialization
    - Implement `documentToMachine()` function
    - Implement `machineToDocument()` function
    - JSON format with version header
    - Efficient encoding for context and sections
    - _Requirements: 1.7, 3.4_

  - [x] 6.2 Write property test for machine format round-trip
    - **Property 9: Machine Format Round-Trip**
    - **Validates: Requirements 1.7, 3.4**

- [x] 7. Integrate into Extension
  - [x] 7.1 Update `dxCore.ts` to use Human Format V3
    - Replace `formatDocument` with `formatDocumentV3`
    - Replace `parseHuman` with `parseHumanV3`
    - _Requirements: 2.1-2.7, 3.1_

  - [x] 7.2 Update `extension.ts` for format detection and conversion
    - Detect input format on file open
    - Convert non-LLM formats to LLM on save
    - Generate cache files on save
    - _Requirements: 1.1-1.7, 3.2-3.4_

  - [x] 7.3 Update `dxDocumentManager.ts` for cache management
    - Call cache manager on document save
    - Delete cache on document delete
    - _Requirements: 4.1-4.5_

- [x] 8. Checkpoint - Ensure all tests pass
  - All unit tests pass (94 total)
  - All property tests pass (9 total)
  - Extension compiles and packages successfully
  - Ask the user if questions arise.

- [x] 9. Update Documentation
  - [x] 9.1 Update `HUMAN.md` with V3 format specification
    - Document new vertical key-value format
    - Document `|` array separator
    - Document section schema syntax
    - _Requirements: 2.1-2.7_

  - [x] 9.2 Update extension README with new features
    - Document multi-format input support
    - Document cache file locations
    - _Requirements: 1.1-1.7, 4.1-4.5_

- [x] 10. Final Checkpoint
  - Ensure all tests pass, ask the user if questions arise.

## Notes

- All tasks are required for comprehensive implementation
- Each task references specific requirements for traceability
- Checkpoints ensure incremental validation
- Property tests validate universal correctness properties
- Unit tests validate specific examples and edge cases
