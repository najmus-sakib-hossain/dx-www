//! Report generator for validation results

use crate::models::{ValidationReport, ViolationCategory, Severity};

/// Output format for reports
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    /// JSON format for CI/CD integration
    Json,
    /// Markdown format for documentation
    Markdown,
    /// Terminal output with colors
    Terminal,
}

impl OutputFormat {
    /// Parse from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "json" => Some(OutputFormat::Json),
            "markdown" | "md" => Some(OutputFormat::Markdown),
            "terminal" | "term" | "console" => Some(OutputFormat::Terminal),
            _ => None,
        }
    }
}

/// Generator for validation reports
pub struct ReportGenerator {
    /// Whether to use colors in terminal output
    use_colors: bool,
}

impl ReportGenerator {
    /// Create a new report generator
    pub fn new() -> Self {
        Self { use_colors: true }
    }
    
    /// Create a report generator without colors
    pub fn without_colors() -> Self {
        Self { use_colors: false }
    }
    
    /// Generate report in the specified format
    pub fn generate(&self, report: &ValidationReport, format: OutputFormat) -> String {
        match format {
            OutputFormat::Json => self.to_json(report),
            OutputFormat::Markdown => self.to_markdown(report),
            OutputFormat::Terminal => self.to_terminal(report),
        }
    }
    
    /// Generate JSON output for CI/CD integration
    pub fn to_json(&self, report: &ValidationReport) -> String {
        serde_json::to_string_pretty(report).unwrap_or_default()
    }
    
    /// Generate Markdown output for documentation
    pub fn to_markdown(&self, report: &ValidationReport) -> String {
        let mut output = String::new();
        
        // Header
        output.push_str("# Crate Lint Validation Report\n\n");
        
        // Summary
        output.push_str("## Summary\n\n");
        output.push_str(&format!(
            "| Metric | Value |\n\
             |--------|-------|\n\
             | Total Crates | {} |\n\
             | Compliant Crates | {} |\n\
             | Compliance Rate | {:.1}% |\n\
             | Total Violations | {} |\n\
             | Critical Violations | {} |\n\
             | Auto-fixable | {} |\n\n",
            report.summary.total_crates,
            report.summary.compliant_crates,
            report.summary.compliance_percentage(),
            report.summary.total_violations,
            report.summary.critical_violations,
            report.summary.auto_fixable,
        ));
        
        // Violations by category
        if !report.by_category.is_empty() {
            output.push_str("## Violations by Category\n\n");
            for category in ViolationCategory::all() {
                if let Some(violations) = report.by_category.get(category) {
                    if !violations.is_empty() {
                        output.push_str(&format!(
                            "- **{}**: {} violations\n",
                            category.display_name(),
                            violations.len()
                        ));
                    }
                }
            }
            output.push('\n');
        }
        
        // Violations by crate
        if !report.by_crate.is_empty() {
            output.push_str("## Violations by Crate\n\n");
            
            let mut crates: Vec<_> = report.by_crate.keys().collect();
            crates.sort();
            
            for crate_name in crates {
                if let Some(violations) = report.by_crate.get(crate_name) {
                    output.push_str(&format!("### {}\n\n", crate_name));
                    
                    for violation in violations {
                        let severity_icon = match violation.severity {
                            Severity::Error => "🔴",
                            Severity::Warning => "🟡",
                            Severity::Info => "🔵",
                        };
                        
                        output.push_str(&format!(
                            "- {} **{}**: {}\n",
                            severity_icon,
                            violation.category.display_name(),
                            violation.message
                        ));
                        
                        if let Some(file) = &violation.file {
                            output.push_str(&format!("  - File: `{}`\n", file.display()));
                        }
                        
                        if let Some(fix) = &violation.fix {
                            let fixable = if fix.auto_fixable { "✅ Auto-fixable" } else { "⚠️ Manual fix required" };
                            output.push_str(&format!("  - Fix: {} - {}\n", fixable, fix.description));
                        }
                    }
                    output.push('\n');
                }
            }
        }
        
        // Footer
        output.push_str("---\n\n");
        output.push_str("*Generated by dx-crate-lint*\n");
        
        output
    }
    
