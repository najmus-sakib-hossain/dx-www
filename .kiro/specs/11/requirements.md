# Requirements Document

## Introduction

This document specifies the requirements for reorganizing the `crates/` folder structure and `docs/` folder to follow professional project conventions. The goal is to transform an inconsistent naming scheme into a clean, logical hierarchy and ensure all documentation is accurate, verified, and professionally organized.

## Glossary

- **Workspace**: The root Cargo workspace containing all crates
- **Crate**: A Rust package/library within the workspace
- **CLI**: Command Line Interface tool
- **WWW**: Web framework related crates
- **JavaScript_Runtime**: JavaScript/TypeScript tooling crates (bundler, runtime, test-runner, etc.)
- **Python_Runtime**: Python tooling crates

## Requirements

### Requirement 1: Preserve Core Tool Crates

**User Story:** As a developer, I want core tool crates to remain at the top level with clean names, so that I can easily locate fundamental functionality.

#### Acceptance Criteria

1. THE Workspace SHALL maintain these crates at `crates/` root level: `check`, `driven`, `dx`, `error`, `font`, `forge`, `generator`, `i18n`, `icon`, `media`, `serializer`, `style`, `workspace`
2. THE Workspace SHALL remove the `stack` crate as it is unused
3. WHEN a core crate is accessed, THE path SHALL follow the pattern `crates/{crate_name}/`

### Requirement 2: Rename CLI Crate

**User Story:** As a developer, I want the CLI crate to have a clean name without prefix, so that it follows professional naming conventions.

#### Acceptance Criteria

1. THE Workspace SHALL rename `dx-cli` to `cli`
2. WHEN the CLI crate is renamed, THE Cargo.toml package name SHALL be updated to `dx-cli` (keeping the published name)
3. WHEN the CLI crate is renamed, THE workspace Cargo.toml path SHALL be updated to `crates/cli`
4. THE CLI crate SHALL remain functional after the rename

### Requirement 3: Rename Core Crate

**User Story:** As a developer, I want the core crate to have a clean name without prefix, so that it follows professional naming conventions.

#### Acceptance Criteria

1. THE Workspace SHALL rename `dx-core` to `core`
2. WHEN the core crate is renamed, THE workspace Cargo.toml path SHALL be updated to `crates/core`
3. WHEN any crate depends on dx-core, THE dependency path SHALL be updated accordingly

### Requirement 4: Rename Reactor Crate

**User Story:** As a developer, I want the reactor crate to have a clean name without prefix, so that it follows professional naming conventions.

#### Acceptance Criteria

1. THE Workspace SHALL rename `dx-reactor` to `reactor`
2. WHEN the reactor crate is renamed, THE workspace Cargo.toml path SHALL be updated to `crates/reactor`
3. WHEN any crate depends on dx-reactor, THE dependency path SHALL be updated accordingly

### Requirement 5: Consolidate JavaScript Crates

**User Story:** As a developer, I want all JavaScript-related crates grouped in a single folder, so that I can easily find JS tooling.

#### Acceptance Criteria

1. THE Workspace SHALL create a `crates/javascript/` directory
2. THE Workspace SHALL move these crates into `crates/javascript/`:
   - `dx-js-bundler` → `javascript/bundler`
   - `dx-js-compatibility` → `javascript/compatibility`
   - `dx-js-monorepo` → `javascript/monorepo`
   - `dx-js-package-manager` → `javascript/package-manager`
   - `dx-js-runtime` → `javascript/runtime`
   - `dx-js-test-runner` → `javascript/test-runner`
3. WHEN a JavaScript crate is moved, THE workspace Cargo.toml paths SHALL be updated
4. WHEN a JavaScript crate is moved, THE inter-crate dependency paths SHALL be updated

### Requirement 6: Consolidate Python Crates

**User Story:** As a developer, I want all Python-related crates grouped in a single folder, so that I can easily find Python tooling.

#### Acceptance Criteria

1. THE Workspace SHALL rename `dx-py` to `python`
2. THE Workspace SHALL move the crate to `crates/python/`
3. WHEN the Python crate is moved, THE workspace Cargo.toml path SHALL be updated

### Requirement 7: Consolidate WWW Crates

**User Story:** As a developer, I want all web framework crates grouped in a single folder, so that I can easily find web-related functionality.

#### Acceptance Criteria

