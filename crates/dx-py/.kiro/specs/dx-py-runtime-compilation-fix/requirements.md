# Requirements Document

## Introduction

This specification addresses the compilation errors in the dx-py-runtime project that prevent the runtime from building. The dx-py-runtime is a high-performance Python runtime written in Rust, but the `dx-py-interpreter` crate has 24 compilation errors due to API mismatches between the interpreter and its dependencies (dx-py-jit, dx-py-reactor). This document defines the requirements to fix all compilation errors and enable successful builds.

## Glossary

- **Interpreter**: The `dx-py-interpreter` crate that executes Python bytecode
- **JIT_Integration**: The module in dx-py-interpreter that interfaces with the JIT compiler
- **Async_Integration**: The module in dx-py-interpreter that interfaces with the async reactor
- **FunctionProfile**: A struct in dx-py-jit that tracks function execution statistics
- **CompilationTier**: An enum in dx-py-jit representing JIT compilation levels
- **TieredJit**: The main JIT compiler struct in dx-py-jit
- **OsrManager**: On-Stack Replacement manager in dx-py-jit for hot loop optimization
- **Reactor**: The async I/O reactor trait in dx-py-reactor
- **PyFuture**: Python-compatible future type in dx-py-reactor

## Requirements

### Requirement 1: Fix Missing dx-py-reactor Dependency

**User Story:** As a developer, I want the dx-py-interpreter to properly depend on dx-py-reactor, so that async integration can compile.

#### Acceptance Criteria

1. WHEN the dx-py-interpreter crate is compiled, THE Build_System SHALL resolve the dx-py-reactor dependency
2. THE Cargo.toml for dx-py-interpreter SHALL include dx-py-reactor as a dependency with the correct path

### Requirement 2: Fix FunctionProfile API Mismatch

**User Story:** As a developer, I want the JIT integration to use the correct FunctionProfile API, so that function profiling works correctly.

#### Acceptance Criteria

1. WHEN creating a new FunctionProfile, THE JIT_Integration SHALL use the correct constructor signature `FunctionProfile::new(bytecode_len: usize, branch_count: usize)`
2. WHEN accessing call count, THE JIT_Integration SHALL use `get_call_count()` method instead of `call_count()` method
3. WHEN recording a function call, THE JIT_Integration SHALL use the `record_call()` method

### Requirement 3: Fix CompilationTier Enum Variants

**User Story:** As a developer, I want the JIT integration to use the correct CompilationTier variants, so that tier promotion logic works correctly.

#### Acceptance Criteria

1. WHEN referencing compilation tiers, THE JIT_Integration SHALL use `CompilationTier::Interpreter` instead of `CompilationTier::Tier0`
2. WHEN referencing compilation tiers, THE JIT_Integration SHALL use `CompilationTier::BaselineJit` instead of `CompilationTier::Tier1`
3. WHEN referencing compilation tiers, THE JIT_Integration SHALL use `CompilationTier::OptimizingJit` instead of `CompilationTier::Tier2`
4. WHEN referencing compilation tiers, THE JIT_Integration SHALL use `CompilationTier::AotOptimized` instead of `CompilationTier::Tier3`

### Requirement 4: Fix FunctionProfile Tier Management

**User Story:** As a developer, I want the JIT integration to properly track and update compilation tiers, so that tier promotion decisions are correct.

#### Acceptance Criteria

1. WHEN the JIT_Integration needs to track current tier, THE JIT_Integration SHALL store tier information locally since FunctionProfile does not have `current_tier()` or `set_tier()` methods
2. THE JIT_Integration SHALL maintain a separate mapping of function names to their current compilation tier
3. WHEN a function is promoted to a new tier, THE JIT_Integration SHALL update the local tier mapping

### Requirement 5: Fix TieredJit::compile Method Signature

**User Story:** As a developer, I want the JIT integration to call the compile method correctly, so that JIT compilation can be triggered.

#### Acceptance Criteria

1. WHEN calling TieredJit::compile, THE JIT_Integration SHALL provide three arguments: `FunctionId`, `CompilationTier`, and `&[u8]` bytecode
2. WHEN calling TieredJit::compile, THE JIT_Integration SHALL convert function names to FunctionId using a consistent mapping
3. WHEN handling compile results, THE JIT_Integration SHALL handle `Option<*const u8>` return type instead of `Result`

### Requirement 6: Remove Non-Existent TieredJit Methods

**User Story:** As a developer, I want the JIT integration to only use methods that exist on TieredJit, so that compilation succeeds.

#### Acceptance Criteria

1. THE JIT_Integration SHALL NOT call `has_compiled()` method on TieredJit since it does not exist
2. THE JIT_Integration SHALL NOT call `deoptimize()` method on TieredJit since it does not exist
3. THE JIT_Integration SHALL use `get_compiled()` to check if a function has compiled code
4. THE JIT_Integration SHALL use `invalidate()` to trigger deoptimization

### Requirement 7: Fix OsrManager Method Calls

**User Story:** As a developer, I want the JIT integration to use the correct OsrManager API, so that on-stack replacement works correctly.

#### Acceptance Criteria

1. THE JIT_Integration SHALL NOT call `can_osr(func_name, offset)` since OsrManager uses `FunctionId` not string names
2. THE JIT_Integration SHALL NOT call `transition(func_name, offset)` since this method does not exist
3. THE JIT_Integration SHALL use `get_entry(FunctionId, loop_header)` to check for OSR availability
4. THE JIT_Integration SHALL use `is_hot(iteration_count)` to determine if a loop is hot enough for OSR

### Requirement 8: Fix Async Integration Reactor API

**User Story:** As a developer, I want the async integration to use the correct reactor API, so that async I/O works correctly.

#### Acceptance Criteria

1. WHEN using ReactorPool, THE Async_Integration SHALL use the correct constructor and methods from dx-py-reactor
2. WHEN using PyFuture, THE Async_Integration SHALL use the correct methods for checking completion and getting results
3. IF ReactorPool does not have `submit_read` or `submit_write` methods, THEN THE Async_Integration SHALL use the appropriate alternative API

### Requirement 9: Successful Compilation

**User Story:** As a developer, I want the entire dx-py-runtime workspace to compile without errors, so that I can build and test the runtime.

#### Acceptance Criteria

1. WHEN running `cargo build --release` in the dx-py-runtime directory, THE Build_System SHALL complete without compilation errors
2. WHEN running `cargo test --lib` in the dx-py-runtime directory, THE Test_Runner SHALL execute all tests successfully
3. THE dx-py-interpreter crate SHALL compile with all its dependencies resolved

### Requirement 10: Maintain Test Coverage

**User Story:** As a developer, I want existing tests to continue passing after the fixes, so that functionality is preserved.

#### Acceptance Criteria

1. WHEN running tests for dx-py-jit, THE Test_Runner SHALL report all tests passing
2. WHEN running tests for dx-py-interpreter, THE Test_Runner SHALL report all tests passing
3. IF any test fails after the fixes, THEN THE Developer SHALL update the test to match the corrected API
