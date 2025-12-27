# Implementation Plan: Crates Cleanup

## Overview

Move the `error` and `debug` crates from `crates/` root to `crates/www/` folder where they belong with other www-related crates.

## Tasks

- [x] 1. Move Error Crate to WWW Folder
  - [x] 1.1 Move the error directory
    - Run `git mv crates/error crates/www/error`
    - _Requirements: 1.1_
  - [x] 1.2 Update workspace Cargo.toml members list
    - Change `"crates/error"` to `"crates/www/error"`
    - _Requirements: 1.2_
  - [x] 1.3 Update workspace Cargo.toml dependency path
    - Change `error = { path = "crates/error" }` to `error = { path = "crates/www/error" }`
    - _Requirements: 1.3_
  - [x] 1.4 Update www/client/Cargo.toml path
    - Change `error = { path = "../../error"` to `error = { path = "../error"`
    - _Requirements: 1.4_

- [x] 2. Move Debug Crate to WWW Folder
  - [x] 2.1 Move the debug directory
    - Run `git mv crates/debug crates/www/debug`
    - _Requirements: 2.1_
  - [x] 2.2 Update workspace Cargo.toml members list
    - Change `"crates/debug"` to `"crates/www/debug"`
    - _Requirements: 2.2_
  - [x] 2.3 Update workspace Cargo.toml dependency path
    - Change `debug = { path = "crates/debug" }` to `debug = { path = "crates/www/debug" }`
    - _Requirements: 2.3_
  - [x] 2.4 Update www/client/Cargo.toml path
    - Change `debug = { path = "../../debug"` to `debug = { path = "../debug"`
    - _Requirements: 2.4_

- [x] 3. Verify Workspace Integrity
  - [x] 3.1 Run cargo check
    - Run `cargo check --workspace`
    - _Requirements: 1.5, 2.5, 3.1_
  - [x] 3.2 Run crate tests
    - Run `cargo test -p error -p debug`
    - _Requirements: 3.1, 3.2_

## Notes

- Use `git mv` to preserve git history
- Package names in Cargo.toml remain unchanged (error, debug)
- Both crates are optional dependencies of www/client
