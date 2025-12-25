# Requirements Document: DX-Py Hardening & Production Readiness

## Introduction

This document specifies requirements for hardening DX-Py to be a battle-tested, production-ready Python package manager. The current implementation has functional scaffolding but lacks real-world integration, error handling, cross-platform support, and robustness needed for professional use.

## Glossary

- **DX-Py**: The ultra-fast Python package manager being hardened
- **PyPI**: Python Package Index, the official package repository
- **PEP_508**: Python Enhancement Proposal for dependency specification format
- **PEP_440**: Python Enhancement Proposal for version identification
- **Wheel**: Python binary package format (.whl files)
- **Lock_File**: DPL format file containing resolved dependency versions
- **Resolver**: Component that determines compatible package versions
- **Cache**: Content-addressable storage for downloaded packages
- **Venv**: Python virtual environment

## Requirements

### Requirement 1: Real PyPI Integration

**User Story:** As a developer, I want dx-py to actually download and install packages from PyPI, so that I can use it as my primary package manager.

#### Acceptance Criteria

1. WHEN the lock command is executed, THE Resolver SHALL fetch real package metadata from PyPI JSON API
2. WHEN resolving dependencies, THE Resolver SHALL parse PEP 508 dependency strings including extras and markers
3. WHEN a package is needed, THE Cache SHALL download the wheel file from PyPI with SHA256 verification
4. WHEN installing packages, THE Installer SHALL extract wheel contents to site-packages
5. IF a network error occurs during download, THEN THE System SHALL retry up to 3 times with exponential backoff
6. WHEN downloading multiple packages, THE System SHALL download in parallel (up to 8 concurrent downloads)

### Requirement 2: PEP 440 Version Parsing

**User Story:** As a developer, I want dx-py to correctly parse all Python version formats, so that version constraints work correctly.

#### Acceptance Criteria

1. THE Version_Parser SHALL parse epoch versions (e.g., "1!2.0.0")
2. THE Version_Parser SHALL parse pre-release versions (e.g., "1.0.0a1", "1.0.0b2", "1.0.0rc1")
3. THE Version_Parser SHALL parse post-release versions (e.g., "1.0.0.post1")
4. THE Version_Parser SHALL parse dev versions (e.g., "1.0.0.dev1")
5. THE Version_Parser SHALL parse local versions (e.g., "1.0.0+local")
6. THE Version_Parser SHALL correctly compare versions according to PEP 440 ordering
7. FOR ALL valid PEP 440 version strings, parsing then formatting SHALL produce an equivalent string (round-trip property)

### Requirement 3: Environment Marker Evaluation

**User Story:** As a developer, I want dx-py to evaluate environment markers, so that platform-specific dependencies are handled correctly.

#### Acceptance Criteria

1. THE Marker_Evaluator SHALL evaluate python_version markers (e.g., "python_version >= '3.8'")
2. THE Marker_Evaluator SHALL evaluate sys_platform markers (e.g., "sys_platform == 'win32'")
3. THE Marker_Evaluator SHALL evaluate platform_system markers (e.g., "platform_system == 'Windows'")
4. THE Marker_Evaluator SHALL evaluate implementation_name markers (e.g., "implementation_name == 'cpython'")
5. THE Marker_Evaluator SHALL evaluate extra markers (e.g., "extra == 'dev'")
6. THE Marker_Evaluator SHALL support boolean operators (and, or, not) in marker expressions
7. IF a marker evaluates to false, THEN THE Resolver SHALL skip that dependency

### Requirement 4: Cross-Platform Wheel Selection

**User Story:** As a developer, I want dx-py to select the correct wheel for my platform, so that native packages work correctly.

#### Acceptance Criteria

1. THE Wheel_Selector SHALL detect the current platform (Windows, macOS, Linux)
2. THE Wheel_Selector SHALL detect the current architecture (x86_64, aarch64, arm64)
3. THE Wheel_Selector SHALL detect the current Python implementation (cpython, pypy)
4. THE Wheel_Selector SHALL parse wheel filename tags (e.g., "cp312-cp312-manylinux_2_17_x86_64")
5. THE Wheel_Selector SHALL prefer platform-specific wheels over universal wheels
6. THE Wheel_Selector SHALL prefer newer manylinux tags over older ones
7. IF no compatible wheel exists, THEN THE System SHALL fall back to source distribution

### Requirement 5: Robust Error Handling

**User Story:** As a developer, I want dx-py to provide clear error messages and recover gracefully, so that I can diagnose and fix issues.

#### Acceptance Criteria

1. WHEN a package is not found on PyPI, THE System SHALL display the package name and suggest similar packages
2. WHEN a version constraint cannot be satisfied, THE System SHALL display the conflicting requirements
3. WHEN a network error occurs, THE System SHALL display the URL and error details
4. WHEN a hash verification fails, THE System SHALL display expected vs actual hash
5. WHEN a wheel is incompatible, THE System SHALL display the platform requirements
6. IF an operation fails, THEN THE System SHALL clean up partial state (no corrupted cache/venv)
7. THE System SHALL log detailed debug information when --verbose flag is used

