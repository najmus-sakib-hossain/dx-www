# Implementation Plan: Crates Reorganization

## Overview

This plan reorganizes the DX workspace crates into a professional hierarchy and ensures all documentation is complete and verified. Tasks are ordered to minimize build breakage during the transition.

## Tasks

- [x] 1. Preparation and Backup
  - [x] 1.1 Create a git branch for the reorganization
    - Run `git checkout -b refactor/crates-reorganization`
    - _Requirements: All_
  - [x] 1.2 Verify current workspace builds
    - Run `cargo check --workspace` to ensure clean starting state
    - _Requirements: 8.3, 9.3, 13.1_

- [x] 2. Remove Unused Crates
  - [x] 2.1 Remove the stack crate
    - Delete `crates/stack/` directory
    - Remove from root Cargo.toml members list
    - _Requirements: 1.2_
  - [x] 2.2 Remove docs/api/stack.md
    - Delete the file
    - _Requirements: 12.4_
  - [x] 2.3 Verify build after removal
    - Run `cargo check --workspace`
    - _Requirements: 8.3_

- [x] 3. Rename Top-Level Crates
  - [x] 3.1 Rename dx-cli to cli
    - Run `git mv crates/dx-cli crates/cli`
    - Update root Cargo.toml member path
    - _Requirements: 2.1, 2.2, 2.3_
  - [x] 3.2 Rename dx-core to core
    - Run `git mv crates/dx-core crates/core`
    - Update root Cargo.toml member path
    - _Requirements: 3.1, 3.2_
  - [x] 3.3 Rename dx-reactor to reactor
    - Run `git mv crates/dx-reactor crates/reactor`
    - Update root Cargo.toml member path
    - _Requirements: 4.1, 4.2_
  - [x] 3.4 Rename dx-py to python
    - Run `git mv crates/dx-py crates/python`
    - Update root Cargo.toml member path
    - _Requirements: 6.1, 6.2, 6.3_
  - [x] 3.5 Update all path dependencies for renamed crates
    - Search all Cargo.toml files for old paths
    - Update paths: `../dx-cli` → `../cli`, `../dx-core` → `../core`, etc.
    - _Requirements: 3.3, 4.3, 9.1_
  - [x] 3.6 Verify build after renames
    - Run `cargo check --workspace`
    - _Requirements: 2.4, 8.3_

- [x] 4. Create JavaScript Crate Group
  - [x] 4.1 Create javascript directory and move bundler
    - Run `mkdir crates/javascript`
    - Run `git mv crates/dx-js-bundler crates/javascript/bundler`
    - _Requirements: 5.1, 5.2_
  - [x] 4.2 Move remaining JavaScript crates
    - Run `git mv crates/dx-js-compatibility crates/javascript/compatibility`
    - Run `git mv crates/dx-js-monorepo crates/javascript/monorepo`
    - Run `git mv crates/dx-js-package-manager crates/javascript/package-manager`
    - Run `git mv crates/dx-js-runtime crates/javascript/runtime`
    - Run `git mv crates/dx-js-test-runner crates/javascript/test-runner`
    - _Requirements: 5.2_
  - [x] 4.3 Update root Cargo.toml for JavaScript crates
    - Update all member paths to new locations
    - _Requirements: 5.3, 8.1_
  - [x] 4.4 Update inter-crate dependencies for JavaScript crates
    - Search and update all path dependencies within JavaScript crates
    - _Requirements: 5.4, 9.1_
  - [x] 4.5 Verify JavaScript crates build
    - Run `cargo check -p dx-js-bundler -p dx-js-runtime` etc.
    - _Requirements: 8.3_

- [x] 5. Create WWW Crate Group
  - [x] 5.1 Create www directory and move main framework
    - Run `mkdir crates/www`
    - Run `git mv crates/dx-www crates/www/core`
    - _Requirements: 7.1, 7.2_
  - [x] 5.2 Move dx-www-core to www/framework-core
    - Run `git mv crates/dx-www-core crates/www/framework-core`
    - _Requirements: 7.2_
  - [x] 5.3 Move remaining WWW crates (batch 1: a11y through dom)
    - Run `git mv crates/dx-www-a11y crates/www/a11y`
    - Run `git mv crates/dx-www-auth crates/www/auth`
    - Run `git mv crates/dx-www-binary crates/www/binary`
    - Run `git mv crates/dx-www-cache crates/www/cache`
    - Run `git mv crates/dx-www-client crates/www/client`
    - Run `git mv crates/dx-www-client-tiny crates/www/client-tiny`
    - Run `git mv crates/dx-www-db crates/www/db`
    - Run `git mv crates/dx-www-dom crates/www/dom`
    - _Requirements: 7.2_
  - [x] 5.4 Move remaining WWW crates (batch 2: fallback through print)
    - Run `git mv crates/dx-www-fallback crates/www/fallback`
    - Run `git mv crates/dx-www-form crates/www/form`
    - Run `git mv crates/dx-www-guard crates/www/guard`
    - Run `git mv crates/dx-www-interaction crates/www/interaction`
    - Run `git mv crates/dx-www-morph crates/www/morph`
    - Run `git mv crates/dx-www-offline crates/www/offline`
    - Run `git mv crates/dx-www-packet crates/www/packet`
    - Run `git mv crates/dx-www-print crates/www/print`
    - _Requirements: 7.2_
  - [x] 5.5 Move remaining WWW crates (batch 3: query through sync)
    - Run `git mv crates/dx-www-query crates/www/query`
    - Run `git mv crates/dx-www-rtl crates/www/rtl`
    - Run `git mv crates/dx-www-sched crates/www/sched`
    - Run `git mv crates/dx-www-server crates/www/server`
    - Run `git mv crates/dx-www-state crates/www/state`
    - Run `git mv crates/dx-www-sync crates/www/sync`
    - _Requirements: 7.2_
  - [x] 5.6 Move dx-db-teleport to www/db-teleport
    - Run `git mv crates/dx-db-teleport crates/www/db-teleport`
    - _Requirements: 7.2_
  - [x] 5.7 Update root Cargo.toml for WWW crates
    - Update all member paths to new locations
    - _Requirements: 7.3, 8.1_
  - [x] 5.8 Update inter-crate dependencies for WWW crates
    - Search and update all path dependencies within WWW crates
    - _Requirements: 7.4, 9.1_
  - [x] 5.9 Verify WWW crates build
    - Run `cargo check` on www crates
    - _Requirements: 8.3_

