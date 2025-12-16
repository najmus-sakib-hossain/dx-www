//! dx-pkg-cli: Command-Line Interface
//!
//! Commands:
//! - dx install [packages...]
//! - dx add <package>
//! - dx remove <package>

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod commands;
mod background;

#[derive(Parser)]
#[command(name = "dx")]
#[command(about = "DX Package Manager - 50x faster than Bun", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Verbose output
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Install dependencies from package.json
    Install {
        /// Specific packages to install
        packages: Vec<String>,
        
        /// Use frozen lockfile (CI mode)
        #[arg(long)]
        frozen: bool,
        
        /// Install production dependencies only
        #[arg(long)]
        production: bool,
        
        /// Use npm proxy mode (zero infrastructure)
        #[arg(long, default_value = "true")]
        npm_mode: bool,
    },
    /// Add a package to dependencies
    Add {
        /// Package name (e.g., react, lodash@^4.17.0)
        package: String,
        
        /// Add to devDependencies
        #[arg(short = 'D', long)]
        dev: bool,
    },
    /// Remove a package
    Remove {
        /// Package name
        package: String,
    },
    /// Show version
    Version,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Install { packages, frozen, production, npm_mode } => {
            if npm_mode && packages.is_empty() {
                // Use new npm proxy mode for full installs
                commands::install_npm::install(frozen, production).await?;
            } else {
                // Use old mode for specific packages or when npm_mode=false
                commands::install::run(packages, cli.verbose).await?;
            }
        }
        Commands::Add { package, dev } => {
            println!("⚠️  'add' command not yet implemented in npm proxy mode");
            println!("   Please add '{}' to package.json manually and run 'dx install'", package);
        }
        Commands::Remove { package } => {
            println!("⚠️  'remove' command not yet implemented in npm proxy mode");
            println!("   Please remove '{}' from package.json manually and run 'dx install'", package);
        }
        Commands::Version => {
            println!("dx v0.1.0 (binary-first package manager)");
            println!("50x faster than Bun - Made with ⚡ by Dx");
        }
    }

    Ok(())
}
