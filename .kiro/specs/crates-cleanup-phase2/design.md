# Design Document: Crates Cleanup Phase 2

## Overview

This design document outlines the approach for the second phase of crates folder professionalization. The focus is on removing development artifacts, consolidating scattered configuration, and ensuring a clean, professional presentation across all crates.

## Architecture

The cleanup follows a systematic approach:
1. Identify all files that need removal or relocation
2. Create archive directories for valuable content
3. Execute cleanup operations
4. Verify the cleanup was successful

## Components and Interfaces

### 1. File Identification

Files to be removed or relocated:

**Development Artifacts (move to docs/archive/):**
- `crates/serializer/STATUS.md`
- `crates/serializer/PLAN.md`
- `crates/driven/COMPLETION_REPORT.md`
- `crates/check/REMOVE_SUBMODULES_PLAN.md`
- `crates/check/ROADMAP.md`
- `crates/www/core/PLANNING.md`

**Cargo.lock in Library Crates (remove):**
- `crates/font/Cargo.lock`
- `crates/icon/Cargo.lock`
- `crates/style/Cargo.lock`
- `crates/media/Cargo.lock`

**Scattered Configuration (evaluate and consolidate):**
- `crates/dx/.kiro/`
- `crates/forge/.kiro/`
- `crates/forge/.dx/`
- `crates/python/.kiro/`
- `crates/www/core/.kiro/`
- `crates/style/.dx/`
- `crates/forge/.github/`
- `crates/serializer/.github/`
- `crates/style/.github/`
- `crates/media/.github/`
- `crates/media/.bmad/`

**Non-Standard Documentation (consolidate to docs/):**
- `crates/check/DX_CHECK.md`
- `crates/check/DX_CONFIG_SPEC.md`
- `crates/check/DXS_FILES_GUIDE.md`
- `crates/check/DXS_FORMAT_SPEC.md`
- `crates/serializer/SERIALIZER.md`
- `crates/driven/ARCHITECTURE.md`
- `crates/driven/BINARY_FORMAT.md`
- `crates/media/MEDIA.md`

### 2. Archive Structure

```
docs/
├── archive/
│   ├── development/
│   │   ├── serializer-status.md
│   │   ├── serializer-plan.md
│   │   ├── driven-completion-report.md
│   │   ├── check-roadmap.md
│   │   └── check-submodules-plan.md
│   └── planning/
│       └── www-core-planning.md
```

## Data Models

### Cleanup Operation

```rust
pub struct CleanupOperation {
    pub source_path: PathBuf,
    pub action: CleanupAction,
    pub destination: Option<PathBuf>,
    pub reason: String,
}

pub enum CleanupAction {
    Delete,
    MoveToArchive,
    MoveToDocsSubdir,
    ConsolidateToRoot,
}
```

## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system.*

### Property 1: No Development Artifacts in Crate Roots

*For any* crate directory, files matching patterns STATUS.md, PLAN.md, COMPLETION_REPORT.md, ROADMAP.md, PLANNING.md SHALL NOT exist in the crate root.

**Validates: Requirements 1.1, 1.2, 1.3, 1.4, 4.4**

### Property 2: No Cargo.lock in Library Crates

*For any* library crate (no [[bin]] target), the crate directory SHALL NOT contain a Cargo.lock file.

**Validates: Requirements 2.1, 2.2**

### Property 3: Minimal Scattered Configuration

*For any* crate directory (excluding root), the directory SHOULD NOT contain .kiro/, .dx/, or .github/ subdirectories unless documented in README.

**Validates: Requirements 3.1, 3.2, 3.3, 3.5**

### Property 4: Standard Documentation Only

*For any* crate directory, documentation files SHALL be limited to README.md, CHANGELOG.md, CONTRIBUTING.md, LICENSE, or contained within a docs/ subdirectory.

**Validates: Requirements 5.1, 5.2, 5.3**

## Error Handling

| Error Type | Handling Strategy |
|------------|-------------------|
| File not found | Skip and log warning |
| Permission denied | Log error and continue |
| Archive directory exists | Append to existing |
| File already archived | Skip with info message |

## Testing Strategy

### Unit Tests
- Verify file pattern matching
- Verify archive path generation
- Verify library crate detection

### Integration Tests
- Run cleanup in dry-run mode
- Verify no data loss
- Verify archive structure

## Implementation Notes

### Phase 1: Create Archive Structure
Create docs/archive/ directories before moving files.

### Phase 2: Move Development Artifacts
Move STATUS.md, PLAN.md, etc. to docs/archive/development/.

### Phase 3: Remove Cargo.lock from Libraries
Delete Cargo.lock from font, icon, style, media crates.

### Phase 4: Evaluate Scattered Config
Review each .kiro/, .dx/, .github/ directory and decide on consolidation.

### Phase 5: Consolidate Documentation
Move non-standard docs to crate-specific docs/ subdirectories.

## File Changes Summary

### New Directories
- `docs/archive/development/`
- `docs/archive/planning/`

### Files to Move
- Development artifacts → docs/archive/development/
- Planning files → docs/archive/planning/

### Files to Delete
- Cargo.lock in library crates
- Empty or redundant configuration directories

### Files to Consolidate
- Non-standard documentation → crate docs/ subdirectories

