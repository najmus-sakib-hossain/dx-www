# Requirements Document

## Introduction

This feature enhances the DX Serializer Human Format to provide a cleaner, more readable configuration format with automatic generation of LLM and Machine format versions. The improvements include fixing the sigil parsing error, expanding abbreviated keys to full names, removing YAML-style indentation in favor of flat TOML-like structure, improving table formatting for wide data, and implementing automatic cache generation with proper path handling.

## Glossary

- **Human_Format**: The human-readable version of DX configuration files, designed for editing in text editors
- **LLM_Format**: Token-optimized format using sigils (#c:, #:, #<letter>()) for efficient LLM context usage
- **Machine_Format**: Binary format optimized for runtime performance (0.70ns access)
- **Sigil**: Special prefix characters in LLM format (#c: for context, #: for reference, #<letter>( for data sections)
- **Cache_Folder**: The .dx/cache directory where generated LLM and Machine format files are stored
- **Abbreviation_Dictionary**: Mapping between short keys (nm, au, ws) and full names (name, author, workspace)
- **Table_Section**: Data sections displayed as Unicode box-drawn tables in Human format
- **Column_Wrapping**: Splitting wide table rows into multiple lines for better readability

## Requirements

### Requirement 1: Fix Sigil Parsing Error

**User Story:** As a developer, I want the Human format parser to correctly handle comment lines starting with '# ', so that I don't get "Unknown sigil" errors when parsing valid Human format files.

#### Acceptance Criteria

1. WHEN the Human_Parser encounters a line starting with '# ' (hash followed by space), THE Human_Parser SHALL treat it as a comment and skip the line
2. WHEN the Human_Parser encounters a line starting with '#c:' (context sigil), THE Human_Parser SHALL parse it as a context section
3. WHEN the Human_Parser encounters a line starting with '#:' (reference sigil), THE Human_Parser SHALL parse it as a reference definition
4. WHEN the Human_Parser encounters a line starting with '#<letter>(' (data section sigil), THE Human_Parser SHALL parse it as a data section header
5. WHEN the Human_Parser encounters decorative comment lines like '# ═══', THE Human_Parser SHALL skip them without error

### Requirement 2: Expand Abbreviated Keys to Full Names

**User Story:** As a developer, I want configuration keys to use full descriptive names instead of abbreviations, so that the configuration is self-documenting and easier to understand.

#### Acceptance Criteria

1. WHEN formatting to Human format, THE Human_Formatter SHALL expand abbreviated keys to their full names (e.g., 'v' → 'version', 'au' → 'author', 'ws' → 'workspace')
2. WHEN parsing Human format, THE Human_Parser SHALL accept both abbreviated and full key names
3. WHEN converting Human format to LLM format, THE Converter SHALL compress full key names back to abbreviations
4. THE Abbreviation_Dictionary SHALL include mappings for: version, author, workspace, editors, and all other common configuration keys
5. WHEN an unknown key is encountered, THE Human_Formatter SHALL pass it through unchanged

### Requirement 3: Flat TOML-like Structure Without Indentation

**User Story:** As a developer, I want the Human format to use a flat structure like TOML instead of YAML-style indentation, so that the configuration is cleaner and easier to read.

#### Acceptance Criteria

1. WHEN formatting config sections, THE Human_Formatter SHALL output key-value pairs without leading indentation
2. WHEN formatting config sections, THE Human_Formatter SHALL align values using spaces after the key name
3. WHEN formatting array values, THE Human_Formatter SHALL output them as comma-separated lists without brackets (e.g., 'workspace = frontend/www, frontend/mobile')
4. WHEN formatting section headers, THE Human_Formatter SHALL use TOML-style brackets (e.g., '[config]', '[forge]')
5. WHEN formatting data tables, THE Human_Formatter SHALL not indent the table borders

### Requirement 4: Improved Section Headers for Data Tables

**User Story:** As a developer, I want data section headers to use the full section name instead of single-letter identifiers, so that the configuration is more readable.

#### Acceptance Criteria

1. WHEN formatting a data section header, THE Human_Formatter SHALL use the full section name in brackets (e.g., '[forge]' instead of '[f]')
2. WHEN formatting a data section, THE Human_Formatter SHALL place the table directly after the section header without indentation
3. WHEN parsing a data section, THE Human_Parser SHALL accept both full names and single-letter identifiers

### Requirement 5: Column Wrapping for Wide Tables

**User Story:** As a developer, I want wide tables to be split into multiple rows for better readability, so that I don't have to scroll horizontally to see all data.

#### Acceptance Criteria

1. WHEN a table row exceeds the configured maximum width, THE Human_Formatter SHALL split the row into multiple display lines
2. WHEN splitting a row, THE Human_Formatter SHALL maintain visual alignment of columns across wrapped lines
3. WHEN splitting a row, THE Human_Formatter SHALL use continuation indicators to show the row continues
4. THE Human_Formatter SHALL allow configuration of the maximum line width (default: 120 characters)
5. WHEN parsing wrapped tables, THE Human_Parser SHALL correctly reconstruct the original row data

### Requirement 6: Automatic Cache Generation

**User Story:** As a developer, I want changes to Human format files to automatically generate corresponding LLM and Machine format files in the cache folder, so that all format versions stay synchronized.

#### Acceptance Criteria

1. WHEN a Human format file is saved, THE Cache_Generator SHALL create the corresponding LLM format file in .dx/cache
2. WHEN a Human format file is saved, THE Cache_Generator SHALL create the corresponding Machine format file in .dx/cache
3. WHEN the source file is in a subfolder, THE Cache_Generator SHALL preserve the subfolder structure in .dx/cache
4. WHEN generating cache files, THE Cache_Generator SHALL use the same base filename with different extensions (.llm, .machine)
5. IF the cache generation fails, THEN THE Cache_Generator SHALL report the error without corrupting existing cache files

### Requirement 7: Path-Aware Cache Structure

**User Story:** As a developer, I want cache files to maintain the same folder structure as source files, so that I can easily find the corresponding cache files.

#### Acceptance Criteria

1. WHEN a source file is at 'config/app.dx', THE Cache_Generator SHALL create cache files at '.dx/cache/config/app.llm' and '.dx/cache/config/app.machine'
2. WHEN a source file is at the project root, THE Cache_Generator SHALL create cache files directly in '.dx/cache/'
3. WHEN creating cache directories, THE Cache_Generator SHALL create parent directories as needed
4. THE Cache_Generator SHALL normalize path separators for cross-platform compatibility

### Requirement 8: Table Structure Integrity

**User Story:** As a developer, I want the table structure to automatically adjust when I add or remove data, so that the formatting remains valid and readable.

#### Acceptance Criteria

1. WHEN a new row is added to a table, THE Human_Formatter SHALL expand column widths as needed to fit the new data
2. WHEN a row is removed from a table, THE Human_Formatter SHALL recalculate column widths based on remaining data
3. WHEN column content changes, THE Human_Formatter SHALL adjust the table borders to maintain alignment
4. THE Human_Formatter SHALL maintain valid Unicode box-drawing characters after any modification
5. WHEN parsing a modified table, THE Human_Parser SHALL correctly extract all row data regardless of column width changes

### Requirement 9: Round-Trip Consistency

**User Story:** As a developer, I want to be able to convert between Human, LLM, and Machine formats without losing data, so that I can use whichever format is most appropriate for my task.

#### Acceptance Criteria

1. FOR ALL valid Human format documents, converting to LLM format and back to Human format SHALL preserve all data values
2. FOR ALL valid LLM format documents, converting to Human format and back to LLM format SHALL preserve all data values
3. FOR ALL valid documents, converting to Machine format and back SHALL preserve all data values
4. WHEN round-tripping, THE Converter SHALL preserve comments and metadata where possible
5. WHEN round-tripping, THE Converter SHALL preserve the order of sections and keys

### Requirement 10: Human Format Pretty Printer

**User Story:** As a developer, I want a pretty printer that can format Human format documents back to valid Human format, so that round-trip parsing works correctly.

#### Acceptance Criteria

1. THE Pretty_Printer SHALL format DxDocument objects into valid Human format strings
2. THE Pretty_Printer SHALL produce output that can be parsed back by the Human_Parser
3. FOR ALL valid DxDocument objects, parsing the pretty-printed output SHALL produce an equivalent document (round-trip property)
