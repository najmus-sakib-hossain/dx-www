//! Crate validation script for the DX monorepo
//!
//! This script scans all crates in the repository and validates them against
//! professionalization requirements including:
//! - Required files (README.md, Cargo.toml, src/)
//! - README content completeness
//! - Naming conventions
//! - Cargo.toml metadata

mod scanner;
mod readme;
mod naming;
mod validation;

use anyhow::Result;
use colored::Colorize;

fn main() -> Result<()> {
    let root = std::env::current_dir()?;
    let crates_dir = root.join("crates");

    if !crates_dir.exists() {
        eprintln!("{}", "Error: crates/ directory not found. Run from repository root.".red());
        std::process::exit(1);
    }

    println!("{}", "=".repeat(70).blue());
    println!("{}", "  DX Crates Professionalization Validator".blue().bold());
    println!("{}", "=".repeat(70).blue());
    println!();

    // Scan for all crates
    let crates = scanner::scan_crates(&crates_dir)?;
    println!("Found {} crates to validate\n", crates.len().to_string().green());

    // Validate each crate
    let mut total_errors = 0;
    let mut total_warnings = 0;

    for crate_info in &crates {
        let result = validation::validate_crate(crate_info);
        
        let errors: Vec<_> = result.issues.iter()
            .filter(|i| matches!(i.severity, validation::Severity::Error))
            .collect();
        let warnings: Vec<_> = result.issues.iter()
            .filter(|i| matches!(i.severity, validation::Severity::Warning))
            .collect();

        if !errors.is_empty() || !warnings.is_empty() {
            println!("{}", format!("📦 {}", crate_info.path.display()).cyan().bold());
            
            for issue in &errors {
                println!("  {} {}", "✗".red(), issue.message);
                if let Some(fix) = &issue.fix_suggestion {
                    println!("    {} {}", "→".yellow(), fix);
                }
            }
            
            for issue in &warnings {
                println!("  {} {}", "⚠".yellow(), issue.message);
                if let Some(fix) = &issue.fix_suggestion {
                    println!("    {} {}", "→".yellow(), fix);
                }
            }
            println!();
        }

        total_errors += errors.len();
        total_warnings += warnings.len();
    }

    // Summary
    println!("{}", "=".repeat(70).blue());
    println!("{}", "  Summary".blue().bold());
    println!("{}", "=".repeat(70).blue());
    println!("  Crates scanned: {}", crates.len());
    println!("  Errors:         {}", if total_errors > 0 { 
        total_errors.to_string().red().to_string() 
    } else { 
        "0".green().to_string() 
    });
    println!("  Warnings:       {}", if total_warnings > 0 { 
        total_warnings.to_string().yellow().to_string() 
    } else { 
        "0".green().to_string() 
    });

    if total_errors > 0 {
        std::process::exit(1);
    }

    Ok(())
}
