//! Stack component traits
//!
//! Defines the interface for each stack component that languages can implement.

use crate::error::StackResult;
use std::path::Path;

/// Base trait for all stack components
pub trait StackComponent: Send + Sync {
    /// Returns the component name
    fn name(&self) -> &str;

    /// Returns the version of this component
    fn version(&self) -> &str;

    /// Returns whether this component is available/enabled
    fn is_available(&self) -> bool {
        true
    }
}

/// Runtime component: Code execution engine
///
/// Implementations:
/// - JavaScript: dx-js-runtime (10x faster than Bun)
/// - Python: dx-py-runtime (future)
pub trait Runtime: StackComponent {
    /// Run a file
    fn run(&self, file: &Path, args: &[String]) -> StackResult<i32>;

    /// Run code from a string
    fn eval(&self, code: &str) -> StackResult<String>;

    /// Run in watch mode
    fn watch(&self, file: &Path) -> StackResult<()>;

    /// Get REPL if supported
    fn repl(&self) -> StackResult<()> {
        Err(crate::error::StackError::NotSupported("REPL".into()))
    }
}

/// Package Manager component: Dependency management
///
/// Implementations:
/// - JavaScript: dx-js-package-manager (50x faster than npm)
/// - Python: dx-py-package-manager (future)
pub trait PackageManager: StackComponent {
    /// Install all dependencies from manifest
    fn install(&self, dev: bool) -> StackResult<()>;

    /// Add a package
    fn add(&self, package: &str, version: Option<&str>, dev: bool) -> StackResult<()>;

    /// Remove a package
    fn remove(&self, package: &str) -> StackResult<()>;

    /// Update packages
    fn update(&self, package: Option<&str>) -> StackResult<()>;

    /// List installed packages
    fn list(&self) -> StackResult<Vec<PackageInfo>>;

    /// Audit packages for vulnerabilities
    fn audit(&self) -> StackResult<AuditResult>;

    /// Initialize a new project
    fn init(&self, name: &str, template: Option<&str>) -> StackResult<()>;
}

/// Package information
#[derive(Debug, Clone)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub is_dev: bool,
}

/// Audit result
#[derive(Debug, Clone)]
pub struct AuditResult {
    pub vulnerabilities: u32,
    pub critical: u32,
    pub high: u32,
    pub moderate: u32,
    pub low: u32,
}

/// Bundler component: Code bundling/compilation
///
/// Implementations:
/// - JavaScript: dx-js-bundler (3.8x faster than Bun)
/// - C/C++: dx-c-bundler (future, build system)
pub trait Bundler: StackComponent {
    /// Bundle files for production
    fn bundle(&self, config: BundleConfig) -> StackResult<BundleResult>;

    /// Start development server with HMR
    fn dev(&self, port: u16) -> StackResult<()>;

    /// Build for production
    fn build(&self, output: &Path) -> StackResult<()>;
}

/// Bundle configuration
#[derive(Debug, Clone, Default)]
pub struct BundleConfig {
    pub entry: String,
    pub output: String,
    pub minify: bool,
    pub sourcemap: bool,
    pub target: Option<String>,
    pub external: Vec<String>,
}

/// Bundle result
#[derive(Debug, Clone)]
pub struct BundleResult {
    pub output_path: String,
    pub size_bytes: u64,
    pub duration_ms: u64,
    pub modules_bundled: u32,
}

/// Monorepo component: Multi-package workspace management
///
/// Implementations:
/// - JavaScript: dx-js-monorepo
pub trait Monorepo: StackComponent {
    /// Initialize a monorepo workspace
    fn init(&self, config: MonorepoConfig) -> StackResult<()>;

    /// List all packages in the workspace
    fn list_packages(&self) -> StackResult<Vec<WorkspacePackage>>;

    /// Run a command across all packages
    fn run_all(&self, command: &str, args: &[String]) -> StackResult<()>;

    /// Run a command in specific packages
    fn run_in(&self, packages: &[String], command: &str, args: &[String]) -> StackResult<()>;

    /// Add a new package to the workspace
    fn add_package(&self, name: &str, template: Option<&str>) -> StackResult<()>;

