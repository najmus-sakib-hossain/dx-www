# Requirements Document

## Introduction

The DX Serializer VS Code Extension needs to be updated to properly handle the DX LLM format (with sigils like `#c`, `#:`, `#<letter>`) and convert between LLM format and Human format correctly. Currently, the extension uses the `hologram` module which handles a different format (`key#field:value`). This update will integrate the proper LLM/Human format conversion from the `llm` module, ensuring the extension only handles `.dx` files (not `.dx.json`, `.dx.yml`, etc.) and provides correct bidirectional conversion.

## Glossary

- **LLM_Format**: Token-optimized format using sigils (`#c` for context, `#:` for references, `#<letter>` for data sections), pipe delimiters (`|`), and abbreviated keys
- **Human_Format**: Beautiful TOML-like display with Unicode tables, expanded key names, section headers, and summaries
- **DX_File**: A file with exactly the `.dx` extension (not compound extensions like `.dx.json`)
- **Sigil**: Single-character prefix markers in LLM format (`#c`, `#:`, `#h`, etc.)
- **Reference**: A reusable value defined with `#:key|value` and referenced with `^key`
- **AbbrevDict**: Dictionary mapping abbreviated keys (like `nm`) to full names (like `name`)
- **DxDocument**: Internal representation that serves as the hub for format conversions
- **DxLens_FileSystem**: Virtual file system provider that intercepts `.dx` file operations

## Requirements

### Requirement 1: LLM Format Parsing

**User Story:** As a developer, I want the extension to correctly parse DX LLM format files, so that I can view them in human-readable format.

#### Acceptance Criteria

1. WHEN a DX_File containing LLM_Format is opened, THE Extension SHALL parse context sections starting with `#c:`
2. WHEN a DX_File containing LLM_Format is opened, THE Extension SHALL parse reference definitions starting with `#:`
3. WHEN a DX_File containing LLM_Format is opened, THE Extension SHALL parse data sections starting with `#<letter>(schema)`
4. WHEN parsing LLM_Format, THE Extension SHALL correctly interpret pipe (`|`) as field separator
5. WHEN parsing LLM_Format, THE Extension SHALL correctly interpret semicolon (`;`) as inline key-value separator
6. WHEN parsing LLM_Format, THE Extension SHALL correctly interpret `+` as boolean true and `-` as boolean false
7. WHEN parsing LLM_Format, THE Extension SHALL correctly interpret `~` as null value
8. WHEN parsing LLM_Format, THE Extension SHALL correctly interpret `^key` as reference pointer
9. WHEN parsing LLM_Format, THE Extension SHALL correctly interpret `*a,b,c` as inline array

### Requirement 2: Human Format Generation

**User Story:** As a developer, I want LLM format files displayed as beautiful human-readable format, so that I can easily read and understand the content.

#### Acceptance Criteria

1. WHEN displaying Human_Format, THE Extension SHALL expand abbreviated keys using AbbrevDict (e.g., `nm` → `name`, `ct` → `count`)
2. WHEN displaying Human_Format, THE Extension SHALL render section headers with box-drawing characters
3. WHEN displaying Human_Format, THE Extension SHALL render data sections as Unicode tables with proper alignment
4. WHEN displaying Human_Format, THE Extension SHALL display booleans as `✓` (true) and `✗` (false) in tables
5. WHEN displaying Human_Format, THE Extension SHALL display null values as `—` in tables
6. WHEN displaying Human_Format, THE Extension SHALL resolve references inline (e.g., `^B` → actual value)
7. WHEN displaying Human_Format, THE Extension SHALL generate summary footers for data sections

### Requirement 3: Human to LLM Conversion

**User Story:** As a developer, I want my edits in human format saved back to LLM format, so that the file remains token-efficient on disk.

#### Acceptance Criteria

