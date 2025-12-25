# Implementation Plan: DX-Py Hardening & Production Readiness

## Overview

This implementation plan transforms dx-py from a functional prototype into a battle-tested, production-ready Python package manager. The work is organized into phases: core parsing, platform support, real integrations, and comprehensive testing.

## Tasks

- [x] 1. Implement PEP 440 Version Parser (dx-py-core)
  - [x] 1.1 Create Pep440Version struct with all components
    - Epoch, release segments, pre/post/dev, local
    - Implement Display trait for formatting
    - _Requirements: 2.1, 2.2, 2.3, 2.4, 2.5_

  - [x] 1.2 Implement PEP 440 parsing with regex
    - Parse epoch (N!)
    - Parse release segments (N.N.N...)
    - Parse pre-release (aN, bN, rcN, alphaN, betaN)
    - Parse post-release (.postN)
    - Parse dev release (.devN)
    - Parse local version (+local)
    - _Requirements: 2.1, 2.2, 2.3, 2.4, 2.5_

  - [x] 1.3 Implement PEP 440 version ordering
    - Epoch comparison (higher epoch wins)
    - Release segment comparison
    - Pre-release ordering (dev < alpha < beta < rc < release)
    - Post-release ordering
    - Local version comparison
    - _Requirements: 2.6_

  - [x]* 1.4 Write property test for version round-trip
    - **Property 1: PEP 440 Version Round-Trip**
    - **Validates: Requirements 2.1, 2.2, 2.3, 2.4, 2.5, 2.7**

  - [x]* 1.5 Write property test for version ordering
    - **Property 2: PEP 440 Version Ordering**
    - **Validates: Requirements 2.6**

- [x] 2. Implement Environment Marker Evaluator (dx-py-compat)
  - [x] 2.1 Create MarkerEnvironment struct
    - Detect python_version, sys_platform, platform_system
    - Detect platform_machine, implementation_name
    - Create current() constructor
    - _Requirements: 3.1, 3.2, 3.3, 3.4_

  - [x] 2.2 Implement marker expression parser
    - Parse comparison operators (==, !=, <, >, <=, >=, ~=, ===)
    - Parse variable names (python_version, sys_platform, etc.)
    - Parse string literals
    - Parse boolean operators (and, or)
    - Parse parentheses for grouping
    - _Requirements: 3.1, 3.2, 3.3, 3.4, 3.5, 3.6_

  - [x] 2.3 Implement marker evaluation
    - Evaluate comparisons against environment
    - Evaluate extra markers
    - Evaluate boolean expressions
    - _Requirements: 3.1, 3.2, 3.3, 3.4, 3.5, 3.6, 3.7_

  - [x]* 2.4 Write property test for marker evaluation
    - **Property 4: Marker Evaluation Consistency**
    - **Validates: Requirements 3.1, 3.2, 3.3, 3.4, 3.5, 3.6**

- [x] 3. Checkpoint - Core parsing complete
  - Ensure all tests pass, ask the user if questions arise.

- [x] 4. Implement Wheel Tag Parser and Selector (dx-py-core)
  - [x] 4.1 Create WheelTag struct and parser
    - Parse wheel filename format: {name}-{version}(-{build})?-{python}-{abi}-{platform}.whl
    - Handle multiple tags (e.g., py2.py3-none-any)
    - _Requirements: 4.4_

  - [x] 4.2 Create PlatformEnvironment detection
    - Detect OS (Windows, macOS, Linux)
    - Detect architecture (x86_64, aarch64, arm64)
    - Detect Python implementation and version
    - Detect manylinux compatibility
    - _Requirements: 4.1, 4.2, 4.3_

  - [x] 4.3 Implement wheel compatibility checking
    - Check Python version compatibility
    - Check ABI compatibility
    - Check platform compatibility
    - _Requirements: 4.4_

  - [x] 4.4 Implement wheel selection priority
    - Score wheels by specificity
    - Prefer platform-specific over universal
    - Prefer newer manylinux over older
    - _Requirements: 4.5, 4.6_

  - [x]* 4.5 Write property test for wheel tag parsing
    - **Property 5: Wheel Tag Parsing**
    - **Validates: Requirements 4.4**

  - [x]* 4.6 Write property test for wheel selection priority
    - **Property 6: Wheel Selection Priority**
    - **Validates: Requirements 4.5, 4.6**

