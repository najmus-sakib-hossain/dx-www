# Implementation Plan: DX-Py Package Manager

## Overview

This implementation plan builds DX-Py, a high-performance Python package manager that is 5-50x faster than uv. The implementation follows an incremental approach, starting with core binary formats and building up to the full CLI. Each task builds on previous work, ensuring no orphaned code.

## Tasks

- [ ] 1. Set up project structure and core crate
  - Create `crates/dx-py-package-manager/` directory structure
  - Create `dx-py-core` crate with Cargo.toml
  - Define magic numbers, protocol version, and security limits
  - Set up error types with thiserror
  - _Requirements: 1.1, 2.1_

- [ ] 2. Implement DPP binary format (dx-py-core)
  - [ ] 2.1 Define DppHeader struct with #[repr(C, packed)]
    - 64-byte fixed header with magic, version, flags
    - Section offsets: metadata, files, bytecode, native, deps
    - Size fields and BLAKE3 hash
    - _Requirements: 1.1, 1.5_

  - [ ] 2.2 Define DppMetadata struct for package metadata
    - Name, version, python_requires fields
    - Variable-length string handling
    - _Requirements: 1.3, 1.4_

  - [ ] 2.3 Write property test for DPP format structure validity
    - **Property 1: DPP Format Structure Validity**
    - **Validates: Requirements 1.1, 1.3, 1.4, 1.5**

- [ ] 3. Implement DPL binary format (dx-py-core)
  - [ ] 3.1 Define DplHeader struct with #[repr(C, packed)]
    - Hash table offset and size for O(1) lookup
    - Package count, Python version, platform metadata
    - Content hash for integrity
    - _Requirements: 2.1, 2.4_

  - [ ] 3.2 Define DplEntry struct (fixed 128 bytes)
    - Name hash, name, version, source type
    - Source integrity hash
    - _Requirements: 2.3_

  - [ ] 3.3 Write property test for DPL format structure validity
    - **Property 3: DPL Format Structure Validity**
    - **Validates: Requirements 2.1, 2.3, 2.4**

- [ ] 4. Implement version types and SIMD comparison (dx-py-core)
  - [ ] 4.1 Define PackedVersion struct for SIMD operations
    - Major, minor, patch as u32 fields
    - Implement Ord, PartialOrd traits
    - _Requirements: 4.1_

  - [ ] 4.2 Implement SIMD version comparison (AVX2)
    - compare_versions_simd function with target_feature
    - Process 8 versions in parallel
    - _Requirements: 4.1_

  - [ ] 4.3 Implement scalar fallback for non-AVX2 systems
    - compare_versions_scalar function
    - Same logic, sequential processing
    - _Requirements: 4.2_

  - [ ] 4.4 Write property test for SIMD/scalar equivalence
    - **Property 5: SIMD/Scalar Version Resolution Equivalence**
    - **Validates: Requirements 4.5, 4.6**

- [ ] 5. Checkpoint - Core types complete
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 6. Implement DPP package reader (dx-py-package-manager)
  - [ ] 6.1 Create dx-py-package-manager crate structure
    - Set up Cargo.toml with dependencies on dx-py-core
    - Create module structure: formats, resolver, installer, cache
    - _Requirements: 1.1_

  - [ ] 6.2 Implement DppPackage struct with memory mapping
    - Open file with memmap2
    - Verify magic number and integrity
    - Zero-copy header access via bytemuck
    - _Requirements: 1.2, 1.6_

  - [ ] 6.3 Implement metadata and section accessors
    - O(1) metadata access via pointer cast
    - Bytecode section accessor
    - Dependency graph accessor
    - _Requirements: 1.3, 1.4, 1.6_

- [ ] 7. Implement DPL lock file operations (dx-py-package-manager)
  - [ ] 7.1 Implement DplLockFile struct with memory mapping
    - Open and verify lock file
    - Build hash table index on load
    - _Requirements: 2.2, 2.6_

  - [ ] 7.2 Implement O(1) package lookup
    - FNV-1a hash function
    - Hash table lookup with collision handling
    - _Requirements: 2.1_

  - [ ] 7.3 Write property test for hash table lookup correctness
    - **Property 6: Hash Table O(1) Lookup Correctness**
    - **Validates: Requirements 2.1**

  - [ ] 7.4 Implement DplBuilder for lock file creation
    - Add packages with name, version, hash
    - Build hash table and serialize to binary
    - _Requirements: 2.7_

  - [ ] 7.5 Write property test for DPL round-trip
    - **Property 4: DPL Round-Trip Consistency**
    - **Validates: Requirements 2.8**

