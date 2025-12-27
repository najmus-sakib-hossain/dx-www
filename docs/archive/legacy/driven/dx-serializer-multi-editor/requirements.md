# Requirements Document

## Introduction

This document specifies the requirements for creating DX serializer extensions for multiple code editors: Neovim, Zed, JetBrains IDEs (Android Studio, IntelliJ IDEA, WebStorm, etc.), and Xcode. Each extension will provide the same core functionality as the existing VS Code extension: transforming DX files between LLM format (dense, token-optimized) and Human format (readable, TOML-like) with syntax highlighting, validation, and format-on-save.

## Glossary

- **DX_File**: A configuration file with `.dx` extension containing project metadata in either LLM or Human format
- **LLM_Format**: Token-optimized dense format using sigils (#c:, #:, #<letter>) for machine processing
- **Human_Format**: Readable TOML-like format with `key = value` pairs and `[section]` headers
- **Parser**: Component that transforms Human format to internal DxDocument representation
- **Formatter**: Component that transforms DxDocument to Human format for display
- **Serializer**: Component that transforms DxDocument to LLM format for disk storage
- **Neovim_Plugin**: Lua-based plugin for Neovim editor
- **Zed_Extension**: Rust/WASM-based extension for Zed editor
- **JetBrains_Plugin**: Kotlin/Java-based plugin for JetBrains IDEs
- **Xcode_Extension**: Swift-based Source Editor Extension for Xcode

## Requirements

### Requirement 1: Core Library Extraction

**User Story:** As a developer, I want a shared core library for DX parsing/formatting, so that all editor extensions use consistent logic.

#### Acceptance Criteria

1. THE Core_Library SHALL be implemented in TypeScript with no editor-specific dependencies
2. THE Core_Library SHALL export `parseLlm`, `parseHumanV3`, `formatDocumentV3`, and `serializeToLlmV3` functions
3. THE Core_Library SHALL be publishable as an npm package for JavaScript/TypeScript consumers
4. THE Core_Library SHALL be compilable to WASM for Rust/native consumers
5. WHEN the Core_Library is updated, THEN all editor extensions SHALL be able to consume the update

### Requirement 2: Neovim Plugin

**User Story:** As a Neovim user, I want a DX serializer plugin, so that I can edit DX files with syntax highlighting and format-on-save.

#### Acceptance Criteria

1. THE Neovim_Plugin SHALL be written in Lua for native Neovim integration
2. THE Neovim_Plugin SHALL provide syntax highlighting for DX Human format
3. THE Neovim_Plugin SHALL transform LLM format to Human format when opening `.dx` files
4. THE Neovim_Plugin SHALL transform Human format to LLM format when saving `.dx` files
5. WHEN a user saves a `.dx` file, THE Neovim_Plugin SHALL format the content before saving
6. THE Neovim_Plugin SHALL display validation errors in the quickfix list
7. THE Neovim_Plugin SHALL be installable via popular plugin managers (lazy.nvim, packer, vim-plug)
8. THE Neovim_Plugin SHALL use Tree-sitter for syntax highlighting if available

### Requirement 3: Zed Extension

**User Story:** As a Zed user, I want a DX serializer extension, so that I can edit DX files with syntax highlighting and format-on-save.

#### Acceptance Criteria

1. THE Zed_Extension SHALL be written in Rust compiled to WASM
2. THE Zed_Extension SHALL provide syntax highlighting for DX Human format via Tree-sitter grammar
3. THE Zed_Extension SHALL transform LLM format to Human format when opening `.dx` files
4. THE Zed_Extension SHALL transform Human format to LLM format when saving `.dx` files
5. WHEN a user saves a `.dx` file, THE Zed_Extension SHALL format the content before saving
6. THE Zed_Extension SHALL display validation errors inline
7. THE Zed_Extension SHALL be publishable to the Zed extension registry

### Requirement 4: JetBrains Plugin

**User Story:** As a JetBrains IDE user (Android Studio, IntelliJ, WebStorm), I want a DX serializer plugin, so that I can edit DX files with syntax highlighting and format-on-save.

#### Acceptance Criteria

1. THE JetBrains_Plugin SHALL be written in Kotlin for modern JetBrains plugin development
2. THE JetBrains_Plugin SHALL provide syntax highlighting for DX Human format
3. THE JetBrains_Plugin SHALL transform LLM format to Human format when opening `.dx` files
4. THE JetBrains_Plugin SHALL transform Human format to LLM format when saving `.dx` files
5. WHEN a user saves a `.dx` file, THE JetBrains_Plugin SHALL format the content before saving
6. THE JetBrains_Plugin SHALL display validation errors in the Problems tool window
7. THE JetBrains_Plugin SHALL be compatible with IntelliJ Platform 2023.1+
8. THE JetBrains_Plugin SHALL be publishable to the JetBrains Marketplace
9. THE JetBrains_Plugin SHALL work in Android Studio, IntelliJ IDEA, WebStorm, PyCharm, and other JetBrains IDEs

### Requirement 5: Xcode Source Editor Extension

**User Story:** As an Xcode user, I want a DX serializer extension, so that I can edit DX files with syntax highlighting and format-on-save.

#### Acceptance Criteria

1. THE Xcode_Extension SHALL be written in Swift as a Source Editor Extension
2. THE Xcode_Extension SHALL provide syntax highlighting for DX Human format
3. THE Xcode_Extension SHALL transform LLM format to Human format via Editor menu command
4. THE Xcode_Extension SHALL transform Human format to LLM format via Editor menu command
5. THE Xcode_Extension SHALL provide a "Format DX" command in the Editor menu
6. THE Xcode_Extension SHALL display validation errors via Xcode's issue navigator when possible
7. THE Xcode_Extension SHALL be distributable via the Mac App Store or direct download

### Requirement 6: Syntax Highlighting Consistency

**User Story:** As a user of any supported editor, I want consistent syntax highlighting, so that DX files look the same across all editors.

#### Acceptance Criteria

1. THE Syntax_Highlighting SHALL use consistent colors for keys, values, sections, and comments
2. THE Syntax_Highlighting SHALL highlight section headers `[section]` distinctly
3. THE Syntax_Highlighting SHALL highlight keys and values with different colors
4. THE Syntax_Highlighting SHALL highlight pipe separators `|` in arrays
5. THE Syntax_Highlighting SHALL highlight quoted strings
6. THE Syntax_Highlighting SHALL highlight the `[stack]` section specially

### Requirement 7: Error Handling and Validation

**User Story:** As a developer, I want clear error messages when DX files have syntax errors, so that I can fix them quickly.

#### Acceptance Criteria

1. WHEN a syntax error is detected, THE Extension SHALL display the error message with line number
2. WHEN a syntax error is detected, THE Extension SHALL highlight the error location in the editor
3. WHEN validation fails on save, THE Extension SHALL preserve the original content
4. THE Extension SHALL validate bracket matching, quote matching, and section header format
5. IF a value starts with `=`, THEN THE Extension SHALL warn about potential malformed input

### Requirement 8: Format-on-Save Behavior

**User Story:** As a developer, I want format-on-save to work consistently, so that my DX files are always properly formatted.

#### Acceptance Criteria

1. WHEN format-on-save is enabled, THE Extension SHALL parse and reformat Human content before saving
2. WHEN format-on-save is enabled, THE Extension SHALL align keys with consistent padding
3. WHEN format-on-save is enabled, THE Extension SHALL preserve section order
4. WHEN the content is already in LLM format, THE Extension SHALL NOT apply Human formatting
5. IF format-on-save fails, THEN THE Extension SHALL save the original content unchanged
