# Requirements Document

## Introduction

This document specifies the requirements for fixing critical bugs in the DX Serializer VSCode extension's human format parser (`humanParserV3.ts`). The parser is currently corrupting files during format-on-save operations due to incorrect handling of key-value lines, particularly in the stack section and nested sections.

## Glossary

- **Human_Parser**: The `parseHumanV3` function that converts human-readable DX format to internal DxDocument representation
- **Human_Formatter**: The `formatDocumentV3` function that converts DxDocument to human-readable format
- **LLM_Format**: The token-optimized format stored on disk (e.g., `#c:nm|dx;v|0.0.1`)
- **Human_Format**: The readable format shown to users (e.g., `name = dx`)
- **Stack_Section**: The `[stack]` section containing language/tool definitions with pipe-separated values
- **Round_Trip**: The process of parsing human format to DxDocument and formatting back to human format
- **Key_Value_Line**: A line in format `key = value` or `key = value1 | value2 | value3`

## Requirements

### Requirement 1: Fix Stack Section Parsing

**User Story:** As a developer, I want the stack section to be parsed correctly, so that language definitions like `js = javascript/typescript | bun | tsc | vite | bun | react` are preserved without corruption.

#### Acceptance Criteria

1. WHEN the Human_Parser encounters a line in the stack section with format `key = value1 | value2 | value3`, THE Human_Parser SHALL extract the key as the text before the first `=` sign and the value as the pipe-separated list after the `=` sign
2. WHEN the Human_Parser parses stack section values, THE Human_Parser SHALL NOT include the `=` sign or any prefix in the stored values
3. WHEN the Human_Parser parses a stack line like `js = javascript/typescript | bun | tsc | vite | bun | react`, THE Human_Parser SHALL store the key as `js` and the value as `javascript/typescript|bun|tsc|vite|bun|react` in the refs map
4. WHEN the Human_Parser encounters whitespace around the `=` sign, THE Human_Parser SHALL trim the whitespace from both key and value

### Requirement 2: Fix Key-Value Line Parsing

**User Story:** As a developer, I want all key-value lines to be parsed correctly, so that values are not corrupted with `"= value"` prefixes.

#### Acceptance Criteria

1. WHEN the Human_Parser parses a key-value line, THE Human_Parser SHALL correctly split on the first `=` sign only
2. WHEN the Human_Parser extracts the value portion, THE Human_Parser SHALL NOT include the `=` sign in the value
3. WHEN the Human_Parser parses a line like `path = @/style`, THE Human_Parser SHALL store key as `path` and value as `@/style`
4. IF the Human_Parser encounters a value that starts with `=`, THEN THE Human_Parser SHALL treat this as a parsing error and report it

### Requirement 3: Fix Nested Section Parsing

**User Story:** As a developer, I want nested sections like `[i18n.locales]` and `[js.dependencies]` to be parsed correctly, so that their key-value pairs are preserved.

#### Acceptance Criteria

1. WHEN the Human_Parser encounters a nested section header like `[i18n.locales]`, THE Human_Parser SHALL correctly identify the parent as `i18n` and child as `locales`
2. WHEN the Human_Parser parses key-value pairs within a nested section, THE Human_Parser SHALL store them with the correct prefix (e.g., `locales_path` for `path` in `[i18n.locales]`)
3. WHEN the Human_Parser reconstructs nested sections into DxSection format, THE Human_Parser SHALL preserve all key-value pairs without corruption

### Requirement 4: Round-Trip Consistency

**User Story:** As a developer, I want the human format to remain consistent after a round-trip (parse → format), so that format-on-save does not corrupt my files.

#### Acceptance Criteria

1. FOR ALL valid human format documents, parsing then formatting SHALL produce semantically equivalent output
2. WHEN a human format document is parsed and then formatted, THE output SHALL contain all original keys and values
3. WHEN a human format document is parsed and then formatted, THE output SHALL NOT contain corrupted values like `"= value"` or `"= @/path"`
4. WHEN the stack section is parsed and formatted, THE output SHALL preserve the original key-value structure with proper alignment

### Requirement 5: Error Handling for Malformed Input

**User Story:** As a developer, I want clear error messages when the parser encounters malformed input, so that I can fix issues in my DX files.

#### Acceptance Criteria

1. IF the Human_Parser encounters a line with multiple `=` signs in the value, THEN THE Human_Parser SHALL correctly parse only the first `=` as the separator
2. IF the Human_Parser encounters a line with no `=` sign (outside of section headers), THEN THE Human_Parser SHALL report a descriptive error with line number
3. IF the Human_Parser encounters an unclosed section header, THEN THE Human_Parser SHALL report a descriptive error with line number and hint

### Requirement 6: Re-enable Format-on-Save

**User Story:** As a developer, I want format-on-save to work correctly, so that my DX files are automatically formatted when I save them.

#### Acceptance Criteria

1. WHEN a user saves a DX file, THE system SHALL parse the human format, format it with proper alignment, and save the result
2. WHEN format-on-save is triggered, THE system SHALL NOT corrupt the file content
3. WHEN format-on-save is triggered, THE system SHALL preserve all data including stack section, nested sections, and dependencies
4. WHEN format-on-save encounters a parsing error, THE system SHALL preserve the original content and display an error message
