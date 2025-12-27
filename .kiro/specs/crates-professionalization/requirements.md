# Requirements Document

## Introduction

This specification defines the requirements for professionalizing the `crates/` folder structure in the DX monorepo. The goal is to establish consistent documentation, naming conventions, file structure, and quality standards across all crates to meet professional open-source project standards.

## Glossary

- **Crate**: A Rust package/library within the monorepo
- **README**: The primary documentation file for a crate
- **Cargo.toml**: The Rust package manifest file
- **Monorepo**: A single repository containing multiple related projects
- **Subcrate**: A crate nested within another crate's directory (e.g., `www/core`)

## Requirements

### Requirement 1: Consistent README Documentation

**User Story:** As a developer, I want every crate to have a professional README, so that I can quickly understand what each crate does and how to use it.

#### Acceptance Criteria

1. THE Documentation_System SHALL ensure every crate has a README.md file
2. WHEN a README.md is created, THE Documentation_System SHALL include: project name, description, installation instructions, usage examples, and license information
3. THE Documentation_System SHALL use consistent badge formatting across all crates
4. THE Documentation_System SHALL remove task instructions, raw prompts, or development notes from README files
5. WHEN a crate has subcrates, THE Documentation_System SHALL provide a summary table linking to subcrate documentation

### Requirement 2: Standardized File Structure

**User Story:** As a contributor, I want a consistent file structure across all crates, so that I can navigate the codebase predictably.

#### Acceptance Criteria

1. THE File_Structure SHALL include these standard files in each crate: README.md, Cargo.toml, src/, LICENSE (or reference to root)
2. THE File_Structure SHALL NOT include Cargo.lock files in library crates (only in binary/application crates)
3. THE File_Structure SHALL consolidate scattered configuration folders (.kiro/, .dx/, .github/) to the repository root where possible
4. IF a crate requires local configuration, THEN THE File_Structure SHALL document the reason in the crate's README
5. THE File_Structure SHALL fix malformed directory names (e.g., `cratesdx-py-runtimedx-py-corebenches`)

### Requirement 3: Consistent Naming Conventions

**User Story:** As a developer, I want consistent naming across all crates, so that I can easily identify and reference them.

#### Acceptance Criteria

1. THE Naming_System SHALL use kebab-case for all crate directory names
2. THE Naming_System SHALL use a consistent prefix pattern (dx-*) for all public crates
3. THE Naming_System SHALL ensure Cargo.toml package names match directory names
4. WHEN a crate is internal-only, THE Naming_System SHALL document this in the crate's README
5. THE Naming_System SHALL fix typos in filenames (e.g., COMPABILITY.md → COMPATIBILITY.md)

### Requirement 4: License and Legal Compliance

**User Story:** As a user of the library, I want clear licensing information, so that I know how I can use the code.

#### Acceptance Criteria

1. THE License_System SHALL ensure every crate either has a LICENSE file or references the root LICENSE
2. THE License_System SHALL include license information in every Cargo.toml file
3. WHEN a crate uses a different license than the root, THE License_System SHALL clearly document this
4. THE License_System SHALL remove committed .env files containing sensitive data

### Requirement 5: Development Artifact Cleanup

**User Story:** As a maintainer, I want the codebase free of development artifacts, so that the repository appears professional and is easy to navigate.

#### Acceptance Criteria

1. THE Cleanup_System SHALL remove or relocate progress tracking files (PHASE3_COMPLETE.md, TASKLIST.md, etc.) to a docs/archive folder
2. THE Cleanup_System SHALL ensure .gitignore properly excludes development artifacts
3. THE Cleanup_System SHALL remove empty documentation files
4. IF development notes are valuable, THEN THE Cleanup_System SHALL move them to a docs/development folder

### Requirement 6: Subcrate Organization (www/)

**User Story:** As a developer, I want the www/ subcrates organized logically, so that I can find related functionality easily.

#### Acceptance Criteria

1. THE Organization_System SHALL group related www/ subcrates into logical categories
2. THE Organization_System SHALL create a www/README.md documenting all subcrates and their purposes
3. WHEN subcrates share common functionality, THE Organization_System SHALL document dependencies clearly
4. THE Organization_System SHALL ensure each www/ subcrate has at minimum a README.md and Cargo.toml

### Requirement 7: Cargo.toml Standardization

**User Story:** As a contributor, I want consistent Cargo.toml files, so that package metadata is complete and uniform.

#### Acceptance Criteria

1. THE Cargo_System SHALL ensure every Cargo.toml includes: name, version, edition, authors, description, license
2. THE Cargo_System SHALL use consistent version formatting across all crates
3. THE Cargo_System SHALL include repository and documentation links where applicable
4. WHEN a crate is not ready for publication, THE Cargo_System SHALL mark it with `publish = false`
