# Implementation Plan: Crates Cleanup Phase 3

## Overview

This implementation plan addresses remaining professionalization issues: scattered .github/ documentation, build artifacts, and empty directories.

## Tasks

- [x] 1. Move .github/ documentation to docs/
  - [x] 1.1 Move crates/style/.github/*.md to crates/style/docs/
    - Move ADVANCED_OPTIMIZATIONS.md, INCREMENTAL_PARSING.md, OPTIMIZATION_SUMMARY.md, PERFORMANCE.md, README_OPTIMIZATION.md, ULTIMATE_PERFORMANCE.md
    - Keep copilot-instructions.md and workflows/ in .github/
    - _Requirements: 1.1, 1.4_

  - [x] 1.2 Move crates/check/.github/*.md to crates/check/docs/
    - Move ADDING_LANGUAGE_SUPPORT.md, ARCHITECTURE.md, LANGUAGE_INTEGRATION_SUMMARY.md
    - _Requirements: 1.1_

  - [x] 1.3 Move crates/forge/USAGE.md to crates/forge/docs/
    - _Requirements: 2.1_

- [x] 2. Clean empty and unnecessary directories
  - [x] 2.1 Remove crates/serializer/.github/ (empty)
    - _Requirements: 1.3, 4.1_

  - [x] 2.2 Evaluate crates/style/.vscode/, .idx/, .gitpod.yml
    - These are IDE-specific, consider removal or gitignore
    - _Requirements: 4.2, 4.3_

- [x] 3. Update .gitignore for build artifacts
  - [x] 3.1 Add proptest-regressions/ pattern
    - _Requirements: 3.4_

  - [x] 3.2 Add crate-specific pkg/ and logs/ patterns
    - _Requirements: 3.2, 3.3_

  - [x] 3.3 Verify target/ directories are covered
    - _Requirements: 3.1_

- [x] 4. Checkpoint - Verify cleanup
  - Run cargo check --workspace
  - Verify no documentation in .github/ directories
  - Verify build artifacts gitignored
  - _Requirements: 5.1, 5.2, 5.3_

## Notes

- Keep copilot-instructions.md in .github/ (GitHub Copilot specific)
- Keep workflows/ in .github/ (GitHub Actions)
- IDE directories (.vscode/, .idx/) may be useful for contributors - evaluate case by case
- proptest-regressions/ contains test failure records - should be gitignored but may be useful locally


## Completion Summary

All tasks completed successfully on December 27, 2025.

### Changes Made:
1. **Documentation consolidated from .github/**:
   - `crates/style/.github/` - 6 optimization docs moved to `docs/`
   - `crates/check/.github/` - 3 architecture docs moved to `docs/`, empty dir removed
   - `crates/forge/USAGE.md` moved to `docs/`

2. **Empty/unnecessary directories removed**:
   - `crates/serializer/.github/` (was empty)
   - `crates/check/.github/` (after moving docs)
   - `crates/style/.vscode/`, `.idx/`, `.gitpod.yml` (IDE-specific)

3. **Build artifacts untracked and gitignored**:
   - `proptest-regressions/` directories (dx, forge, serializer)
   - `logs/` directories
   - `pkg/` directories (WASM output)
   - Cleaned up duplicate patterns in .gitignore

4. **.gitignore cleaned and updated**:
   - Removed duplicate entries
   - Added `**/proptest-regressions/`
   - Added `**/logs/`
   - Added `**/pkg/`
   - Added `**/.idx/` and `**/.gitpod.yml`
   - Added `**/.venv/`

### Remaining .github/ directories (intentionally kept):
- `crates/forge/.github/workflows/` - CI configuration
- `crates/style/.github/workflows/` + `copilot-instructions.md`
- `crates/media/.github/copilot-instructions.md`
