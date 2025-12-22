//! Core Lint Engine
//!
//! Binary Rule Fusion Engine - executes all rules in a single AST traversal.

use crate::cache::AstCache;
use crate::config::CheckerConfig;
use crate::diagnostics::Diagnostic;
use crate::project::ProjectProfile;
use crate::rules::{Rule, RuleContext, RuleRegistry};
use ignore::WalkBuilder;
use oxc_allocator::Allocator;
use oxc_ast::AstKind;
use oxc_ast::Visit;
use oxc_parser::Parser;
use oxc_span::SourceType;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Result of a check operation
#[derive(Debug, Clone)]
pub struct CheckResult {
    /// All diagnostics found
    pub diagnostics: Vec<Diagnostic>,
    /// Number of files checked
    pub files_checked: usize,
    /// Total time taken
    pub duration: Duration,
    /// Files per second
    pub files_per_second: f64,
}

impl CheckResult {
    /// Check if there are any errors
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.severity == crate::diagnostics::DiagnosticSeverity::Error)
    }

    /// Check if there are any warnings
    pub fn has_warnings(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.severity == crate::diagnostics::DiagnosticSeverity::Warning)
    }

    /// Get error count
    pub fn error_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|d| d.severity == crate::diagnostics::DiagnosticSeverity::Error)
            .count()
    }

    /// Get warning count
    pub fn warning_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|d| d.severity == crate::diagnostics::DiagnosticSeverity::Warning)
            .count()
    }
}

/// The main checker engine
pub struct Checker {
    /// Configuration
    config: CheckerConfig,
    /// Rule registry
    registry: RuleRegistry,
    /// AST cache (optional)
    cache: Option<AstCache>,
    /// Project profile (auto-detected)
    profile: Option<ProjectProfile>,
}

impl Checker {
    /// Create a new checker with default configuration
    pub fn new(config: CheckerConfig) -> Self {
        let registry = RuleRegistry::from_config(&config.rules);

        Self {
            config,
            registry,
            cache: None,
            profile: None,
        }
    }

    /// Create a checker with project auto-detection
    pub fn with_auto_detect(root: &Path) -> Self {
        let config = CheckerConfig::auto_detect(root);
        let profile = ProjectProfile::detect(root);
        let registry = RuleRegistry::from_config(&config.rules);

        Self {
            config,
            registry,
            cache: None,
            profile: Some(profile),
        }
    }

    /// Enable AST caching
    pub fn with_cache(mut self, cache: AstCache) -> Self {
        self.cache = Some(cache);
        self
    }

    /// Check a single file
    pub fn check_file(&self, path: &Path) -> Result<Vec<Diagnostic>, CheckError> {
        let source = std::fs::read_to_string(path).map_err(|e| CheckError::Io {
            path: path.to_path_buf(),
            error: e,
        })?;

        self.check_source(path, &source)
    }

    /// Check source code directly
    pub fn check_source(&self, path: &Path, source: &str) -> Result<Vec<Diagnostic>, CheckError> {
        let source_type = SourceType::from_path(path).unwrap_or_default();

        // Parse the source
        let allocator = Allocator::default();
        let parser = Parser::new(&allocator, source, source_type);
        let result = parser.parse();

        // Collect parse errors
        let mut diagnostics: Vec<Diagnostic> = result
            .errors
            .iter()
            .map(|e| {
                Diagnostic::error(
                    path.to_path_buf(),
                    crate::diagnostics::Span::new(0, 0),
                    "parse-error",
                    e.to_string(),
                )
            })
            .collect();

        // If there are parse errors, return early
        if !diagnostics.is_empty() {
            return Ok(diagnostics);
        }

        // Create rule context
        let mut ctx = RuleContext::new(path, source);

        // Run file-level checks
        for (rule, _severity) in self.registry.enabled_rules() {
            rule.check_file(source, &mut ctx);
        }

        // Traverse AST and run rules
        let mut visitor = LintVisitor::new(&self.registry, &mut ctx);
        visitor.visit_program(&result.program);

        // Run end checks
        for (rule, _severity) in self.registry.enabled_rules() {
            rule.check_end(&mut ctx);
        }

        // Collect diagnostics
        diagnostics.extend(ctx.take_diagnostics());

        Ok(diagnostics)
    }