- [ ] 8. Checkpoint - Binary formats complete
  - Ensure all tests pass, ask the user if questions arise.


- [ ] 9. Implement wheel to DPP converter
  - [ ] 9.1 Implement wheel file parser
    - Parse ZIP structure
    - Extract METADATA, RECORD files
    - Parse WHEEL file for platform info
    - _Requirements: 1.7_

  - [ ] 9.2 Implement DppBuilder for package creation
    - Add files with compression
    - Generate bytecode section from .py files
    - Build dependency graph from metadata
    - _Requirements: 1.7_

  - [ ] 9.3 Implement DPP to inspectable format serializer
    - Pretty-print header and metadata
    - List files with sizes
    - _Requirements: 1.8_

  - [ ] 9.4 Write property test for wheel conversion round-trip
    - **Property 2: DPP Wheel Conversion Round-Trip**
    - **Validates: Requirements 1.7, 1.8**

- [ ] 10. Implement content-addressable cache
  - [ ] 10.1 Implement GlobalCache struct
    - Content-addressed storage by BLAKE3 hash
    - Directory structure: cache_root/{hash[0:2]}/{hash[2:4]}/{hash}
    - _Requirements: 6.3_

  - [ ] 10.2 Implement cache operations
    - ensure() - get or download package
    - get_path() - return cached package path
    - _Requirements: 6.2_

  - [ ] 10.3 Implement garbage collection
    - Track last access time
    - Remove packages older than threshold
    - _Requirements: 6.5_

  - [ ] 10.4 Write property test for deduplication
    - **Property 7: Content-Addressable Storage Deduplication**
    - **Validates: Requirements 6.2, 6.3**

- [ ] 11. Implement installer with hard link optimization
  - [ ] 11.1 Implement Installer struct
    - Support HardLink and Copy strategies
    - Install packages to site-packages
    - _Requirements: 6.2_

  - [ ] 11.2 Implement hard link installation
    - Create hard links from cache to venv
    - Handle cross-filesystem fallback to copy
    - _Requirements: 6.2_

- [ ] 12. Checkpoint - Package operations complete
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 13. Implement PyPI registry client
  - [ ] 13.1 Implement PyPiClient struct
    - HTTP client with connection pooling
    - JSON API for package metadata
    - _Requirements: 5.1_

  - [ ] 13.2 Implement package metadata fetching
    - Get abbreviated metadata (versions, dependencies)
    - Parse version constraints
    - _Requirements: 5.2, 5.3_

  - [ ] 13.3 Implement parallel download manager
    - Download multiple packages concurrently
    - Verify integrity with BLAKE3
    - _Requirements: 3.4, 3.7_

- [ ] 14. Implement dependency resolver
  - [ ] 14.1 Implement PubGrub-based resolver
    - Version constraint satisfaction
    - Conflict detection and reporting
    - _Requirements: 4.3_

  - [ ] 14.2 Implement resolution with SIMD version comparison
    - Use SIMD for batch version filtering
    - Fall back to scalar when needed
    - _Requirements: 4.1, 4.2, 4.4_

  - [ ] 14.3 Implement resolution hint cache
    - Store successful resolutions
    - Lookup by input hash
    - Delta resolution for similar sets
    - _Requirements: 7.1, 7.3, 7.4_

  - [ ] 14.4 Write property test for hint cache correctness
    - **Property 9: Resolution Hint Cache Correctness**
    - **Validates: Requirements 7.1, 7.3**

- [ ] 15. Checkpoint - Resolution complete
  - Ensure all tests pass, ask the user if questions arise.


