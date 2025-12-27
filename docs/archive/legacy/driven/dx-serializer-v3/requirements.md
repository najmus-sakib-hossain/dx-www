# Requirements Document

## Introduction

DX Serializer V3 enhances the VS Code extension to support automatic conversion from any serialization format (JSON, YAML, TOML, CSV) to DX format, with a new vertical key-value human format and automatic cache generation for human and machine versions.

## Glossary

- **DX_Serializer**: The VS Code extension that handles DX file transformations
- **LLM_Format**: The compact sigil-based format stored on disk (`#c:nm|value`)
- **Human_Format**: The readable vertical key-value format shown to users
- **Machine_Format**: A compact binary or optimized format for machine consumption
- **Cache_Directory**: The `.dx/cache` folder where human and machine versions are stored
- **Source_Format**: Input formats like JSON, YAML, TOML, CSV that can be converted to DX

## Requirements

### Requirement 1: Multi-Format Input Support

**User Story:** As a developer, I want to paste JSON, YAML, TOML, or CSV content into a DX file, so that it automatically converts to DX format.

#### Acceptance Criteria

1. WHEN a user pastes JSON content into a DX file, THE DX_Serializer SHALL detect and convert it to LLM_Format
2. WHEN a user pastes YAML content into a DX file, THE DX_Serializer SHALL detect and convert it to LLM_Format
3. WHEN a user pastes TOML content into a DX file, THE DX_Serializer SHALL detect and convert it to LLM_Format
4. WHEN a user pastes CSV content into a DX file, THE DX_Serializer SHALL detect and convert it to LLM_Format
5. WHEN conversion succeeds, THE DX_Serializer SHALL save the LLM_Format to the main file on disk
6. WHEN conversion succeeds, THE DX_Serializer SHALL generate Human_Format and save to Cache_Directory
7. WHEN conversion succeeds, THE DX_Serializer SHALL generate Machine_Format and save to Cache_Directory

### Requirement 2: Vertical Key-Value Human Format

**User Story:** As a developer, I want to see my DX configuration in a clean vertical key-value format, so that it's easy to read and edit.

#### Acceptance Criteria

1. THE Human_Format SHALL display config values as `key = value` pairs without section headers for config
2. THE Human_Format SHALL use `[section]` headers only for data sections (forge, stack, style, etc.)
3. THE Human_Format SHALL align all `=` signs to the longest key name plus padding
4. THE Human_Format SHALL use `|` as array separator instead of `,`
5. THE Human_Format SHALL display each data section row as vertical key-value pairs under the section header
6. THE Human_Format SHALL NOT use table formatting for data sections
7. THE Human_Format SHALL quote strings containing spaces with double quotes

### Requirement 3: Bidirectional Sync

**User Story:** As a developer, I want changes in the human format to automatically update the LLM format and regenerate cache files, so that all versions stay in sync.

#### Acceptance Criteria

1. WHEN a user edits the Human_Format in the editor, THE DX_Serializer SHALL parse the changes
2. WHEN parsing succeeds, THE DX_Serializer SHALL update the LLM_Format on disk
3. WHEN the LLM_Format is updated, THE DX_Serializer SHALL regenerate the Human_Format cache
4. WHEN the LLM_Format is updated, THE DX_Serializer SHALL regenerate the Machine_Format cache
5. IF parsing fails, THE DX_Serializer SHALL show validation errors without saving

### Requirement 4: Cache File Management

**User Story:** As a developer, I want cache files to be automatically managed in `.dx/cache`, so that I can access human and machine versions easily.

#### Acceptance Criteria

1. THE DX_Serializer SHALL create `.dx/cache` directory if it doesn't exist
2. THE DX_Serializer SHALL save Human_Format as `{filename}.human` in Cache_Directory
3. THE DX_Serializer SHALL save Machine_Format as `{filename}.machine` in Cache_Directory
4. WHEN the source file path contains subdirectories, THE DX_Serializer SHALL preserve the path structure in Cache_Directory
5. WHEN the source file is deleted, THE DX_Serializer SHALL remove corresponding cache files

### Requirement 5: Format Detection

**User Story:** As a developer, I want the extension to automatically detect the input format, so that I don't have to specify it manually.

#### Acceptance Criteria

1. WHEN content starts with `{` or `[`, THE DX_Serializer SHALL detect it as JSON
2. WHEN content contains YAML indicators (`:`, `-` at line start, `---`), THE DX_Serializer SHALL detect it as YAML
3. WHEN content contains `[section]` headers with `key = value` pairs, THE DX_Serializer SHALL detect it as TOML
4. WHEN content contains comma-separated values with consistent columns, THE DX_Serializer SHALL detect it as CSV
5. WHEN content starts with `#c:` or `#:` or `#<letter>(`, THE DX_Serializer SHALL detect it as LLM_Format
6. WHEN content matches Human_Format patterns, THE DX_Serializer SHALL detect it as Human_Format
