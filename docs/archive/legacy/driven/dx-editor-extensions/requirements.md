# Requirements Document

## Introduction

This feature enhances the existing VSCode DX Serializer extension to automatically reformat table alignment when `.dx` files are saved. When users edit key-value pairs in the human-readable DX format, the table columns should automatically realign based on the new longest key length, ensuring consistent visual formatting without manual adjustment.

The primary use case is editing configuration files where keys may be added, removed, or renamed - the `=` signs should always align vertically within each section after save.

## Glossary

- **DX_Format**: The human-readable serialization format for `.dx` files with aligned key-value pairs
- **Table_Alignment**: The process of padding keys so all `=` signs align vertically within a section
- **Formatter**: The component that transforms DX content to properly aligned format
- **Parser**: The component that reads DX content and extracts structure (humanParserV3)
- **Section**: A block of content starting with `[section_name]` header or config values at file start
- **Key_Value_Pair**: A line in format `key = value` with padding spaces before `=`
- **Pipe_Separator**: The `|` character used to separate array elements with ` | ` spacing
- **Format_On_Save**: Automatic formatting triggered when user saves the file

## Requirements

### Requirement 1: Format on Save

**User Story:** As a developer editing `.dx` files, I want the table alignment to automatically reformat when I save, so that my configuration files always have properly aligned columns.

#### Acceptance Criteria

1. WHEN a user saves a `.dx` file, THE Formatter SHALL recalculate padding for all key-value pairs
2. WHEN a key is added or renamed to be longer than existing keys, THE Formatter SHALL increase padding for all keys in that section
3. WHEN a key is removed or shortened, THE Formatter SHALL decrease padding to match the new longest key
4. THE Formatter SHALL align all `=` signs within a section to the same column position
5. THE Formatter SHALL preserve the semantic content of the file (values unchanged)
6. WHEN formatting completes, THE Extension SHALL update the editor display with formatted content

### Requirement 2: Section-Based Alignment

**User Story:** As a developer, I want each section to have independent alignment, so that sections with short keys don't have excessive padding.

#### Acceptance Criteria

1. THE Formatter SHALL calculate padding independently for each section
2. THE Formatter SHALL treat config values (before any `[section]` header) as one section
3. THE Formatter SHALL treat each `[section]` block as a separate alignment group
4. THE Formatter SHALL handle nested sections like `[i18n.locales]` as separate alignment groups
5. WHEN a section has only one key, THE Formatter SHALL still apply minimum padding (default: 20 chars)

### Requirement 3: Stack Section Table Formatting

**User Story:** As a developer, I want the `[stack]` section to format as a proper table with aligned columns, so that technology stacks are easy to read.

#### Acceptance Criteria

1. WHEN formatting `[stack]` section, THE Formatter SHALL align pipe-separated columns
2. THE Formatter SHALL calculate column widths based on the longest value in each column
3. THE Formatter SHALL pad each cell to match its column width
4. THE Formatter SHALL preserve the ` | ` separator between columns
5. WHEN a stack row has fewer columns than others, THE Formatter SHALL NOT add extra padding

### Requirement 4: Preserve User Intent

**User Story:** As a developer, I want formatting to preserve my content exactly, so that I don't lose any data when saving.

#### Acceptance Criteria

1. THE Formatter SHALL preserve all string values exactly (including quoted strings)
2. THE Formatter SHALL preserve all numeric values exactly
3. THE Formatter SHALL preserve all array elements in their original order
4. THE Formatter SHALL preserve section order as written by the user
5. THE Formatter SHALL preserve comments (lines starting with `#` or `//`)
6. FOR ALL valid DX documents, parsing then formatting SHALL produce semantically equivalent output (round-trip property)

### Requirement 5: Error Handling

**User Story:** As a developer, I want clear feedback when formatting fails, so that I can fix syntax errors.

#### Acceptance Criteria

1. IF the file contains invalid DX syntax, THEN THE Formatter SHALL NOT modify the file
2. WHEN parsing fails, THE Extension SHALL display an error message with line number
3. WHEN parsing fails, THE Extension SHALL preserve the original file content
4. THE Extension SHALL update the status bar to indicate validation state

### Requirement 6: Configuration

**User Story:** As a developer, I want to configure formatting behavior, so that I can match my team's style preferences.

#### Acceptance Criteria

1. THE Configuration SHALL allow setting minimum key padding width (default: 20)
2. THE Configuration SHALL allow enabling/disabling format-on-save
3. WHEN configuration changes, THE Extension SHALL apply new settings immediately
4. THE Configuration SHALL be accessible via VSCode settings UI

### Requirement 7: Nested Section Formatting

**User Story:** As a developer, I want nested sections like `[js.dependencies]` to display with their full names, so that the file is readable and self-documenting.

#### Acceptance Criteria

1. THE Formatter SHALL output nested sections with full names like `[js.dependencies]` not abbreviated `[j]`
2. THE Formatter SHALL output keys without prefixes (e.g., `react` not `dependencies_react`)
3. THE Formatter SHALL preserve the parent.child naming convention for nested sections
4. WHEN parsing `[js.dependencies]`, THE Parser SHALL recognize `js` as parent and `dependencies` as subsection
5. THE Formatter SHALL align keys within nested sections independently

### Requirement 8: Section Name Expansion

**User Story:** As a developer, I want section names to be human-readable, so that I can understand the file structure at a glance.

#### Acceptance Criteria

1. THE Formatter SHALL expand abbreviated section names to full names (e.g., `d` → `driven`, `g` → `generator`)
2. THE Formatter SHALL output `[driven]` not `[data]` or `[d]`
3. THE Formatter SHALL output `[generator]` not `[g]`
4. THE Formatter SHALL preserve custom section names that are not in the abbreviation dictionary
5. THE Formatter SHALL handle language-specific sections: `[js.dependencies]`, `[python.dependencies]`, `[rust.dependencies]`
