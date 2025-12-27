# Implementation Plan: Crates Professionalization

## Overview

This implementation plan transforms the crates folder into a professional, consistent structure through validation scripts, cleanup automation, and manual documentation improvements. Tasks are organized in phases: validation infrastructure first, then fixes, then CI integration.

## Tasks

- [x] 1. Create crate scanning and validation infrastructure
  - [x] 1.1 Create the validation script structure
    - Create `scripts/validate-crates.rs` with main entry point
    - Implement crate discovery logic to find all Cargo.toml files
    - Parse Cargo.toml files using `toml` crate
    - _Requirements: 2.1, 7.1_

  - [ ]* 1.2 Write property test for crate discovery
    - **Property 1: Required Files Existence**
    - Test that scanner correctly identifies crates with/without required files
    - **Validates: Requirements 1.1, 2.1**

  - [x] 1.3 Implement README analysis functions
    - Create section detection for: title, description, installation, usage, license
    - Detect badge patterns and format consistency
    - Detect inappropriate content (task instructions, prompts)
    - _Requirements: 1.2, 1.3, 1.4_

  - [ ]* 1.4 Write property test for README content validation
    - **Property 2: README Content Completeness**
    - **Property 4: No Development Instructions in README**
    - **Validates: Requirements 1.2, 1.4**

  - [x] 1.5 Implement naming validation
    - Validate kebab-case directory names with regex `^[a-z][a-z0-9-]*$`
    - Compare directory names with Cargo.toml package names
    - _Requirements: 3.1, 3.3_

  - [ ]* 1.6 Write property test for naming validation
    - **Property 8: Valid Directory Names**
    - **Property 9: Name Consistency**
    - **Validates: Requirements 2.5, 3.1, 3.3**

- [x] 2. Checkpoint - Ensure validation infrastructure works
  - Run validation script against crates/ directory
  - Review generated report for accuracy
  - Ensure all tests pass, ask the user if questions arise

- [x] 3. Implement file existence and cleanup validation
  - [x] 3.1 Implement forbidden file detection
    - Detect Cargo.lock in library crates
    - Detect .env files (excluding .env.example)
    - Detect progress tracking files (PHASE*.md, TASKLIST.md, etc.)
    - Detect empty markdown files
    - _Requirements: 2.2, 4.4, 5.1, 5.3_

  - [ ]* 3.2 Write property test for forbidden files
    - **Property 6: No Cargo.lock in Libraries**
    - **Property 11: No Sensitive Files**
    - **Property 12: No Progress Tracking Files**
    - **Property 13: No Empty Documentation Files**
    - **Validates: Requirements 2.2, 4.4, 5.1, 5.3**

  - [x] 3.3 Implement license compliance checking
    - Check for LICENSE file in crate directory
    - Check for license field in Cargo.toml
    - Check for license reference in README
    - _Requirements: 4.1, 4.2_

  - [ ]* 3.4 Write property test for license compliance
    - **Property 10: License Compliance**
    - **Validates: Requirements 4.1, 4.2**

  - [x] 3.5 Implement Cargo.toml completeness validation
    - Check for required fields: name, version, edition, description
    - Validate version format (semver)
    - Check for repository and documentation links
    - _Requirements: 7.1, 7.2, 7.3_

  - [ ]* 3.6 Write property test for Cargo.toml validation
    - **Property 15: Cargo.toml Required Fields**
    - **Property 16: Version Format Consistency**
    - **Validates: Requirements 7.1, 7.2**

- [x] 4. Checkpoint - Full validation working
  - Run complete validation and generate full report
  - Ensure all tests pass, ask the user if questions arise