### Requirement 6: Async/Parallel Operations

**User Story:** As a developer, I want dx-py to perform operations in parallel, so that installations are fast.

#### Acceptance Criteria

1. THE Download_Manager SHALL download multiple packages concurrently
2. THE Resolver SHALL fetch package metadata in parallel during resolution
3. THE Installer SHALL install multiple packages in parallel when no dependencies exist between them
4. THE System SHALL limit concurrent operations to avoid overwhelming the system
5. THE System SHALL display progress for long-running operations

### Requirement 7: Real Virtual Environment Management

**User Story:** As a developer, I want dx-py to create and manage real virtual environments, so that I can isolate project dependencies.

#### Acceptance Criteria

1. WHEN creating a venv, THE Venv_Manager SHALL create a valid Python virtual environment
2. THE Venv_Manager SHALL copy or symlink the Python interpreter correctly
3. THE Venv_Manager SHALL generate working activation scripts for bash, zsh, fish, and PowerShell
4. THE Venv_Manager SHALL set up pip and setuptools in the venv
5. WHEN the run command is executed, THE System SHALL activate the venv and run the command
6. THE System SHALL support venv creation on Windows, macOS, and Linux

### Requirement 8: Real Python Version Management

**User Story:** As a developer, I want dx-py to download and manage Python versions, so that I don't need to install Python separately.

#### Acceptance Criteria

1. THE Python_Manager SHALL download Python from python-build-standalone releases
2. THE Python_Manager SHALL verify download integrity with SHA256
3. THE Python_Manager SHALL extract and install Python to the managed directory
4. THE Python_Manager SHALL support Windows, macOS (Intel and ARM), and Linux
5. THE Python_Manager SHALL list available Python versions from the release API
6. WHEN a pinned version is not installed, THE System SHALL offer to install it

### Requirement 9: Real Build and Publish

**User Story:** As a developer, I want dx-py to build and publish packages, so that I can distribute my code.

#### Acceptance Criteria

1. THE Build_System SHALL build wheel packages from pyproject.toml
2. THE Build_System SHALL build source distributions (sdist)
3. THE Build_System SHALL support PEP 517 build backends (hatchling, setuptools, flit, etc.)
4. THE Publish_System SHALL upload packages to PyPI using the upload API
5. THE Publish_System SHALL support API token authentication
6. THE Publish_System SHALL support custom repository URLs (TestPyPI, private registries)

### Requirement 10: Real Tool Management

**User Story:** As a developer, I want dx-py to install and run global tools, so that I can use CLI tools without polluting my project.

#### Acceptance Criteria

1. THE Tool_Manager SHALL create isolated virtual environments for each tool
2. THE Tool_Manager SHALL install the tool package and its dependencies
3. THE Tool_Manager SHALL create wrapper scripts in a bin directory
4. THE Tool_Manager SHALL add the bin directory to PATH (or instruct user how to)
5. WHEN running a tool ephemerally, THE System SHALL create a temporary venv, install, run, and clean up
6. THE Tool_Manager SHALL support upgrading installed tools

### Requirement 11: Configuration and Settings

**User Story:** As a developer, I want to configure dx-py behavior, so that I can customize it for my workflow.

#### Acceptance Criteria

1. THE System SHALL read configuration from pyproject.toml [tool.dx-py] section
2. THE System SHALL read global configuration from ~/.config/dx-py/config.toml
3. THE System SHALL support environment variables for configuration (DX_PY_CACHE_DIR, etc.)
4. THE System SHALL support configuring PyPI index URL
5. THE System SHALL support configuring extra index URLs
6. THE System SHALL support configuring trusted hosts for private registries

### Requirement 12: Workspace Support

**User Story:** As a developer, I want dx-py to support monorepo workspaces, so that I can manage multiple related packages.

#### Acceptance Criteria

1. THE Workspace_Manager SHALL detect workspace configuration in pyproject.toml
2. THE Workspace_Manager SHALL enumerate workspace members from glob patterns
3. THE Workspace_Manager SHALL resolve dependencies across all workspace members
4. THE Workspace_Manager SHALL support path dependencies between workspace members
5. WHEN installing in a workspace, THE System SHALL install all workspace members in development mode

### Requirement 13: Comprehensive Testing

**User Story:** As a maintainer, I want comprehensive tests, so that I can be confident in the code quality.

#### Acceptance Criteria

1. THE Test_Suite SHALL include integration tests that hit real PyPI
2. THE Test_Suite SHALL include tests for all supported platforms (Windows, macOS, Linux)
3. THE Test_Suite SHALL include property-based tests for parsers and formatters
4. THE Test_Suite SHALL achieve at least 80% code coverage
5. THE Test_Suite SHALL include performance regression tests

## Notes

- All network operations should have configurable timeouts
- All file operations should be atomic where possible
- The system should work offline when packages are cached
- Error messages should be actionable and user-friendly
