# Requirements Document

## Introduction

DX-Py is a high-performance Python package manager designed to be 5-50x faster than uv by leveraging binary-first architecture, SIMD acceleration, dx-reactor for platform-native async I/O (io_uring/kqueue/IOCP), and zero-copy memory operations. The system consists of two main components: dx-py-package-manager for binary package operations and dx-py-project-manager for project lifecycle management.

## Glossary

- **DPP (Dx Python Package)**: Binary package format with zero-copy access, replacing standard wheel format
- **DPL (Dx Python Lock)**: Binary lock file format with O(1) lookups, replacing TOML-based lockfiles
- **DPI (Dx Python Index)**: Binary metadata index format for registry operations
- **DPRP (Dx Python Registry Protocol)**: Binary registry protocol for batched requests
- **dx-reactor**: Cross-platform async I/O library using io_uring (Linux), kqueue (macOS), IOCP (Windows)
- **PubGrub**: Version resolution algorithm used for dependency resolution
- **SIMD**: Single Instruction Multiple Data - parallel processing for version comparison
- **FUSE**: Filesystem in Userspace - enables zero-disk installation via mount
- **Content-Addressable_Storage**: Storage system where content is addressed by its hash
- **Resolution_Hints**: Pre-computed resolution snapshots for incremental updates

## Requirements

### Requirement 1: Binary Package Format (DPP)

**User Story:** As a developer, I want packages stored in a binary format with zero-copy access, so that metadata retrieval is 1000x faster than parsing ZIP/JSON.

#### Acceptance Criteria

1. THE DPP_Format SHALL use a fixed 64-byte header for O(1) section access
2. WHEN a DPP package is loaded, THE System SHALL memory-map the file without parsing
3. THE DPP_Format SHALL include pre-compiled bytecode section for direct execution
4. THE DPP_Format SHALL include pre-resolved dependency graph section
5. THE DPP_Format SHALL use BLAKE3 hash for integrity verification
6. WHEN metadata is accessed, THE System SHALL return it via pointer cast in under 0.01ms
7. THE Converter SHALL transform standard wheel format to DPP format
8. THE DPP_Pretty_Printer SHALL serialize DPP packages back to inspectable format

### Requirement 2: Binary Lock File (DPL)

**User Story:** As a developer, I want lock files in binary format with hash table lookups, so that package lookups are O(1) instead of O(n) parsing.

#### Acceptance Criteria

1. THE DPL_Format SHALL use a hash table for O(1) package lookup by name
2. WHEN a lock file is read, THE System SHALL memory-map it without parsing
3. THE DPL_Format SHALL use fixed 128-byte entries for predictable layout
4. THE DPL_Format SHALL include platform and Python version metadata
5. WHEN integrity is verified, THE System SHALL use SIMD-accelerated BLAKE3
6. THE System SHALL read lock files in under 0.1ms (vs 50ms for TOML)
7. THE System SHALL write lock files in under 0.2ms (vs 80ms for TOML)
8. FOR ALL valid DPL files, parsing then serializing SHALL produce an equivalent file (round-trip property)

### Requirement 3: dx-reactor I/O Integration

**User Story:** As a developer, I want package downloads to use platform-native async I/O, so that network operations are 3-5x faster than standard async.

#### Acceptance Criteria

1. WHEN running on Linux, THE Download_Manager SHALL use io_uring for batched I/O
2. WHEN running on macOS, THE Download_Manager SHALL use kqueue for async I/O
3. WHEN running on Windows, THE Download_Manager SHALL use IOCP for async I/O
4. THE Download_Manager SHALL submit multiple downloads in a single syscall batch
5. THE Download_Manager SHALL use registered buffers for zero-copy transfers
6. THE System SHALL use thread-per-core architecture to eliminate lock contention
7. WHEN downloading 100 packages, THE System SHALL complete in under 3 seconds (warm cache)

### Requirement 4: SIMD-Accelerated Version Resolution

**User Story:** As a developer, I want version comparisons to use SIMD instructions, so that resolution is 8x faster than sequential comparison.

#### Acceptance Criteria

1. THE Resolver SHALL compare 8 versions in parallel using AVX2 instructions
2. WHEN AVX2 is unavailable, THE Resolver SHALL fall back to NEON (ARM) or scalar
3. THE Resolver SHALL implement binary-first PubGrub algorithm
4. WHEN resolving 1000 versions, THE System SHALL complete in under 1ms
5. THE Resolver SHALL produce deterministic results regardless of SIMD availability
6. FOR ALL version constraints and candidate sets, SIMD resolution SHALL produce the same result as scalar resolution

### Requirement 5: Binary Registry Protocol (DPRP)

**User Story:** As a developer, I want to resolve entire dependency trees in a single network request, so that network round-trips are reduced by 50-150x.

#### Acceptance Criteria

