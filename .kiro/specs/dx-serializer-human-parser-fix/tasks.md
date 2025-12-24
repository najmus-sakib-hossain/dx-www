# Implementation Plan: DX Serializer Human Parser Fix

## Overview

This implementation plan fixes critical bugs in the human format parser that cause file corruption during format-on-save operations. The fixes target `humanParserV3.ts` and `dxLensFileSystem.ts`.

## Tasks

- [x] 1. Fix key-value line parsing in humanParserV3.ts
  - [x] 1.1 Update `parseKeyValueLineV3` to add validation for values starting with `=`
    - Add warning log when value starts with `=` (indicates upstream issue)
    - Ensure proper trimming of both key and value
    - _Requirements: 2.1, 2.2, 2.3_

  - [x] 1.2 Write property test for key-value parsing correctness
    - **Property 2: Key-Value Parsing Correctness**
    - **Validates: Requirements 1.1, 1.4, 2.1, 5.1**

- [x] 2. Fix stack section parsing in humanParserV3.ts
  - [x] 2.1 Update stack section handling in `parseHumanV3` function
    - Fix the line that stores refs: `const refValue = value.split(' | ').map(v => v.trim()).filter(v => v).join('|');`
    - Ensure the value from `parseKeyValueLineV3` is used correctly (already trimmed, no `=` prefix)
    - _Requirements: 1.1, 1.2, 1.3, 1.4_

  - [x] 2.2 Write property test for no value corruption
    - **Property 3: No Value Corruption**
    - **Validates: Requirements 1.2, 2.2**

- [x] 3. Fix nested section parsing in humanParserV3.ts
  - [x] 3.1 Review and fix nested section key-value storage
    - Ensure keys are stored with correct prefix (e.g., `dependencies_react`)
    - Preserve hyphenated package names (e.g., `actix-web`)
    - _Requirements: 3.1, 3.2_

  - [x] 3.2 Review and fix nested section reconstruction
    - Ensure all fields are preserved when converting to DxSection format
    - Verify section order is maintained
    - _Requirements: 3.3_

  - [x] 3.3 Write property test for nested section preservation
    - **Property 4: Nested Section Preservation**
    - **Validates: Requirements 3.1, 3.2, 3.3**

- [x] 4. Implement round-trip consistency verification
  - [x] 4.1 Create test utility for comparing parsed documents
    - Compare context maps
    - Compare refs maps
    - Compare sections (schema and rows)
    - _Requirements: 4.1, 4.2_

  - [x] 4.2 Write property test for round-trip consistency
    - **Property 1: Round-Trip Consistency**
    - **Validates: Requirements 4.1, 4.2, 4.3, 4.4, 6.2, 6.3**

- [x] 5. Checkpoint - Verify parser fixes
  - Run all tests to ensure parser fixes work correctly
  - Manually test with the `dx` file to verify no corruption
  - Ensure all tests pass, ask the user if questions arise

- [x] 6. Re-enable format-on-save in dxLensFileSystem.ts
  - [x] 6.1 Update `writeFile` method to format content before saving
    - Parse human content with `parseHumanV3`
    - If successful, format with `formatDocumentV3`
    - Use formatted content for saving
    - _Requirements: 6.1, 6.2, 6.3_

  - [x] 6.2 Add error handling for format-on-save failures
    - If parsing fails, preserve original content
    - Display error message to user
    - _Requirements: 6.4_

- [x] 7. Final checkpoint - Build and test extension
  - Build VSIX package: `npx @vscode/vsce package --allow-missing-repository`
  - Install and test with real DX files
  - Verify format-on-save works without corruption
  - Ensure all tests pass, ask the user if questions arise

## Notes

- All tasks including property-based tests are required
- The `dx` file in the workspace root can be used for manual testing
- Build command: `npx @vscode/vsce package --allow-missing-repository`
- Install command: `kiro --install-extension "path\to\vsix" --force`
