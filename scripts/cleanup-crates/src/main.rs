//! Cleanup script for crates professionalization
//!
//! This script removes or relocates development artifacts from the crates directory.
//! It supports dry-run mode for safe preview of changes.

use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(name = "cleanup-crates")]
#[command(about = "Clean up development artifacts from crates directory")]
struct Args {
    /// Dry run mode - show what would be done without making changes
    #[arg(short, long, default_value = "true")]
    dry_run: bool,

    /// Actually perform the cleanup (disables dry-run)
    #[arg(long)]
    execute: bool,
}

/// Files/patterns to remove from crate directories
const REMOVE_PATTERNS: &[&str] = &[
    ".env",
];

/// Files to relocate to docs/archive/
const RELOCATE_PATTERNS: &[&str] = &[
    "PHASE",
    "TASKLIST",
    "PROGRESS",
    "_COMPLETE",
    "_STATUS",
    "IMPLEMENTATION_STATUS",
];

/// Cargo.lock should be removed from library crates
const CARGO_LOCK: &str = "Cargo.lock";

fn main() -> Result<()> {
    let args = Args::parse();
    let dry_run = !args.execute;

    let root = std::env::current_dir()?;
    let crates_dir = root.join("crates");
    let archive_dir = root.join("docs").join("archive").join("crate-artifacts");

    if !crates_dir.exists() {
        eprintln!("{}", "Error: crates/ directory not found. Run from repository root.".red());
        std::process::exit(1);
    }

    println!("{}", "=".repeat(70).blue());
    println!("{}", "  DX Crates Cleanup Script".blue().bold());
    if dry_run {
        println!("{}", "  [DRY RUN MODE - No changes will be made]".yellow());
    }
    println!("{}", "=".repeat(70).blue());
    println!();

    let mut actions: Vec<CleanupAction> = Vec::new();

    // Scan for files to clean up
    for entry in WalkDir::new(&crates_dir)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| !is_excluded_dir(e.path()))
    {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        // Check for files to remove
        if REMOVE_PATTERNS.iter().any(|p| file_name == *p) {
            actions.push(CleanupAction::Remove(path.to_path_buf()));
            continue;
        }

        // Check for files to relocate
        let upper_name = file_name.to_uppercase();
        if RELOCATE_PATTERNS.iter().any(|p| upper_name.contains(p)) && file_name.ends_with(".md") {
            let relative = path.strip_prefix(&crates_dir).unwrap_or(path);
            let dest = archive_dir.join(relative);
            actions.push(CleanupAction::Relocate(path.to_path_buf(), dest));
            continue;
        }

        // Check for Cargo.lock in library crates
        if file_name == CARGO_LOCK {
            if is_library_crate(path.parent().unwrap()) {
                actions.push(CleanupAction::Remove(path.to_path_buf()));
            }
        }
    }

    // Print and optionally execute actions
    if actions.is_empty() {
        println!("{}", "No cleanup actions needed!".green());
        return Ok(());
    }

    println!("Found {} items to clean up:\n", actions.len());

    for action in &actions {
        match action {
            CleanupAction::Remove(path) => {
                println!("  {} {}", "DELETE".red(), path.display());
            }
            CleanupAction::Relocate(src, dest) => {
                println!("  {} {} → {}", "MOVE".yellow(), src.display(), dest.display());
            }
        }
    }

    println!();

    if dry_run {
        println!("{}", "To execute these changes, run with --execute flag".yellow());
    } else {
        println!("{}", "Executing cleanup...".green());
        
        // Create archive directory if needed
        if !archive_dir.exists() {
            fs::create_dir_all(&archive_dir)
                .context("Failed to create archive directory")?;
        }

        for action in &actions {
            match action {
                CleanupAction::Remove(path) => {
                    fs::remove_file(path)
                        .with_context(|| format!("Failed to remove {}", path.display()))?;
                    println!("  {} {}", "✓".green(), path.display());
                }
                CleanupAction::Relocate(src, dest) => {
                    if let Some(parent) = dest.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    fs::rename(src, dest)
                        .with_context(|| format!("Failed to move {} to {}", src.display(), dest.display()))?;
                    println!("  {} {} → {}", "✓".green(), src.display(), dest.display());
                }
            }
        }

        println!("\n{}", "Cleanup complete!".green().bold());
    }

    Ok(())
}

#[derive(Debug)]
enum CleanupAction {
    Remove(PathBuf),
    Relocate(PathBuf, PathBuf),
}

/// Check if a directory should be excluded from scanning
fn is_excluded_dir(path: &Path) -> bool {
    let excluded = [
        "target",
        "node_modules",
        ".git",
        ".vscode",
        "proptest-regressions",
    ];

    path.file_name()
        .and_then(|n| n.to_str())
        .map(|name| excluded.contains(&name))
        .unwrap_or(false)
}

/// Check if a crate directory is a library crate (has src/lib.rs, no src/main.rs)
fn is_library_crate(crate_dir: &Path) -> bool {
    let has_lib = crate_dir.join("src/lib.rs").exists();
    let has_main = crate_dir.join("src/main.rs").exists();
    
    // It's a library if it has lib.rs and no main.rs
    has_lib && !has_main
}
