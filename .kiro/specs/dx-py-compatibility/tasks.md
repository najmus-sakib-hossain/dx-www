# Implementation Plan: dx-py-compatibility

## Overview

This implementation plan covers creating the dx-py-compability crate with full Python runtime and uv configuration support, and reorganizing the crate structure by renaming dx-py-project-manager to dx-py-workspace.

## Tasks

- [x] 1. Reorganize crate structure
  - [x] 1.1 Rename dx-py-project-manager to dx-py-workspace
    - Copy `crates/dx-py/crates/dx-py-package-manager/dx-py-project-manager/` to `crates/dx-py/crates/dx-py-workspace/`
    - Update Cargo.toml package name from "dx-py-project-manager" to "dx-py-workspace"
    - Update all internal module references
    - _Requirements: 5.1, 5.2, 5.3, 5.4_
  - [x] 1.2 Update dependent crates
    - Update dx-py-package-manager/Cargo.toml to remove dx-py-project-manager dependency
    - Update any crates that depend on dx-py-project-manager to use dx-py-workspace
    - Update workspace Cargo.toml members list
    - _Requirements: 5.5_
  - [x] 1.3 Remove old dx-py-project-manager from dx-py-package-manager
    - Delete `crates/dx-py/crates/dx-py-package-manager/dx-py-project-manager/` directory
    - Verify dx-py-package-manager builds without the subcrate
    - _Requirements: 8.5_

- [-] 2. Create dx-py-compability crate skeleton
  - [-] 2.1 Initialize crate structure
    - Create `crates/dx-py/crates/dx-py-compability/Cargo.toml` with dependencies
    - Create `src/lib.rs` with module declarations
    - Create module directories: runtime/, uv/, markers/, platform/, venv/, config/
    - _Requirements: 8.2, 8.3, 8.4_
  - [ ] 2.2 Define core types
    - Implement PythonVersion with PEP 440 support
    - Implement Architecture and InstallationSource enums
    - Implement error types (DetectionError, ConfigError, MarkerError, VenvError)
    - _Requirements: 1.2, 1.4_

- [ ] 3. Implement Python runtime detection
  - [ ] 3.1 Implement RuntimeDetector
    - Create detector.rs with search path logic
    - Implement detection for system, pyenv, conda, homebrew, Windows Store locations
    - Implement Python executable discovery and validation
    - _Requirements: 1.1, 1.5_
  - [ ] 3.2 Implement version extraction
    - Create version.rs with PythonVersion parsing
    - Implement version range validation (3.8-3.13)
    - Extract version by executing `python --version`
    - _Requirements: 1.2, 1.3_
  - [ ] 3.3 Implement capability detection
    - Create capabilities.rs
    - Detect pip, venv, ssl, sqlite availability
    - Extract ABI tag from Python
    - _Requirements: 1.2_
  - [ ] 3.4 Write property test for Python version validation
    - **Property 2: Python Version Range Validation**
    - **Validates: Requirements 1.3, 1.4**

- [ ] 4. Implement uv configuration support
  - [ ] 4.1 Implement UvConfig types
    - Create config.rs with UvConfig struct
    - Implement PythonPreference enum
    - Add serde derive for TOML serialization
    - _Requirements: 2.3, 2.5_
  - [ ] 4.2 Implement UvConfigLoader
    - Implement load_uv_toml() for uv.toml parsing
    - Implement load_pyproject_uv() for [tool.uv] section
    - Implement merge_with_dxpy() with precedence logic
    - _Requirements: 2.1, 2.2, 2.4_
  - [ ] 4.3 Write property test for uv config parsing
    - **Property 3: uv Configuration Parsing Completeness**
    - **Validates: Requirements 2.1, 2.2, 2.3, 2.5**
  - [ ] 4.4 Write property test for configuration precedence
    - **Property 4: Configuration Precedence**
    - **Validates: Requirements 2.4**

- [ ] 5. Checkpoint - Verify runtime and uv modules
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 6. Implement PEP 508 marker evaluation
  - [ ] 6.1 Implement MarkerEnvironment
    - Create evaluator.rs with MarkerEnvironment struct
    - Implement current() for system detection
    - Implement from_runtime() for Python-specific environment
    - _Requirements: 3.2_
  - [ ] 6.2 Implement marker parser
    - Create parser.rs with MarkerExpr AST
    - Implement tokenizer for marker expressions
    - Implement recursive descent parser
    - Support all operators: ==, !=, <, <=, >, >=, ~=, in, not in
    - _Requirements: 3.1, 3.3_
  - [ ] 6.3 Implement MarkerEvaluator
    - Implement evaluate() method
    - Add LRU cache for evaluation results
    - Return MarkerError with position for invalid expressions
    - _Requirements: 3.1, 3.4, 3.5_
  - [ ] 6.4 Write property test for marker evaluation
    - **Property 5: Marker Evaluation Correctness**
    - **Validates: Requirements 3.1, 3.2, 3.3**
  - [ ] 6.5 Write property test for marker caching
    - **Property 6: Marker Evaluation Caching**
    - **Validates: Requirements 3.5**