1. WHEN saving a DX_File, THE Extension SHALL convert Human_Format back to LLM_Format
2. WHEN converting to LLM_Format, THE Extension SHALL compress expanded keys back to abbreviations
3. WHEN converting to LLM_Format, THE Extension SHALL convert `true`/`✓` to `+` and `false`/`✗` to `-`
4. WHEN converting to LLM_Format, THE Extension SHALL convert `null`/`—` to `~`
5. WHEN converting to LLM_Format, THE Extension SHALL detect repeated strings and create references
6. FOR ALL valid DxDocument objects, converting to LLM_Format then parsing back SHALL produce equivalent documents (round-trip property)

### Requirement 4: File Type Filtering

**User Story:** As a developer, I want only pure `.dx` files to be handled by the extension, so that compound extensions like `.dx.json` work normally.

#### Acceptance Criteria

1. THE Extension SHALL only intercept files ending exactly with `.dx`
2. WHEN a file has extension `.dx.json`, THE Extension SHALL NOT intercept it
3. WHEN a file has extension `.dx.yml`, THE Extension SHALL NOT intercept it
4. WHEN a file has extension `.dx.yaml`, THE Extension SHALL NOT intercept it
5. WHEN a file has extension `.dx.toml`, THE Extension SHALL NOT intercept it
6. WHEN a file has extension `.dx.bak`, THE Extension SHALL NOT intercept it
7. WHEN a file has any other compound extension after `.dx`, THE Extension SHALL NOT intercept it

### Requirement 5: WASM Core Integration

**User Story:** As a developer, I want the extension to use the Rust WASM core for fast conversions, so that transformations are sub-millisecond.

#### Acceptance Criteria

1. THE Extension SHALL use the `llm_to_human` function from the serializer crate for LLM→Human conversion
2. THE Extension SHALL use the `human_to_llm` function from the serializer crate for Human→LLM conversion
3. IF the WASM core fails to load, THE Extension SHALL fall back to a TypeScript implementation
4. THE TypeScript fallback SHALL provide identical conversion behavior to the WASM core
5. THE Extension SHALL complete transformations in sub-millisecond time for typical files

### Requirement 6: Popular Key Abbreviations

**User Story:** As a developer, I want consistent key abbreviation mappings, so that conversions are predictable.

#### Acceptance Criteria

1. THE AbbrevDict SHALL map `nm` to `name`
2. THE AbbrevDict SHALL map `tt` to `title`
3. THE AbbrevDict SHALL map `ds` to `description`
4. THE AbbrevDict SHALL map `st` to `status`
5. THE AbbrevDict SHALL map `ct` to `count`
6. THE AbbrevDict SHALL map `pr` to `price`
7. THE AbbrevDict SHALL map `em` to `email`
8. THE AbbrevDict SHALL map `ur` to `url`
9. THE AbbrevDict SHALL support context-aware expansion (e.g., `s` → `sunny` in hikes context, `s` → `status` in orders context)

### Requirement 7: Error Handling

**User Story:** As a developer, I want clear error messages when parsing fails, so that I can fix issues in my files.

#### Acceptance Criteria

1. WHEN parsing fails due to invalid sigil, THE Extension SHALL report the line number and invalid sigil
2. WHEN parsing fails due to malformed reference, THE Extension SHALL report the reference key
3. WHEN parsing fails due to schema mismatch, THE Extension SHALL report expected vs actual column count
4. WHEN a referenced key is not defined, THE Extension SHALL display the unresolved reference as `^key`

### Requirement 8: Validation

**User Story:** As a developer, I want real-time validation of my DX files, so that I can catch errors before saving.

#### Acceptance Criteria

1. WHEN content is edited, THE Extension SHALL validate syntax immediately
2. WHEN validation fails, THE Extension SHALL show diagnostics with line and column information
3. WHEN validation fails, THE Extension SHALL provide actionable hints for fixing errors
4. WHEN content is valid, THE Extension SHALL clear all diagnostics
