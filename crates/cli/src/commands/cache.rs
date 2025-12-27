//! Cache Management Commands
//!
//! Commands for managing the .dx cache directory.

use anyhow::Result;
use clap::Subcommand;
use console::style;

use crate::dx_dir::{format_size, DxDir, DX_SUBDIRS};

/// Cache management commands
#[derive(Subcommand, Debug)]
pub enum CacheCommands {
    /// Initialize .dx directory structure
    Init,

    /// Show cache status and sizes
    Status {
        /// Show detailed breakdown
        #[arg(short, long)]
        detailed: bool,
    },

    /// Clean cache directories
    Clean {
        /// Specific subdirectory to clean (e.g., "forge", "cache")
        #[arg(short, long)]
        target: Option<String>,

        /// Clean all cache directories
        #[arg(short, long)]
        all: bool,

        /// Skip confirmation
        #[arg(short = 'y', long)]
        yes: bool,
    },

    /// Show path to .dx directory
    Path {
        /// Specific subdirectory
        subdir: Option<String>,
    },

    /// List all cache subdirectories
    List,
}

impl CacheCommands {
    pub async fn execute(self) -> Result<()> {
        let project_root = std::env::current_dir()?;
        let dx_dir = DxDir::new(&project_root);

        match self {
            CacheCommands::Init => init_cache(&dx_dir).await,
            CacheCommands::Status { detailed } => show_status(&dx_dir, detailed).await,
            CacheCommands::Clean { target, all, yes } => clean_cache(&dx_dir, target, all, yes).await,
            CacheCommands::Path { subdir } => show_path(&dx_dir, subdir).await,
            CacheCommands::List => list_subdirs().await,
        }
    }
}

async fn init_cache(dx_dir: &DxDir) -> Result<()> {
    println!(
        "{} Initializing .dx directory structure...",
        style("⚙").cyan()
    );

    dx_dir.init()?;

    println!(
        "{} Created .dx directory with {} subdirectories",
        style("✓").green(),
        DX_SUBDIRS.len()
    );

    println!();
    println!("  Directory: {}", style(dx_dir.root().display()).cyan());
    println!();
    println!("  Subdirectories:");
    for (i, subdir) in DX_SUBDIRS.iter().enumerate() {
        let prefix = if i == DX_SUBDIRS.len() - 1 { "└──" } else { "├──" };
        println!("  {} {}", prefix, style(subdir).dim());
    }

    Ok(())
}

async fn show_status(dx_dir: &DxDir, detailed: bool) -> Result<()> {
    if !dx_dir.exists() {
        println!(
            "{} .dx directory not found",
            style("⚠").yellow()
        );
        println!(
            "  Initialize with: {}",
            style("dx cache init").cyan()
        );
        return Ok(());
    }

    let total_size = dx_dir.total_size()?;
    
    println!(
        "{} DX Cache Status",
        style("📦").cyan()
    );
    println!();
    println!(
        "  Location:   {}",
        style(dx_dir.root().display()).cyan()
    );
    println!(
        "  Total size: {}",
        style(format_size(total_size)).yellow()
    );
    println!();

    if detailed {
        let status = dx_dir.status();
        
        println!("  {:<20} {:>10} {:>8}", 
            style("Directory").bold(),
            style("Size").bold(),
            style("Files").bold()
        );
        println!("  {}", "─".repeat(40));

        for s in &status {
            if s.exists && (s.size > 0 || s.file_count > 0) {
                println!(
                    "  {:<20} {:>10} {:>8}",
                    s.name,
                    format_size(s.size),
                    s.file_count
                );
            }
        }

        // Show empty directories
        let empty: Vec<_> = status.iter().filter(|s| !s.exists || s.size == 0).collect();
        if !empty.is_empty() {
            println!();
            println!("  {} Empty directories: {}", 
                style("ℹ").blue(),
                empty.iter().map(|s| s.name.as_str()).collect::<Vec<_>>().join(", ")
            );
        }
    } else {
        // Show top 5 by size
        let mut status = dx_dir.status();
        status.sort_by(|a, b| b.size.cmp(&a.size));
        
        let top: Vec<_> = status.iter().filter(|s| s.size > 0).take(5).collect();
        
        if !top.is_empty() {
            println!("  Top directories by size:");
            for s in top {
                println!(
                    "    {} {} ({})",
                    style("•").dim(),
                    s.name,
                    style(format_size(s.size)).yellow()
                );
            }
        }
        
        println!();
        println!(
            "  Use {} for detailed breakdown",
            style("dx cache status --detailed").cyan()
        );
    }

    Ok(())
}

async fn clean_cache(dx_dir: &DxDir, target: Option<String>, all: bool, yes: bool) -> Result<()> {
    if !dx_dir.exists() {
        println!(
            "{} .dx directory not found",
            style("ℹ").blue()
        );
        return Ok(());
    }

    if let Some(target) = target {
        // Clean specific subdirectory
        if !DX_SUBDIRS.contains(&target.as_str()) {
            println!(
                "{} Unknown subdirectory: {}",
                style("✗").red(),
                target
            );
            println!(
                "  Available: {}",
                DX_SUBDIRS.join(", ")
            );
            return Ok(());
        }

        let size = dx_dir.subdir_size(&target)?;
        
        if !yes {
            println!(
                "{} Clean .dx/{} ({})? [y/N]",
                style("?").yellow(),
                target,
                format_size(size)
            );
            
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            if !input.trim().eq_ignore_ascii_case("y") {
                println!("  Cancelled");
                return Ok(());
            }
        }

        dx_dir.clean_subdir(&target)?;
        println!(
            "{} Cleaned .dx/{}",
            style("✓").green(),
            target
        );
    } else if all {
        // Clean all
        let total_size = dx_dir.total_size()?;
        
        if !yes {
            println!(
                "{} Clean ALL .dx cache directories ({})? [y/N]",
                style("⚠").yellow(),
                format_size(total_size)
            );
            
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            if !input.trim().eq_ignore_ascii_case("y") {
                println!("  Cancelled");
                return Ok(());
            }
        }

        dx_dir.clean_all()?;
        println!(
            "{} Cleaned all .dx cache directories",
            style("✓").green()
        );
    } else {
        println!(
            "{} Specify --target <dir> or --all",
            style("ℹ").blue()
        );
        println!();
        println!("  Examples:");
        println!("    {} - Clean forge cache", style("dx cache clean --target forge").cyan());
        println!("    {} - Clean all caches", style("dx cache clean --all").cyan());
    }

    Ok(())
}

async fn show_path(dx_dir: &DxDir, subdir: Option<String>) -> Result<()> {
    if let Some(subdir) = subdir {
        if !DX_SUBDIRS.contains(&subdir.as_str()) {
            println!(
                "{} Unknown subdirectory: {}",
                style("✗").red(),
                subdir
            );
            return Ok(());
        }
        println!("{}", dx_dir.subdir(&subdir).display());
    } else {
        println!("{}", dx_dir.root().display());
    }
    Ok(())
}

async fn list_subdirs() -> Result<()> {
    println!(
        "{} DX Cache Subdirectories",
        style("📁").cyan()
    );
    println!();
    
    for subdir in DX_SUBDIRS {
        println!("  .dx/{}", style(subdir).cyan());
    }
    
    println!();
    println!("  Total: {} directories", DX_SUBDIRS.len());

    Ok(())
}
