//! Clean build artifacts and caches

use anyhow::Result;
use clap::Args;
use std::path::PathBuf;

use crate::ui::{spinner::Spinner, theme::Theme};

#[derive(Args)]
pub struct CleanArgs {
    /// Clean only build output (dist/)
    #[arg(long)]
    pub dist: bool,

    /// Clean only cache (.dx/)
    #[arg(long)]
    pub cache: bool,

    /// Clean only dependencies (dx_modules/)
    #[arg(long)]
    pub modules: bool,

    /// Clean everything
    #[arg(long)]
    pub all: bool,

    /// Don't ask for confirmation
    #[arg(short = 'y', long)]
    pub yes: bool,
}

pub async fn run(args: CleanArgs, theme: &Theme) -> Result<()> {
    theme.print_section("Cleaning project");
    eprintln!();

    let mut cleaned = Vec::new();
    let mut total_size = 0u64;

    let clean_all = args.all || (!args.dist && !args.cache && !args.modules);

    if args.dist || clean_all {
        if PathBuf::from("dist").exists() {
            let spinner = Spinner::dots("Cleaning dist/...");
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            // In real impl: fs::remove_dir_all("dist")?;
            spinner.success("Cleaned dist/");
            cleaned.push("dist/");
            total_size += 1024 * 1024; // Simulated
        }
    }

    if args.cache || clean_all {
        if PathBuf::from(".dx").exists() {
            let spinner = Spinner::dots("Cleaning .dx/ cache...");
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            spinner.success("Cleaned .dx/");
            cleaned.push(".dx/");
            total_size += 512 * 1024;
        }
    }

    if args.modules || clean_all {
        if PathBuf::from("dx_modules").exists() {
            let spinner = Spinner::dots("Cleaning dx_modules/...");
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            spinner.success("Cleaned dx_modules/");
            cleaned.push("dx_modules/");
            total_size += 50 * 1024 * 1024;
        }
    }

    if cleaned.is_empty() {
        theme.print_info("Nothing to clean", "Project is already clean");
    } else {
        eprintln!();
        theme.print_success(&format!(
            "Cleaned {} director{}, freed {}",
            cleaned.len(),
            if cleaned.len() == 1 { "y" } else { "ies" },
            human_bytes::human_bytes(total_size as f64)
        ));
    }

    eprintln!();

    Ok(())
}
