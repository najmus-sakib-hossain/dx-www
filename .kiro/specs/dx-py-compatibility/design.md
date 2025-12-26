# Design Document: dx-py-compatibility

## Overview

This design document describes the implementation of the `dx-py-compability` crate and the reorganization of the dx-py crate structure. The primary goals are:

1. Create a comprehensive compatibility layer for Python runtime detection and uv configuration support
2. Reorganize the crate structure by renaming `dx-py-project-manager` to `dx-py-workspace`
3. Ensure dx-py can run in any production environment with proper Python and platform detection

## Architecture

The dx-py-compability crate will be organized into the following modules:

```
crates/dx-py/crates/dx-py-compability/
├── Cargo.toml
├── src/
│   ├── lib.rs              # Public API exports
│   ├── runtime/
│   │   ├── mod.rs          # Runtime detection module
│   │   ├── detector.rs     # Python interpreter detection
│   │   ├── version.rs      # Version parsing and validation
│   │   └── capabilities.rs # Runtime capability detection
│   ├── uv/
│   │   ├── mod.rs          # uv compatibility module
│   │   ├── config.rs       # uv.toml and [tool.uv] parsing
│   │   └── migration.rs    # Migration helpers
│   ├── markers/
│   │   ├── mod.rs          # PEP 508 markers module
│   │   ├── parser.rs       # Marker expression parser
│   │   ├── evaluator.rs    # Marker evaluation engine
│   │   └── cache.rs        # Evaluation result cache
│   ├── platform/
│   │   ├── mod.rs          # Platform detection module
│   │   ├── detector.rs     # OS/arch/ABI detection
│   │   ├── wheel_tags.rs   # Wheel tag generation
│   │   └── manylinux.rs    # manylinux/musllinux support
│   ├── venv/
│   │   ├── mod.rs          # Virtual environment module
│   │   ├── builder.rs      # Venv creation
│   │   ├── pyvenv_cfg.rs   # pyvenv.cfg generation
│   │   └── activation.rs   # Activation script generation
│   └── config/
│       ├── mod.rs          # Configuration module
│       ├── types.rs        # Configuration types
│       └── serde.rs        # Serialization/deserialization
└── tests/
    ├── runtime_tests.rs
    ├── uv_tests.rs
    ├── marker_tests.rs
    ├── platform_tests.rs
    ├── venv_tests.rs
    └── config_tests.rs
```

### Final Crate Structure

After reorganization, `crates/dx-py/crates/` will contain:

```
crates/dx-py/crates/
├── dx-py-runtime/          # Python runtime implementation
├── dx-py-package-manager/  # Package management (without dx-py-project-manager)
├── dx-py-workspace/        # Renamed from dx-py-project-manager
├── dx-py-test-runner/      # Test runner implementation
└── dx-py-compability/      # New compatibility crate
```

## Components and Interfaces

### PythonRuntime

Represents a detected Python interpreter with its properties.

```rust
/// Detected Python runtime information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythonRuntime {
    /// Path to the Python executable
    pub executable: PathBuf,
    /// Python version (major, minor, patch)
    pub version: PythonVersion,
    /// Platform architecture (x86_64, aarch64, etc.)
    pub architecture: Architecture,
    /// Installation source (system, pyenv, conda, etc.)
    pub source: InstallationSource,
    /// Available capabilities
    pub capabilities: RuntimeCapabilities,
}

/// Python version with PEP 440 support
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PythonVersion {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
    pub pre_release: Option<PreRelease>,
}

/// Installation source for Python
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InstallationSource {
    System,
    Pyenv,
    Conda,
    Homebrew,
    WindowsStore,
    Custom(PathBuf),
}

/// Runtime capabilities
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeCapabilities {
    pub has_pip: bool,
    pub has_venv: bool,
    pub has_ssl: bool,
    pub has_sqlite: bool,
    pub abi_tag: String,
}
```

### RuntimeDetector

Detects Python installations on the system.

```rust
/// Detects Python installations across the system
pub struct RuntimeDetector {
    search_paths: Vec<PathBuf>,
    cache: Option<DetectionCache>,
}

impl RuntimeDetector {
    /// Create a new detector with default search paths
    pub fn new() -> Self;
    
    /// Detect all Python installations
    pub fn detect_all(&self) -> Result<Vec<PythonRuntime>, DetectionError>;
    
    /// Find a specific Python version
    pub fn find_version(&self, version: &VersionRequirement) -> Result<Option<PythonRuntime>, DetectionError>;
    
    /// Add custom search path
    pub fn add_search_path(&mut self, path: PathBuf);
}
```

