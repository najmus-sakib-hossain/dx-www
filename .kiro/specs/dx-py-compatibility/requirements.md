# Requirements Document

## Introduction

This specification covers the reorganization and enhancement of the dx-py crate structure to support all Python runtime and uv configurations correctly, enabling dx-py to run in any production environment. The work involves:

1. Creating a proper `dx-py-compability` crate with full Python runtime and uv configuration support
2. Renaming `dx-py-project-manager` to `dx-py-workspace` and moving it to the top-level crates folder
3. Ensuring the final structure has five main crates: `dx-py-runtime`, `dx-py-package-manager`, `dx-py-workspace`, `dx-py-test-runner`, `dx-py-compability`

## Glossary

- **dx-py**: Ultra-fast Python package manager written in Rust
- **dx-py-compability**: Crate responsible for Python runtime detection, version compatibility, and uv configuration support
- **dx-py-workspace**: Crate for Python project lifecycle and workspace management (renamed from dx-py-project-manager)
- **uv**: Astral's fast Python package manager that dx-py aims to be compatible with
- **Python_Runtime**: A specific Python interpreter installation with its version and capabilities
- **Environment_Marker**: PEP 508 markers for conditional dependencies based on platform/Python version
- **pyproject.toml**: Standard Python project configuration file (PEP 517/518/621)
- **Virtual_Environment**: Isolated Python environment with its own packages

## Requirements

### Requirement 1: Python Runtime Detection and Support

**User Story:** As a developer, I want dx-py to automatically detect and support all installed Python runtimes, so that I can use dx-py with any Python version in my production environment.

#### Acceptance Criteria

1. WHEN dx-py starts, THE dx-py-compability crate SHALL detect all installed Python interpreters on the system
2. WHEN a Python interpreter is detected, THE dx-py-compability crate SHALL extract its version, architecture, and capabilities
3. THE dx-py-compability crate SHALL support Python versions 3.8 through 3.13
4. WHEN a Python version is not supported, THE dx-py-compability crate SHALL return a descriptive error message
5. THE dx-py-compability crate SHALL detect Python installations from standard locations (system, pyenv, conda, homebrew, Windows Store)

### Requirement 2: uv Configuration Compatibility

**User Story:** As a developer migrating from uv, I want dx-py to read and respect uv configuration files, so that I can seamlessly switch between tools.

#### Acceptance Criteria

1. WHEN a uv.toml configuration file exists, THE dx-py-compability crate SHALL parse and apply its settings
2. WHEN a pyproject.toml contains [tool.uv] section, THE dx-py-compability crate SHALL parse and apply those settings
3. THE dx-py-compability crate SHALL support uv configuration options: index-url, extra-index-url, find-links, no-binary, only-binary
4. WHEN uv configuration conflicts with dx-py configuration, THE dx-py-compability crate SHALL prefer dx-py settings with a warning
5. THE dx-py-compability crate SHALL support uv's python-version and python-preference settings

### Requirement 3: Environment Marker Evaluation

**User Story:** As a developer, I want dx-py to correctly evaluate PEP 508 environment markers, so that platform-specific dependencies are handled correctly.

#### Acceptance Criteria

1. THE dx-py-compability crate SHALL evaluate all PEP 508 environment markers (os_name, sys_platform, platform_machine, etc.)
2. WHEN evaluating markers, THE dx-py-compability crate SHALL use the target Python runtime's properties
3. THE dx-py-compability crate SHALL support marker operators: ==, !=, <, <=, >, >=, ~=, in, not in
4. WHEN a marker expression is invalid, THE dx-py-compability crate SHALL return a parse error with position information
5. THE dx-py-compability crate SHALL cache marker evaluation results for performance

### Requirement 4: Platform Compatibility Detection

**User Story:** As a developer, I want dx-py to correctly identify platform compatibility for wheels, so that the correct packages are installed.

#### Acceptance Criteria

1. THE dx-py-compability crate SHALL detect the current platform's OS, architecture, and ABI
2. THE dx-py-compability crate SHALL generate compatible wheel tags for the current platform
3. WHEN selecting wheels, THE dx-py-compability crate SHALL prioritize by specificity (platform-specific > manylinux > any)
4. THE dx-py-compability crate SHALL support manylinux1, manylinux2010, manylinux2014, and manylinux_x_y tags
5. THE dx-py-compability crate SHALL support musllinux tags for Alpine Linux environments

### Requirement 5: Workspace Crate Reorganization

**User Story:** As a maintainer, I want the dx-py-project-manager crate renamed to dx-py-workspace and moved to the top-level crates folder, so that the crate structure is consistent and clear.

#### Acceptance Criteria

1. THE dx-py-workspace crate SHALL be located at crates/dx-py/crates/dx-py-workspace
2. THE dx-py-workspace crate SHALL retain all functionality from dx-py-project-manager
3. WHEN the crate is renamed, THE Cargo.toml SHALL update the package name to "dx-py-workspace"
4. THE dx-py-workspace crate SHALL update all internal module references to use the new name
5. WHEN other crates depend on dx-py-project-manager, THOSE dependencies SHALL be updated to dx-py-workspace

### Requirement 6: Virtual Environment Compatibility

**User Story:** As a developer, I want dx-py to create virtual environments compatible with standard Python tooling, so that I can use them with any Python tool.

#### Acceptance Criteria

1. THE dx-py-compability crate SHALL create virtual environments following PEP 405 specification
2. WHEN creating a venv, THE dx-py-compability crate SHALL generate correct pyvenv.cfg with home, include-system-site-packages, and version
3. THE dx-py-compability crate SHALL create activation scripts for bash, zsh, fish, csh, and PowerShell
4. WHEN a venv is created, THE dx-py-compability crate SHALL symlink or copy the Python interpreter correctly per platform
5. THE dx-py-compability crate SHALL support --system-site-packages and --copies options

### Requirement 7: Configuration Serialization

**User Story:** As a developer, I want dx-py configuration to be serializable and deserializable, so that settings can be persisted and shared.

#### Acceptance Criteria

1. THE dx-py-compability crate SHALL serialize configuration to TOML format
2. THE dx-py-compability crate SHALL deserialize configuration from TOML format
3. WHEN serializing then deserializing configuration, THE dx-py-compability crate SHALL produce an equivalent configuration object (round-trip)
4. THE dx-py-compability crate SHALL validate configuration values during deserialization
5. IF configuration contains invalid values, THEN THE dx-py-compability crate SHALL return descriptive validation errors

### Requirement 8: Crate Structure Validation

**User Story:** As a maintainer, I want the final crate structure to be validated, so that all five required crates exist with proper organization.

#### Acceptance Criteria

1. THE crates/dx-py/crates folder SHALL contain exactly five top-level crate folders
2. THE folder names SHALL be: dx-py-runtime, dx-py-package-manager, dx-py-workspace, dx-py-test-runner, dx-py-compability
3. EACH crate folder SHALL contain a valid Cargo.toml file
4. EACH crate folder SHALL contain a src directory with lib.rs or main.rs
5. THE dx-py-package-manager crate SHALL NOT contain dx-py-project-manager as a subcrate after reorganization
