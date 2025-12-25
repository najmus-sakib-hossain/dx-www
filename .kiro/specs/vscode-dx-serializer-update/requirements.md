# Requirements Document

## Introduction

This document specifies requirements for updating the VS Code DX Serializer extension to align with the battle-hardened serializer Rust crate. The extension currently uses a TypeScript implementation for parsing and formatting. This update will ensure the extension benefits from the security improvements and validation limits implemented in the Rust crate, while preserving the complex auto-save and document management functionality.

**CRITICAL CONSTRAINT**: The extension's auto-save, document management, and file system provider logic is complex and fragile. This update MUST NOT modify any of those components. Only the parsing, validation, and formatting logic should be updated.

## Glossary

- **LLM_Format**: The token-efficient sigil-based format stored on disk (#c:, #:, #<letter>)
- **Human_Format_V3**: The human-readable TOML-like format displayed in the editor
- **Machine_Format**: The binary format stored in cache files
- **DxDocument**: The internal representation of a parsed DX file
- **Parser**: Components that read text/binary and produce DxDocument
- **Formatter**: Components that convert DxDocument to text output
- **Validator**: Components that check syntax and semantic correctness

## Requirements

### Requirement 1: Input Size Validation

**User Story:** As a developer, I want the extension to reject excessively large files, so that the editor remains responsive and memory usage is bounded.

#### Acceptance Criteria

1. WHEN the Parser receives input exceeding 100MB, THE Parser SHALL return a clear error indicating size limit exceeded
2. WHEN a file exceeds the size limit, THE extension SHALL display an error message in the status bar
3. THE size limit constant SHALL be defined as MAX_INPUT_SIZE = 104,857,600 bytes (100 MB)

### Requirement 2: Recursion Depth Protection

**User Story:** As a developer, I want the extension to handle deeply nested structures safely, so that malformed files cannot crash the editor.

#### Acceptance Criteria

1. WHEN the Parser encounters structures nested deeper than 1000 levels, THE Parser SHALL return a recursion limit error
2. THE recursion limit constant SHALL be defined as MAX_RECURSION_DEPTH = 1000
3. WHEN recursion limit is exceeded, THE error message SHALL include the current depth and maximum allowed

### Requirement 3: Table Row Limits

**User Story:** As a developer, I want the extension to limit table sizes, so that extremely large tables cannot exhaust memory.

#### Acceptance Criteria

1. WHEN the Parser encounters a table with more than 10 million rows, THE Parser SHALL return a table size error
2. THE table row limit constant SHALL be defined as MAX_TABLE_ROWS = 10,000,000
3. WHEN table limit is exceeded, THE error message SHALL include the row count and maximum allowed

### Requirement 4: Enhanced Error Types

**User Story:** As a developer, I want clear error messages with specific error types, so that I can quickly diagnose and fix issues.

#### Acceptance Criteria

1. THE Parser SHALL define an InputTooLarge error type with size and max fields
2. THE Parser SHALL define a RecursionLimitExceeded error type with depth and max fields
3. THE Parser SHALL define a TableTooLarge error type with rows and max fields
4. WHEN validation fails, THE error message SHALL include actionable hints

### Requirement 5: Validation Consistency

**User Story:** As a developer, I want the TypeScript validation to match the Rust crate behavior, so that files valid in the extension are also valid when processed by Rust tools.

#### Acceptance Criteria

1. THE TypeScript Parser SHALL use the same validation limits as the Rust crate
2. THE TypeScript Parser SHALL produce equivalent error messages for the same invalid inputs
3. FOR ALL inputs that the Rust crate rejects, THE TypeScript Parser SHALL also reject them

### Requirement 6: README Documentation Update

**User Story:** As a developer, I want the extension README to document the security limits, so that I understand the constraints.

#### Acceptance Criteria

1. THE README SHALL document the MAX_INPUT_SIZE limit (100 MB)
2. THE README SHALL document the MAX_RECURSION_DEPTH limit (1000 levels)
3. THE README SHALL document the MAX_TABLE_ROWS limit (10 million rows)
4. THE README SHALL reference the battle-hardened serializer crate

### Requirement 7: Preserve Existing Functionality

**User Story:** As a developer, I want all existing extension features to continue working, so that the update doesn't break my workflow.

#### Acceptance Criteria

1. THE extension SHALL continue to support auto-save with grace period
2. THE extension SHALL continue to support the dxlens:// virtual file system
3. THE extension SHALL continue to support format detection (JSON, YAML, TOML, CSV, LLM)
4. THE extension SHALL continue to support section order preservation
5. THE extension SHALL continue to generate cache files (.human, .machine)
6. THE extension SHALL NOT modify dxDocumentManager.ts
7. THE extension SHALL NOT modify dxLensFileSystem.ts
8. THE extension SHALL NOT modify extension.ts activation logic