### UvConfig

Represents uv configuration settings.

```rust
/// uv configuration settings
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct UvConfig {
    /// Primary package index URL
    pub index_url: Option<String>,
    /// Additional index URLs
    pub extra_index_urls: Vec<String>,
    /// Local package directories
    pub find_links: Vec<PathBuf>,
    /// Packages to never install as binary
    pub no_binary: Vec<String>,
    /// Packages to only install as binary
    pub only_binary: Vec<String>,
    /// Target Python version
    pub python_version: Option<PythonVersion>,
    /// Python selection preference
    pub python_preference: Option<PythonPreference>,
}

/// Python selection preference
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PythonPreference {
    OnlyManaged,
    Managed,
    System,
    OnlySystem,
}
```

### UvConfigLoader

Loads uv configuration from various sources.

```rust
/// Loads uv configuration from files
pub struct UvConfigLoader;

impl UvConfigLoader {
    /// Load from uv.toml file
    pub fn load_uv_toml(path: &Path) -> Result<UvConfig, ConfigError>;
    
    /// Load from pyproject.toml [tool.uv] section
    pub fn load_pyproject_uv(path: &Path) -> Result<Option<UvConfig>, ConfigError>;
    
    /// Merge uv config with dx-py config (dx-py takes precedence)
    pub fn merge_with_dxpy(uv: UvConfig, dxpy: &DxPyConfig) -> MergedConfig;
}
```

### MarkerEnvironment

Represents the environment for PEP 508 marker evaluation.

```rust
/// Environment for marker evaluation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarkerEnvironment {
    pub os_name: String,
    pub sys_platform: String,
    pub platform_machine: String,
    pub platform_python_implementation: String,
    pub platform_release: String,
    pub platform_system: String,
    pub platform_version: String,
    pub python_version: String,
    pub python_full_version: String,
    pub implementation_name: String,
    pub implementation_version: String,
}

impl MarkerEnvironment {
    /// Create from current system
    pub fn current() -> Self;
    
    /// Create from a specific Python runtime
    pub fn from_runtime(runtime: &PythonRuntime) -> Self;
}
```

### MarkerEvaluator

Evaluates PEP 508 marker expressions.

```rust
/// Evaluates PEP 508 marker expressions
pub struct MarkerEvaluator {
    environment: MarkerEnvironment,
    cache: LruCache<String, bool>,
}

impl MarkerEvaluator {
    /// Create evaluator for an environment
    pub fn new(environment: MarkerEnvironment) -> Self;
    
    /// Evaluate a marker expression
    pub fn evaluate(&mut self, marker: &str) -> Result<bool, MarkerError>;
    
    /// Parse a marker expression into AST
    pub fn parse(marker: &str) -> Result<MarkerExpr, MarkerError>;
}

/// Marker expression AST
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MarkerExpr {
    Compare {
        left: MarkerValue,
        op: MarkerOp,
        right: MarkerValue,
    },
    And(Box<MarkerExpr>, Box<MarkerExpr>),
    Or(Box<MarkerExpr>, Box<MarkerExpr>),
}

/// Marker comparison operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkerOp {
    Equal,
    NotEqual,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,
    Compatible,  // ~=
    In,
    NotIn,
}
```

### PlatformDetector

Detects current platform properties.

```rust
/// Platform detection
pub struct PlatformDetector;

impl PlatformDetector {
    /// Detect current platform
    pub fn detect() -> Platform;
}

/// Platform information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Platform {
    pub os: Os,
    pub arch: Architecture,
    pub abi: Option<String>,
    pub libc: Option<Libc>,
}

/// Operating system
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Os {
    Linux,
    Windows,
    MacOs,
    FreeBsd,
    Other(String),
}

/// CPU architecture
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Architecture {
    X86_64,
    X86,
    Aarch64,
    Arm,
    Other(String),
}

/// C library type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Libc {
    Glibc { major: u32, minor: u32 },
    Musl { major: u32, minor: u32 },
}
```

### WheelTagGenerator

Generates compatible wheel tags.

