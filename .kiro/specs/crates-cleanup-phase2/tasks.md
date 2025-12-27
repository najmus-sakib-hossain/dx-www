# Implementation Plan: Crates Cleanup Phase 2

## Overview

This implementation plan addresses remaining professionalization issues in the crates folder, focusing on removing development artifacts, Cargo.lock files from library crates, and consolidating scattered configuration.

## Tasks

- [x] 1. Create archive directory structure
  - [x] 1.1 Create docs/archive/development/ directory
    - For storing development status and planning files
    - _Requirements: 1.6_

  - [x] 1.2 Create docs/archive/planning/ directory
    - For storing planning and roadmap files
    - _Requirements: 1.6_

- [x] 2. Move development artifacts to archive
  - [x] 2.1 Move crates/serializer/STATUS.md to docs/archive/development/
    - Rename to serializer-status.md for clarity
    - _Requirements: 1.1_

  - [x] 2.2 Move crates/serializer/PLAN.md to docs/archive/development/
    - Rename to serializer-plan.md for clarity
    - _Requirements: 1.2_

  - [x] 2.3 Move crates/driven/COMPLETION_REPORT.md to docs/archive/development/
    - Rename to driven-completion-report.md for clarity
    - _Requirements: 1.3_

  - [x] 2.4 Move crates/check/ROADMAP.md to docs/archive/planning/
    - Rename to check-roadmap.md for clarity
    - _Requirements: 1.4_

  - [x] 2.5 Move crates/check/REMOVE_SUBMODULES_PLAN.md to docs/archive/planning/
    - Rename to check-submodules-plan.md for clarity
    - _Requirements: 1.5_

  - [x] 2.6 Move crates/www/core/PLANNING.md to docs/archive/planning/
    - Rename to www-core-planning.md for clarity
    - _Requirements: 4.4_

- [x] 3. Remove Cargo.lock from library crates
  - [x] 3.1 Remove crates/font/Cargo.lock
    - Font is a library crate
    - _Requirements: 2.2_

  - [x] 3.2 Remove crates/icon/Cargo.lock
    - Icon is a library crate
    - _Requirements: 2.2_

  - [x] 3.3 Remove crates/style/Cargo.lock
    - Style is a library crate
    - _Requirements: 2.2_

  - [x] 3.4 Remove crates/media/Cargo.lock
    - Media has a binary but is primarily a library
    - _Requirements: 2.2_

- [x] 4. Checkpoint - Verify cleanup progress
  - Ensure archive files exist
  - Ensure Cargo.lock files removed
  - Ensure all tests pass, ask the user if questions arise

- [x] 5. Evaluate and clean scattered configuration
  - [x] 5.1 KEEP crates/dx/.kiro/ - contains dx-cli-hardening spec (valuable)
    - _Requirements: 3.1_

  - [x] 5.2 KEEP crates/forge/.kiro/ - contains platform-native-io-hardening spec (valuable)
    - _Requirements: 3.1_

  - [x] 5.3 SKIP crates/forge/.dx/ - already gitignored, not tracked (runtime cache)
    - _Requirements: 3.2_

  - [x] 5.4 KEEP crates/python/.kiro/ - contains 7 dx-py specs (valuable)
    - _Requirements: 3.1_

  - [x] 5.5 KEEP crates/www/core/.kiro/ - contains cross-platform-io-reactor spec (valuable)
    - _Requirements: 3.1_

  - [x] 5.6 SKIP crates/style/.dx/ - already gitignored, not tracked (runtime cache)
    - _Requirements: 3.2_

  - [x] 5.7 KEEP crates/media/.bmad/ - contains extensive agent configuration (valuable)
    - _Requirements: 3.4_

- [x] 6. Consolidate non-standard documentation
  - [x] 6.1 Move crates/check/*.md spec files to crates/check/docs/
    - Moved DX_CHECK.md, DX_CONFIG_SPEC.md, DXS_FILES_GUIDE.md, DXS_FORMAT_SPEC.md
    - _Requirements: 5.2, 5.3_

  - [x] 6.2 Move crates/serializer/SERIALIZER.md to crates/serializer/docs/
    - docs/ subdirectory already existed
    - _Requirements: 5.2, 5.3_

  - [x] 6.3 Move crates/driven/*.md spec files to crates/driven/docs/
    - Moved ARCHITECTURE.md, BINARY_FORMAT.md
    - Created docs/ subdirectory
    - _Requirements: 5.2, 5.3_

  - [x] 6.4 Move crates/media/MEDIA.md to crates/media/docs/
    - Renamed to MEDIA_OVERVIEW.md to avoid conflict with existing MEDIA.md in docs/
    - _Requirements: 5.2, 5.3_

- [x] 7. Final checkpoint - All cleanup complete
  - Verified workspace compiles with `cargo check --workspace`
  - No development artifacts in crate roots
  - No Cargo.lock in library crates
  - Non-standard documentation consolidated to docs/ subdirectories
  - Scattered configuration evaluated and valuable content preserved

## Notes

- Always check file contents before deletion to preserve valuable information
- Use git mv for moves to preserve history
- Verify workspace still compiles after each major change
- Some .github/ directories in crates may contain crate-specific CI and should be evaluated individually



## Completion Summary

All tasks completed successfully on December 27, 2025.

### Changes Made:
1. **Development artifacts archived** - STATUS.md, PLAN.md, COMPLETION_REPORT.md, ROADMAP.md, PLANNING.md moved to docs/archive/
2. **Cargo.lock removed** from library crates (font, icon, style, media)
3. **Scattered configuration evaluated** - .kiro/ specs and .bmad/ agent configs preserved (valuable), .dx/ runtime cache already gitignored
4. **Documentation consolidated** - Non-standard docs moved to crate-specific docs/ subdirectories

### Crates Now Professional:
- Clean root directories with only standard files (README.md, Cargo.toml, CHANGELOG.md, LICENSE)
- Documentation properly organized in docs/ subdirectories
- No development artifacts cluttering crate roots
- Valuable specs and configurations preserved in appropriate locations
