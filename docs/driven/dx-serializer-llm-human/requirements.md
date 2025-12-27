# Requirements Document

## Introduction

This feature enhances the DX Serializer crate to support three interconvertible formats: LLM (token-optimized), Human (beautiful TOML-like display), and Machine (binary). The LLM format achieves 3x+ efficiency over TOON through semantic density, single-character sigils, and reference compression. The Human format provides beautiful table rendering with Unicode box-drawing, key expansion, and auto-generated summaries. All three formats must convert to each other correctly.

## Glossary

- **DX_Serializer**: The Rust crate at `crates/serializer` that handles serialization/deserialization
- **LLM_Format**: Token-optimized format using sigils (`#c`, `#:`, `#<letter>`) for minimal token usage
- **Human_Format**: Beautiful TOML-like format with tables, indentation, and expanded key names
- **Machine_Format**: Binary format for runtime with 0.70ns access (already implemented)
- **Sigil**: Single-character prefix that denotes section type (e.g., `#c` for config, `#:` for references)
- **Reference**: A reusable value defined once and referenced via `^<key>` pointer
- **Abbreviation_Dictionary**: Mapping of short keys to full names (e.g., `nm` → `name`)
- **Context_Section**: The `#c` section containing metadata key-value pairs
- **Data_Section**: A `#<letter>` section containing schema-defined tabular data

## Requirements

### Requirement 1: LLM Format Parsing

**User Story:** As a developer, I want to parse LLM-optimized DX format, so that I can efficiently process token-dense data from LLM outputs.

#### Acceptance Criteria

1. WHEN the Parser receives input starting with `#c:`, THE Parser SHALL parse it as a context section with semicolon-separated key-value pairs
2. WHEN the Parser receives input starting with `#:`, THE Parser SHALL parse it as a reference definition with format `#:<key>|<value>`
3. WHEN the Parser receives input starting with `#<letter>(`, THE Parser SHALL parse it as a data section with schema in parentheses
4. WHEN the Parser encounters `^<key>` in a value, THE Parser SHALL resolve it as a reference pointer
5. WHEN the Parser encounters `+` as a value, THE Parser SHALL interpret it as boolean true
6. WHEN the Parser encounters `-` as a value, THE Parser SHALL interpret it as boolean false
7. WHEN the Parser encounters `~` as a value, THE Parser SHALL interpret it as null
8. WHEN the Parser encounters `*<items>` as a value, THE Parser SHALL interpret it as an inline array with comma-separated items

### Requirement 2: LLM Format Serialization

**User Story:** As a developer, I want to serialize data to LLM-optimized format, so that I can minimize token usage when sending data to LLMs.

#### Acceptance Criteria

1. WHEN the Serializer outputs a context section, THE Serializer SHALL format it as `#c:<key>|<val>;<key>|<val>`
2. WHEN the Serializer detects repeated strings longer than 5 characters appearing 2+ times, THE Serializer SHALL create references and use `^<key>` pointers
3. WHEN the Serializer outputs a data section, THE Serializer SHALL format schema as `#<id>(<col>|<col>|...)` followed by pipe-delimited rows
4. WHEN the Serializer outputs boolean true, THE Serializer SHALL use `+`
5. WHEN the Serializer outputs boolean false, THE Serializer SHALL use `-`
6. WHEN the Serializer outputs null, THE Serializer SHALL use `~`
7. WHEN the Serializer outputs an array, THE Serializer SHALL use `*<item>,<item>,...` format

### Requirement 3: Human Format Rendering

**User Story:** As a developer, I want to render DX data in a beautiful human-readable format, so that I can easily read and edit configuration files.

#### Acceptance Criteria

1. WHEN the Human_Formatter renders a section, THE Human_Formatter SHALL output a centered header with `═` box-drawing characters at 80 characters width
2. WHEN the Human_Formatter renders config key-value pairs, THE Human_Formatter SHALL use 4-space indentation and right-pad keys for alignment
3. WHEN the Human_Formatter renders a data section with rows, THE Human_Formatter SHALL output a Unicode box-drawn table with `┌─┬─┐`, `│`, `├─┼─┤`, `└─┴─┘` characters
4. WHEN the Human_Formatter renders boolean true in a table cell, THE Human_Formatter SHALL display `✓`
5. WHEN the Human_Formatter renders boolean false in a table cell, THE Human_Formatter SHALL display `✗`
6. WHEN the Human_Formatter renders null in a table cell, THE Human_Formatter SHALL display `—`
7. WHEN the Human_Formatter renders a reference, THE Human_Formatter SHALL resolve it inline and optionally show `# ref: <key>` comment
8. WHEN the Human_Formatter completes a table, THE Human_Formatter SHALL generate a summary footer with counts and sums of numeric columns

