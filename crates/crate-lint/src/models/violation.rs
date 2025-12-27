//! Violation and report models for validation results

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// A single validation violation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Violation {
    /// Unique identifier for this violation type
    pub id: String,
    /// Name of the crate with the violation
    pub crate_name: String,
    /// Category of the violation
    pub category: ViolationCategory,
    /// Severity level
    pub severity: Severity,
    /// Human-readable message describing the violation
    pub message: String,
    /// File where the violation was found
    pub file: Option<PathBuf>,
    /// Line number in the file
    pub line: Option<usize>,
    /// Suggested fix if available
    pub fix: Option<Fix>,
}

impl Violation {
    /// Create a new violation
    pub fn new(
        id: impl Into<String>,
        crate_name: impl Into<String>,
        category: ViolationCategory,
        severity: Severity,
        message: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            crate_name: crate_name.into(),
            category,
            severity,
            message: message.into(),
            file: None,
            line: None,
            fix: None,
        }
    }
    
    /// Add file location to the violation
    pub fn with_file(mut self, file: impl Into<PathBuf>) -> Self {
        self.file = Some(file.into());
        self
    }
    
    /// Add line number to the violation
    pub fn with_line(mut self, line: usize) -> Self {
        self.line = Some(line);
        self
    }
    
    /// Add a fix suggestion
    pub fn with_fix(mut self, fix: Fix) -> Self {
        self.fix = Some(fix);
        self
    }
    
    /// Check if this violation can be auto-fixed
    pub fn is_auto_fixable(&self) -> bool {
        self.fix.as_ref().map(|f| f.auto_fixable).unwrap_or(false)
    }
}

/// Categories of violations
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ViolationCategory {
    /// Cargo.toml metadata issues
    Metadata,
    /// Naming convention violations
    Naming,
    /// Documentation file issues
    Documentation,
    /// License file issues
    License,
    /// Directory structure issues
    Structure,
    /// Dependency management issues
    Dependency,
}

impl ViolationCategory {
    /// Get all categories
    pub fn all() -> &'static [ViolationCategory] {
        &[
            ViolationCategory::Metadata,
            ViolationCategory::Naming,
            ViolationCategory::Documentation,
            ViolationCategory::License,
            ViolationCategory::Structure,
            ViolationCategory::Dependency,
        ]
    }
    
    /// Get display name for the category
    pub fn display_name(&self) -> &'static str {
        match self {
            ViolationCategory::Metadata => "Metadata",
            ViolationCategory::Naming => "Naming",
            ViolationCategory::Documentation => "Documentation",
            ViolationCategory::License => "License",
            ViolationCategory::Structure => "Structure",
            ViolationCategory::Dependency => "Dependency",
        }
    }
}

/// Severity levels for violations
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    /// Informational suggestion
    Info,
    /// Should be fixed but not blocking
    Warning,
    /// Must be fixed before publishing
    Error,
}

impl Severity {
    /// Get display name for the severity
    pub fn display_name(&self) -> &'static str {
        match self {
            Severity::Info => "Info",
            Severity::Warning => "Warning",
            Severity::Error => "Error",
        }
    }
    
    /// Get ANSI color code for terminal output
    pub fn color(&self) -> &'static str {
        match self {
            Severity::Info => "\x1b[36m",    // Cyan
            Severity::Warning => "\x1b[33m", // Yellow
            Severity::Error => "\x1b[31m",   // Red
        }
    }
}

/// A suggested fix for a violation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Fix {
    /// Description of what the fix does
    pub description: String,
    /// Whether this fix can be applied automatically
    pub auto_fixable: bool,
    /// File changes required for the fix
    pub changes: Vec<FileChange>,
}

impl Fix {
    /// Create a new fix
    pub fn new(description: impl Into<String>, auto_fixable: bool) -> Self {
        Self {
            description: description.into(),
            auto_fixable,
            changes: Vec::new(),
        }
    }
    
    /// Add a file change to the fix
    pub fn with_change(mut self, change: FileChange) -> Self {
        self.changes.push(change);
        self
    }
}

/// A file change operation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileChange {
    /// Path to the file
    pub file: PathBuf,
    /// The operation to perform
    pub operation: ChangeOperation,
}

impl FileChange {
    /// Create a new file change
    pub fn new(file: impl Into<PathBuf>, operation: ChangeOperation) -> Self {
        Self {
            file: file.into(),
            operation,
        }
    }
}

/// Types of file change operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ChangeOperation {
    /// Create a new file with content
    Create { content: String },
    /// Modify existing content
    Modify { old: String, new: String },
    /// Delete a file
    Delete,
}