- [x] 6. Checkpoint - Verify Full Workspace Build
  - Ensure all tests pass, ask the user if questions arise.
  - Run `cargo check --workspace`
  - Run `cargo build --workspace`
  - _Requirements: 8.3, 9.3_

- [ ] 7. Property Test: Path Dependencies Valid
  - **Property 2: All Path Dependencies Are Valid**
  - Write script to scan all Cargo.toml files and verify path dependencies point to existing directories
  - **Validates: Requirements 3.3, 4.3, 5.4, 7.4, 9.1**

- [ ] 8. Property Test: Workspace Members Match Structure
  - **Property 3: Workspace Members Match Directory Structure**
  - Write script to verify all members in root Cargo.toml exist as directories
  - **Validates: Requirements 8.1, 8.2**

- [ ] 9. Update Documentation References
  - [ ] 9.1 Update docs/README.md
    - Remove stack reference from performance table if present
    - Update any crate path references
    - _Requirements: 15.3, 15.4_
  - [ ] 9.2 Update docs/api/ files
    - Update crate references to new paths
    - Remove stack.md (already done in 2.2)
    - _Requirements: 15.1, 15.5_
  - [ ] 9.3 Update docs/crates/ files
    - Update any path references
    - _Requirements: 15.1_
  - [ ] 9.4 Search and replace old crate names in all docs
    - Replace `dx-cli` path refs with `cli`
    - Replace `dx-core` path refs with `core`
    - Replace `dx-reactor` path refs with `reactor`
    - Replace `dx-py` path refs with `python`
    - Replace `dx-js-*` path refs with `javascript/*`
    - Replace `dx-www-*` path refs with `www/*`
    - _Requirements: 15.1, 15.5_

- [ ] 10. Audit and Complete Documentation
  - [ ] 10.1 Find all TODO/WIP/FIXME in docs
    - Run `rg -i "TODO|WIP|FIXME|TBD" docs/`
    - List all occurrences
    - _Requirements: 12.1_
  - [ ] 10.2 Complete or remove incomplete documentation
    - Address each TODO/WIP found
    - Either complete the content or remove the placeholder
    - _Requirements: 12.2, 12.5_
  - [ ] 10.3 Verify benchmark claims have methodology
    - Check each claim in docs/reference/benchmarks/
    - Ensure reproducible test instructions exist
    - _Requirements: 11.1, 11.2, 11.3_
  - [ ] 10.4 Clean up archive folder
    - Organize files into appropriate subdirectories
    - Convert or remove improperly formatted .txt files
    - _Requirements: 14.1, 14.3_

- [ ] 11. Property Test: No Incomplete Documentation
  - **Property 6: No Incomplete Documentation**
  - Write script to scan docs/ for TODO, WIP, FIXME, TBD patterns
  - Verify count is zero
  - **Validates: Requirements 12.1, 12.2, 12.5**

- [ ] 12. Property Test: No Stale References
  - **Property 7: No Stale Documentation References**
  - Write script to check for old crate path references in docs
  - Check for references to removed crates (stack)
  - **Validates: Requirements 15.1, 15.2, 15.4, 15.5**

- [ ] 13. Property Test: Documentation Links Valid
  - **Property 8: Documentation Links Are Valid**
  - Write script to verify all internal links in docs/README.md point to existing files
  - **Validates: Requirements 15.3**

- [ ] 14. Run Full Test Suite
  - [ ] 14.1 Run cargo test
    - Run `cargo test --workspace`
    - Ensure all tests pass
    - _Requirements: 13.1_
  - [ ] 14.2 Run benchmark tests if applicable
    - Run any benchmark verification tests
    - _Requirements: 13.2, 11.5_

- [ ] 15. Final Checkpoint - Complete Verification
  - Ensure all tests pass, ask the user if questions arise.
  - Run `cargo check --workspace`
  - Run `cargo test --workspace`
  - Verify no TODO/WIP in docs
  - Verify no old crate references in docs
  - _Requirements: All_

## Notes

- Use `git mv` for all moves to preserve git history
- Package names in Cargo.toml are preserved (e.g., `dx-cli` stays as package name even though folder is `cli`)
- Run `cargo check` frequently to catch issues early
- The reorganization should be done in a single branch and merged atomically
- All property tests are required for comprehensive verification
