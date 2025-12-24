# Requirements Document

## Introduction

The DX Serializer VS Code Extension provides seamless editing of `.dx` files by displaying a human-readable format in the editor while storing a dense, token-efficient format on disk. This enables developers to work with readable code while LLMs and Git benefit from the compact representation. The extension must handle auto-save correctly, preserve all values without truncation, and provide real-time validation.

## Glossary

- **DX_File**: A file with the `.dx` extension containing serialized data in either dense or human-readable format
- **Dense_Format**: The compact, token-efficient representation stored on disk, optimized for LLMs and version control
- **Human_Format**: The readable, indented representation displayed in the VS Code editor
- **DxLens_FileSystem**: A virtual file system provider that intercepts `.dx` file operations to transform between formats
- **Document_Manager**: The component responsible for tracking document state, validation, and coordinating saves
- **Grace_Period**: A configurable delay after the last keystroke before auto-save writes to disk
- **Smart_Quoting**: Automatic selection of quote characters to properly handle special characters like apostrophes
- **WASM_Core**: The Rust-compiled WebAssembly module providing high-performance format transformations
- **Validation_Result**: The outcome of syntax checking including success status, error details, and hints

## Requirements

### Requirement 1: File Format Transformation

**User Story:** As a developer, I want `.dx` files to appear human-readable in my editor while being stored in dense format on disk, so that I can read and edit them easily while LLMs get token-efficient content.

#### Acceptance Criteria

1. WHEN a DX_File is opened in VS Code, THE DxLens_FileSystem SHALL transform the Dense_Format content to Human_Format for display
2. WHEN a DX_File is saved, THE DxLens_FileSystem SHALL transform the Human_Format content to Dense_Format before writing to disk
3. THE Human_Format SHALL include proper indentation with configurable indent size (2 or 4 spaces)
4. THE Human_Format SHALL include aligned colons after keys for readability
5. THE Dense_Format SHALL remove all unnecessary whitespace and comments
6. FOR ALL valid DX content, transforming from Dense_Format to Human_Format and back SHALL produce equivalent Dense_Format content (round-trip property)

### Requirement 2: Value Preservation

**User Story:** As a developer, I want all my data values preserved exactly, so that URLs, special characters, and long strings are never truncated or corrupted.

#### Acceptance Criteria

1. WHEN transforming content, THE WASM_Core SHALL preserve all string values exactly without truncation
2. WHEN a string contains URLs, THE WASM_Core SHALL preserve the complete URL including query parameters and fragments
3. WHEN a string contains apostrophes (e.g., "don't"), THE WASM_Core SHALL use Smart_Quoting to wrap with double quotes
4. WHEN a string contains both single and double quotes, THE WASM_Core SHALL escape double quotes and use double quote delimiters
5. WHEN a string contains escape sequences, THE WASM_Core SHALL preserve them correctly through transformation

### Requirement 3: Auto-Save Compatibility

**User Story:** As a developer, I want auto-save to work correctly without corrupting my files, so that incomplete code during typing doesn't get saved to disk.

#### Acceptance Criteria

1. WHEN auto-save triggers during active typing, THE Document_Manager SHALL skip the save if the Grace_Period has not elapsed since the last keystroke
2. WHEN content fails validation, THE Document_Manager SHALL skip the save and display a status bar warning
3. THE Grace_Period SHALL be configurable between 500ms and 10000ms with a default of 2000ms
4. WHEN a save is skipped due to validation failure, THE Document_Manager SHALL preserve the last valid Dense_Format on disk
5. WHEN the user explicitly saves (Ctrl+S), THE Document_Manager SHALL respect validation settings

### Requirement 4: Syntax Validation

**User Story:** As a developer, I want real-time syntax validation, so that I can see errors immediately and understand how to fix them.

#### Acceptance Criteria

1. WHEN content is edited, THE Document_Manager SHALL validate syntax and update diagnostics immediately
2. WHEN validation fails due to unclosed brackets, THE Validation_Result SHALL include the line and column of the opening bracket
3. WHEN validation fails due to unclosed strings, THE Validation_Result SHALL include the line and column where the string started
4. WHEN validation fails due to mismatched brackets, THE Validation_Result SHALL include a hint about which bracket was expected
5. THE Validation_Result SHALL include actionable hints for all error types
6. WHEN content is valid, THE Document_Manager SHALL clear all diagnostics for the file

### Requirement 5: File Type Filtering

