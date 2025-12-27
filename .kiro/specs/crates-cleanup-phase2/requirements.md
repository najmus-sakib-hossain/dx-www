# Requirements Document

## Introduction

This specification defines the requirements for the second phase of crates folder professionalization. The first phase established basic structure and documentation. This phase focuses on removing development artifacts, consolidating scattered configuration, and ensuring consistent professional presentation across all crates.

## Glossary

- **Development_Artifact**: Files created during development that should not be in production (STATUS.md, PLAN.md, COMPLETION_REPORT.md, etc.)
- **Scattered_Config**: Configuration directories (.kiro/, .dx/, .github/) that exist in individual crates instead of the repository root
- **Progress_File**: Files tracking development progress (PHASE*.md, TASKLIST.md, ROADMAP.md, etc.)
- **Cargo.lock**: Lock file that should only exist in binary crates, not library crates

## Requirements

### Requirement 1: Remove Development Artifacts from Crates

**User Story:** As a user browsing the repository, I want to see only production-relevant files, so that the codebase appears professional and is easy to navigate.

#### Acceptance Criteria

1. THE Cleanup_System SHALL remove or relocate STATUS.md files from crate directories
2. THE Cleanup_System SHALL remove or relocate PLAN.md files from crate directories
3. THE Cleanup_System SHALL remove or relocate COMPLETION_REPORT.md files from crate directories
4. THE Cleanup_System SHALL remove or relocate ROADMAP.md files from crate directories
5. THE Cleanup_System SHALL remove or relocate REMOVE_SUBMODULES_PLAN.md files from crate directories
6. IF development notes contain valuable information, THEN THE Cleanup_System SHALL move them to docs/archive/

### Requirement 2: Remove Cargo.lock from Library Crates

**User Story:** As a Rust developer, I want library crates to not include Cargo.lock files, so that downstream users get proper dependency resolution.

#### Acceptance Criteria

1. THE Cleanup_System SHALL identify all library crates (crates without [[bin]] targets)
2. THE Cleanup_System SHALL remove Cargo.lock from library crates
3. THE Cleanup_System SHALL preserve Cargo.lock in binary crates (dx, cli, check)
4. THE Cleanup_System SHALL update .gitignore to prevent future Cargo.lock commits in library crates

### Requirement 3: Consolidate Scattered Configuration Directories

**User Story:** As a contributor, I want configuration directories consolidated at the repository root, so that I can find all configuration in one place.

#### Acceptance Criteria

1. THE Cleanup_System SHALL identify .kiro/ directories in individual crates
2. THE Cleanup_System SHALL identify .dx/ directories in individual crates
3. THE Cleanup_System SHALL identify .github/ directories in individual crates
4. THE Cleanup_System SHALL remove or consolidate these directories to the repository root
5. IF a crate requires local configuration, THEN THE Cleanup_System SHALL document the reason in the crate's README

### Requirement 4: Clean Up Nested Workspace Artifacts

**User Story:** As a maintainer, I want nested workspaces to be clean and professional, so that the entire repository maintains consistent quality.

#### Acceptance Criteria

1. THE Cleanup_System SHALL audit crates/check/ for development artifacts
2. THE Cleanup_System SHALL audit crates/python/ for development artifacts
3. THE Cleanup_System SHALL audit crates/javascript/ subdirectories for development artifacts
4. THE Cleanup_System SHALL remove PLANNING.md files from nested crates

### Requirement 5: Standardize Documentation Files

**User Story:** As a developer, I want consistent documentation across all crates, so that I can easily find information about each crate.

#### Acceptance Criteria

1. THE Documentation_System SHALL ensure each crate has only standard documentation files (README.md, CHANGELOG.md, CONTRIBUTING.md, LICENSE)
2. THE Documentation_System SHALL remove or relocate non-standard documentation files (DX_CHECK.md, DX_CONFIG_SPEC.md, etc.)
3. WHEN a crate has extensive documentation, THE Documentation_System SHALL consolidate it into a docs/ subdirectory within the crate