- [ ] 7. Implement platform detection and wheel tags
  - [ ] 7.1 Implement PlatformDetector
    - Create detector.rs with Platform struct
    - Detect OS, architecture, ABI
    - Detect libc type (glibc/musl) on Linux
    - _Requirements: 4.1_
  - [ ] 7.2 Implement WheelTagGenerator
    - Create wheel_tags.rs with WheelTag struct
    - Generate compatible tags in priority order
    - Implement is_compatible() and select_best()
    - _Requirements: 4.2, 4.3_
  - [ ] 7.3 Implement manylinux/musllinux support
    - Create manylinux.rs
    - Parse manylinux1, manylinux2010, manylinux2014, manylinux_x_y tags
    - Parse musllinux_x_y tags
    - _Requirements: 4.4, 4.5_
  - [ ] 7.4 Write property test for wheel tag ordering
    - **Property 7: Wheel Tag Priority Ordering**
    - **Validates: Requirements 4.2, 4.3**
  - [ ] 7.5 Write property test for Linux wheel tags
    - **Property 8: Linux Wheel Tag Support**
    - **Validates: Requirements 4.4, 4.5**

- [ ] 8. Checkpoint - Verify markers and platform modules
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 9. Implement virtual environment creation
  - [ ] 9.1 Implement VenvBuilder
    - Create builder.rs with VenvBuilder struct
    - Implement VenvOptions for configuration
    - Create directory structure per PEP 405
    - _Requirements: 6.1_
  - [ ] 9.2 Implement pyvenv.cfg generation
    - Create pyvenv_cfg.rs
    - Generate home, include-system-site-packages, version fields
    - Support additional optional fields
    - _Requirements: 6.2_
  - [ ] 9.3 Implement activation script generation
    - Create activation.rs
    - Generate scripts for bash, zsh, fish, csh, PowerShell
    - Handle platform-specific path separators
    - _Requirements: 6.3_
  - [ ] 9.4 Implement Python interpreter linking
    - Implement symlink creation on Unix
    - Implement copy on Windows or when --copies specified
    - Handle --system-site-packages option
    - _Requirements: 6.4, 6.5_
  - [ ] 9.5 Write property test for venv compliance
    - **Property 9: Virtual Environment PEP 405 Compliance**
    - **Validates: Requirements 6.1, 6.2, 6.3, 6.4, 6.5**

- [ ] 10. Implement configuration serialization
  - [ ] 10.1 Implement DxPyConfig types
    - Create types.rs with DxPyConfig struct
    - Add serde derives for TOML support
    - Implement Default trait
    - _Requirements: 7.1, 7.2_
  - [ ] 10.2 Implement validation
    - Create serde.rs with custom deserializers
    - Validate field values during deserialization
    - Return descriptive validation errors
    - _Requirements: 7.4, 7.5_
  - [ ] 10.3 Write property test for config round-trip
    - **Property 10: Configuration Round-Trip**
    - **Validates: Requirements 7.1, 7.2, 7.3**
  - [ ] 10.4 Write property test for config validation
    - **Property 11: Configuration Validation**
    - **Validates: Requirements 7.4, 7.5**

- [ ] 11. Final integration and verification
  - [ ] 11.1 Wire all modules together
    - Export public API from lib.rs
    - Add re-exports for common types
    - Write module documentation
    - _Requirements: 8.1, 8.2_
  - [ ] 11.2 Verify final crate structure
    - Confirm all 5 crates exist: dx-py-runtime, dx-py-package-manager, dx-py-workspace, dx-py-test-runner, dx-py-compability
    - Verify each has Cargo.toml and src/lib.rs
    - Run cargo check on workspace
    - _Requirements: 8.1, 8.2, 8.3, 8.4_
  - [ ] 11.3 Write integration tests
    - Test full runtime detection flow
    - Test venv creation with real Python
    - Test marker evaluation against current platform

- [ ] 12. Final checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

## Notes

- All tasks are required for comprehensive implementation
- Each task references specific requirements for traceability
- Checkpoints ensure incremental validation
- Property tests validate universal correctness properties using proptest
- Unit tests validate specific examples and edge cases
