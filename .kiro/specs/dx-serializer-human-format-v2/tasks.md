# Implementation Plan: DX Serializer Human Format V2

## Overview

This implementation plan transforms the DX Serializer Human Format to provide cleaner output, fix the sigil parsing error, and enable automatic cache generation. The implementation follows an incremental approach, building on the existing codebase.

## Tasks

- [x] 1. Fix Sigil Parsing Error in Human Parser
  - [x] 1.1 Update is_comment_line() to handle '# ' prefix correctly
    - Modify the condition to check for '# ' (hash + space) as a comment
    - Add handling for decorative lines like '# ═══'
    - Ensure '#c:', '#:', and '#<letter>(' are NOT treated as comments
    - _Requirements: 1.1, 1.5_
  - [x] 1.2 Update parse_line() to skip comment lines before sigil checking
    - Move comment detection before sigil parsing
    - Return early for comment lines without error
    - _Requirements: 1.1, 1.2, 1.3, 1.4_
  - [x] 1.3 Write property test for comment and sigil parsing
    - **Property 1: Comment and sigil parsing**
    - **Validates: Requirements 1.1, 1.2, 1.3, 1.4**

- [x] 2. Extend Abbreviation Dictionary
  - [x] 2.1 Add new key mappings to AbbrevDict
    - Add "v" → "version" (contextual default)
    - Add "ws" → "workspace"
    - Add "eds" → "editors"
    - Add "repo" → "repository"
    - Add "cont" → "container"
    - Add "ci" → "ci_cd"
    - _Requirements: 2.4_
  - [x] 2.2 Write unit tests for new abbreviation mappings
    - Test expansion of new keys
    - Test compression of new full names
    - _Requirements: 2.4_

- [x] 3. Implement HumanFormatterV2 with Flat Structure
  - [x] 3.1 Create FormatterConfig with new options
    - Add expand_keys option (default: true)
    - Add max_line_width option (default: 120)
    - Add use_flat_structure option (default: true)
    - _Requirements: 3.1, 5.4_
  - [x] 3.2 Implement format_config_section() without indentation
    - Remove leading indentation from key-value pairs
    - Align '=' signs using spaces after key names
    - Format arrays as comma-separated lists without brackets
    - _Requirements: 3.1, 3.2, 3.3_
  - [x] 3.3 Implement format_data_section() with full section names
    - Use full section name in brackets (e.g., '[forge]' not '[f]')
    - Place table directly after header without indentation
    - _Requirements: 4.1, 4.2_
  - [x] 3.4 Write property test for format structure validation
    - **Property 4: Format structure validation**
    - **Validates: Requirements 3.1, 3.2, 3.3, 3.4, 3.5, 4.1, 4.2**

- [x] 4. Implement Table Wrapper for Wide Tables
  - [x] 4.1 Create TableWrapper struct with configuration
    - Store max_width setting
    - Implement needs_wrapping() to check if wrapping is needed
    - _Requirements: 5.4_
  - [x] 4.2 Implement calculate_widths() for optimal column sizing
    - Calculate minimum width for each column based on content
    - Consider max_line_width constraint
    - _Requirements: 5.1_
  - [x] 4.3 Implement wrap_row() for splitting wide rows
    - Split row into multiple display lines when exceeding max width
    - Maintain column alignment across wrapped lines
    - Add continuation indicators (e.g., '...' or '↓')
    - _Requirements: 5.1, 5.2, 5.3_
  - [x] 4.4 Write property test for table wrapping round-trip
    - **Property 6: Table wrapping round-trip**
    - **Validates: Requirements 5.1, 5.2, 5.3, 5.5**

- [x] 5. Update Human Parser for V2 Format
  - [x] 5.1 Update parse_key_value() to accept full key names
    - Accept both abbreviated and full key names
    - Normalize to abbreviated form internally
    - _Requirements: 2.2_
  - [x] 5.2 Update parse_section_header() to accept full section names
    - Map full names to single-letter IDs
    - Accept both formats for backward compatibility
    - _Requirements: 4.3_
  - [x] 5.3 Implement parse_wrapped_table() for multi-line rows
    - Detect continuation indicators
    - Reconstruct original row data from wrapped lines
    - _Requirements: 5.5_
  - [x] 5.4 Write property test for parser flexibility
    - **Property 3: Parser key name flexibility**
    - **Property 5: Parser section name flexibility**
    - **Validates: Requirements 2.2, 4.3**

