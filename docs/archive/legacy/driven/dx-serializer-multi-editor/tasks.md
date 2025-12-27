# Implementation Plan: DX Serializer Multi-Editor Extensions

## Overview

This implementation plan creates DX serializer extensions for Neovim, Zed, JetBrains IDEs, and Xcode. The approach extracts the core library first, then builds each editor extension using the shared core.

## Tasks

- [ ] 1. Extract DX Core Library
  - [ ] 1.1 Create `crates/dx-core` package structure
    - Create package.json with no editor dependencies
    - Set up TypeScript configuration for library output
    - Configure exports for ESM and CommonJS
    - _Requirements: 1.1, 1.2_

  - [ ] 1.2 Extract core modules from vscode-dx-serializer
    - Copy llmParser.ts, humanParserV3.ts, humanFormatterV3.ts
    - Remove all vscode imports and dependencies
    - Create index.ts with public API exports
    - _Requirements: 1.1, 1.2_

  - [ ] 1.3 Add high-level transformation functions
    - Implement `llmToHuman(content: string): TransformResult`
    - Implement `humanToLlm(content: string): TransformResult`
    - Implement `formatHuman(content: string): TransformResult`
    - _Requirements: 1.2_

  - [ ] 1.4 Write property tests for core library
    - **Property 1: LLM to Human Round-Trip Consistency**
    - **Property 2: Human to LLM Round-Trip Consistency**
    - **Validates: Requirements 2.3, 2.4, 3.3, 3.4, 4.3, 4.4, 5.3, 5.4**

- [ ] 2. Checkpoint - Verify core library
  - Ensure all tests pass
  - Verify no editor-specific dependencies
  - Test npm package locally

- [ ] 3. Create Neovim Plugin (`dx.nvim`)
  - [ ] 3.1 Set up plugin structure
    - Create `crates/neovim-dx-serializer/` directory
    - Create lua/dx/init.lua with setup function
    - Create ftdetect/dx.vim for filetype detection
    - _Requirements: 2.1, 2.7_

  - [ ] 3.2 Implement core transformation in Lua
    - Port parsing logic to Lua (or use FFI to call Node.js)
    - Implement llm_to_human and human_to_llm functions
    - Handle error cases gracefully
    - _Requirements: 2.3, 2.4_

  - [ ] 3.3 Create syntax highlighting
    - Create syntax/dx.vim with Vim syntax rules
    - Create queries/dx/highlights.scm for Tree-sitter
    - Define highlight groups for keys, values, sections, pipes
    - _Requirements: 2.2, 2.8, 6.1, 6.2, 6.3, 6.4, 6.5_

  - [ ] 3.4 Implement auto-commands for file events
    - Create autocmds.lua for BufRead and BufWrite events
    - Transform LLM to Human on file open
    - Transform Human to LLM on file save
    - _Requirements: 2.3, 2.4, 2.5_

  - [ ] 3.5 Implement validation and error display
    - Parse content and collect errors
    - Display errors in quickfix list
    - Highlight error locations in buffer
    - _Requirements: 2.6, 7.1, 7.2_

  - [ ] 3.6 Write tests for Neovim plugin
    - Test transformation functions
    - Test syntax highlighting patterns
    - Test error handling
    - _Requirements: 7.3, 7.4_

- [ ] 4. Checkpoint - Verify Neovim plugin
  - Test installation via lazy.nvim
  - Test file open/save workflow
  - Verify syntax highlighting works

- [ ] 5. Create Zed Extension (`dx-zed`)
  - [ ] 5.1 Set up extension structure
    - Create `crates/zed-dx-serializer/` directory
    - Create Cargo.toml with zed-extension-api dependency
    - Create extension.toml manifest
    - _Requirements: 3.1, 3.7_

  - [ ] 5.2 Create Tree-sitter grammar for DX
    - Create grammars/dx/grammar.js
    - Define rules for sections, keys, values, arrays
    - Generate parser with tree-sitter-cli
    - _Requirements: 3.2_

  - [ ] 5.3 Write property test for grammar tokenization
    - **Property 3: Syntax Highlighting Tokenization**
    - **Validates: Requirements 6.2, 6.3, 6.4, 6.5**

  - [ ] 5.4 Create highlight queries
    - Create languages/dx/highlights.scm
    - Map grammar nodes to highlight groups
    - _Requirements: 3.2, 6.1, 6.2, 6.3, 6.4, 6.5, 6.6_

  - [ ] 5.5 Implement language server
    - Create src/lib.rs with Extension trait implementation
    - Implement transformation on file open/save
    - Handle validation and diagnostics
    - _Requirements: 3.3, 3.4, 3.5, 3.6_

  - [ ] 5.6 Write tests for Zed extension
    - Test grammar parsing
    - Test transformation functions
    - Test error handling
    - _Requirements: 7.3, 7.4_

