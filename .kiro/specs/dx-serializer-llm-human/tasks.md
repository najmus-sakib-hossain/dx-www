# Implementation Plan: DX Serializer LLM and Human Formats

## Overview

This implementation plan breaks down the DX Serializer LLM and Human format feature into discrete coding tasks. The approach is incremental: first establish core data types, then implement LLM format, then Human format, then conversions, and finally playground tests.

## Tasks

- [x] 1. Set up core data types and abbreviation dictionary
  - [x] 1.1 Create `DxDocument`, `DxSection`, and enhanced `DxValue` types in `crates/serializer/src/llm/types.rs`
    - Define `DxDocument` with context, refs, and sections fields
    - Define `DxSection` with schema and rows
    - Extend `DxValue` enum with `Ref(String)` variant for reference pointers
    - _Requirements: 1.4, 5.1_

  - [x] 1.2 Create comprehensive `AbbrevDict` in `crates/serializer/src/llm/abbrev.rs`
    - Implement 50+ key abbreviation mappings organized by category
    - Implement context-aware expansion for ambiguous keys (s, w, t, etc.)
    - Implement `expand()` and `compress()` methods
    - _Requirements: 5.1, 5.2, 5.3_

  - [x] 1.3 Write property test for key abbreviation round-trip
    - **Property 6: Key Abbreviation Round-Trip**
    - **Validates: Requirements 5.1-5.3, 6.1, 7.1**

- [x] 2. Implement LLM format parser
  - [x] 2.1 Create `LlmParser` in `crates/serializer/src/llm/parser.rs`
    - Implement `parse()` method to parse full LLM format string
    - Implement `parse_context()` for `#c:key|val;key|val` format
    - Implement `parse_reference()` for `#:key|value` format
    - Implement `parse_section_header()` for `#x(col|col)` format
    - Implement `parse_row()` for pipe-delimited data rows
    - Implement `parse_value()` for +/-/~/^key/*array values
    - _Requirements: 1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8_

  - [x] 2.2 Create `LlmSerializer` in `crates/serializer/src/llm/serializer.rs`
    - Implement `serialize()` method to output LLM format string
    - Implement `find_repeated_strings()` for automatic reference creation
    - Implement `serialize_context()` for context section output
    - Implement `serialize_section()` for data section output
    - Implement `serialize_value()` for value output with +/-/~/^key/*array
    - _Requirements: 2.1, 2.2, 2.3, 2.4, 2.5, 2.6, 2.7_

  - [x] 2.3 Write property test for LLM format round-trip
    - **Property 1: LLM Format Round-Trip**
    - **Validates: Requirements 1.1-1.8, 2.1-2.7, 9.1**

- [x] 3. Checkpoint - Ensure LLM format tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [x] 4. Implement Human format formatter
  - [x] 4.1 Create `HumanFormatter` in `crates/serializer/src/llm/human_formatter.rs`
    - Implement `HumanFormatConfig` with table_style, indent_size, max_width options
    - Implement `format()` method to output beautiful Human format
    - Implement `format_section_header()` with centered title and `═` box-drawing
    - Implement `format_config()` with 4-space indentation and key alignment
    - Implement `format_data_section()` to render tables
    - _Requirements: 3.1, 3.2_

  - [x] 4.2 Implement Unicode table rendering in `HumanFormatter`
    - Implement `build_table()` with Unicode box-drawing characters (┌─┬─┐ │ ├─┼─┤ └─┴─┘)
    - Implement column width calculation and alignment
    - Implement `format_cell_value()` with ✓/✗/— for booleans/null
    - Implement `generate_summary()` for table footer with counts and sums
    - _Requirements: 3.3, 3.4, 3.5, 3.6, 3.7, 3.8_

  - [x] 4.3 Create `HumanParser` in `crates/serializer/src/llm/human_parser.rs`
    - Implement `parse()` method to parse Human format string
    - Implement `parse_section_header()` for `[section]` headers
    - Implement `parse_key_value()` for `key = "value"` pairs
    - Implement `parse_table()` for Unicode box-drawn tables
    - Implement `parse_cell_value()` with ✓/✗/— recognition
    - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5, 4.6_

  - [x] 4.4 Write property test for Human format round-trip
    - **Property 2: Human Format Round-Trip**
    - **Validates: Requirements 3.1-3.8, 4.1-4.6, 9.2**

- [x] 5. Checkpoint - Ensure Human format tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [x] 6. Implement format conversions
  - [x] 6.1 Create conversion functions in `crates/serializer/src/llm/convert.rs`
    - Implement `llm_to_human()` conversion function
    - Implement `human_to_llm()` conversion function
    - Implement `llm_to_machine()` conversion function
    - Implement `machine_to_llm()` conversion function
    - Implement `human_to_machine()` conversion function
    - Implement `machine_to_human()` conversion function
    - _Requirements: 6.1-6.5, 7.1-7.5, 8.1-8.3_

  - [x] 6.2 Write property test for LLM↔Human conversion round-trip
    - **Property 3: LLM↔Human Conversion Round-Trip**
    - **Validates: Requirements 6.1-6.5, 7.1-7.5, 9.3**

  - [x] 6.3 Write property test for special value preservation
    - **Property 4: Special Value Preservation**
    - **Validates: Requirements 1.5-1.7, 2.4-2.6, 3.4-3.6, 4.4-4.6, 6.3, 7.3**

  - [x] 6.4 Write property test for reference resolution
    - **Property 5: Reference Resolution Correctness**
    - **Validates: Requirements 1.4, 2.2, 3.7, 6.2, 7.2**

- [ ] 7. Checkpoint - Ensure conversion tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 8. Create module structure and public API
  - [ ] 8.1 Create `crates/serializer/src/llm/mod.rs` module file
    - Export all public types: `DxDocument`, `DxSection`, `DxValue`, `AbbrevDict`
    - Export parser/serializer: `LlmParser`, `LlmSerializer`
    - Export formatter/parser: `HumanFormatter`, `HumanParser`, `HumanFormatConfig`
    - Export conversion functions
    - _Requirements: All_

  - [ ] 8.2 Update `crates/serializer/src/lib.rs` to export llm module
    - Add `pub mod llm;`
    - Re-export key types at crate root for convenience
    - _Requirements: All_

- [ ] 9. Create playground test files
  - [ ] 9.1 Create LLM format example file at `playground/dx-llm-example.dx`
    - Include context section with multiple key-value pairs
    - Include reference definitions
    - Include data section with schema and rows
    - Demonstrate all value types: strings, numbers, booleans (+/-), null (~), arrays (*), references (^)
    - _Requirements: 10.1_

  - [ ] 9.2 Create Human format example file at `playground/dx-human-example.dx`
    - Include beautiful section headers with box-drawing
    - Include config section with aligned keys
    - Include data table with Unicode borders
    - Include summary footer
    - _Requirements: 10.1_

  - [ ] 9.3 Create Rust test file at `playground/src/dx_format_test.rs`
    - Test LLM→Human→LLM round-trip conversion
    - Test Human→LLM→Human round-trip conversion
    - Test LLM→Machine→LLM round-trip conversion
    - Print results for visual verification
    - _Requirements: 10.2, 10.3, 10.4_

- [ ] 10. Final checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

## Notes

- All tasks are required for comprehensive implementation
- Each task references specific requirements for traceability
- Checkpoints ensure incremental validation
- Property tests validate universal correctness properties
- Unit tests validate specific examples and edge cases
- The `proptest` crate should be used for property-based testing in Rust