```rust
/// Generates wheel tags for package selection
pub struct WheelTagGenerator {
    platform: Platform,
    python: PythonRuntime,
}

impl WheelTagGenerator {
    /// Create generator for platform and Python
    pub fn new(platform: Platform, python: PythonRuntime) -> Self;
    
    /// Generate all compatible tags in priority order
    pub fn generate_tags(&self) -> Vec<WheelTag>;
    
    /// Check if a wheel is compatible
    pub fn is_compatible(&self, wheel: &WheelFilename) -> bool;
    
    /// Select best wheel from candidates
    pub fn select_best(&self, wheels: &[WheelFilename]) -> Option<&WheelFilename>;
}

/// Wheel tag (python-abi-platform)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WheelTag {
    pub python: String,
    pub abi: String,
    pub platform: String,
    pub priority: u32,
}
```

### VenvBuilder

Creates virtual environments.

```rust
/// Virtual environment builder
pub struct VenvBuilder {
    python: PythonRuntime,
    options: VenvOptions,
}

/// Venv creation options
#[derive(Debug, Clone, Default)]
pub struct VenvOptions {
    pub system_site_packages: bool,
    pub copies: bool,
    pub clear: bool,
    pub upgrade: bool,
}

impl VenvBuilder {
    /// Create builder for a Python runtime
    pub fn new(python: PythonRuntime) -> Self;
    
    /// Set options
    pub fn with_options(mut self, options: VenvOptions) -> Self;
    
    /// Build the virtual environment
    pub fn build(&self, path: &Path) -> Result<VirtualEnvironment, VenvError>;
}

/// Created virtual environment
#[derive(Debug)]
pub struct VirtualEnvironment {
    pub path: PathBuf,
    pub python: PathBuf,
    pub scripts_dir: PathBuf,
    pub site_packages: PathBuf,
}
```

## Data Models

### DxPyConfig

Main configuration structure.

```rust
/// dx-py configuration
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct DxPyConfig {
    /// Target Python version
    pub python_version: Option<PythonVersion>,
    /// Primary package index
    pub index_url: Option<String>,
    /// Additional indexes
    pub extra_index_urls: Vec<String>,
    /// Cache directory
    pub cache_dir: Option<PathBuf>,
    /// Maximum concurrent downloads
    pub max_concurrent_downloads: Option<u32>,
    /// uv compatibility settings
    pub uv_compat: Option<UvConfig>,
}
```

### Error Types

```rust
/// Runtime detection errors
#[derive(Debug, thiserror::Error)]
pub enum DetectionError {
    #[error("No Python installation found")]
    NoPythonFound,
    #[error("Python version {0} is not supported (requires 3.8-3.13)")]
    UnsupportedVersion(PythonVersion),
    #[error("Failed to execute Python: {0}")]
    ExecutionError(#[from] std::io::Error),
    #[error("Failed to parse Python output: {0}")]
    ParseError(String),
}

/// Configuration errors
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Failed to parse TOML: {0}")]
    TomlError(#[from] toml::de::Error),
    #[error("Invalid configuration value: {field} - {message}")]
    ValidationError { field: String, message: String },
}

/// Marker evaluation errors
#[derive(Debug, thiserror::Error)]
pub enum MarkerError {
    #[error("Invalid marker syntax at position {position}: {message}")]
    ParseError { position: usize, message: String },
    #[error("Unknown marker variable: {0}")]
    UnknownVariable(String),
    #[error("Invalid operator for marker comparison")]
    InvalidOperator,
}

/// Virtual environment errors
#[derive(Debug, thiserror::Error)]
pub enum VenvError {
    #[error("Failed to create directory: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Python executable not found: {0}")]
    PythonNotFound(PathBuf),
    #[error("Failed to create symlink: {0}")]
    SymlinkError(String),
}
```

## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system-essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

### Property 1: Python Installation Detection Completeness

*For any* valid Python installation at a standard location (system, pyenv, conda, homebrew, Windows Store), the RuntimeDetector SHALL detect it and return a valid PythonRuntime with correct version, architecture, and source information.

**Validates: Requirements 1.1, 1.2, 1.5**

### Property 2: Python Version Range Validation

*For any* Python version, the dx-py-compability crate SHALL accept versions in the range 3.8.0 to 3.13.x and reject versions outside this range with an appropriate error.

**Validates: Requirements 1.3, 1.4**

### Property 3: uv Configuration Parsing Completeness

