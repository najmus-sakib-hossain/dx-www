# Requirements Document: Crates Cleanup Phase 3

## Introduction

This document specifies requirements for the third phase of crates folder professionalization. The focus is on consolidating scattered `.github/` documentation, removing build artifacts, and ensuring consistent gitignore patterns across all crates.

## Glossary

- **Crate**: A Rust package within the crates/ directory
- **Build_Artifact**: Generated files from compilation (target/, pkg/, logs/)
- **GitHub_Directory**: The .github/ folder typically used for GitHub-specific configuration
- **Proptest_Regressions**: Test failure records from property-based testing

## Requirements

### Requirement 1: Consolidate .github/ Documentation

**User Story:** As a developer, I want documentation files to be in standard locations, so that I can easily find project documentation.

#### Acceptance Criteria

1. WHEN documentation files exist in .github/ directories, THE System SHALL move them to the crate's docs/ subdirectory
2. WHEN .github/ contains only workflows/, THE System SHALL keep the directory for CI configuration
3. WHEN .github/ is empty or contains only non-workflow files, THE System SHALL remove the directory after moving docs
4. THE System SHALL preserve copilot-instructions.md files in .github/ (GitHub-specific)

### Requirement 2: Move Remaining Non-Standard Documentation

**User Story:** As a developer, I want all documentation in standard locations, so that the crate structure is consistent.

#### Acceptance Criteria

1. WHEN USAGE.md exists in crate root, THE System SHALL move it to docs/
2. WHEN non-standard .md files exist in crate root, THE System SHALL move them to docs/

### Requirement 3: Gitignore Build Artifacts

**User Story:** As a developer, I want build artifacts properly gitignored, so that the repository stays clean.

#### Acceptance Criteria

1. THE System SHALL ensure target/ directories are gitignored
2. THE System SHALL ensure pkg/ directories (WASM output) are gitignored
3. THE System SHALL ensure logs/ directories are gitignored
4. THE System SHALL ensure proptest-regressions/ directories are gitignored

### Requirement 4: Clean Empty Directories

**User Story:** As a developer, I want no empty configuration directories, so that the crate structure is clean.

#### Acceptance Criteria

1. WHEN .github/ directory is empty, THE System SHALL remove it
2. WHEN .vscode/ directory exists in crates (not root), THE System SHALL evaluate for removal
3. WHEN .idx/ directory exists, THE System SHALL evaluate for removal (Gitpod-specific)

### Requirement 5: Standardize Crate Structure

**User Story:** As a developer, I want consistent crate structure, so that navigation is predictable.

#### Acceptance Criteria

1. THE System SHALL ensure each crate has only standard files in root (README.md, Cargo.toml, CHANGELOG.md, LICENSE, CONTRIBUTING.md, build.rs)
2. THE System SHALL ensure all other documentation is in docs/ subdirectory
3. THE System SHALL ensure configuration files (.gitignore, .env.example) are minimal and necessary