    /// Generate terminal output with colors
    pub fn to_terminal(&self, report: &ValidationReport) -> String {
        let mut output = String::new();
        
        // Reset color
        let reset = if self.use_colors { "\x1b[0m" } else { "" };
        let bold = if self.use_colors { "\x1b[1m" } else { "" };
        let green = if self.use_colors { "\x1b[32m" } else { "" };
        let yellow = if self.use_colors { "\x1b[33m" } else { "" };
        let red = if self.use_colors { "\x1b[31m" } else { "" };
        let cyan = if self.use_colors { "\x1b[36m" } else { "" };
        
        // Header
        output.push_str(&format!("\n{}=== Crate Lint Validation Report ==={}\n\n", bold, reset));
        
        // Summary
        let compliance_color = if report.summary.compliance_percentage() >= 90.0 {
            green
        } else if report.summary.compliance_percentage() >= 70.0 {
            yellow
        } else {
            red
        };
        
        output.push_str(&format!(
            "{}Summary:{}\n\
             ├─ Total crates:      {}\n\
             ├─ Compliant:         {}{}{}\n\
             ├─ Compliance rate:   {}{:.1}%{}\n\
             ├─ Total violations:  {}\n\
             ├─ Critical:          {}{}{}\n\
             └─ Auto-fixable:      {}\n\n",
            bold, reset,
            report.summary.total_crates,
            green, report.summary.compliant_crates, reset,
            compliance_color, report.summary.compliance_percentage(), reset,
            report.summary.total_violations,
            red, report.summary.critical_violations, reset,
            report.summary.auto_fixable,
        ));
        
        // Violations by category
        if !report.by_category.is_empty() {
            output.push_str(&format!("{}Violations by Category:{}\n", bold, reset));
            for category in ViolationCategory::all() {
                if let Some(violations) = report.by_category.get(category) {
                    if !violations.is_empty() {
                        output.push_str(&format!(
                            "├─ {}: {}\n",
                            category.display_name(),
                            violations.len()
                        ));
                    }
                }
            }
            output.push('\n');
        }
        
        // Detailed violations
        if !report.violations.is_empty() {
            output.push_str(&format!("{}Violations:{}\n", bold, reset));
            
            for violation in &report.violations {
                let severity_color = match violation.severity {
                    Severity::Error => red,
                    Severity::Warning => yellow,
                    Severity::Info => cyan,
                };
                
                output.push_str(&format!(
                    "{}[{}]{} {}{}{}: {}\n",
                    severity_color,
                    violation.severity.display_name(),
                    reset,
                    bold,
                    violation.crate_name,
                    reset,
                    violation.message
                ));
                
                if let Some(file) = &violation.file {
                    output.push_str(&format!("   └─ File: {}\n", file.display()));
                }
            }
            output.push('\n');
        }
        
        // Final status
        if report.is_compliant() {
            output.push_str(&format!("{}✓ All crates are compliant!{}\n", green, reset));
        } else if report.has_errors() {
            output.push_str(&format!(
                "{}✗ {} critical violations must be fixed before publishing{}\n",
                red, report.summary.critical_violations, reset
            ));
        } else {
            output.push_str(&format!(
                "{}⚠ {} warnings should be addressed{}\n",
                yellow, report.summary.total_violations, reset
            ));
        }
        
        output
    }
    
    /// Generate a compact summary line
    pub fn to_summary_line(&self, report: &ValidationReport) -> String {
        if report.is_compliant() {
            format!(
                "✓ {} crates validated, all compliant",
                report.summary.total_crates
            )
        } else {
            format!(
                "✗ {} crates validated, {} violations ({} critical)",
                report.summary.total_crates,
                report.summary.total_violations,
                report.summary.critical_violations
            )
        }
    }
}

