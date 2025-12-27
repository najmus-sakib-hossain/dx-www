//! dx-crate-lint CLI
//!
//! Command-line interface for validating DX ecosystem crate standards.

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use dx_crate_lint::{
    CrateScanner,
    ReportGenerator, OutputFormat,
    MetadataValidator, NamingValidator, DocumentationValidator,
    LicenseValidator, StructureValidator, DependencyValidator,
    ValidationReport, Violation,
};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "dx-crate-lint")]
#[command(about = "Validation tool for DX ecosystem crate standards")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Lint the workspace for violations
    Lint {
        /// Output format (terminal, json, markdown)
        #[arg(short, long, default_value = "terminal")]
        format: String,
        
        /// Path to workspace root
        #[arg(short, long, default_value = ".")]
        path: String,
        
        /// Only show errors (hide warnings and info)
        #[arg(long)]
        errors_only: bool,
        
        /// Fail with exit code 1 if any violations found
        #[arg(long)]
        strict: bool,
    },
    
    /// Generate a detailed report
    Report {
        /// Output file path
        #[arg(short, long)]
        output: Option<String>,
        
        /// Output format (json, markdown)
        #[arg(short, long, default_value = "markdown")]
        format: String,
        
        /// Path to workspace root
        #[arg(short, long, default_value = ".")]
        path: String,
    },
    
    /// Auto-fix violations
    Fix {
        /// Skip confirmation prompts
        #[arg(short, long)]
        yes: bool,
        
        /// Dry run (show what would be fixed)
        #[arg(long)]
        dry_run: bool,
        
        /// Path to workspace root
        #[arg(short, long, default_value = ".")]
        path: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Lint { format, path, errors_only, strict } => {
            run_lint(&path, &format, errors_only, strict)
        }
        Commands::Report { output, format, path } => {
            run_report(&path, &format, output.as_deref())
        }
        Commands::Fix { yes, dry_run, path } => {
            run_fix(&path, yes, dry_run)
        }
    }
}

/// Run the lint command
fn run_lint(path: &str, format: &str, errors_only: bool, strict: bool) -> Result<()> {
    let workspace_path = PathBuf::from(path);
    
    // Parse output format
    let output_format = OutputFormat::from_str(format)
        .context(format!("Invalid output format: {}. Use terminal, json, or markdown", format))?;
    
    // Scan workspace
    let scanner = CrateScanner::new(&workspace_path)
        .context("Failed to create crate scanner")?;
    
    let crates = scanner.scan()
        .context("Failed to scan workspace")?;
    
    if crates.is_empty() {
        println!("No crates found in workspace");
        return Ok(());
    }
    
    // Run validators
    let mut all_violations: Vec<Violation> = Vec::new();
    
    let metadata_validator = MetadataValidator::new(None);
    let naming_validator = NamingValidator::new();
    let doc_validator = DocumentationValidator::new();
    let license_validator = LicenseValidator::new();
    let structure_validator = StructureValidator::new();
    let dep_validator = DependencyValidator::new();
    
    for crate_info in &crates {
        all_violations.extend(metadata_validator.validate(crate_info));
        all_violations.extend(naming_validator.validate(crate_info));
        all_violations.extend(doc_validator.validate(crate_info));
        all_violations.extend(license_validator.validate(crate_info));
        all_violations.extend(structure_validator.validate(crate_info));
        all_violations.extend(dep_validator.validate(crate_info));
    }
    
    // Check for version conflicts across crates
    all_violations.extend(dep_validator.find_duplicates(&crates));
    
    // Filter violations if errors_only
    if errors_only {
        all_violations.retain(|v| v.severity == dx_crate_lint::Severity::Error);
    }
    
    // Generate report
    let report = ValidationReport::from_violations(all_violations, crates.len());
    let generator = ReportGenerator::new();
    let output = generator.generate(&report, output_format);
    
    println!("{}", output);
    
    // Exit with error code if strict mode and violations found
    if strict && report.has_errors() {
        std::process::exit(1);
    }
    
    Ok(())
}