1. THE Workspace SHALL create a `crates/www/` directory
2. THE Workspace SHALL move these crates into `crates/www/` with clean names:
   - `dx-www` → `www/core` (main framework)
   - `dx-www-a11y` → `www/a11y`
   - `dx-www-auth` → `www/auth`
   - `dx-www-binary` → `www/binary`
   - `dx-www-cache` → `www/cache`
   - `dx-www-client` → `www/client`
   - `dx-www-client-tiny` → `www/client-tiny`
   - `dx-www-core` → `www/framework-core` (to avoid conflict with main www)
   - `dx-www-db` → `www/db`
   - `dx-www-dom` → `www/dom`
   - `dx-www-fallback` → `www/fallback`
   - `dx-www-form` → `www/form`
   - `dx-www-guard` → `www/guard`
   - `dx-www-interaction` → `www/interaction`
   - `dx-www-morph` → `www/morph`
   - `dx-www-offline` → `www/offline`
   - `dx-www-packet` → `www/packet`
   - `dx-www-print` → `www/print`
   - `dx-www-query` → `www/query`
   - `dx-www-rtl` → `www/rtl`
   - `dx-www-sched` → `www/sched`
   - `dx-www-server` → `www/server`
   - `dx-www-state` → `www/state`
   - `dx-www-sync` → `www/sync`
   - `dx-db-teleport` → `www/db-teleport`
3. WHEN a WWW crate is moved, THE workspace Cargo.toml paths SHALL be updated
4. WHEN a WWW crate is moved, THE inter-crate dependency paths SHALL be updated

### Requirement 8: Update Workspace Configuration

**User Story:** As a developer, I want the workspace Cargo.toml to reflect the new structure, so that cargo commands work correctly.

#### Acceptance Criteria

1. WHEN any crate is moved or renamed, THE root Cargo.toml members list SHALL be updated
2. WHEN any crate is moved or renamed, THE root Cargo.toml workspace dependencies SHALL be updated
3. THE Workspace SHALL compile successfully after all changes with `cargo check`

### Requirement 9: Update Internal Dependencies

**User Story:** As a developer, I want all internal crate dependencies to work after reorganization, so that the project builds correctly.

#### Acceptance Criteria

1. WHEN a crate path changes, ALL Cargo.toml files referencing it via path SHALL be updated
2. WHEN a crate is renamed, ALL `use` statements in Rust code SHALL remain valid (package names preserved)
3. THE entire workspace SHALL pass `cargo build` after reorganization

### Requirement 10: Maintain Debug Crate

**User Story:** As a developer, I want the debug crate preserved in its current location, so that debugging tools remain accessible.

#### Acceptance Criteria

1. THE Workspace SHALL keep the `debug` crate at `crates/debug/`
2. THE debug crate SHALL not be moved or renamed


### Requirement 11: Verify All Benchmark Claims

**User Story:** As a developer, I want all benchmark claims to be verified and accurate, so that the project maintains professional credibility.

#### Acceptance Criteria

1. THE Documentation SHALL audit all benchmark claims in `docs/reference/benchmarks/`
2. WHEN a benchmark claim exists, THE claim SHALL have reproducible test methodology documented
3. THE main `docs/README.md` performance table SHALL link to verification details for each claim
4. THE Documentation SHALL ensure all "X times faster" claims have corresponding passing tests
5. WHEN benchmark tests are run, THE results SHALL match documented claims within reasonable variance

### Requirement 12: Complete All In-Progress Documentation

**User Story:** As a developer, I want all documentation to be complete and accurate, so that there are no unfinished sections.

#### Acceptance Criteria

1. THE Documentation SHALL identify all "TODO", "WIP", "in-progress", or incomplete sections
2. WHEN incomplete documentation is found, THE documentation SHALL be completed or removed
3. THE Documentation SHALL ensure `docs/api/` contains complete documentation for all implemented features
4. THE Documentation SHALL remove `docs/api/stack.md` since the stack crate is being removed
5. THE Documentation SHALL ensure no placeholder text remains in any documentation file

### Requirement 13: Ensure All Tests Pass

**User Story:** As a developer, I want all tests to pass before claiming features are complete, so that the project is production-ready.

#### Acceptance Criteria

1. THE Project SHALL have all unit tests passing with `cargo test`
2. THE Project SHALL have all benchmark tests passing
3. WHEN a feature is documented, THE corresponding tests SHALL exist and pass
4. THE Documentation SHALL not claim features that have failing tests

### Requirement 14: Clean Archive and Remove Redundant Files

**User Story:** As a project maintainer, I want the documentation structure to be clean and professional, so that developers can easily navigate.

#### Acceptance Criteria

1. THE Documentation SHALL organize `docs/archive/` with clear subdirectories
2. THE Documentation SHALL remove duplicate or redundant documentation files
3. THE Documentation SHALL ensure `.txt` files in archive are properly formatted or converted to markdown
4. THE Documentation SHALL remove any temporary or debug files from docs

### Requirement 15: Update Documentation References After Reorganization

**User Story:** As a developer, I want documentation links to work after reorganization, so that I can navigate the docs effectively.

#### Acceptance Criteria

1. WHEN crates are renamed or moved, THE corresponding documentation SHALL be updated
2. WHEN documentation files are moved, THE cross-references SHALL be updated
3. THE `docs/README.md` navigation table SHALL reflect the actual folder structure
4. THE Documentation SHALL remove references to removed crates (e.g., stack)
5. THE Documentation SHALL update all internal links to use new crate paths
