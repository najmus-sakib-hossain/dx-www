# Implementation Plan: DX-Py-Runtime Compilation Fix

## Overview

This implementation plan fixes all 24 compilation errors in dx-py-runtime by updating the dx-py-interpreter crate to use the correct APIs from dx-py-jit and dx-py-reactor. The approach is incremental: fix dependencies first, then JIT integration, then async integration, with verification at each step.

## Tasks

- [x] 1. Fix Missing Dependency
  - [x] 1.1 Add dx-py-reactor dependency to dx-py-interpreter Cargo.toml
    - Add `dx-py-reactor = { path = "../dx-py-reactor" }` to dependencies section
    - _Requirements: 1.1, 1.2_

- [x] 2. Fix JIT Integration Module
  - [x] 2.1 Add FunctionIdMapper helper struct
    - Create struct to map function names to FunctionId
    - Implement `get_or_create` method for consistent ID assignment
    - Use RwLock<HashMap> for thread-safe access
    - Use AtomicU64 for ID generation
    - _Requirements: 5.2_

  - [x] 2.2 Add local tier tracking to JitIntegration
    - Add `tiers: RwLock<HashMap<FunctionId, CompilationTier>>` field
    - Add `func_ids: FunctionIdMapper` field
    - Update constructor to initialize new fields
    - _Requirements: 4.1, 4.2_

  - [x] 2.3 Fix CompilationTier enum variant names
    - Replace `CompilationTier::Tier0` with `CompilationTier::Interpreter`
    - Replace `CompilationTier::Tier1` with `CompilationTier::BaselineJit`
    - Replace `CompilationTier::Tier2` with `CompilationTier::OptimizingJit`
    - Replace `CompilationTier::Tier3` with `CompilationTier::AotOptimized`
    - _Requirements: 3.1, 3.2, 3.3, 3.4_

  - [x] 2.4 Fix record_call method
    - Update to accept bytecode parameter for profile creation
    - Use `profile.get_call_count()` instead of `profile.call_count()`
    - Use local tier tracking instead of `profile.current_tier()`
    - Update tier in local map instead of `profile.set_tier()`
    - _Requirements: 2.2, 2.3, 4.3_

  - [x] 2.5 Fix get_tier method
    - Use local tier tracking with FunctionIdMapper
    - Return `CompilationTier::Interpreter` as default
    - _Requirements: 4.1_

  - [x] 2.6 Fix compile method
    - Convert function name to FunctionId using mapper
    - Call `jit.compile(func_id, tier, bytecode)` with 3 arguments
    - Handle `Option<*const u8>` return type instead of Result
    - _Requirements: 5.1, 5.2, 5.3_

  - [x] 2.7 Fix has_compiled method
    - Use `jit.get_compiled(func_id).is_some()` instead of `jit.has_compiled()`
    - _Requirements: 6.1, 6.3_

  - [x] 2.8 Fix deoptimize method
    - Use `jit.invalidate(func_id)` instead of `jit.deoptimize()`
    - Reset tier to Interpreter in local tracking
    - _Requirements: 6.2, 6.4_

  - [x] 2.9 Fix OSR methods (can_osr and do_osr)
    - Use `osr.get_entry(func_id, offset).is_some()` for can_osr
    - Remove call to non-existent `osr.transition()` method
    - Handle OSR entry retrieval for do_osr
    - _Requirements: 7.1, 7.2, 7.3, 7.4_

  - [x] 2.10 Fix stats method
    - Use local tier tracking for tier counts
    - Use `profile.get_call_count()` for call counts
    - Use correct CompilationTier variant names in match
    - _Requirements: 3.1, 3.2, 3.3, 3.4_

  - [x] 2.11 Write property test for FunctionIdMapper consistency
    - **Property 1: Function ID Mapping Consistency**
    - **Validates: Requirements 5.2**

  - [x] 2.12 Write property test for tier tracking
    - **Property 2: Tier Tracking Consistency**
    - **Property 3: Tier Promotion Monotonicity**
    - **Validates: Requirements 4.1, 4.2, 4.3**

- [x] 3. Checkpoint - Verify JIT Integration Compiles
  - Ensure `cargo build -p dx-py-interpreter` succeeds for JIT-related code
  - Ask the user if questions arise

- [x] 4. Fix Async Integration Module
  - [x] 4.1 Update imports to use dx-py-reactor types
    - Import Reactor, ReactorPool, PyFuture, ReactorStats from dx-py-reactor
    - Import IoBuffer, IoOperation if needed
    - _Requirements: 8.1, 8.2_

  - [x] 4.2 Fix AsyncRuntime init method
    - Use `create_reactor(0)` or appropriate ReactorPool constructor
    - Handle ReactorError properly
    - _Requirements: 8.1_

  - [x] 4.3 Fix PyFuture usage
    - Adapt to actual PyFuture API methods
    - Update `is_complete()`, `is_cancelled()`, `error()`, `result()` calls if needed
    - _Requirements: 8.2_

  - [x] 4.4 Fix or stub read_file and write_file methods
    - If ReactorPool has submit methods, use them
    - Otherwise, use lower-level Reactor API with IoOperation
    - Or stub with NotImplemented error for now
    - _Requirements: 8.3_

  - [x] 4.5 Fix stats method
    - Use correct ReactorStats API from dx-py-reactor
    - _Requirements: 8.1_

- [x] 5. Checkpoint - Verify Async Integration Compiles
  - Ensure `cargo build -p dx-py-interpreter` succeeds for async-related code
  - Ask the user if questions arise

- [x] 6. Update Tests
  - [x] 6.1 Update JIT integration tests
    - Fix test_tier_promotion to use correct tier names
    - Fix test_stats to use correct tier names
    - Update any tests using old API
    - _Requirements: 10.2_

  - [x] 6.2 Update async integration tests
    - Ensure tests compile with new imports
    - _Requirements: 10.2_

- [x] 7. Final Verification
  - [x] 7.1 Run full workspace build
    - Execute `cargo build --release` in dx-py-runtime directory
    - Verify zero compilation errors
    - _Requirements: 9.1, 9.3_

  - [x] 7.2 Run all library tests
    - Execute `cargo test --lib` in dx-py-runtime directory
    - Verify all tests pass
    - _Requirements: 9.2, 10.1, 10.2_

  - [x] 7.3 Run dx-py-jit tests specifically
    - Execute `cargo test --lib -p dx-py-jit`
    - Verify all JIT tests still pass
    - _Requirements: 10.1_

  - [x] 7.4 Run dx-py-interpreter tests specifically
    - Execute `cargo test --lib -p dx-py-interpreter`
    - Verify all interpreter tests pass
    - _Requirements: 10.2_

- [x] 8. Final Checkpoint
  - Ensure all tests pass, ask the user if questions arise

## Notes

- All tasks are required for comprehensive testing
- Each task references specific requirements for traceability
- Checkpoints ensure incremental validation
- Property tests validate universal correctness properties
- Unit tests validate specific examples and edge cases
- The JIT integration is fixed first since it has more errors (20+)
- Async integration is fixed second (4 errors related to missing import)