impl Default for ReportGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Violation, ViolationCategory, Severity, Fix};

    fn create_test_report() -> ValidationReport {
        let violations = vec![
            Violation::new(
                "test-001",
                "crate-a",
                ViolationCategory::Metadata,
                Severity::Error,
                "Missing version.workspace",
            ),
            Violation::new(
                "test-002",
                "crate-a",
                ViolationCategory::Documentation,
                Severity::Warning,
                "Missing README.md",
            ).with_fix(Fix::new("Create README.md", true)),
            Violation::new(
                "test-003",
                "crate-b",
                ViolationCategory::License,
                Severity::Error,
                "Missing LICENSE file",
            ),
        ];
        
        ValidationReport::from_violations(violations, 5)
    }

    #[test]
    fn test_json_output() {
        let generator = ReportGenerator::new();
        let report = create_test_report();
        let json = generator.to_json(&report);
        
        assert!(json.contains("\"total_crates\": 5"));
        assert!(json.contains("\"total_violations\": 3"));
        assert!(json.contains("crate-a"));
        assert!(json.contains("crate-b"));
    }

    #[test]
    fn test_markdown_output() {
        let generator = ReportGenerator::new();
        let report = create_test_report();
        let md = generator.to_markdown(&report);
        
        assert!(md.contains("# Crate Lint Validation Report"));
        assert!(md.contains("## Summary"));
        assert!(md.contains("| Total Crates | 5 |"));
        assert!(md.contains("### crate-a"));
        assert!(md.contains("### crate-b"));
        assert!(md.contains("🔴")); // Error icon
        assert!(md.contains("🟡")); // Warning icon
    }

    #[test]
    fn test_terminal_output() {
        let generator = ReportGenerator::new();
        let report = create_test_report();
        let term = generator.to_terminal(&report);
        
        assert!(term.contains("=== Crate Lint Validation Report ==="));
        assert!(term.contains("Total crates:"));
        assert!(term.contains("crate-a"));
        assert!(term.contains("crate-b"));
    }

    #[test]
    fn test_terminal_output_no_colors() {
        let generator = ReportGenerator::without_colors();
        let report = create_test_report();
        let term = generator.to_terminal(&report);
        
        // Should not contain ANSI escape codes
        assert!(!term.contains("\x1b["));
    }

    #[test]
    fn test_summary_line_compliant() {
        let generator = ReportGenerator::new();
        let report = ValidationReport::from_violations(vec![], 5);
        let summary = generator.to_summary_line(&report);
        
        assert!(summary.contains("✓"));
        assert!(summary.contains("all compliant"));
    }

    #[test]
    fn test_summary_line_violations() {
        let generator = ReportGenerator::new();
        let report = create_test_report();
        let summary = generator.to_summary_line(&report);
        
        assert!(summary.contains("✗"));
        assert!(summary.contains("3 violations"));
        assert!(summary.contains("2 critical"));
    }

    #[test]
    fn test_output_format_parsing() {
        assert_eq!(OutputFormat::from_str("json"), Some(OutputFormat::Json));
        assert_eq!(OutputFormat::from_str("JSON"), Some(OutputFormat::Json));
        assert_eq!(OutputFormat::from_str("markdown"), Some(OutputFormat::Markdown));
        assert_eq!(OutputFormat::from_str("md"), Some(OutputFormat::Markdown));
        assert_eq!(OutputFormat::from_str("terminal"), Some(OutputFormat::Terminal));
        assert_eq!(OutputFormat::from_str("term"), Some(OutputFormat::Terminal));
        assert_eq!(OutputFormat::from_str("invalid"), None);
    }

    #[test]
    fn test_generate_method() {
        let generator = ReportGenerator::new();
        let report = create_test_report();
        
        let json = generator.generate(&report, OutputFormat::Json);
        assert!(json.starts_with('{'));
        
        let md = generator.generate(&report, OutputFormat::Markdown);
        assert!(md.starts_with("# "));
        
        let term = generator.generate(&report, OutputFormat::Terminal);
        assert!(term.contains("==="));
    }
}