- [x] 5. Create templates and fix infrastructure
  - [x] 5.1 Create README template
    - Create `templates/README.template.md` with standard sections
    - Include badge placeholders with shields.io format
    - Include installation, usage, and license sections
    - _Requirements: 1.2, 1.3_

  - [x] 5.2 Create Cargo.toml template
    - Create `templates/Cargo.template.toml` with all required fields
    - Include placeholder values for customization
    - _Requirements: 7.1_

  - [x] 5.3 Create cleanup script
    - Create `scripts/cleanup-crates.rs`
    - Implement dry-run mode for safe preview
    - Handle file deletion and relocation
    - _Requirements: 5.1, 5.2_

- [x] 6. Execute cleanup and fixes
  - [x] 6.1 Remove Cargo.lock from library crates
    - Identify all library crates with Cargo.lock
    - Remove Cargo.lock files
    - Update .gitignore if needed
    - _Requirements: 2.2_

  - [x] 6.2 Remove/relocate development artifacts
    - Move crates/check/PHASE*.md to docs/archive/
    - Move crates/check/TASKLIST.md to docs/archive/
    - Remove crates/forge/.env (keep .env.example)
    - _Requirements: 5.1, 4.4_

  - [x] 6.3 Fix naming issues
    - Rename crates/python/COMPABILITY.md to COMPATIBILITY.md
    - Fix malformed directory name `cratesdx-py-runtimedx-py-corebenches`
    - _Requirements: 3.5, 2.5_

  - [x] 6.4 Remove empty documentation files
    - Delete crates/python/COMPABILITY.md (empty)
    - Scan for and remove other empty .md files
    - _Requirements: 5.3_

- [x] 7. Checkpoint - Cleanup complete
  - Run validation to confirm cleanup success
  - Ensure all tests pass, ask the user if questions arise

- [x] 8. Fix README documentation
  - [x] 8.1 Rewrite crates/font/README.md
    - Replace task instructions with proper documentation
    - Add overview, installation, usage sections
    - Add badges and license information
    - _Requirements: 1.2, 1.4_

  - [x] 8.2 Create crates/cli/README.md
    - Create new README following template
    - Document CLI functionality and usage
    - _Requirements: 1.1, 1.2_

  - [x] 8.3 Create crates/www/README.md
    - Document all 28 subcrates in a table
    - Group by category (core, data, network, etc.)
    - Link to individual subcrate READMEs
    - _Requirements: 1.5, 6.2_

  - [x] 8.4 Create crates/javascript/README.md
    - Document all JavaScript subcrates
    - Include bundler, runtime, test-runner, etc.
    - _Requirements: 1.5_

  - [ ]* 8.5 Write property test for subcrate documentation
    - **Property 5: Subcrate Documentation**
    - **Validates: Requirements 1.5, 6.2**

- [x] 9. Complete Cargo.toml metadata
  - [x] 9.1 Add missing fields to all Cargo.toml files
    - Add authors field where missing
    - Add description field where missing
    - Add repository and documentation links
    - _Requirements: 7.1, 7.3_

  - [x] 9.2 Standardize license fields
    - Ensure all Cargo.toml have license = "MIT OR Apache-2.0"
    - Or document exceptions clearly
    - _Requirements: 4.2_

- [x] 10. Ensure www subcrates have required files
  - [x] 10.1 Audit www subcrates for README.md
    - Check all 28 subcrates for README.md
    - Create minimal READMEs where missing
    - _Requirements: 6.4_

  - [ ]* 10.2 Write property test for www subcrate completeness
    - **Property 14: WWW Subcrate Completeness**
    - **Validates: Requirements 6.4**

- [x] 11. Update .gitignore
  - Add patterns for development artifacts
  - Add Cargo.lock exclusion for library crates
  - Add .env exclusion (keep .env.example)
  - _Requirements: 5.2_

- [x] 12. Final checkpoint - All validation passes
  - Run full validation suite
  - Ensure zero errors, minimal warnings
  - Ensure all tests pass, ask the user if questions arise

## Notes

- Tasks marked with `*` are optional property-based tests
- Each property test references specific design document properties
- Checkpoints ensure incremental validation of progress
- Manual README fixes (task 8) require human judgment for content quality
- The validation script can be integrated into CI after completion
