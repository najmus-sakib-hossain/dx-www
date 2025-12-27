# Requirements Document

## Introduction

This specification addresses the critical issues identified in the DX project to transform it from a prototype with scattered implementation into a production-ready, professional software project. The goal is to consolidate, fix, and polish the codebase to enable a successful launch.

## Glossary

- **DX_Project**: The complete DX monorepo including all crates, extensions, and tooling
- **Crate**: A Rust package/library within the DX workspace
- **Workspace**: The Cargo workspace containing all DX crates
- **Integration**: External projects that were embedded into the repository
- **Daemon**: The Forge background process that monitors files and coordinates tools
- **Extension**: The VS Code extension for DX tooling

## Requirements

### Requirement 1: Workspace Consolidation

**User Story:** As a developer, I want a unified workspace structure, so that I can build and test all components with a single command.

#### Acceptance Criteria

1. THE DX_Project SHALL have exactly ONE Cargo.lock file at the root level
2. THE DX_Project SHALL use a single Rust edition (2024) across all crates
3. WHEN running `cargo build --workspace`, THE Build_System SHALL compile all crates without errors
4. WHEN running `cargo test --workspace`, THE Test_System SHALL execute all tests without exclusions
5. THE DX_Project SHALL remove all nested workspace configurations from excluded crates
6. THE DX_Project SHALL consolidate the 12 separate Cargo.lock files into the root lock file

### Requirement 2: External Integration Removal

**User Story:** As a maintainer, I want external dependencies managed through Cargo, so that I can receive upstream updates and maintain license compliance.

#### Acceptance Criteria

1. THE DX_Project SHALL remove all embedded external projects from the integrations folder
2. THE DX_Project SHALL replace embedded code with proper crates.io dependencies where available
3. THE DX_Project SHALL document any forked dependencies with proper attribution
4. IF external code must be vendored, THEN THE DX_Project SHALL maintain git history and upstream tracking
5. THE DX_Project SHALL remove the following embedded projects: actix-web, axum, cargo, nextjs, rkyv, svelte, toon, uv, vscode, vscode-gitlens

### Requirement 3: Code Quality Standards

**User Story:** As a developer, I want clean, safe code, so that the project is maintainable and reliable.

#### Acceptance Criteria

1. THE DX_Project SHALL have zero `unimplemented!()` macros in production code paths
2. THE DX_Project SHALL have zero `todo!()` macros in production code paths
3. THE DX_Project SHALL replace all `println!()` statements with proper logging using the `tracing` crate
4. THE DX_Project SHALL remove all `thread::sleep()` placeholder implementations
5. THE DX_Project SHALL replace string error returns with proper error types using `thiserror`
6. THE DX_Project SHALL remove all `#[allow(dead_code)]` annotations by either using or deleting the code
7. THE DX_Project SHALL minimize unsafe code and document all remaining unsafe blocks with safety comments
8. THE DX_Project SHALL pass `cargo clippy --workspace -- -D warnings` without errors

### Requirement 4: Test Coverage

**User Story:** As a developer, I want comprehensive test coverage, so that I can confidently make changes without breaking functionality.

#### Acceptance Criteria

1. THE DX_Project SHALL fix all 10 failing tests identified in the serializer
2. THE DX_Project SHALL fix all 2 hanging tests (infinite loops)
3. THE DX_Project SHALL fix all 12+ broken import paths
4. WHEN a crate has production code, THE Crate SHALL have corresponding test coverage
5. THE DX_Project SHALL have tests for all 22 dx-www crates currently without tests
6. THE DX_Project SHALL remove or properly configure proptest-regressions directories

### Requirement 5: Crate Naming Consistency

**User Story:** As a developer, I want consistent naming conventions, so that I can easily navigate and understand the codebase.

#### Acceptance Criteria

1. THE DX_Project SHALL use the `dx-` prefix for all crates
2. THE DX_Project SHALL rename `serializer` to `dx-serializer`
3. THE DX_Project SHALL rename `style` to `dx-style`
4. THE DX_Project SHALL rename `forge` to `dx-forge`
5. THE DX_Project SHALL rename `workspace` to `dx-workspace`
6. THE DX_Project SHALL rename `stack` to `dx-stack`
7. THE DX_Project SHALL rename `generator` to `dx-generator`
8. THE DX_Project SHALL rename `font` to `dx-font`
9. THE DX_Project SHALL rename `icon` to `dx-icon`
10. THE DX_Project SHALL rename `i18n` to `dx-i18n`
11. THE DX_Project SHALL rename `media` to `dx-media`
12. THE DX_Project SHALL rename `error` to `dx-error`
13. THE DX_Project SHALL update all import paths to reflect new names
14. THE DX_Project SHALL remove dependency aliases that cause import confusion

### Requirement 6: Documentation Organization

**User Story:** As a developer, I want organized documentation, so that I can find information quickly and onboard new contributors.

#### Acceptance Criteria

1. THE DX_Project SHALL have a single authoritative README.md at the root
2. THE DX_Project SHALL organize all documentation under the `/docs` folder with clear hierarchy
3. THE DX_Project SHALL remove all date-based filenames (e.g., PROGRESS_DEC16.md)
4. THE DX_Project SHALL fix all typos in filenames (e.g., COMPABILITY.md → COMPATIBILITY.md)
5. THE DX_Project SHALL remove random files from the root directory (dx, image.png, KIRO.md)
6. THE DX_Project SHALL consolidate draft documents into proper spec folders
7. THE DX_Project SHALL have each crate contain only a README.md, not separate docs folders

