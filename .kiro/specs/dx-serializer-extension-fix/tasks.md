# Implementation Plan: DX Serializer Extension Fix

## Overview

This plan updates the DX Serializer VS Code extension to properly handle the DX LLM format with sigils (`#c`, `#:`, `#<letter>`) and convert between LLM and Human formats correctly. The implementation focuses on updating the TypeScript fallback in `dxCore.ts` and ensuring the WASM module uses the correct conversion functions.

## Tasks

- [ ] 1. Update WASM bindings to use LLM format conversion
  - [ ] 1.1 Update `wasm.rs` to use `llm_to_human` and `human_to_llm` functions
    - Replace hologram Inflater/Deflater with llm module functions
    - Update `to_human()` to call `llm_to_human()`
    - Update `to_dense()` to call `human_to_llm()`
    - _Requirements: 5.1, 5.2_
  - [ ] 1.2 Write property test for WASM round-trip
    - **Property 1: LLM to Human to LLM Round-Trip**
    - **Validates: Requirements 1.1-1.9, 2.1-2.6, 3.1-3.5, 3.6**

- [ ] 2. Implement TypeScript LLM Parser
  - [ ] 2.1 Create LLM parser types and interfaces
    - Define `DxDocument`, `DxSection`, `DxValue` types
    - Define parser state machine types
    - _Requirements: 1.1-1.9_
  - [ ] 2.2 Implement context section parser (`#c:`)
    - Parse `#c:key|value;key|value` format
    - Handle all value types (string, number, bool, null, array, ref)
    - _Requirements: 1.1, 1.4, 1.5, 1.6, 1.7, 1.8, 1.9_
  - [ ] 2.3 Implement reference section parser (`#:`)
    - Parse `#:key|value` format
    - Store in refs map
    - _Requirements: 1.2_
  - [ ] 2.4 Implement data section parser (`#<letter>(schema)`)
    - Parse schema from parentheses
    - Parse subsequent rows as pipe-separated values
    - _Requirements: 1.3, 1.4_
  - [ ] 2.5 Write property test for LLM parsing
    - **Property 1: LLM to Human to LLM Round-Trip**
    - **Validates: Requirements 1.1-1.9**

- [ ] 3. Implement TypeScript Human Formatter
  - [ ] 3.1 Create abbreviation dictionary
    - Implement `AbbrevDict` class with expand/compress methods
    - Add all standard abbreviations (nm→name, ct→count, etc.)
    - Add context-aware expansion logic
    - _Requirements: 6.1-6.9_
  - [ ] 3.2 Implement section header formatter
    - Generate box-drawing character headers
    - Center section titles
    - _Requirements: 2.2_
  - [ ] 3.3 Implement config section formatter
    - Format key-value pairs with expanded keys
    - Align values for readability
    - _Requirements: 2.1_
  - [ ] 3.4 Implement data section table formatter
    - Generate Unicode table with box-drawing characters
    - Auto-calculate column widths
    - Right-align numbers, center booleans
    - Display booleans as ✓/✗
    - Display nulls as —
    - _Requirements: 2.3, 2.4, 2.5_
  - [ ] 3.5 Implement reference resolution
    - Resolve `^key` references to actual values
    - Add comment showing original reference
    - _Requirements: 2.6_
  - [ ] 3.6 Implement summary footer generator
    - Calculate totals for numeric columns
    - Count boolean true values
    - _Requirements: 2.7_
  - [ ] 3.7 Write property test for key abbreviation consistency
    - **Property 6: Key Abbreviation Consistency**
    - **Validates: Requirements 2.1, 3.2, 6.1-6.9**

- [ ] 4. Implement TypeScript Human Parser
  - [ ] 4.1 Implement section header parser
    - Recognize `[section]` headers
    - Map section names to single-letter IDs
    - _Requirements: 3.1_
  - [ ] 4.2 Implement config section parser
    - Parse `key = value` format
    - Compress expanded keys back to abbreviations
    - _Requirements: 3.2_
  - [ ] 4.3 Implement table parser
    - Recognize Unicode table structure
    - Extract schema from header row
    - Parse data rows
    - Convert ✓/✗ back to booleans
    - Convert — back to null
    - _Requirements: 3.3, 3.4_
  - [ ] 4.4 Implement reference detection
    - Detect repeated strings
    - Create reference definitions
    - _Requirements: 3.5_
  - [ ] 4.5 Write property test for Human to LLM round-trip
    - **Property 2: Human to LLM to Human Round-Trip**
    - **Validates: Requirements 3.1-3.5, 3.6**

- [ ] 5. Checkpoint - Verify round-trip conversions
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 6. Update dxCore.ts with new implementations
  - [ ] 6.1 Replace `formatDx` with LLM-aware formatter
    - Use new LLM parser and Human formatter
    - Handle empty input
    - _Requirements: 1.1-1.9, 2.1-2.7_
  - [ ] 6.2 Replace `minifyDx` with LLM-aware compressor
    - Use new Human parser and LLM serializer
    - Handle empty input
    - _Requirements: 3.1-3.5_
  - [ ] 6.3 Update validation to handle LLM format
    - Validate sigil syntax
    - Validate reference definitions
    - Validate schema/row consistency
    - _Requirements: 8.1-8.4_
  - [ ] 6.4 Write property test for WASM/TypeScript equivalence
    - **Property 8: WASM and TypeScript Equivalence**
    - **Validates: Requirements 5.3, 5.4**

- [ ] 7. Verify file extension filtering
  - [ ] 7.1 Review and test `isExactlyDxFile` function
    - Verify `.dx` files are accepted
    - Verify `.dx.json`, `.dx.yml`, etc. are rejected
    - _Requirements: 4.1-4.7_
  - [ ] 7.2 Write property test for file extension filtering
    - **Property 7: File Extension Filtering**
    - **Validates: Requirements 4.1-4.7**

- [ ] 8. Checkpoint - Full integration test
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 9. Create example files for testing
  - [ ] 9.1 Create LLM format example file
    - Include context section, references, data sections
    - Include all value types
    - _Requirements: 1.1-1.9_
  - [ ] 9.2 Create Human format example file
    - Include formatted tables, expanded keys
    - Include resolved references
    - _Requirements: 2.1-2.7_

- [ ] 10. Final checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

## Notes

- All tasks are required for comprehensive implementation
- Each task references specific requirements for traceability
- Checkpoints ensure incremental validation
- Property tests validate universal correctness properties
- Unit tests validate specific examples and edge cases
- The TypeScript fallback must produce identical results to the WASM core