- [ ] 16. Implement project manager (dx-py-project-manager)
  - [ ] 16.1 Create dx-py-project-manager crate
    - Set up Cargo.toml with dependencies
    - Create module structure: python, venv, workspace, scripts
    - _Requirements: 8.1_

  - [ ] 16.2 Implement PythonManager
    - Discover system Python installations
    - Install Python from pre-built binaries
    - Pin Python version per project
    - _Requirements: 8.1, 8.2, 8.3, 8.5_

  - [ ] 16.3 Implement VenvManager
    - Create virtual environments with symlinks
    - Generate activation scripts (bash, fish, PowerShell)
    - Cache venv skeletons for reuse
    - _Requirements: 9.1, 9.2, 9.3, 9.4_

  - [ ] 16.4 Implement WorkspaceManager
    - Load workspace config from pyproject.toml/dx
    - Enumerate workspace members
    - Resolve dependencies across workspace
    - _Requirements: 10.1, 10.2, 10.4_

- [ ] 17. Implement compatibility layer (dx-py-compat)
  - [ ] 17.1 Create dx-py-compat crate
    - Set up Cargo.toml
    - Create module structure: pip, uv, pyproject
    - _Requirements: 12.1_

  - [ ] 17.2 Implement pyproject.toml parser
    - Parse [project] section
    - Parse [tool.dx-py] section
    - Parse dependencies and optional-dependencies
    - _Requirements: 12.1, 12.4_

  - [ ] 17.3 Implement binary pyproject.dx format
    - Convert TOML to binary
    - Convert binary back to TOML
    - _Requirements: 12.1_

  - [ ] 17.4 Write property test for pyproject.toml round-trip
    - **Property 8: pyproject.toml Round-Trip Conversion**
    - **Validates: Requirements 12.4, 12.5**

- [ ] 18. Checkpoint - Project management complete
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 19. Implement CLI (dx-py-cli)
  - [ ] 19.1 Create dx-py-cli crate with clap
    - Set up Cargo.toml with clap dependency
    - Define main CLI structure with subcommands
    - _Requirements: 11.1_

  - [ ] 19.2 Implement init command
    - Create pyproject.toml with project metadata
    - Initialize virtual environment
    - _Requirements: 11.1_

  - [ ] 19.3 Implement add/remove commands
    - Add dependencies to pyproject.toml
    - Remove dependencies
    - Update lock file
    - _Requirements: 11.2_

  - [ ] 19.4 Implement lock/sync/install commands
    - lock: Generate lock file from dependencies
    - sync: Install from lock file
    - install: lock + sync
    - _Requirements: 11.3_

  - [ ] 19.5 Implement run command
    - Execute command in virtual environment
    - Pass arguments to subprocess
    - _Requirements: 11.4_

  - [ ] 19.6 Implement python subcommand
    - python install: Install Python version
    - python list: List installed versions
    - python pin: Pin version for project
    - _Requirements: 11.5_

  - [ ] 19.7 Implement tool subcommand
    - tool install: Install tool globally
    - tool run: Run tool ephemerally
    - tool list: List installed tools
    - _Requirements: 11.6_

  - [ ] 19.8 Implement build/publish commands
    - build: Build package for distribution
    - publish: Upload to PyPI
    - _Requirements: 11.7_

  - [ ] 19.9 Implement shell completions
    - Generate completions for bash, zsh, fish, PowerShell
    - _Requirements: 11.8_

- [ ] 20. Checkpoint - CLI complete
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 21. Performance optimization and benchmarks
  - [ ] 21.1 Create benchmark suite
    - Cold install benchmark (100 deps)
    - Warm install benchmark (100 deps)
    - Resolution benchmark (1000 versions)
    - Lock file read/write benchmarks
    - _Requirements: 13.1, 13.2, 13.3_

  - [ ] 21.2 Profile and optimize hot paths
    - Memory allocation optimization
    - I/O batching optimization
    - Cache hit rate optimization
    - _Requirements: 13.4, 13.5_

- [ ] 22. Final checkpoint - All tests pass
  - Ensure all tests pass, ask the user if questions arise.
  - Verify performance targets are met
  - Documentation review

## Notes

- All property-based tests are required for comprehensive correctness validation
- Each task references specific requirements for traceability
- Checkpoints ensure incremental validation
- Property tests validate universal correctness properties
- Unit tests validate specific examples and edge cases
