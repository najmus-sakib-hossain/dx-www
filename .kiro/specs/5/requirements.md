# Requirements Document

## Introduction

This feature adds Human Format V3 parsing support to the Rust serializer crate, enabling the WASM `toDense` function to correctly convert Human V3 format back to LLM format. Currently, the Rust `HumanParser` only supports the old Human format with `[config]` section headers, but Human V3 uses a different structure without config headers.

## Glossary

- **Human_V3_Parser**: The Rust parser component that parses Human Format V3 into DxDocument
- **LLM_Format**: The token-optimized dense format stored on disk (e.g., `#c:nm|dx;v|0.0.1`)
- **Human_V3_Format**: The human-readable TOML-like format without `[config]` headers
- **DxDocument**: The internal document representation with context, refs, and sections
- **WASM_Serializer**: The WebAssembly module exposing `toDense` and `toHuman` functions
- **Abbreviation_Dictionary**: Mapping between full key names and abbreviated keys (e.g., `name` ↔ `nm`)

## Requirements

### Requirement 1: Parse Config Section Without Header

**User Story:** As a developer, I want the Rust parser to recognize key-value pairs at the start of a file as config values, so that Human V3 format without `[config]` headers is correctly parsed.

#### Acceptance Criteria

1. WHEN the parser encounters key-value pairs before any section header, THE Human_V3_Parser SHALL treat them as config context values
2. WHEN a key uses a full name (e.g., `name`, `version`), THE Human_V3_Parser SHALL compress it to the abbreviated form (e.g., `nm`, `v`)
3. WHEN the config section contains arrays with pipe separators (e.g., `workspace = @/www | @/backend`), THE Human_V3_Parser SHALL parse them as array values

### Requirement 2: Parse Stack Section for References

**User Story:** As a developer, I want the Rust parser to recognize `[stack]` sections as reference definitions, so that language stack references are correctly preserved.

#### Acceptance Criteria

1. WHEN the parser encounters a `[stack]` section header, THE Human_V3_Parser SHALL parse subsequent key-value pairs as reference definitions
2. WHEN a stack entry has pipe-separated values (e.g., `js = javascript/typescript | bun | tsc`), THE Human_V3_Parser SHALL join them with `|` for the refs map
3. THE Human_V3_Parser SHALL preserve the reference key names without abbreviation (e.g., `js`, `python`, `rust`)

### Requirement 3: Parse Data Sections with Full Names

**User Story:** As a developer, I want the Rust parser to recognize section headers with full names, so that `[forge]`, `[style]`, `[media]` etc. are correctly mapped to section IDs.

#### Acceptance Criteria

1. WHEN the parser encounters a section header with a full name (e.g., `[forge]`), THE Human_V3_Parser SHALL map it to the correct section ID (e.g., `f`)
2. WHEN a section contains key-value pairs, THE Human_V3_Parser SHALL compress keys to abbreviated forms for the schema
3. WHEN a section has a single row of data, THE Human_V3_Parser SHALL create a section with one row containing all values

### Requirement 4: Parse Nested Sections

**User Story:** As a developer, I want the Rust parser to recognize nested section headers like `[i18n.locales]` and `[js.dependencies]`, so that nested data is correctly structured.

#### Acceptance Criteria

1. WHEN the parser encounters a nested section header (e.g., `[i18n.locales]`), THE Human_V3_Parser SHALL identify the parent section ID
2. WHEN parsing nested section keys, THE Human_V3_Parser SHALL prefix them with the subsection name (e.g., `locales_path`, `locales_default`)
3. WHEN multiple nested sections share a parent (e.g., `[i18n.locales]` and `[i18n.ttses]`), THE Human_V3_Parser SHALL merge them into a single section

### Requirement 5: Handle Quoted Strings

**User Story:** As a developer, I want the Rust parser to correctly handle quoted strings, so that values with spaces are preserved.

#### Acceptance Criteria

1. WHEN a value is wrapped in double quotes (e.g., `"Enhanced Developing Experience"`), THE Human_V3_Parser SHALL remove the quotes and preserve the content
2. WHEN a value is wrapped in single quotes, THE Human_V3_Parser SHALL remove the quotes and preserve the content
3. WHEN a value contains no quotes and no spaces, THE Human_V3_Parser SHALL use it as-is

### Requirement 6: Handle Special Value Types

**User Story:** As a developer, I want the Rust parser to correctly identify value types, so that booleans, numbers, nulls, and arrays are properly typed.

#### Acceptance Criteria

1. WHEN a value is `-` or `~`, THE Human_V3_Parser SHALL parse it as a null value
2. WHEN a value is `true` or `false`, THE Human_V3_Parser SHALL parse it as a boolean value
3. WHEN a value matches a numeric pattern, THE Human_V3_Parser SHALL parse it as a number value
4. WHEN a value contains ` | ` separators, THE Human_V3_Parser SHALL parse it as an array value

### Requirement 7: Round-Trip Consistency

**User Story:** As a developer, I want Human V3 to LLM conversion to preserve all data, so that editing in Human V3 format doesn't lose information.

#### Acceptance Criteria

1. FOR ALL valid Human V3 documents, parsing then serializing to LLM format SHALL produce equivalent data to the original
2. FOR ALL valid LLM documents, converting to Human V3 then back to LLM SHALL produce equivalent data
3. THE Human_V3_Parser SHALL preserve section order when converting back to LLM format

### Requirement 8: WASM Integration

**User Story:** As a developer, I want the WASM `toDense` function to use the Human V3 parser, so that the VS Code extension uses the battle-hardened Rust implementation.

#### Acceptance Criteria

1. WHEN `toDense` receives Human V3 format input, THE WASM_Serializer SHALL use the Human_V3_Parser
2. WHEN `toDense` receives LLM format input (starting with `#`), THE WASM_Serializer SHALL return it unchanged
3. IF parsing fails, THEN THE WASM_Serializer SHALL return an error with line number and hint
