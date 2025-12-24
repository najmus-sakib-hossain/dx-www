//! CLI Module
//!
//! Command-line interface for dx-check.

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

/// Dx Check - The binary-first linter
#[derive(Parser, Debug)]
#[command(
    name = "dx-check",
    version,
    about = "The binary-first linter - 10x faster than Biome",
    long_about = "Dx Check is a high-performance code linter and formatter built in Rust.\n\
                  It uses binary protocols and SIMD acceleration to achieve unprecedented speed."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Files or directories to check
    #[arg(default_value = ".")]
    pub paths: Vec<PathBuf>,

    /// Apply safe fixes automatically
    #[arg(short, long)]
    pub fix: bool,

    /// Output format
    #[arg(short, long, value_enum, default_value = "pretty")]
    pub format: OutputFormat,

    /// Number of threads (0 = auto)
    #[arg(short, long, default_value = "0")]
    pub threads: usize,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Suppress all output except errors
    #[arg(short, long)]
    pub quiet: bool,

    /// Configuration file path
    #[arg(short, long)]
    pub config: Option<PathBuf>,

    /// Disable caching
    #[arg(long)]
    pub no_cache: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Check files for issues
    Check {
        /// Files or directories to check
        #[arg(default_value = ".")]
        paths: Vec<PathBuf>,

        /// Apply safe fixes automatically
        #[arg(short, long)]
        fix: bool,
    },

    /// Format files
    Format {
        /// Files or directories to format
        #[arg(default_value = ".")]
        paths: Vec<PathBuf>,

        /// Check if files are formatted (don't modify)
        #[arg(long)]
        check: bool,
    },

    /// Initialize configuration
    Init {
        /// Overwrite existing configuration
        #[arg(short, long)]
        force: bool,
    },

    /// Show project analysis
    Analyze {
        /// Directory to analyze
        #[arg(default_value = ".")]
        path: PathBuf,
    },

    /// Manage rules
    Rule {
        #[command(subcommand)]
        command: RuleCommands,
    },

    /// Show cache information
    Cache {
        #[command(subcommand)]
        command: CacheCommands,
    },

    /// Run in watch mode
    Watch {
        /// Files or directories to watch
        #[arg(default_value = ".")]
        paths: Vec<PathBuf>,
    },

    /// Start LSP server
    Lsp,
}

#[derive(Subcommand, Debug)]
pub enum RuleCommands {
    /// List all available rules
    List {
        /// Filter by category
        #[arg(short, long)]
        category: Option<String>,

        /// Show only enabled rules
        #[arg(long)]
        enabled: bool,
    },

    /// Show rule details
    Show {
        /// Rule name
        rule: String,
    },

    /// Enable a rule
    Enable {
        /// Rule name
        rule: String,

        /// Severity level
        #[arg(short, long, value_enum, default_value = "warn")]
        severity: SeverityArg,
    },