*For any* valid uv.toml file or pyproject.toml with [tool.uv] section, the UvConfigLoader SHALL parse all supported fields (index-url, extra-index-url, find-links, no-binary, only-binary, python-version, python-preference) and produce an equivalent UvConfig object.

**Validates: Requirements 2.1, 2.2, 2.3, 2.5**

### Property 4: Configuration Precedence

*For any* configuration where both uv and dx-py settings exist for the same field, the merged configuration SHALL use the dx-py value and emit a warning about the override.

**Validates: Requirements 2.4**

### Property 5: Marker Evaluation Correctness

*For any* valid PEP 508 marker expression using supported operators (==, !=, <, <=, >, >=, ~=, in, not in) and variables, the MarkerEvaluator SHALL produce the correct boolean result based on the target environment.

**Validates: Requirements 3.1, 3.2, 3.3**

### Property 6: Marker Evaluation Caching

*For any* marker expression evaluated multiple times with the same environment, the MarkerEvaluator SHALL return cached results for subsequent evaluations.

**Validates: Requirements 3.5**

### Property 7: Wheel Tag Priority Ordering

*For any* set of compatible wheel tags, the WheelTagGenerator SHALL order them by specificity: platform-specific tags before manylinux tags before 'any' tags.

**Validates: Requirements 4.2, 4.3**

### Property 8: Linux Wheel Tag Support

*For any* valid manylinux (manylinux1, manylinux2010, manylinux2014, manylinux_x_y) or musllinux tag, the WheelTagGenerator SHALL correctly parse and include it in compatibility checks.

**Validates: Requirements 4.4, 4.5**

### Property 9: Virtual Environment PEP 405 Compliance

*For any* virtual environment created by VenvBuilder, the resulting directory structure SHALL contain: pyvenv.cfg with required fields (home, include-system-site-packages, version), activation scripts for all supported shells (bash, zsh, fish, csh, PowerShell), and a correctly linked/copied Python interpreter.

**Validates: Requirements 6.1, 6.2, 6.3, 6.4, 6.5**

### Property 10: Configuration Round-Trip

*For any* valid DxPyConfig object, serializing to TOML and deserializing back SHALL produce an equivalent configuration object.

**Validates: Requirements 7.1, 7.2, 7.3**

### Property 11: Configuration Validation

*For any* TOML input with invalid configuration values, the deserializer SHALL return a ConfigError::ValidationError with the field name and descriptive message.

**Validates: Requirements 7.4, 7.5**

## Error Handling

### Detection Errors
- Return `DetectionError::NoPythonFound` when no Python installations are found
- Return `DetectionError::UnsupportedVersion` for Python < 3.8 or > 3.13
- Return `DetectionError::ExecutionError` when Python process fails to execute
- Return `DetectionError::ParseError` when Python output cannot be parsed

### Configuration Errors
- Return `ConfigError::IoError` for file read failures
- Return `ConfigError::TomlError` for TOML syntax errors
- Return `ConfigError::ValidationError` for semantic validation failures

### Marker Errors
- Return `MarkerError::ParseError` with position for syntax errors
- Return `MarkerError::UnknownVariable` for unrecognized marker variables
- Return `MarkerError::InvalidOperator` for unsupported operator combinations

### Venv Errors
- Return `VenvError::IoError` for filesystem operation failures
- Return `VenvError::PythonNotFound` when source Python doesn't exist
- Return `VenvError::SymlinkError` when symlink creation fails (Windows)

## Testing Strategy

### Unit Tests
- Test individual components in isolation
- Mock filesystem for path detection tests
- Mock Python execution for runtime detection tests
- Test edge cases: empty configs, invalid versions, malformed markers

### Property-Based Tests
Property-based tests will use the `proptest` crate to verify correctness properties:

1. **Python Version Validation**: Generate random version tuples, verify acceptance/rejection
2. **uv Config Parsing**: Generate random valid TOML, verify round-trip
3. **Marker Evaluation**: Generate random marker expressions, verify against reference implementation
4. **Wheel Tag Ordering**: Generate random tag sets, verify ordering invariants
5. **Config Round-Trip**: Generate random configs, verify serialize/deserialize equivalence

Each property test will run minimum 100 iterations and be tagged with:
- **Feature: dx-py-compatibility, Property {N}: {property_text}**

### Integration Tests
- Test full detection flow with real Python installations
- Test venv creation with actual filesystem operations
- Test marker evaluation against real platform values
