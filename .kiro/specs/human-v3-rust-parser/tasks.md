# Implementation Plan: Human V3 Rust Parser

## Overview

This plan implements a Human V3 parser in Rust for the serializer crate, enabling the WASM `toDense` function to correctly handle Human Format V3. The implementation follows the existing patterns in the crate and integrates with the battle-hardened security limits.

## Tasks

- [x] 1. Create SectionNameDict for name-to-ID mapping
  - Create `crates/serializer/src/llm/section_names.rs`
  - Implement bidirectional mapping: forge↔f, style↔y, media↔m, etc.
  - Add all section mappings from TypeScript `SECTION_NAMES`
  - _Requirements: 3.1_

- [x] 2. Create HumanV3Parser structure
  - [x] 2.1 Create `crates/serializer/src/llm/human_v3_parser.rs`
    - Define `HumanV3Parser` struct with `AbbrevDict` and `SectionNameDict`
    - Define `HumanV3ParseError` with line, column, message, hint
    - Define `SectionHeader` enum for simple and nested headers
    - _Requirements: 1.1, 3.1, 4.1_

  - [x] 2.2 Implement `parse_value` function
    - Handle quoted strings (double and single quotes)
    - Handle null markers (`-`, `~`)
    - Handle booleans (`true`, `false`)
    - Handle numbers (integer and float patterns)
    - Handle arrays (pipe separators)
    - _Requirements: 5.1, 5.2, 5.3, 6.1, 6.2, 6.3, 6.4_

  - [x] 2.3 Write property test for value parsing
    - **Property 6: Quoted String Handling**
    - **Property 7: Value Type Detection**
    - **Validates: Requirements 5.1, 5.2, 5.3, 6.3, 6.4**

- [x] 3. Implement config section parsing
  - [x] 3.1 Implement `parse_config_section` function
    - Parse key-value pairs before any section header
    - Compress keys using `AbbrevDict`
    - Handle arrays with pipe separators
    - _Requirements: 1.1, 1.2, 1.3_

  - [x] 3.2 Write property test for config parsing
    - **Property 1: Config Parsing**
    - **Property 2: Array Parsing Consistency**
    - **Validates: Requirements 1.1, 1.2, 1.3**

- [x] 4. Implement section header parsing
  - [x] 4.1 Implement `parse_section_header` function
    - Parse simple headers: `[section]`
    - Parse nested headers: `[parent.child]`
    - Map full names to section IDs
    - _Requirements: 3.1, 4.1_

  - [x] 4.2 Write property test for section name mapping
    - **Property 4: Section Name Mapping**
    - **Validates: Requirements 3.1, 3.2, 3.3**

- [ ] 5. Implement stack section parsing
  - [ ] 5.1 Implement `parse_stack_section` function
    - Parse `[stack]` section as reference definitions
    - Preserve key names without abbreviation
    - Join pipe-separated values with `|`
    - _Requirements: 2.1, 2.2, 2.3_

  - [ ] 5.2 Write property test for stack section
    - **Property 3: Stack Section Preservation**
    - **Validates: Requirements 2.1, 2.2, 2.3**

- [ ] 6. Implement data section parsing
  - [ ] 6.1 Implement `parse_data_section` function
    - Parse key-value pairs into schema and row
    - Compress keys for schema
    - Handle single-row sections
    - _Requirements: 3.2, 3.3_

  - [ ] 6.2 Implement `parse_nested_section` function
    - Parse nested sections like `[i18n.locales]`
    - Prefix keys with subsection name
    - Track subsection order for merging
    - _Requirements: 4.1, 4.2_

  - [ ] 6.3 Implement nested section merging
    - Merge multiple nested sections into single parent section
    - Preserve subsection order in schema
    - _Requirements: 4.3_

  - [ ] 6.4 Write property test for nested section merging
    - **Property 5: Nested Section Merging**
    - **Validates: Requirements 4.1, 4.2, 4.3**

- [ ] 7. Implement main parse function
  - [ ] 7.1 Implement `HumanV3Parser::parse` function
    - Detect config section (lines before first header)
    - Process sections in order
    - Track section order in document
    - Handle all section types (stack, data, nested)
    - _Requirements: 1.1, 2.1, 3.1, 4.1, 7.3_

  - [ ] 7.2 Write property test for round-trip consistency
    - **Property 8: Round-Trip Consistency**
    - **Validates: Requirements 7.1, 7.2, 7.3**

- [ ] 8. Checkpoint - Ensure all parser tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 9. Integrate with WASM toDense
  - [ ] 9.1 Update `human_to_llm` function in `convert.rs`
    - Add format detection (LLM vs Human V3)
    - Use `HumanV3Parser` for Human V3 input
    - Pass through LLM format unchanged
    - _Requirements: 8.1, 8.2_

  - [ ] 9.2 Update `to_dense` in `wasm.rs`
    - Use updated `human_to_llm` function
    - Return detailed errors with line/column
    - _Requirements: 8.1, 8.2, 8.3_

  - [ ] 9.3 Write property test for format detection
    - **Property 9: Format Detection and Passthrough**
    - **Validates: Requirements 8.1, 8.2**

- [ ] 10. Update module exports
  - Add `human_v3_parser` to `llm/mod.rs`
  - Add `section_names` to `llm/mod.rs`
  - Export `HumanV3Parser` and `HumanV3ParseError`
  - _Requirements: 8.1_

- [ ] 11. Build and test WASM
  - Run `wasm-pack build` to compile WASM
  - Copy WASM files to VS Code extension
  - Test with VS Code extension
  - _Requirements: 8.1, 8.2_

- [ ] 12. Final checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

## Notes

- All property-based tests are required for comprehensive coverage
- Each property test should run minimum 100 iterations
- The parser should reuse existing `AbbrevDict` from `abbreviations.rs`
- Error handling should provide helpful hints for common mistakes
- Section order tracking is critical for round-trip consistency