/// Run the report command
fn run_report(path: &str, format: &str, output_file: Option<&str>) -> Result<()> {
    let workspace_path = PathBuf::from(path);
    
    // Parse output format
    let output_format = OutputFormat::from_str(format)
        .context(format!("Invalid output format: {}. Use json or markdown", format))?;
    
    // Scan workspace
    let scanner = CrateScanner::new(&workspace_path)
        .context("Failed to create crate scanner")?;
    
    let crates = scanner.scan()
        .context("Failed to scan workspace")?;
    
    if crates.is_empty() {
        println!("No crates found in workspace");
        return Ok(());
    }
    
    // Run validators
    let mut all_violations: Vec<Violation> = Vec::new();
    
    let metadata_validator = MetadataValidator::new(None);
    let naming_validator = NamingValidator::new();
    let doc_validator = DocumentationValidator::new();
    let license_validator = LicenseValidator::new();
    let structure_validator = StructureValidator::new();
    let dep_validator = DependencyValidator::new();
    
    for crate_info in &crates {
        all_violations.extend(metadata_validator.validate(crate_info));
        all_violations.extend(naming_validator.validate(crate_info));
        all_violations.extend(doc_validator.validate(crate_info));
        all_violations.extend(license_validator.validate(crate_info));
        all_violations.extend(structure_validator.validate(crate_info));
        all_violations.extend(dep_validator.validate(crate_info));
    }
    
    // Check for version conflicts
    all_violations.extend(dep_validator.find_duplicates(&crates));
    
    // Generate report
    let report = ValidationReport::from_violations(all_violations, crates.len());
    let generator = ReportGenerator::new();
    let output = generator.generate(&report, output_format);
    
    // Write to file or stdout
    if let Some(file_path) = output_file {
        std::fs::write(file_path, &output)
            .context(format!("Failed to write report to {}", file_path))?;
        println!("Report written to: {}", file_path);
    } else {
        println!("{}", output);
    }
    
    Ok(())
}

/// Run the fix command
fn run_fix(path: &str, auto_confirm: bool, dry_run: bool) -> Result<()> {
    let workspace_path = PathBuf::from(path);
    
    // Scan workspace
    let scanner = CrateScanner::new(&workspace_path)
        .context("Failed to create crate scanner")?;
    
    let crates = scanner.scan()
        .context("Failed to scan workspace")?;
    
    if crates.is_empty() {
        println!("No crates found in workspace");
        return Ok(());
    }
    
    // Use AutoFixer to generate fixes for all crates
    let auto_fixer = dx_crate_lint::AutoFixer::new();
    let mut all_fixes = Vec::new();
    
    for crate_info in &crates {
        all_fixes.extend(auto_fixer.generate_fixes(crate_info));
    }
    
    // Also collect fixes from violations
    let mut all_violations: Vec<Violation> = Vec::new();
    
    let metadata_validator = MetadataValidator::new(None);
    let naming_validator = NamingValidator::new();
    let doc_validator = DocumentationValidator::new();
    let license_validator = LicenseValidator::new();
    let structure_validator = StructureValidator::new();
    let dep_validator = DependencyValidator::new();
    
    for crate_info in &crates {
        all_violations.extend(metadata_validator.validate(crate_info));
        all_violations.extend(naming_validator.validate(crate_info));
        all_violations.extend(doc_validator.validate(crate_info));
        all_violations.extend(license_validator.validate(crate_info));
        all_violations.extend(structure_validator.validate(crate_info));
        all_violations.extend(dep_validator.validate(crate_info));
    }
    
    // Add fixes from violations
    all_fixes.extend(auto_fixer.fixes_from_violations(&all_violations));
    
    // Deduplicate fixes by description
    all_fixes.sort_by(|a, b| a.description.cmp(&b.description));
    all_fixes.dedup_by(|a, b| a.description == b.description);
    
    // Filter to auto-fixable only
    let fixable: Vec<_> = all_fixes.iter().filter(|f| f.auto_fixable).collect();
    
    if fixable.is_empty() {
        println!("No auto-fixable violations found");
        return Ok(());
    }
    
    println!("Found {} auto-fixable issues:", fixable.len());
    for fix in &fixable {
        println!("  - {}", fix.description);
        for change in &fix.changes {
            println!("    File: {}", change.file.display());
        }
    }
    
    if dry_run {
        println!("\n(dry run - no changes made)");
        return Ok(());
    }
    
    // Confirm before applying fixes
    if !auto_confirm {
        println!("\nApply these fixes? [y/N]");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Aborted");
            return Ok(());
        }
    }
    
    // Apply fixes using AutoFixer
    let applied = auto_fixer.apply_all_fixes(&all_fixes)
        .context("Failed to apply fixes")?;
    
    println!("\nApplied {} fixes", applied);
    
    Ok(())
}
