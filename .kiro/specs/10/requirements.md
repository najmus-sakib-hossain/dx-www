# Requirements Document

## Introduction

This document specifies the requirements for cleaning up the crates folder by moving www-related crates (`error` and `debug`) into the `crates/www/` folder where they belong.

## Glossary

- **Workspace**: The root Cargo workspace containing all crates
- **WWW**: Web framework related crates located in `crates/www/`
- **Error_Crate**: Binary error boundaries crate for component isolation
- **Debug_Crate**: DevTools bridge crate for debugging binary protocols

## Requirements

### Requirement 1: Move Error Crate to WWW Folder

**User Story:** As a developer, I want the error crate in the www folder, so that all www-related crates are grouped together.

#### Acceptance Criteria

1. THE Workspace SHALL move `crates/error/` to `crates/www/error/`
2. WHEN the error crate is moved, THE workspace Cargo.toml members list SHALL be updated from `"crates/error"` to `"crates/www/error"`
3. WHEN the error crate is moved, THE workspace dependency path SHALL be updated from `"crates/error"` to `"crates/www/error"`
4. WHEN the error crate is moved, THE `crates/www/client/Cargo.toml` path dependency SHALL be updated from `"../../error"` to `"../error"`
5. THE Workspace SHALL compile successfully after the move with `cargo check --workspace`

### Requirement 2: Move Debug Crate to WWW Folder

**User Story:** As a developer, I want the debug crate in the www folder, so that all www-related crates are grouped together.

#### Acceptance Criteria

1. THE Workspace SHALL move `crates/debug/` to `crates/www/debug/`
2. WHEN the debug crate is moved, THE workspace Cargo.toml members list SHALL be updated from `"crates/debug"` to `"crates/www/debug"`
3. WHEN the debug crate is moved, THE workspace dependency path SHALL be updated from `"crates/debug"` to `"crates/www/debug"`
4. WHEN the debug crate is moved, THE `crates/www/client/Cargo.toml` path dependency SHALL be updated from `"../../debug"` to `"../debug"`
5. THE Workspace SHALL compile successfully after the move with `cargo check --workspace`

### Requirement 3: Verify Workspace Integrity

**User Story:** As a developer, I want the workspace to build correctly after reorganization, so that development can continue without issues.

#### Acceptance Criteria

1. THE entire workspace SHALL pass `cargo check --workspace` after all moves
2. WHEN crates are moved, ALL path dependencies SHALL resolve correctly
3. THE workspace members list SHALL accurately reflect the new folder structure
