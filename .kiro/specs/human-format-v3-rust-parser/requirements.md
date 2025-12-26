# Requirements Document

## Introduction

This document specifies the requirements for adding Human Format V3 parsing support to the Rust serializer crate. Currently, the Rust `HumanParser` only supports the old Human format with `[config]` section headers, but the VS Code extension now uses Human Format V3 which has a different structure (no `[config]` header, key-value pairs at the top, TOML-like syntax). This causes the WASM `toDense` function to fail when converting Human V3 to LLM format.

## Glossary

- **Human_Format_V3**: The current human-readable format used by the VS Code extension, featuring TOML-like syntax without `[config]` headers
- **LLM_Format**: The token-optimized dense format stored on disk (e.g., `#c:nm|dx;v|0.0.1`)
- **Rust_Serializer**: The battle-hardened Rust crate that provides parsing and serialization via WASM
- **HumanParser**: The Rust struct responsible for parsing human-readable format to DxDocument
- **WASM_Module**: The WebAssembly module compiled from Rust, used by the VS Code extension

## Requirements

### Requirement 1: Detect Human Format V3

**User Story:** As a developer, I want the Rust parser to automatically detect Human Format V3, so that it can parse both old and new human formats.

#### Acceptance Criteria

1. WHEN the input starts with key-value pairs without a section header, THE HumanParser SHALL detect it as Human Format V3
2. WHEN the input starts with `[config]` or `[configuration]` header, THE HumanParser SHALL detect it as old Human format
3. WHEN the input starts with LLM sigil `#`, THE HumanParser SHALL return the input unchanged (already LLM format)

### Requirement 2: Parse Human V3 Config Section

**User Story:** As a developer, I want the Rust parser to parse Human V3 config values, so that top-level key-value pairs are correctly converted to context.

#### Acceptance Criteria

1. WHEN parsing Human V3 format, THE HumanParser SHALL treat all key-value pairs before the first section header as config/context values
2. WHEN a key uses full names (e.g., `name`, `version`), THE HumanParser SHALL compress them to abbreviations (e.g., `nm`, `v`)
3. WHEN a value contains spaces and is quoted, THE HumanParser SHALL remove the quotes and preserve the content
4. WHEN a value uses pipe separator (` | `), THE HumanParser SHALL parse it as an array

### Requirement 3: Parse Human V3 Section Headers

**User Story:** As a developer, I want the Rust parser to parse Human V3 section headers, so that sections like `[forge]`, `[stack]`, `[i18n.locales]` are correctly identified.

#### Acceptance Criteria

1. WHEN a line matches `[section_name]`, THE HumanParser SHALL start a new section
2. WHEN a section name is a full name (e.g., `forge`, `style`), THE HumanParser SHALL convert it to the abbreviated ID (e.g., `f`, `y`)
3. WHEN a section name contains a dot (e.g., `i18n.locales`), THE HumanParser SHALL parse it as a nested section
4. WHEN the section is `[stack]`, THE HumanParser SHALL parse its contents as reference definitions

### Requirement 4: Parse Human V3 Data Sections

**User Story:** As a developer, I want the Rust parser to parse Human V3 data sections, so that section key-value pairs are correctly converted to schema and rows.

#### Acceptance Criteria

1. WHEN parsing a data section, THE HumanParser SHALL collect key-value pairs into schema (keys) and row values
2. WHEN a key uses full names (e.g., `path`, `themes`), THE HumanParser SHALL compress them to abbreviations (e.g., `pt`, `th`)
3. WHEN a key contains hyphens (e.g., `dx-package-1`), THE HumanParser SHALL preserve it as-is (package names)
4. WHEN a value is `-` or `~`, THE HumanParser SHALL parse it as null

### Requirement 5: Parse Human V3 Nested Sections

**User Story:** As a developer, I want the Rust parser to parse nested sections like `[i18n.locales]` and `[js.dependencies]`, so that they are correctly converted to prefixed schema keys.

#### Acceptance Criteria

1. WHEN parsing `[parent.child]` section, THE HumanParser SHALL prefix all keys with `child_` (e.g., `locales_pt`)
2. WHEN the parent is `i18n`, THE HumanParser SHALL use section ID `i`
3. WHEN the parent is `js`, `python`, or `rust`, THE HumanParser SHALL use section IDs `j`, `p`, `r` respectively
4. WHEN multiple nested sections share the same parent, THE HumanParser SHALL merge them into a single section

### Requirement 6: Parse Human V3 Stack Section

**User Story:** As a developer, I want the Rust parser to parse the `[stack]` section, so that reference definitions are correctly stored.

#### Acceptance Criteria

1. WHEN parsing `[stack]` section, THE HumanParser SHALL store entries as reference definitions (refs)
2. WHEN a stack value uses pipe separator (` | `), THE HumanParser SHALL join values with `|` (no spaces)
3. WHEN the stack key is a reference name (e.g., `js`, `python`), THE HumanParser SHALL preserve it as-is

### Requirement 7: Round-Trip Consistency

**User Story:** As a developer, I want Human V3 to LLM conversion to be lossless, so that data is preserved when saving files.

#### Acceptance Criteria

1. FOR ALL valid Human V3 documents, parsing then serializing to LLM SHALL produce valid LLM format
2. FOR ALL valid Human V3 documents, the round-trip (Human V3 → LLM → Human V3) SHALL preserve all data values
3. FOR ALL valid LLM documents, the round-trip (LLM → Human V3 → LLM) SHALL produce equivalent LLM output

### Requirement 8: Abbreviation Dictionary Sync

**User Story:** As a developer, I want the Rust abbreviation dictionary to match the TypeScript version, so that key expansion/compression is consistent.

#### Acceptance Criteria

1. THE Rust AbbrevDict SHALL include all abbreviations from the TypeScript ABBREVIATIONS dictionary
2. THE Rust AbbrevDict SHALL map `vr` to `version` (not `variant`)
3. THE Rust AbbrevDict SHALL map `vt` to `variant`
4. THE Rust section name mapping SHALL match the TypeScript SECTION_NAMES dictionary

### Requirement 9: WASM Integration

**User Story:** As a developer, I want the WASM `toDense` function to use the new Human V3 parser, so that the VS Code extension can use WASM for all conversions.

#### Acceptance Criteria

1. WHEN `toDense` receives Human V3 input, THE WASM module SHALL parse it correctly and return valid LLM format
2. WHEN `toDense` receives old Human format input, THE WASM module SHALL still parse it correctly (backward compatibility)
3. WHEN `toDense` receives LLM format input, THE WASM module SHALL return it unchanged
