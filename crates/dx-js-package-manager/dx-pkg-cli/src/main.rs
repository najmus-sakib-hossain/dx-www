//! dx-pkg-cli: Command-Line Interface
//!
//! Commands:
//! - dx install [packages...]
//! - dx add <package>
//! - dx remove <package>

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod background;
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

        /// Use frozen lockfile (CI mode)
        #[arg(long)]
        frozen: bool,

        /// Install production dependencies only
        #[arg(long)]
        production: bool,

        /// Use npm proxy mode (zero infrastructure)
        #[arg(long, default_value = "true")]
        npm_mode: bool,

        /// Use v3 Binary Dawn mode (5 innovations)
        #[arg(long)]
        v3: bool,
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
    /// Run benchmark
    Benchmark {
        /// Number of runs
        #[arg(short, long, default_value = "3")]
        runs: usize,

        /// Use v3 mode
        #[arg(long)]
        v3: bool,
    },
    /// Show version
    Version,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Install {
            packages,
            frozen,
            production,
            npm_mode,
            v3,
        } => {
            if v3 {
                // Use new v3.0 Binary Dawn mode
                commands::install_v3::install_v3(frozen, production).await?;
            } else if npm_mode && packages.is_empty() {
                // Use v1.6 npm proxy mode for full installs
                commands::install_npm::install(frozen, production).await?;
            } else {
                // Use old mode for specific packages or when npm_mode=false
                commands::install::run(packages, cli.verbose).await?;
            }
        }
        Commands::Benchmark { runs, v3 } => {
            if v3 {
                commands::install_v3::benchmark_v3(runs).await?;
            } else {
                println!("⚠️  Benchmark only available for v3 mode");
                println!("   Use: dx benchmark --v3 --runs 3");
            }
        }
        Commands::Add { package, dev } => {
            println!("⚠️  'add' command not yet implemented in npm proxy mode");
            println!("   Please add '{}' to package.json manually and run 'dx install'", package);
        }
        Commands::Remove { package } => {
            println!("⚠️  'remove' command not yet implemented in npm proxy mode");
            println!(
                "   Please remove '{}' from package.json manually and run 'dx install'",
                package
            );
        }
        Commands::Version => {
            println!("dx v3.0.0 (Binary Dawn Edition)");
            println!("3x faster than Bun - Made with ⚡ by Dx");
        }
    }

    Ok(())
}
