# Implementation Plan: DX Serializer VS Code Extension

## Overview

This implementation plan creates a VS Code extension that provides seamless editing of `.dx` files with human-readable display and dense storage. The implementation uses TypeScript for the extension with a Rust/WASM core for high-performance transformations.

## Tasks

- [ ] 1. Set up extension project structure
  - Create `vscode-dx-serializer/` directory in `crates/` folder
  - Create `package.json` with extension manifest, dependencies, and contribution points
  - Create `tsconfig.json` for TypeScript compilation
  - Create `language-configuration.json` for bracket matching and comments
  - Create `syntaxes/dx.tmLanguage.json` for syntax highlighting
  - Create `.vscode/launch.json` for debugging
  - _Requirements: 10.1-10.6, 11.1-11.5_

- [ ] 2. Implement WASM core bindings
  - [ ] 2.1 Create `crates/serializer/src/wasm.rs` with WASM bindings
    - Implement `DxSerializer` struct with `to_human`, `to_dense`, `validate`, `is_saveable` methods
    - Implement `ValidationResult` and `TransformResult` structs
    - Implement smart quoting logic for apostrophes and mixed quotes
    - Add wasm-bindgen annotations for JavaScript interop
    - _Requirements: 1.1, 1.2, 2.1-2.5_
  - [ ] 2.2 Write property test for round-trip transformation
    - **Property 1: Round-trip transformation consistency**
    - **Validates: Requirements 1.1, 1.2, 1.6**
  - [ ] 2.3 Write property test for string preservation
    - **Property 3: String value preservation**
    - **Validates: Requirements 2.1, 2.2, 2.3, 2.4, 2.5**

- [ ] 3. Build WASM and create build script
  - Create `scripts/build-wasm.sh` for building WASM with wasm-pack
  - Update `crates/serializer/Cargo.toml` with wasm feature and dependencies
  - Build WASM to `vscode-dx-serializer/wasm/` directory
  - Verify WASM bindings are generated correctly
  - _Requirements: 12.1, 12.4_

- [ ] 4. Implement utility functions
  - [ ] 4.1 Create `src/utils.ts` with helper functions
    - Implement `isExactlyDxFile()` for file type detection
    - Implement `getDiskUri()` and `getLensUri()` for URI conversion
    - Implement `debounce()` for save operations
    - _Requirements: 5.1, 5.2, 5.3_
  - [ ] 4.2 Write property test for file type filtering
    - **Property 6: File type filtering correctness**
    - **Validates: Requirements 5.1, 5.2, 5.3**

- [ ] 5. Implement DxCore wrapper
  - [ ] 5.1 Create `src/dxCore.ts` with WASM wrapper and fallback
    - Implement `loadDxCore()` function to load WASM with fallback
    - Implement `DxCore` interface with `toHuman`, `toDense`, `validate`, `isSaveable`
    - Implement TypeScript fallback for `formatDx()`, `minifyDx()`, `validateDx()`
    - Implement `smartQuote()` function for proper quoting
    - _Requirements: 12.2, 12.3_
  - [ ] 5.2 Write property test for WASM/fallback equivalence
    - **Property 7: WASM and Fallback Equivalence**
    - **Validates: Requirements 12.3**

- [ ] 6. Checkpoint - Verify core transformation logic
  - Ensure all WASM tests pass
  - Ensure fallback implementation matches WASM behavior
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 7. Implement DxDocumentManager
  - [ ] 7.1 Create `src/dxDocumentManager.ts` with document state management
    - Implement `DocumentState` interface for tracking document state
    - Implement `initializeDocument()` for opening files
    - Implement `saveDocument()` with validation gating and grace period
    - Implement `forceSave()` for bypassing validation
    - Implement `handleExternalChange()` for file watcher events
    - Implement `forceRefresh()` for manual refresh
    - Implement diagnostic updates for validation errors
    - _Requirements: 3.1-3.5, 4.1, 4.6, 6.1-6.5_
  - [ ] 7.2 Write property test for validation error quality
    - **Property 5: Validation error quality**
    - **Validates: Requirements 4.2, 4.3, 4.4, 4.5**

- [ ] 8. Implement DxLensFileSystem
  - Create `src/dxLensFileSystem.ts` with virtual file system provider
  - Implement `readFile()` to transform dense to human format
  - Implement `writeFile()` to transform human to dense format
  - Implement `stat()` to return file stats with human content size
  - Implement `watch()` for file change events
  - Wire up to DocumentManager for state coordination
  - _Requirements: 1.1, 1.2, 1.3, 1.4, 1.5_

- [ ] 9. Implement extension entry point
  - Create `src/extension.ts` with activation logic
  - Load WASM core with fallback handling
  - Initialize DocumentManager and LensFileSystem
  - Register FileSystemProvider for 'dxlens' scheme
  - Set up file watcher for external changes
  - Implement auto-redirect from file:// to dxlens:// for .dx files
  - _Requirements: 1.1, 6.1_

- [ ] 10. Checkpoint - Verify file system integration
  - Test opening .dx files shows human format
  - Test saving .dx files writes dense format
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 11. Implement user commands
  - Register "DX: Refresh from Disk" command
  - Register "DX: Force Save" command
  - Register "DX: Show Dense View" command
  - Add refresh button to editor title bar for .dx files
  - _Requirements: 7.1, 7.2, 7.3, 7.4_

- [ ] 12. Implement status bar
  - Create status bar item for validation status
  - Show green checkmark when valid
  - Show warning with error message when invalid
  - Show temporary message when auto-save is skipped
  - Wire click to show dense preview
  - _Requirements: 8.1, 8.2, 8.3, 8.4_

- [ ] 13. Implement configuration handling
  - Read `dx.validateBeforeSave` setting
  - Read `dx.autoSaveGracePeriod` setting
  - Read `dx.indentSize` setting
  - Apply configuration changes without restart
  - _Requirements: 9.1, 9.2, 9.3, 9.4_

- [ ] 14. Checkpoint - Full extension functionality
  - Test all commands work correctly
  - Test status bar updates correctly
  - Test configuration changes apply
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 15. Write property test for human format structure
  - **Property 2: Human format structure**
  - **Validates: Requirements 1.3, 1.4, 1.5**

- [ ] 16. Write property test for smart quoting
  - **Property 4: Smart quoting correctness**
  - **Validates: Requirements 2.3, 2.4**

- [ ] 17. Add extension assets
  - Copy `media/logo.png` for extension icon
  - Copy `media/file-extension-dark.png` for file icon
  - Update package.json with icon references
  - _Requirements: 10.1_

- [ ] 18. Final checkpoint - Complete extension
  - Run full test suite
  - Verify all property tests pass
  - Test with VS Code auto-save enabled
  - Test with Cursor/Copilot if available
  - Ensure all tests pass, ask the user if questions arise.

## Notes

- All tasks are required for comprehensive implementation
- Each task references specific requirements for traceability
- Checkpoints ensure incremental validation
- Property tests validate universal correctness properties
- Unit tests validate specific examples and edge cases
- The WASM core provides performance, TypeScript fallback ensures reliability
