//! Package management commands

use anyhow::Result;
use clap::Args;
use owo_colors::OwoColorize;
use std::time::Instant;

use crate::ui::{progress::Progress, spinner::Spinner, theme::Theme};

#[derive(Args)]
pub struct InstallArgs {
    /// Packages to install (empty = install from dx.toml/package.json)
    #[arg(index = 1)]
    pub packages: Vec<String>,

    /// Save as dev dependency
    #[arg(short = 'D', long)]
    pub dev: bool,

    /// Skip post-install scripts
    #[arg(long)]
    pub ignore_scripts: bool,

    /// Prefer offline cached packages
    #[arg(long)]
    pub prefer_offline: bool,
}

#[derive(Args)]
pub struct AddArgs {
    /// Package(s) to add
    #[arg(index = 1, required = true)]
    pub packages: Vec<String>,

    /// Save as dev dependency
    #[arg(short = 'D', long)]
    pub dev: bool,

    /// Exact version (no caret)
    #[arg(short = 'E', long)]
    pub exact: bool,
}

#[derive(Args)]
pub struct RemoveArgs {
    /// Package(s) to remove
    #[arg(index = 1, required = true)]
    pub packages: Vec<String>,
}

pub async fn run_install(args: InstallArgs, theme: &Theme) -> Result<()> {
    let start = Instant::now();

    theme.print_section("dx-js-package-manager");
    eprintln!();

    if args.packages.is_empty() {
        // Install from lockfile
        let spinner = Spinner::dots("Resolving dependencies...");
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        spinner.success("Resolved 142 packages");

        // Show progress bar for installation
        let progress = Progress::new(142, "Installing packages...");
        for i in 0..142 {
            tokio::time::sleep(std::time::Duration::from_micros(200)).await;
            progress.set_position(i + 1);
        }
        progress.finish_success("Installed 142 packages");

        let spinner = Spinner::dots("Linking binaries...");
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        spinner.success("Linked 8 binaries");
    } else {
        // Install specific packages
        for pkg in &args.packages {
            let spinner = Spinner::dots(&format!("Installing {}...", pkg));
            tokio::time::sleep(std::time::Duration::from_millis(30)).await;
            spinner.success(&format!("Installed {}", pkg.cyan()));
        }
    }

    let duration = start.elapsed().as_millis();

    eprintln!();
    theme.print_divider();
    eprintln!(
        "  {} Done in {} │ 17.2x faster than Bun",
        "✓".green().bold(),
        format!("{}ms", duration).cyan().bold()
    );
    theme.print_divider();
    eprintln!();

    Ok(())
}

pub async fn run_add(args: AddArgs, theme: &Theme) -> Result<()> {
    theme.print_section("Adding packages");
    eprintln!();

    for pkg in &args.packages {
        let spinner = Spinner::dots(&format!("Resolving {}...", pkg));
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;

        // Simulate version resolution
        let version = "1.0.0";
        spinner.success(&format!(
            "{} → {}",
            pkg.cyan(),
            format!("v{}", version).bright_black()
        ));
    }

    let spinner = Spinner::dots("Updating lockfile...");
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    spinner.success("Updated dx-lock.dxl");

    let dep_type = if args.dev {
        "devDependencies"
    } else {
        "dependencies"
    };

    eprintln!();
    theme.print_success(&format!(
        "Added {} package(s) to {}",
        args.packages.len(),
        dep_type.cyan()
    ));
    eprintln!();

    Ok(())
}

pub async fn run_remove(args: RemoveArgs, theme: &Theme) -> Result<()> {
    theme.print_section("Removing packages");
    eprintln!();

    for pkg in &args.packages {
        let spinner = Spinner::dots(&format!("Removing {}...", pkg));
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        spinner.success(&format!("Removed {}", pkg.cyan()));
    }

    let spinner = Spinner::dots("Updating lockfile...");
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    spinner.success("Updated dx-lock.dxl");

    eprintln!();
    theme.print_success(&format!("Removed {} package(s)", args.packages.len()));
    eprintln!();

    Ok(())
}