/// Complete validation report
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ValidationReport {
    /// Summary statistics
    pub summary: ReportSummary,
    /// All violations found
    pub violations: Vec<Violation>,
    /// Violations grouped by crate
    pub by_crate: HashMap<String, Vec<Violation>>,
    /// Violations grouped by category
    pub by_category: HashMap<ViolationCategory, Vec<Violation>>,
}

impl ValidationReport {
    /// Create a new report from a list of violations
    pub fn from_violations(violations: Vec<Violation>, total_crates: usize) -> Self {
        let mut by_crate: HashMap<String, Vec<Violation>> = HashMap::new();
        let mut by_category: HashMap<ViolationCategory, Vec<Violation>> = HashMap::new();
        
        for violation in &violations {
            by_crate
                .entry(violation.crate_name.clone())
                .or_default()
                .push(violation.clone());
            by_category
                .entry(violation.category)
                .or_default()
                .push(violation.clone());
        }
        
        let compliant_crates = total_crates - by_crate.len();
        let critical_violations = violations
            .iter()
            .filter(|v| v.severity == Severity::Error)
            .count();
        let auto_fixable = violations.iter().filter(|v| v.is_auto_fixable()).count();
        
        let summary = ReportSummary {
            total_crates,
            compliant_crates,
            total_violations: violations.len(),
            critical_violations,
            auto_fixable,
        };
        
        Self {
            summary,
            violations,
            by_crate,
            by_category,
        }
    }
    
    /// Check if the report has any errors
    pub fn has_errors(&self) -> bool {
        self.summary.critical_violations > 0
    }
    
    /// Check if all crates are compliant
    pub fn is_compliant(&self) -> bool {
        self.summary.total_violations == 0
    }
}

/// Summary statistics for a validation report
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReportSummary {
    /// Total number of crates scanned
    pub total_crates: usize,
    /// Number of crates with no violations
    pub compliant_crates: usize,
    /// Total number of violations found
    pub total_violations: usize,
    /// Number of error-level violations
    pub critical_violations: usize,
    /// Number of violations that can be auto-fixed
    pub auto_fixable: usize,
}

impl ReportSummary {
    /// Calculate compliance percentage
    pub fn compliance_percentage(&self) -> f64 {
        if self.total_crates == 0 {
            100.0
        } else {
            (self.compliant_crates as f64 / self.total_crates as f64) * 100.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_violation_creation() {
        let violation = Violation::new(
            "metadata-001",
            "test-crate",
            ViolationCategory::Metadata,
            Severity::Error,
            "Missing version.workspace",
        )
        .with_file("Cargo.toml")
        .with_line(5);
        
        assert_eq!(violation.id, "metadata-001");
        assert_eq!(violation.crate_name, "test-crate");
        assert_eq!(violation.category, ViolationCategory::Metadata);
        assert_eq!(violation.severity, Severity::Error);
        assert_eq!(violation.file, Some(PathBuf::from("Cargo.toml")));
        assert_eq!(violation.line, Some(5));
    }

    #[test]
    fn test_report_from_violations() {
        let violations = vec![
            Violation::new("m-001", "crate-a", ViolationCategory::Metadata, Severity::Error, "Error 1"),
            Violation::new("m-002", "crate-a", ViolationCategory::Naming, Severity::Warning, "Warning 1"),
            Violation::new("m-003", "crate-b", ViolationCategory::Metadata, Severity::Error, "Error 2"),
        ];
        
        let report = ValidationReport::from_violations(violations, 5);
        
        assert_eq!(report.summary.total_crates, 5);
        assert_eq!(report.summary.compliant_crates, 3); // 5 - 2 crates with violations
        assert_eq!(report.summary.total_violations, 3);
        assert_eq!(report.summary.critical_violations, 2);
        assert_eq!(report.by_crate.len(), 2);
        assert_eq!(report.by_category.len(), 2);
    }

    #[test]
    fn test_fix_auto_fixable() {
        let fix = Fix::new("Add version.workspace = true", true)
            .with_change(FileChange::new(
                "Cargo.toml",
                ChangeOperation::Modify {
                    old: "version = \"0.1.0\"".to_string(),
                    new: "version.workspace = true".to_string(),
                },
            ));
        
        let violation = Violation::new(
            "m-001",
            "test",
            ViolationCategory::Metadata,
            Severity::Error,
            "Missing workspace inheritance",
        )
        .with_fix(fix);
        
        assert!(violation.is_auto_fixable());
    }
}