    /// Disable a rule
    Disable {
        /// Rule name
        rule: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum CacheCommands {
    /// Show cache statistics
    Stats,

    /// Clear the cache
    Clear,

    /// Show cache directory path
    Path,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    /// Human-readable with colors
    Pretty,
    /// Compact single-line format
    Compact,
    /// JSON output
    Json,
    /// GitHub Actions format
    Github,
    /// JUnit XML format
    Junit,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum SeverityArg {
    Off,
    Warn,
    Error,
}

impl Cli {
    /// Parse CLI arguments
    pub fn parse_args() -> Self {
        Self::parse()
    }
}

/// Terminal output helpers
pub mod output {
    use colored::Colorize;
    use crate::diagnostics::{Diagnostic, DiagnosticSeverity, LineIndex};

    /// Print a diagnostic with context
    pub fn print_diagnostic(diagnostic: &Diagnostic, source: &str) {
        let line_index = LineIndex::new(source);
        let (start_lc, end_lc) = diagnostic.span.to_line_col(&line_index);

        // Header
        let severity_str = match diagnostic.severity {
            DiagnosticSeverity::Error => "error".red().bold(),
            DiagnosticSeverity::Warning => "warning".yellow().bold(),
            DiagnosticSeverity::Info => "info".blue().bold(),
            DiagnosticSeverity::Hint => "hint".cyan().bold(),
        };

        println!(
            "{}{}{} {}",
            severity_str,
            "[".dimmed(),
            diagnostic.rule_id.dimmed(),
            "]".dimmed(),
        );
        println!("  {} {}", "-->".blue(), diagnostic.file.display());
        println!();

        // Source context
        let lines: Vec<&str> = source.lines().collect();
        let line_num = start_lc.line as usize;
        
        if line_num > 0 && line_num <= lines.len() {
            let line = lines[line_num - 1];
            let line_num_str = format!("{:4}", line_num);
            
            println!("  {} {}", line_num_str.blue(), "|".blue());
            println!("  {} {} {}", line_num_str.blue(), "|".blue(), line);
            
            // Underline
            let start_col = (start_lc.col - 1) as usize;
            let end_col = if start_lc.line == end_lc.line {
                (end_lc.col - 1) as usize
            } else {
                line.len()
            };
            let underline_len = (end_col - start_col).max(1);
            
            let underline = format!(
                "{}{}",
                " ".repeat(start_col),
                "^".repeat(underline_len)
            );
            
            let underline_colored = match diagnostic.severity {
                DiagnosticSeverity::Error => underline.red(),
                DiagnosticSeverity::Warning => underline.yellow(),
                _ => underline.cyan(),
            };
            
            println!("  {} {} {}", "    ".blue(), "|".blue(), underline_colored);
        }

        // Message
        println!("  {} {}", "=".blue(), diagnostic.message);

        // Suggestion
        if let Some(ref suggestion) = diagnostic.suggestion {
            println!("  {} {}: {}", "=".blue(), "help".green(), suggestion);
        }

        println!();
    }

    /// Print summary
    pub fn print_summary(
        files_checked: usize,
        errors: usize,
        warnings: usize,
        duration_ms: u64,
        files_per_second: f64,
    ) {
        println!();
        
        if errors == 0 && warnings == 0 {
            println!(
                "{} {} files checked in {}ms ({:.0} files/sec)",
                "✓".green().bold(),
                files_checked,
                duration_ms,
                files_per_second
            );
        } else {
            let error_str = if errors > 0 {
                format!("{} errors", errors).red().bold().to_string()
            } else {
                String::new()
            };
            
            let warning_str = if warnings > 0 {
                format!("{} warnings", warnings).yellow().bold().to_string()
            } else {
                String::new()
            };
            
            let separator = if errors > 0 && warnings > 0 { ", " } else { "" };
            
            println!(
                "{} {} files checked: {}{}{} ({}ms, {:.0} files/sec)",
                "✗".red().bold(),
                files_checked,
                error_str,
                separator,
                warning_str,
                duration_ms,
                files_per_second
            );
        }
    }

    /// Print project profile
    pub fn print_profile(profile: &crate::project::ProjectProfile) {
        println!("{}", "🔍 Project Analysis".bold());
        println!("{}", "─".repeat(40));
        
        if !profile.frameworks.is_empty() {
            let frameworks: Vec<_> = profile.frameworks.iter().map(|f| f.as_str()).collect();
            println!("  {}: {}", "Frameworks".cyan(), frameworks.join(", "));
        }
        
        println!(
            "  {}: {}",
            "Language".cyan(),
            match profile.language {
                crate::project::Language::JavaScript => "JavaScript",
                crate::project::Language::TypeScript => "TypeScript",
            }
        );
        
        if let Some(ref test) = profile.test_framework {
            println!(
                "  {}: {:?}",
                "Test Runner".cyan(),
                test
            );
        }
        
        if let Some(ref mono) = profile.monorepo {
            println!(
                "  {}: {:?} ({} packages)",
                "Monorepo".cyan(),
                mono.kind,
                mono.packages.len()
            );
        }
        
        println!(
            "  {}: {:?}",
            "Package Manager".cyan(),
            profile.package_manager
        );
        
        println!();
        println!("{}", "📐 Inferred Style".bold());
        println!("{}", "─".repeat(40));
        println!(
            "  {}: {}",
            "Semicolons".cyan(),
            if profile.style.semicolons { "Yes" } else { "No" }
        );
        println!(
            "  {}: {:?}",
            "Quotes".cyan(),
            profile.style.quotes
        );
        println!(
            "  {}: {:?}",
            "Indentation".cyan(),
            profile.style.indent
        );
        println!();
    }
}