**User Story:** As a developer, I want only pure `.dx` files to be transformed, so that compound extensions like `.dx.json` are handled normally.

#### Acceptance Criteria

1. THE DxLens_FileSystem SHALL only intercept files ending exactly with `.dx`
2. WHEN a file has a compound extension (e.g., `.dx.json`, `.dx.yml`, `.dx.bak`), THE DxLens_FileSystem SHALL NOT intercept it
3. WHEN a file is opened with scheme other than `file`, THE DxLens_FileSystem SHALL NOT intercept it

### Requirement 6: External Change Handling

**User Story:** As a developer, I want my editor to update when files change externally (git, other editors), so that I always see the current content.

#### Acceptance Criteria

1. WHEN a DX_File changes on disk from an external source, THE Document_Manager SHALL detect the change
2. WHEN an external change is detected, THE Document_Manager SHALL transform the new Dense_Format to Human_Format
3. WHEN an external change is detected, THE Document_Manager SHALL update the editor view with the new content
4. WHEN the extension itself writes a file, THE Document_Manager SHALL NOT treat it as an external change
5. WHEN a DX_File is deleted externally, THE Document_Manager SHALL clean up associated state

### Requirement 7: User Commands

**User Story:** As a developer, I want commands to manually control the extension, so that I can refresh from disk or force save when needed.

#### Acceptance Criteria

1. WHEN the user executes "DX: Refresh from Disk", THE Document_Manager SHALL reload the file from disk and re-transform
2. WHEN the user executes "DX: Force Save", THE Document_Manager SHALL save without validation checks
3. WHEN the user executes "DX: Show Dense View", THE Extension SHALL open a read-only preview of the Dense_Format
4. THE refresh command SHALL be available in the editor title bar for `.dx` files

### Requirement 8: Status Indication

**User Story:** As a developer, I want to see the validation status at a glance, so that I know if my file is valid and saveable.

#### Acceptance Criteria

1. WHEN a DX_File is open and valid, THE Extension SHALL show a green checkmark status bar item
2. WHEN a DX_File is open and invalid, THE Extension SHALL show a warning status bar item with the error message
3. WHEN auto-save is skipped due to validation, THE Extension SHALL show a temporary status bar message
4. WHEN clicking the status bar item, THE Extension SHALL show the dense format preview

### Requirement 9: Configuration Options

**User Story:** As a developer, I want to configure the extension behavior, so that it fits my workflow preferences.

#### Acceptance Criteria

1. THE Extension SHALL provide a `dx.validateBeforeSave` boolean setting (default: true)
2. THE Extension SHALL provide a `dx.autoSaveGracePeriod` number setting (default: 2000ms)
3. THE Extension SHALL provide a `dx.indentSize` number setting with values 2 or 4 (default: 2)
4. WHEN configuration changes, THE Extension SHALL apply them without requiring restart

### Requirement 10: Syntax Highlighting

**User Story:** As a developer, I want syntax highlighting for `.dx` files, so that the code is easier to read and understand.

#### Acceptance Criteria

1. THE Extension SHALL provide TextMate grammar for `.dx` files
2. THE grammar SHALL highlight strings (single and double quoted)
3. THE grammar SHALL highlight numbers (integers, decimals, scientific notation)
4. THE grammar SHALL highlight keywords (true, false, null)
5. THE grammar SHALL highlight comments (line and block)
6. THE grammar SHALL highlight punctuation (brackets, colons, commas)

### Requirement 11: Language Configuration

**User Story:** As a developer, I want proper editor behavior for `.dx` files, so that brackets auto-close and code folds correctly.

#### Acceptance Criteria

1. THE Extension SHALL configure auto-closing pairs for brackets and quotes
2. THE Extension SHALL configure bracket matching for `{}`, `[]`, `()`
3. THE Extension SHALL configure comment toggling for `//` and `/* */`
4. THE Extension SHALL configure folding based on bracket markers
5. THE Extension SHALL configure proper indentation rules

### Requirement 12: WASM Core Performance

**User Story:** As a developer, I want transformations to be fast, so that there's no noticeable delay when opening or saving files.

#### Acceptance Criteria

1. THE WASM_Core SHALL complete transformations in sub-millisecond time for typical files
2. IF the WASM_Core fails to load, THE Extension SHALL fall back to a TypeScript implementation
3. THE fallback implementation SHALL provide identical transformation behavior
4. THE WASM binary SHALL be optimized for size using release profile settings