### Requirement 7: Security Hardening

**User Story:** As a security-conscious developer, I want secure code practices, so that the project doesn't introduce vulnerabilities.

#### Acceptance Criteria

1. THE DX_Project SHALL remove all committed .env files
2. THE DX_Project SHALL add .env to .gitignore
3. THE DX_Project SHALL remove hardcoded credentials from example code
4. THE DX_Project SHALL audit and document all unsafe code blocks
5. THE DX_Project SHALL remove dangerous clippy suppressions like `#[allow(clippy::uninit_vec)]`
6. THE DX_Project SHALL use safe alternatives to `std::mem::transmute` where possible

### Requirement 8: Build Artifact Cleanup

**User Story:** As a developer, I want a clean repository, so that cloning and building is fast and predictable.

#### Acceptance Criteria

1. THE DX_Project SHALL remove all committed node_modules directories
2. THE DX_Project SHALL remove all committed target directories
3. THE DX_Project SHALL remove all committed out directories
4. THE DX_Project SHALL update .gitignore to prevent future commits of build artifacts
5. THE DX_Project SHALL consolidate per-crate .gitignore files into the root .gitignore

### Requirement 9: Extension Renaming and Integration

**User Story:** As a user, I want a properly named VS Code extension, so that it reflects the full DX tooling capabilities.

#### Acceptance Criteria

1. THE Extension SHALL be renamed from "vscode-dx-serializer" to "dx-vscode"
2. THE Extension SHALL be moved from `crates/vscode-dx-serializer` to `extension/`
3. THE Extension SHALL integrate with dx-forge daemon
4. THE Extension SHALL support all DX tools, not just the serializer
5. THE Extension SHALL have proper package.json metadata reflecting the DX brand

### Requirement 10: Forge Daemon Implementation

**User Story:** As a developer, I want a functional daemon, so that file watching and tool coordination actually works.

#### Acceptance Criteria

1. THE Daemon SHALL implement actual file watching instead of sleep placeholders
2. THE Daemon SHALL implement actual cache warming functionality
3. THE Daemon SHALL implement actual tool coordination
4. THE Daemon SHALL remove all `tokio::time::sleep` placeholder implementations
5. THE Daemon SHALL use proper async patterns for background tasks
6. WHEN the daemon starts, THE Daemon SHALL log its status using proper logging

### Requirement 11: Performance Claim Verification

**User Story:** As a user, I want honest performance claims, so that I can trust the project's benchmarks.

#### Acceptance Criteria

1. THE README SHALL clearly distinguish between verified benchmarks and target goals
2. THE README SHALL remove or mark unverified claims (2,500,000+ RPS, 5,000,000+ RPS, etc.)
3. THE DX_Project SHALL provide reproducible benchmark scripts for all verified claims
4. THE DX_Project SHALL include benchmark results with hardware specifications
5. IF a claim is a target, THEN THE README SHALL mark it with "🎯 Target" label

### Requirement 12: Orphaned Code Cleanup

**User Story:** As a maintainer, I want no orphaned code, so that every file in the repository serves a purpose.

#### Acceptance Criteria

1. THE DX_Project SHALL add Cargo.toml to crates/dx-core or remove the folder
2. THE DX_Project SHALL remove all commented-out module declarations
3. THE DX_Project SHALL remove all empty stub folders in .dx/ that are not used
4. THE DX_Project SHALL ensure every crate in /crates has a valid Cargo.toml
5. THE DX_Project SHALL remove deprecated code marked with `#[deprecated]` if not needed for compatibility

### Requirement 13: Error Handling Standardization

**User Story:** As a developer, I want consistent error handling, so that debugging is straightforward.

#### Acceptance Criteria

1. THE DX_Project SHALL define error types using `thiserror` for each major crate
2. THE DX_Project SHALL replace all `Err("string literal")` with typed errors
3. THE DX_Project SHALL replace `.unwrap()` in non-test code with proper error handling using `?`
4. THE DX_Project SHALL replace `.expect()` in non-test code with proper error handling
5. THE DX_Project SHALL implement `std::error::Error` for all custom error types

### Requirement 14: CI/CD Pipeline

**User Story:** As a maintainer, I want automated quality checks, so that code quality is enforced on every commit.

#### Acceptance Criteria

1. THE DX_Project SHALL have GitHub Actions workflow for `cargo build --workspace`
2. THE DX_Project SHALL have GitHub Actions workflow for `cargo test --workspace`
3. THE DX_Project SHALL have GitHub Actions workflow for `cargo clippy --workspace -- -D warnings`
4. THE DX_Project SHALL have GitHub Actions workflow for `cargo fmt --check`
5. THE DX_Project SHALL block merges on CI failures
6. THE DX_Project SHALL run CI on both Linux and Windows

### Requirement 15: Crate Consolidation

**User Story:** As a maintainer, I want a manageable number of crates, so that the project is maintainable.

#### Acceptance Criteria

1. THE DX_Project SHALL consolidate related dx-www-* crates where functionality overlaps
2. THE DX_Project SHALL merge dx-www-cache, dx-www-state, and dx-www-sync if they have overlapping concerns
3. THE DX_Project SHALL evaluate each crate for necessity and remove unused crates
4. THE DX_Project SHALL target a maximum of 30 actively maintained crates
5. THE DX_Project SHALL document the purpose of each remaining crate in a CRATES.md file