1. THE DPRP_Request SHALL batch multiple package queries in one binary request
2. THE DPRP_Request SHALL include platform requirements for server-side filtering
3. THE DPRP_Response SHALL include pre-computed resolution for the dependency tree
4. THE DPRP_Response SHALL be memory-mappable after download
5. WHEN resolving 100 packages, THE System SHALL use 1-2 requests (vs 300 for JSON API)
6. THE Registry_Client SHALL use HTTP/3 with 0-RTT for repeat connections

### Requirement 6: Zero-Copy Installation

**User Story:** As a developer, I want packages installed via memory mapping or FUSE mount, so that installation is 500-15000x faster than file extraction.

#### Acceptance Criteria

1. WHEN running on Linux, THE Installer SHALL support FUSE mount for zero-disk installation
2. THE Installer SHALL support hard link optimization for shared cache
3. THE Installer SHALL use content-addressable storage for deduplication
4. WHEN installing a 100MB package, THE System SHALL complete in under 1ms (FUSE mode)
5. THE Cache SHALL implement garbage collection for unused packages
6. THE System SHALL reduce disk usage by 40% through deduplication

### Requirement 7: Resolution Hint Cache

**User Story:** As a developer, I want previous resolutions cached and reused, so that repeated operations are 10-1000x faster.

#### Acceptance Criteria

1. THE Hint_Cache SHALL store successful resolutions with input hash
2. WHEN an exact resolution match exists, THE System SHALL return it in under 0.1ms
3. THE Hint_Cache SHALL support delta resolution from similar package sets
4. WHEN less than 10% of packages changed, THE System SHALL use delta resolution
5. THE Hint_Cache SHALL include validity timestamps based on package update frequency
6. WHEN adding 1 package to a resolved set, THE System SHALL resolve in under 10ms

### Requirement 8: Python Version Management

**User Story:** As a developer, I want to install and manage Python versions, so that I can easily switch between Python versions per project.

#### Acceptance Criteria

1. THE Python_Manager SHALL install Python versions from pre-built binaries
2. THE Python_Manager SHALL discover system-installed Python versions
3. THE Python_Manager SHALL pin Python version per project
4. WHEN installing Python 3.12, THE System SHALL complete in under 200ms
5. THE Python_Manager SHALL support multiple concurrent Python versions

### Requirement 9: Virtual Environment Management

**User Story:** As a developer, I want ultra-fast virtual environment creation, so that project setup is 10x faster.

#### Acceptance Criteria

1. THE Venv_Manager SHALL create virtual environments in under 10ms
2. THE Venv_Manager SHALL support shared venv optimization across projects
3. THE Venv_Manager SHALL generate shell activation scripts
4. WHEN a venv already exists, THE System SHALL reuse it without recreation

### Requirement 10: Project Workspace Support

**User Story:** As a developer, I want Cargo-style workspace support, so that I can manage monorepos with multiple Python packages.

#### Acceptance Criteria

1. THE Workspace_Manager SHALL support multiple packages in a single repository
2. THE Workspace_Manager SHALL share dependencies across workspace members
3. WHEN switching workspaces, THE System SHALL complete in under 100ms
4. THE Workspace_Manager SHALL support workspace-level lock files

### Requirement 11: CLI Interface

**User Story:** As a developer, I want a comprehensive CLI for all package management operations, so that I can manage Python projects efficiently.

#### Acceptance Criteria

1. THE CLI SHALL support `init` command for project initialization
2. THE CLI SHALL support `add` and `remove` commands for dependency management
3. THE CLI SHALL support `lock` and `sync` commands for lock file operations
4. THE CLI SHALL support `run` command for script execution in venv
5. THE CLI SHALL support `python` subcommand for Python version management
6. THE CLI SHALL support `tool` subcommand for global tool installation (pipx replacement)
7. THE CLI SHALL support `build` and `publish` commands for package publishing
8. THE CLI SHALL provide shell completions for bash, zsh, fish, and PowerShell

### Requirement 12: Compatibility Layer

**User Story:** As a developer, I want compatibility with existing pip and uv workflows, so that I can migrate gradually.

#### Acceptance Criteria

1. THE Compat_Layer SHALL parse and convert pyproject.toml to binary format
2. THE Compat_Layer SHALL provide pip CLI compatibility mode
3. THE Compat_Layer SHALL provide uv CLI compatibility mode
4. WHEN converting pyproject.toml, THE System SHALL preserve all metadata
5. FOR ALL valid pyproject.toml files, converting to binary and back SHALL produce equivalent content (round-trip property)

### Requirement 13: Performance Targets

**User Story:** As a developer, I want guaranteed performance improvements over uv, so that my development workflow is significantly faster.

#### Acceptance Criteria

1. THE System SHALL achieve cold install (100 deps) in under 0.5 seconds (vs 2.5s uv)
2. THE System SHALL achieve warm install (100 deps) in under 0.1 seconds (vs 0.5s uv)
3. THE System SHALL achieve dependency resolution in under 20ms (vs 170ms uv)
4. THE System SHALL use under 50MB peak memory (vs 150MB uv)
5. THE System SHALL reduce disk cache size by 40% through deduplication