- [ ] 6. Checkpoint - Verify Zed extension
  - Build WASM module
  - Test in Zed editor
  - Verify syntax highlighting and transformations

- [ ] 7. Create JetBrains Plugin (`dx-intellij`)
  - [ ] 7.1 Set up plugin structure
    - Create `crates/jetbrains-dx-serializer/` directory
    - Create build.gradle.kts with IntelliJ Platform plugin
    - Create plugin.xml descriptor
    - _Requirements: 4.1, 4.7, 4.8_

  - [ ] 7.2 Implement language support
    - Create DxLanguage.kt and DxFileType.kt
    - Register file type for .dx extension
    - Create file icon
    - _Requirements: 4.2_

  - [ ] 7.3 Implement syntax highlighting
    - Create DxSyntaxHighlighter.kt with TextAttributesKey definitions
    - Create DxLexer.kt for tokenization
    - Map tokens to highlight colors
    - _Requirements: 4.2, 6.1, 6.2, 6.3, 6.4, 6.5, 6.6_

  - [ ] 7.4 Implement virtual file system
    - Create DxVirtualFileSystem.kt
    - Transform LLM to Human on file read
    - Transform Human to LLM on file write
    - _Requirements: 4.3, 4.4, 4.5_

  - [ ] 7.5 Implement validation and annotations
    - Create DxAnnotator.kt for error highlighting
    - Display errors in Problems tool window
    - Highlight error locations in editor
    - _Requirements: 4.6, 7.1, 7.2_

  - [ ] 7.6 Write tests for JetBrains plugin
    - Test parser and lexer
    - Test transformation functions
    - Test error handling
    - _Requirements: 7.3, 7.4, 4.9_

- [ ] 8. Checkpoint - Verify JetBrains plugin
  - Build plugin ZIP
  - Test in IntelliJ IDEA
  - Test in Android Studio
  - Verify syntax highlighting and transformations

- [ ] 9. Create Xcode Extension (`DXSerializer`)
  - [ ] 9.1 Set up Xcode project structure
    - Create `crates/xcode-dx-serializer/` directory
    - Create Xcode project with Source Editor Extension target
    - Configure code signing and entitlements
    - _Requirements: 5.1, 5.7_

  - [ ] 9.2 Implement core transformation in Swift
    - Create DXCore.swift with transformation functions
    - Port parsing logic to Swift
    - Handle error cases gracefully
    - _Requirements: 5.3, 5.4_

  - [ ] 9.3 Implement editor commands
    - Create SourceEditorCommand.swift
    - Implement "Convert to Human Format" command
    - Implement "Convert to LLM Format" command
    - Implement "Format DX" command
    - _Requirements: 5.3, 5.4, 5.5_

  - [ ] 9.4 Implement syntax highlighting (if possible)
    - Research Xcode syntax highlighting options
    - Create UTI definition for .dx files
    - _Requirements: 5.2_

  - [ ] 9.5 Implement validation display
    - Show validation errors via alerts or issue navigator
    - _Requirements: 5.6, 7.1, 7.2_

  - [ ] 9.6 Write tests for Xcode extension
    - Test transformation functions
    - Test error handling
    - _Requirements: 7.3, 7.4_

- [ ] 10. Checkpoint - Verify Xcode extension
  - Build and sign extension
  - Test in Xcode
  - Verify commands work correctly

- [ ] 11. Implement shared property tests
  - [ ] 11.1 Write property test for validation error line numbers
    - **Property 4: Validation Error Line Numbers**
    - **Validates: Requirements 7.1, 7.4**

  - [ ] 11.2 Write property test for content preservation
    - **Property 5: Content Preservation on Failure**
    - **Validates: Requirements 7.3, 8.5**

  - [ ] 11.3 Write property test for key alignment
    - **Property 6: Format-on-Save Key Alignment**
    - **Validates: Requirements 8.1, 8.2**

  - [ ] 11.4 Write property test for section order
    - **Property 7: Section Order Preservation**
    - **Validates: Requirements 8.3**

  - [ ] 11.5 Write property test for LLM format detection
    - **Property 8: LLM Format Detection**
    - **Validates: Requirements 8.4**

  - [ ] 11.6 Write property test for malformed value warning
    - **Property 9: Malformed Value Warning**
    - **Validates: Requirements 7.5**

- [ ] 12. Final checkpoint - Integration testing
  - Test all extensions with the same DX files
  - Verify consistent behavior across editors
  - Document installation instructions for each editor
  - Ensure all tests pass, ask the user if questions arise

## Notes

- All tasks including property-based tests are required
- The core library should be completed first as all extensions depend on it
- Each editor extension can be developed in parallel after the core library
- Build commands:
  - Core: `npm run build` in `crates/dx-core`
  - Neovim: Copy to `~/.config/nvim/lua/` or use plugin manager
  - Zed: `cargo build --release` then package WASM
  - JetBrains: `./gradlew buildPlugin`
  - Xcode: Build in Xcode, archive for distribution
