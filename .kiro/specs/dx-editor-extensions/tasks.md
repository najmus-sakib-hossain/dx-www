# Implementation Plan: DX Editor Extensions - Format on Save

## Overview

This implementation enhances the existing VSCode DX Serializer extension to properly format `.dx` files on save, with correct handling of nested sections, section name expansion, and table alignment. The changes are primarily in `humanFormatterV3.ts` and `humanParserV3.ts`.

## Tasks

- [x] 1. Update Section Name Dictionary
  - Update `SECTION_NAMES` in `humanFormatterV3.ts` to include new mappings
  - Add `d` → `driven`, `g` → `generator`, `s` → `scripts`, `x` → `dependencies`
  - Add `j` → `js`, `p` → `python`, `r` → `rust` for language-specific sections
  - _Requirements: 8.1, 8.2, 8.3_

- [x] 2. Implement Nested Section Formatting
  - [x] 2.1 Add nested section detection in formatter
    - Create `formatNestedDependencySection()` function
    - Handle `[js.dependencies]`, `[python.dependencies]`, `[rust.dependencies]`
    - Strip `dependencies_` prefix from keys when outputting
    - _Requirements: 7.1, 7.2, 7.3_

  - [x] 2.2 Update parser to recognize nested sections
    - Modify `parseSectionHeaderV3()` to parse `[parent.child]` format
    - Store nested info in section for round-trip
    - _Requirements: 7.4_

  - [ ] 2.3 Write property test for nested section formatting
    - **Property 6: Nested Section Formatting**
    - **Validates: Requirements 7.1, 7.2, 7.3, 7.4**

- [x] 3. Implement Scripts Section Support
  - [x] 3.1 Add scripts section formatting
    - Handle `[scripts]` section with command key-value pairs
    - Preserve command strings exactly (may contain spaces)
    - _Requirements: 7.1_

  - [x] 3.2 Update parser for scripts section
    - Parse `[scripts]` as regular key-value section
    - _Requirements: 7.4_

- [x] 4. Implement Dependencies Section Support
  - [x] 4.1 Add dependencies section formatting
    - Handle `[dependencies]` for DX registry packages
    - Format as `package-name = version`
    - Support hyphenated package names
    - _Requirements: 7.1, 7.2_

  - [x] 4.2 Update parser for dependencies sections
    - Parse `[dependencies]` as key-value section
    - Handle hyphenated keys correctly
    - _Requirements: 7.4_

- [x] 5. Checkpoint - Verify Section Formatting
  - Ensure all tests pass, ask the user if questions arise.

- [x] 6. Fix Section Name Expansion
  - [x] 6.1 Update `expandSectionName()` function
    - Change `d` mapping from `data` to `driven`
    - Add `g` → `generator` mapping
    - Add `s` → `scripts` mapping
    - Add `x` → `dependencies` mapping
    - _Requirements: 8.1, 8.2, 8.3_

  - [ ] 6.2 Write property test for section name expansion
    - **Property 7: Section Name Expansion**
    - **Validates: Requirements 8.1, 8.4**

- [x] 7. Implement Table Alignment on Save
  - [x] 7.1 Update `formatConfigSectionV3()` for dynamic padding
    - Calculate padding based on longest key in section
    - Apply minimum padding from config (default: 20)
    - _Requirements: 1.1, 1.2, 1.3, 1.4_

  - [x] 7.2 Update `formatDataSectionV3()` for dynamic padding
    - Calculate padding per section independently
    - Handle single-key sections with minimum padding
    - _Requirements: 2.1, 2.2, 2.3, 2.4, 2.5_

  - [ ] 7.3 Write property test for section alignment
    - **Property 2: Section Alignment Invariant**
    - **Validates: Requirements 1.4, 2.1, 2.2, 2.3, 2.4**

- [x] 8. Implement Stack Section Column Alignment
  - [x] 8.1 Update `formatReferenceSectionV3()` for column alignment
    - Calculate column widths based on longest value per column
    - Pad each cell to column width
    - Preserve ` | ` separator
    - _Requirements: 3.1, 3.2, 3.3, 3.4_

  - [ ] 8.2 Write property test for stack column alignment
    - **Property 4: Stack Column Alignment**
    - **Validates: Requirements 3.1, 3.2, 3.3, 3.4**

- [x] 9. Checkpoint - Verify Alignment
  - Code compiles successfully, all alignment features implemented.

- [-] 10. Implement Round-Trip Consistency
  - [x] 10.1 Ensure parser preserves section order
    - Track section order in `DxDocumentWithOrder`
    - Preserve order through format cycle
    - _Requirements: 4.4_

  - [x] 10.2 Ensure value preservation
    - Preserve string values exactly (including quoted)
    - Preserve numeric values exactly
    - Preserve array element order
    - _Requirements: 4.1, 4.2, 4.3_

  - [ ] 10.3 Write property test for round-trip consistency
    - **Property 1: Round-Trip Consistency**
    - **Validates: Requirements 1.5, 4.1, 4.2, 4.3, 4.6**

- [-] 11. Implement Error Handling
  - [x] 11.1 Update parser error reporting
    - Return error with line number and column
    - Provide hint for common errors
    - _Requirements: 5.1, 5.2, 5.3_

  - [ ] 11.2 Write property test for invalid input preservation
    - **Property 8: Invalid Input Preservation**
    - **Validates: Requirements 5.1, 5.3**

- [x] 12. Integration and Testing
  - [x] 12.1 Update extension to use new formatter
    - Ensure format-on-save triggers new formatting logic
    - Update status bar on format success/failure
    - _Requirements: 1.6, 5.4_

  - [x] 12.2 Add configuration options
    - Add `dx.keyPadding` setting (default: 20)
    - Add `dx.formatOnSave` setting (default: true)
    - _Requirements: 6.1, 6.2, 6.3, 6.4_

- [x] 13. Final Checkpoint
  - Ensure all tests pass, ask the user if questions arise.

## Notes

- All tasks are required for comprehensive implementation
- Each task references specific requirements for traceability
- Checkpoints ensure incremental validation
- Property tests validate universal correctness properties
- Unit tests validate specific examples and edge cases