- [x] 5. Implement Enhanced PEP 508 Parser (dx-py-package-manager)
  - [x] 5.1 Enhance DependencySpec parser
    - Parse URL dependencies (package @ url)
    - Parse path dependencies (package @ file://path)
    - Parse extras correctly
    - Parse version constraints with PEP 440
    - Parse markers with new evaluator
    - _Requirements: 1.2_

  - [x]* 5.2 Write property test for dependency parsing round-trip
    - **Property 3: PEP 508 Dependency Parsing Round-Trip**
    - **Validates: Requirements 1.2**

- [x] 6. Checkpoint - Parsing complete
  - Ensure all tests pass, ask the user if questions arise.

- [x] 7. Implement Async Download Manager (dx-py-package-manager)
  - [x] 7.1 Add tokio and async reqwest dependencies
    - Update Cargo.toml with async runtime
    - _Requirements: 6.1_

  - [x] 7.2 Create DownloadManager struct
    - Configure max concurrent downloads (default 8)
    - Configure retry count and delay
    - Configure timeouts
    - _Requirements: 1.5, 1.6, 6.1, 6.4_

  - [x] 7.3 Implement download with retry
    - Exponential backoff on failure
    - SHA256 verification
    - Progress callback support
    - _Requirements: 1.3, 1.5_

  - [x] 7.4 Implement parallel download
    - Use tokio::spawn for concurrent downloads
    - Respect max_concurrent limit
    - Aggregate results
    - _Requirements: 1.6, 6.1_

  - [x]* 7.5 Write property test for SHA256 verification
    - **Property 7: SHA256 Verification**
    - **Validates: Requirements 1.3, 8.2**

- [x] 8. Implement Real PyPI Client (dx-py-package-manager)
  - [x] 8.1 Create AsyncPyPiClient
    - Async HTTP client with connection pooling
    - Support for extra index URLs
    - User-Agent header
    - _Requirements: 1.1, 11.4, 11.5_

  - [x] 8.2 Implement package metadata fetching
    - Fetch from /pypi/{name}/json
    - Parse releases and files
    - Cache metadata in memory
    - _Requirements: 1.1_

  - [x] 8.3 Implement wheel selection with platform awareness
    - Use WheelTag parser
    - Use PlatformEnvironment
    - Select best compatible wheel
    - Fall back to sdist if no wheel
    - _Requirements: 4.5, 4.6, 4.7_

- [x] 9. Implement Real Resolver (dx-py-package-manager)
  - [x] 9.1 Create PyPiResolver
    - Integrate AsyncPyPiClient
    - Integrate MarkerEvaluator
    - Integrate Pep440Version
    - _Requirements: 1.1, 1.2_

  - [x] 9.2 Implement resolution with marker filtering
    - Evaluate markers during resolution
    - Skip dependencies with false markers
    - Handle extras correctly
    - _Requirements: 3.7_

  - [x] 9.3 Implement resolution with real PyPI data
    - Fetch versions from PyPI
    - Fetch dependencies for each version
    - Use PubGrub algorithm
    - _Requirements: 1.1_

- [x] 10. Checkpoint - Network integration complete
  - Ensure all tests pass, ask the user if questions arise.

- [x] 11. Implement Real Wheel Installer (dx-py-package-manager)
  - [x] 11.1 Implement wheel extraction
    - Extract wheel ZIP to site-packages
    - Handle .data directory (scripts, headers, etc.)
    - Create .dist-info directory
    - _Requirements: 1.4_

  - [x] 11.2 Implement RECORD file handling
    - Parse RECORD file for file list
    - Verify file hashes during extraction
    - Write updated RECORD after install
    - _Requirements: 1.4_

  - [x] 11.3 Implement script wrapper generation
    - Generate entry point scripts
    - Handle console_scripts and gui_scripts
    - Platform-specific script format
    - _Requirements: 1.4_

  - [x] 11.4 Implement uninstall
    - Read RECORD file
    - Remove all installed files
    - Remove .dist-info directory
    - _Requirements: 1.4_

- [x] 12. Implement Real Virtual Environment Manager (dx-py-project-manager)
  - [x] 12.1 Implement real venv creation
    - Use Python's venv module or manual creation
    - Copy/symlink Python interpreter
    - Create pyvenv.cfg
    - _Requirements: 7.1, 7.2_

  - [x] 12.2 Implement activation script generation
    - Generate bash activation script
    - Generate zsh activation script
    - Generate fish activation script
    - Generate PowerShell activation script
    - _Requirements: 7.3_

  - [x] 12.3 Implement pip/setuptools bootstrap
    - Download pip wheel from PyPI
    - Install pip into venv
    - Install setuptools into venv
    - _Requirements: 7.4_

  - [x]* 12.4 Write property test for activation script validity
    - **Property 11: Activation Script Validity**
    - **Validates: Requirements 7.3**

- [x] 13. Checkpoint - Installation complete
  - Ensure all tests pass, ask the user if questions arise.

- [x] 14. Implement Real Python Manager (dx-py-project-manager)
  - [x] 14.1 Implement python-build-standalone API client
    - Fetch releases from GitHub API
    - Parse release assets for platform/arch
    - _Requirements: 8.5_

  - [x] 14.2 Implement Python download and extraction
    - Download release archive
    - Verify SHA256
    - Extract to managed directory
    - _Requirements: 8.1, 8.2, 8.3_

  - [x] 14.3 Implement cross-platform support
    - Windows: .zip archives
    - macOS: .tar.gz archives (Intel and ARM)
    - Linux: .tar.gz archives
    - _Requirements: 8.4_

- [ ] 15. Implement Configuration System (dx-py-compat)
  - [ ] 15.1 Create Config struct
    - Define all configuration options
    - Set sensible defaults
    - _Requirements: 11.1, 11.2, 11.3_

  - [ ] 15.2 Implement config loading
    - Load from environment variables
    - Load from ~/.config/dx-py/config.toml
    - Load from pyproject.toml [tool.dx-py]
    - Merge with priority (env > project > global > default)
    - _Requirements: 11.1, 11.2, 11.3, 11.4, 11.5, 11.6_

  - [ ]* 15.3 Write property test for configuration layering
    - **Property 9: Configuration Layering**
    - **Validates: Requirements 11.1, 11.3**

- [ ] 16. Implement Real Build System (dx-py-cli)
  - [ ] 16.1 Implement PEP 517 build frontend
    - Parse build-system from pyproject.toml
    - Create isolated build environment
    - Call build backend hooks
    - _Requirements: 9.1, 9.2, 9.3_

  - [ ] 16.2 Implement wheel building
    - Call build_wheel hook
    - Copy wheel to output directory
    - _Requirements: 9.1_

  - [ ] 16.3 Implement sdist building
    - Call build_sdist hook
    - Copy sdist to output directory
    - _Requirements: 9.2_

- [ ] 17. Implement Real Publish System (dx-py-cli)
  - [ ] 17.1 Implement PyPI upload
    - Use multipart form upload
    - Support API token authentication
    - Support custom repository URLs
    - _Requirements: 9.4, 9.5, 9.6_

- [ ] 18. Implement Real Tool Manager (dx-py-project-manager)
  - [ ] 18.1 Implement tool installation
    - Create isolated venv for tool
    - Install tool package
    - Create wrapper scripts
    - _Requirements: 10.1, 10.2, 10.3_

  - [ ] 18.2 Implement ephemeral tool execution
    - Create temporary venv
    - Install tool
    - Run command
    - Clean up
    - _Requirements: 10.5_

  - [ ] 18.3 Implement tool upgrade
    - Upgrade package in tool venv
    - _Requirements: 10.6_

- [ ] 19. Checkpoint - All features complete
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 20. Implement Robust Error Handling
  - [ ] 20.1 Implement atomic file operations
    - Write to temp file, then rename
    - Clean up on failure
    - _Requirements: 5.6_

  - [ ] 20.2 Implement cleanup on failure
    - Remove partial cache entries
    - Remove partial venv directories
    - Roll back partial installs
    - _Requirements: 5.6_

  - [ ]* 20.3 Write property test for cleanup on failure
    - **Property 8: Cleanup on Failure**
    - **Validates: Requirements 5.6**

- [ ] 21. Implement Enhanced Workspace Support (dx-py-project-manager)
  - [ ] 21.1 Implement workspace member enumeration
    - Parse workspace.members glob patterns
    - Enumerate matching directories
    - _Requirements: 12.1, 12.2_

  - [ ] 21.2 Implement path dependency resolution
    - Detect path dependencies
    - Resolve relative paths
    - Install in editable mode
    - _Requirements: 12.4, 12.5_

  - [ ]* 21.3 Write property test for workspace enumeration
    - **Property 10: Workspace Member Enumeration**
    - **Validates: Requirements 12.1, 12.2**

- [ ] 22. Update CLI Commands (dx-py-cli)
  - [ ] 22.1 Update lock command
    - Use real PyPiResolver
    - Write real lock file with hashes
    - _Requirements: 1.1_

  - [ ] 22.2 Update sync command
    - Use real WheelInstaller
    - Download from cache or PyPI
    - _Requirements: 1.4_

  - [ ] 22.3 Update run command
    - Properly activate venv
    - Handle PATH correctly
    - _Requirements: 7.5_

  - [ ] 22.4 Update python commands
    - Use real PythonManager
    - Actually download Python
    - _Requirements: 8.1, 8.2, 8.3_

  - [ ] 22.5 Update tool commands
    - Use real ToolManager
    - Actually install tools
    - _Requirements: 10.1, 10.2, 10.5_

  - [ ] 22.6 Update build command
    - Use real BuildSystem
    - Actually build packages
    - _Requirements: 9.1, 9.2_

  - [ ] 22.7 Update publish command
    - Use real PublishClient
    - Actually upload packages
    - _Requirements: 9.4_

- [ ] 23. Checkpoint - CLI integration complete
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 24. Integration Tests
  - [ ] 24.1 Add integration test for real PyPI resolution
    - Resolve requests package
    - Verify transitive dependencies
    - _Requirements: 13.1_

  - [ ] 24.2 Add integration test for wheel installation
    - Download and install a real package
    - Verify files in site-packages
    - _Requirements: 13.1_

  - [ ] 24.3 Add integration test for venv creation
    - Create venv with real Python
    - Run Python in venv
    - _Requirements: 13.2_

- [ ] 25. Final Checkpoint - Production Ready
  - All tests pass
  - All property tests pass
  - Integration tests pass
  - Documentation updated

## Notes

- Tasks marked with `*` are property-based tests
- Each task references specific requirements for traceability
- Checkpoints ensure incremental validation
- Integration tests require network access
- Cross-platform testing requires CI/CD setup
