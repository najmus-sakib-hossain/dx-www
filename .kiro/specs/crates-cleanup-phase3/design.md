# Design Document: Crates Cleanup Phase 3

## Overview

This design document outlines the approach for the third phase of crates folder professionalization. The focus is on consolidating scattered `.github/` documentation files, ensuring proper gitignore patterns for build artifacts, and cleaning up empty or unnecessary directories.

## Architecture

The cleanup follows a systematic approach:
1. Audit all .github/ directories for documentation vs CI configuration
2. Move documentation files to appropriate docs/ subdirectories
3. Update .gitignore for build artifacts
4. Remove empty or unnecessary directories

## Components and Interfaces

### 1. Files to Relocate

**crates/style/.github/ documentation → crates/style/docs/:**
- ADVANCED_OPTIMIZATIONS.md
- INCREMENTAL_PARSING.md
- OPTIMIZATION_SUMMARY.md
- PERFORMANCE.md
- README_OPTIMIZATION.md
- ULTIMATE_PERFORMANCE.md
- (KEEP: copilot-instructions.md, workflows/)

**crates/check/.github/ documentation → crates/check/docs/:**
- ADDING_LANGUAGE_SUPPORT.md
- ARCHITECTURE.md
- LANGUAGE_INTEGRATION_SUMMARY.md

**crates/forge/ root → crates/forge/docs/:**
- USAGE.md

### 2. Directories to Clean

**Empty .github/ directories:**
- crates/serializer/.github/ (empty - remove)

**Build artifact directories (ensure gitignored):**
- crates/check/target/
- crates/serializer/pkg/
- crates/forge/logs/
- crates/*/proptest-regressions/

**IDE/Editor directories in crates:**
- crates/style/.vscode/
- crates/style/.idx/
- crates/style/.gitpod.yml

### 3. Gitignore Updates

Add patterns to root .gitignore:
```gitignore
# Proptest regression files
**/proptest-regressions/

# Crate-specific build artifacts
crates/*/pkg/
crates/*/logs/
```

## Data Models

### Cleanup Operation

```rust
pub enum CleanupAction {
    MoveToDocsSubdir { source: PathBuf, dest: PathBuf },
    RemoveEmptyDir { path: PathBuf },
    AddGitignorePattern { pattern: String },
    RemoveTrackedArtifact { path: PathBuf },
}
```

## Correctness Properties

### Property 1: No Documentation in .github/

*For any* crate's .github/ directory, it SHALL contain only workflows/, copilot-instructions.md, or GitHub-specific configuration files (not general documentation).

**Validates: Requirements 1.1, 1.2, 1.3**

### Property 2: Build Artifacts Gitignored

*For any* build artifact directory (target/, pkg/, logs/, proptest-regressions/), it SHALL be covered by a .gitignore pattern.

**Validates: Requirements 3.1, 3.2, 3.3, 3.4**

### Property 3: Standard Root Files Only

*For any* crate root directory, only standard files (README.md, Cargo.toml, CHANGELOG.md, LICENSE, CONTRIBUTING.md, build.rs, .gitignore, .env.example) SHALL exist.

**Validates: Requirements 5.1, 5.2**

## Error Handling

| Error Type | Handling Strategy |
|------------|-------------------|
| File not found | Skip and log warning |
| Directory not empty | List contents and skip |
| Permission denied | Log error and continue |

## Testing Strategy

### Verification Steps
- Verify no .md files in .github/ (except copilot-instructions.md)
- Verify build artifacts are gitignored
- Verify workspace compiles after changes

## Implementation Notes

### Phase 1: Move .github/ Documentation
Move documentation files from .github/ to docs/ subdirectories.

### Phase 2: Clean Empty Directories
Remove empty .github/ directories and evaluate IDE directories.

### Phase 3: Update Gitignore
Add patterns for build artifacts not currently covered.

### Phase 4: Verify
Run cargo check and verify structure.