### Requirement 4: Human Format Parsing

**User Story:** As a developer, I want to parse human-readable DX format back to internal representation, so that I can edit files in human format and convert them.

#### Acceptance Criteria

1. WHEN the Human_Parser receives a `[section]` header, THE Human_Parser SHALL parse it as a section start
2. WHEN the Human_Parser receives indented `key = "value"` lines, THE Human_Parser SHALL parse them as key-value pairs
3. WHEN the Human_Parser receives a Unicode box-drawn table, THE Human_Parser SHALL extract column headers and row data
4. WHEN the Human_Parser encounters `✓` in a table cell, THE Human_Parser SHALL interpret it as boolean true
5. WHEN the Human_Parser encounters `✗` in a table cell, THE Human_Parser SHALL interpret it as boolean false
6. WHEN the Human_Parser encounters `—` in a table cell, THE Human_Parser SHALL interpret it as null

### Requirement 5: Key Abbreviation Dictionary

**User Story:** As a developer, I want a comprehensive key abbreviation dictionary, so that LLM and Human formats can convert keys correctly.

#### Acceptance Criteria

1. THE Abbreviation_Dictionary SHALL contain mappings for at least 50 common keys including: `nm`→`name`, `tt`→`title`, `ds`→`description`, `st`→`status`, `cr`→`created`, `up`→`updated`, `pr`→`price`, `qt`→`quantity`, `em`→`email`, `ur`→`url`
2. WHEN expanding a key, THE Abbreviation_Dictionary SHALL use context-aware expansion for ambiguous keys (e.g., `s` → `sunny` in hikes context, `s` → `status` in orders context)
3. WHEN compressing a key, THE Abbreviation_Dictionary SHALL use the shortest unambiguous abbreviation
4. THE Abbreviation_Dictionary SHALL be stored in a central location and loaded once at startup

### Requirement 6: Format Conversion - LLM to Human

**User Story:** As a developer, I want to convert LLM format to Human format, so that I can display token-efficient data beautifully.

#### Acceptance Criteria

1. WHEN converting LLM to Human, THE Converter SHALL expand all abbreviated keys using the Abbreviation_Dictionary
2. WHEN converting LLM to Human, THE Converter SHALL resolve all `^<key>` references to their full values
3. WHEN converting LLM to Human, THE Converter SHALL transform `+`/`-` to `✓`/`✗` in table cells
4. WHEN converting LLM to Human, THE Converter SHALL generate section headers with box-drawing characters
5. WHEN converting LLM to Human, THE Converter SHALL calculate column widths and align table content

### Requirement 7: Format Conversion - Human to LLM

**User Story:** As a developer, I want to convert Human format to LLM format, so that I can store files efficiently.

#### Acceptance Criteria

1. WHEN converting Human to LLM, THE Converter SHALL compress all keys using the Abbreviation_Dictionary
2. WHEN converting Human to LLM, THE Converter SHALL detect repeated strings and create references
3. WHEN converting Human to LLM, THE Converter SHALL transform `✓`/`✗`/`true`/`false` to `+`/`-`
4. WHEN converting Human to LLM, THE Converter SHALL strip table formatting and output pipe-delimited rows
5. WHEN converting Human to LLM, THE Converter SHALL remove section headers and summaries

### Requirement 8: Format Conversion - LLM/Human to Machine

**User Story:** As a developer, I want to convert LLM or Human format to Machine binary format, so that I can achieve fast runtime access.

#### Acceptance Criteria

1. WHEN converting to Machine format, THE Converter SHALL produce binary output compatible with the existing zero-copy reader
2. WHEN converting to Machine format, THE Converter SHALL preserve all data types and values exactly
3. WHEN converting from Machine format, THE Converter SHALL be able to produce both LLM and Human output

### Requirement 9: Round-Trip Correctness

**User Story:** As a developer, I want format conversions to be lossless, so that I can convert between formats without data loss.

#### Acceptance Criteria

1. FOR ALL valid LLM documents, converting to Human and back to LLM SHALL produce semantically equivalent output
2. FOR ALL valid Human documents, converting to LLM and back to Human SHALL produce semantically equivalent output
3. FOR ALL valid documents, converting through any sequence of format conversions SHALL preserve all data values

### Requirement 10: Playground Test Files

**User Story:** As a developer, I want test files in the playground folder, so that I can verify format conversions work correctly.

#### Acceptance Criteria

1. THE playground folder SHALL contain at least one `.dx` file in LLM format demonstrating all sigil types
2. THE playground folder SHALL contain conversion test scripts that convert LLM→Human→LLM and verify round-trip
3. THE playground folder SHALL contain conversion test scripts that convert Human→LLM→Human and verify round-trip
4. THE playground folder SHALL contain conversion test scripts that convert to Machine format and back