    /// Get dependency graph
    fn graph(&self) -> StackResult<DependencyGraph>;
}

/// Monorepo configuration
#[derive(Debug, Clone, Default)]
pub struct MonorepoConfig {
    pub packages_dir: String,
    pub shared_dependencies: bool,
    pub hoist: bool,
}

/// Workspace package info
#[derive(Debug, Clone)]
pub struct WorkspacePackage {
    pub name: String,
    pub path: String,
    pub version: String,
    pub private: bool,
}

/// Dependency graph
#[derive(Debug, Clone, Default)]
pub struct DependencyGraph {
    pub nodes: Vec<String>,
    pub edges: Vec<(String, String)>,
}

/// Compatibility component: Cross-version/platform compatibility
///
/// Implementations:
/// - JavaScript: dx-js-compatibility
pub trait Compatibility: StackComponent {
    /// Check compatibility of the current project
    fn check(&self) -> StackResult<CompatibilityReport>;

    /// Apply polyfills/shims for target
    fn apply(&self, target: &str) -> StackResult<()>;

    /// List supported targets
    fn targets(&self) -> StackResult<Vec<CompatibilityTarget>>;
}

/// Compatibility report
#[derive(Debug, Clone)]
pub struct CompatibilityReport {
    pub compatible: bool,
    pub issues: Vec<CompatibilityIssue>,
}

/// Compatibility issue
#[derive(Debug, Clone)]
pub struct CompatibilityIssue {
    pub file: String,
    pub line: u32,
    pub message: String,
    pub severity: IssueSeverity,
}

/// Issue severity
#[derive(Debug, Clone, Copy)]
pub enum IssueSeverity {
    Error,
    Warning,
    Info,
}

/// Compatibility target
#[derive(Debug, Clone)]
pub struct CompatibilityTarget {
    pub name: String,
    pub description: String,
}

/// Test Runner component: Test execution framework
///
/// Implementations:
/// - JavaScript: dx-js-test-runner (26x faster than Jest)
pub trait TestRunner: StackComponent {
    /// Run all tests
    fn run(&self, config: TestConfig) -> StackResult<TestResult>;

    /// Run tests in watch mode
    fn watch(&self, pattern: Option<&str>) -> StackResult<()>;

    /// Generate coverage report
    fn coverage(&self) -> StackResult<CoverageReport>;
}

/// Test configuration
#[derive(Debug, Clone, Default)]
pub struct TestConfig {
    pub pattern: Option<String>,
    pub coverage: bool,
    pub parallel: bool,
    pub bail: bool,
    pub timeout_ms: Option<u64>,
}

/// Test result
#[derive(Debug, Clone)]
pub struct TestResult {
    pub passed: u32,
    pub failed: u32,
    pub skipped: u32,
    pub duration_ms: u64,
    pub failures: Vec<TestFailure>,
}

/// Test failure
#[derive(Debug, Clone)]
pub struct TestFailure {
    pub name: String,
    pub file: String,
    pub message: String,
    pub stack: Option<String>,
}

/// Coverage report
#[derive(Debug, Clone)]
pub struct CoverageReport {
    pub lines_total: u32,
    pub lines_covered: u32,
    pub branches_total: u32,
    pub branches_covered: u32,
    pub functions_total: u32,
    pub functions_covered: u32,
}

impl CoverageReport {
    /// Calculate line coverage percentage
    pub fn line_percentage(&self) -> f64 {
        if self.lines_total == 0 {
            100.0
        } else {
            (self.lines_covered as f64 / self.lines_total as f64) * 100.0
        }
    }

    /// Calculate branch coverage percentage
    pub fn branch_percentage(&self) -> f64 {
        if self.branches_total == 0 {
            100.0
        } else {
            (self.branches_covered as f64 / self.branches_total as f64) * 100.0
        }
    }

    /// Calculate function coverage percentage
    pub fn function_percentage(&self) -> f64 {
        if self.functions_total == 0 {
            100.0
        } else {
            (self.functions_covered as f64 / self.functions_total as f64) * 100.0
        }
    }
}