    /// Check a directory or file path
    pub fn check_path(&self, path: &Path) -> Result<CheckResult, CheckError> {
        let start = Instant::now();

        // Collect files to check
        let files = self.collect_files(path)?;
        let file_count = files.len();

        // Check files in parallel
        let diagnostics: Vec<Diagnostic> = if self.config.parallel.threads == 1 {
            // Single-threaded for debugging
            files
                .iter()
                .flat_map(|f| self.check_file(f).unwrap_or_default())
                .collect()
        } else {
            // Parallel execution
            use rayon::prelude::*;
            files
                .par_iter()
                .flat_map(|f| self.check_file(f).unwrap_or_default())
                .collect()
        };

        let duration = start.elapsed();
        let files_per_second = file_count as f64 / duration.as_secs_f64();

        Ok(CheckResult {
            diagnostics,
            files_checked: file_count,
            duration,
            files_per_second,
        })
    }

    /// Collect files to check based on include/exclude patterns
    fn collect_files(&self, root: &Path) -> Result<Vec<PathBuf>, CheckError> {
        let mut files = Vec::new();

        if root.is_file() {
            files.push(root.to_path_buf());
            return Ok(files);
        }

        let walker = WalkBuilder::new(root)
            .standard_filters(true) // Respect .gitignore
            .hidden(true) // Skip hidden files
            .build();

        for entry in walker.flatten() {
            let path = entry.path();

            if !path.is_file() {
                continue;
            }

            // Check if file matches include patterns
            let matches_include = self.config.include.iter().any(|pattern| {
                glob::Pattern::new(pattern)
                    .map(|p| p.matches_path(path))
                    .unwrap_or(false)
            });

            if !matches_include {
                // Check by extension as fallback
                let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                let is_js_ts = matches!(ext, "js" | "jsx" | "ts" | "tsx" | "mjs" | "cjs");
                if !is_js_ts {
                    continue;
                }
            }

            // Check if file matches exclude patterns
            let matches_exclude = self.config.exclude.iter().any(|pattern| {
                glob::Pattern::new(pattern)
                    .map(|p| p.matches_path(path))
                    .unwrap_or(false)
            });

            if matches_exclude {
                continue;
            }

            files.push(path.to_path_buf());
        }

        Ok(files)
    }

    /// Get the rule registry
    pub fn registry(&self) -> &RuleRegistry {
        &self.registry
    }

    /// Get the project profile
    pub fn profile(&self) -> Option<&ProjectProfile> {
        self.profile.as_ref()
    }
}

/// Visitor for traversing AST and running rules
struct LintVisitor<'a, 'ctx> {
    registry: &'a RuleRegistry,
    ctx: &'a mut RuleContext<'ctx>,
}

impl<'a, 'ctx> LintVisitor<'a, 'ctx> {
    fn new(registry: &'a RuleRegistry, ctx: &'a mut RuleContext<'ctx>) -> Self {
        Self { registry, ctx }
    }

    fn check_node(&mut self, kind: &AstKind<'_>) {
        for (rule, _severity) in self.registry.enabled_rules() {
            rule.check(kind, self.ctx);
        }
    }
}

impl<'a, 'ctx> Visit<'_> for LintVisitor<'a, 'ctx> {
    fn enter_node(&mut self, kind: AstKind<'_>) {
        self.check_node(&kind);
    }
}

/// Errors that can occur during checking
#[derive(Debug)]
pub enum CheckError {
    /// IO error reading file
    Io {
        path: PathBuf,
        error: std::io::Error,
    },
    /// Parse error
    Parse {
        path: PathBuf,
        message: String,
    },
}

impl std::fmt::Display for CheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io { path, error } => {
                write!(f, "Failed to read {}: {}", path.display(), error)
            }
            Self::Parse { path, message } => {
                write!(f, "Failed to parse {}: {}", path.display(), message)
            }
        }
    }
}

impl std::error::Error for CheckError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_source_no_errors() {
        let checker = Checker::new(CheckerConfig::default());
        let source = "const x = 1;";
        let diagnostics = checker
            .check_source(Path::new("test.js"), source)
            .unwrap();
        // Should have no errors for clean code
        assert!(diagnostics.iter().all(|d| d.rule_id != "parse-error"));
    }

    #[test]
    fn test_check_source_with_debugger() {
        let checker = Checker::new(CheckerConfig::default());
        let source = "debugger;";
        let diagnostics = checker
            .check_source(Path::new("test.js"), source)
            .unwrap();
        // Should detect debugger statement
        assert!(diagnostics.iter().any(|d| d.rule_id == "no-debugger"));
    }
}