- [x] 6. Checkpoint - Verify Parser and Formatter Work Together
  - Ensure all tests pass, ask the user if questions arise.

- [-] 7. Implement Cache Generator
  - [x] 7.1 Create CacheConfig and CacheGenerator structs
    - Define cache_root path
    - Add flags for generate_llm and generate_machine
    - _Requirements: 6.1, 6.2_
  - [x] 7.2 Implement map_path_to_cache() for path preservation
    - Extract relative path from source
    - Preserve subfolder structure in cache
    - Normalize path separators
    - _Requirements: 6.3, 7.1, 7.2, 7.3, 7.4_
  - [x] 7.3 Implement generate() for cache file creation
    - Convert document to LLM format and write to cache
    - Convert document to Machine format and write to cache
    - Use atomic writes (temp file + rename)
    - _Requirements: 6.1, 6.2, 6.4_
  - [x] 7.4 Implement error handling for cache generation
    - Don't corrupt existing files on failure
    - Return detailed error information
    - _Requirements: 6.5_
  - [x] 7.5 Write property test for cache generation
    - **Property 7: Cache generation with path preservation**
    - **Property 8: Cache error handling**
    - **Validates: Requirements 6.1, 6.2, 6.3, 6.4, 6.5, 7.3, 7.4**

- [x] 8. Implement Table Dynamic Sizing
  - [x] 8.1 Update build_table() to recalculate widths dynamically
    - Recalculate column widths when rows change
    - Adjust borders to match new widths
    - _Requirements: 8.1, 8.2, 8.3_
  - [x] 8.2 Add validation for Unicode box-drawing characters
    - Verify all border characters are valid Unicode
    - Ensure proper corner and intersection characters
    - _Requirements: 8.4_
  - [x] 8.3 Write property test for table dynamic sizing
    - **Property 9: Table dynamic sizing**
    - **Validates: Requirements 8.1, 8.2, 8.3, 8.4, 8.5**

- [x] 9. Implement Round-Trip Conversion Functions
  - [x] 9.1 Implement human_to_llm_v2() conversion
    - Parse Human format to DxDocument
    - Serialize DxDocument to LLM format
    - _Requirements: 9.1_
  - [x] 9.2 Implement llm_to_human_v2() conversion
    - Parse LLM format to DxDocument
    - Format DxDocument to Human format
    - _Requirements: 9.2_
  - [x] 9.3 Update machine format conversions
    - Ensure Machine format preserves all data
    - Verify round-trip consistency
    - _Requirements: 9.3_
  - [x] 9.4 Write property test for format round-trip
    - **Property 10: Format round-trip consistency**
    - **Validates: Requirements 9.1, 9.2, 9.3, 9.4, 9.5**

- [ ] 10. Implement Pretty Printer
  - [ ] 10.1 Create PrettyPrinter struct
    - Wrap HumanFormatterV2 with validation
    - Ensure output is always parseable
    - _Requirements: 10.1, 10.2_
  - [ ] 10.2 Write property test for pretty printer round-trip
    - **Property 11: Pretty printer round-trip**
    - **Validates: Requirements 10.1, 10.2, 10.3**

- [ ] 11. Checkpoint - Verify All Round-Trip Properties
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 12. Integration and Wiring
  - [ ] 12.1 Update mod.rs to export new V2 types
    - Export HumanParserV2, HumanFormatterV2
    - Export CacheGenerator, CacheConfig
    - Export TableWrapper
    - _Requirements: All_
  - [ ] 12.2 Update lib.rs to re-export V2 types
    - Add convenience functions for V2 format
    - Maintain backward compatibility with V1
    - _Requirements: All_
  - [ ] 12.3 Create example usage in examples/ or tests/
    - Demonstrate Human format V2 usage
    - Show cache generation workflow
    - _Requirements: All_

- [ ] 13. Final Checkpoint - All Tests Pass
  - Ensure all tests pass, ask the user if questions arise.

## Notes

- All tasks are required for comprehensive implementation
- Each task references specific requirements for traceability
- Checkpoints ensure incremental validation
- Property tests validate universal correctness properties
- Unit tests validate specific examples and edge cases
