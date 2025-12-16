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
        Commands::Install { packages } => {
            commands::install::run(packages, cli.verbose).await?;
        }
        Commands::Add { package, dev } => {
            commands::add::run(&package, dev, cli.verbose).await?;
        }
        Commands::Remove { package } => {
            commands::remove::run(&package, cli.verbose).await?;
        }
        Commands::Version => {
            println!("dx v0.1.0 (binary-first package manager)");
            println!("50x faster than Bun - Made with ⚡ by Dx");
        }
    }

    Ok(())
}
