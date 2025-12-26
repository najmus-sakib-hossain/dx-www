//! DX CLI - The Unified Command Line Interface
//!
//! Controls all DX tools and the Forge daemon.

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod commands;
mod config;
mod daemon_client;
mod dx_dir;

use commands::{
    branch::BranchCommands, 
    cache::CacheCommands,
    config::ConfigCommands, 
    forge::ForgeCommands, 
    tools::ToolsCommands
};

// Internal crate imports
use dx_compiler as www;
use style;

#[derive(Parser)]
#[command(name = "dx")]
#[command(about = "The Unified CLI for the DX ecosystem", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Forge daemon control (start, stop, status)
    #[command(subcommand)]
    Forge(ForgeCommands),

    /// Tool management (list, run, enable, disable)
    #[command(subcommand)]
    Tools(ToolsCommands),

    /// Traffic branch control (status, approve, reject)
    #[command(subcommand)]
    Branch(BranchCommands),

    /// Configuration management
    #[command(subcommand)]
    Config(ConfigCommands),

    /// Cache directory management (.dx folder)
    #[command(subcommand)]
    Cache(CacheCommands),

    /// Create a new DX project
    New {
        /// Project name
        name: String,

        /// Use template (minimal, default, full)
        #[arg(short, long, default_value = "default")]
        template: String,
    },

    /// Build the project
    Build {
        /// Entry point directory
        #[arg(short, long, default_value = "pages")]
        entry: PathBuf,

        /// Output directory
        #[arg(short, long, default_value = "dist")]
        output: PathBuf,

        /// Enable verbose logging
        #[arg(short, long)]
        verbose: bool,

        /// Skip WASM optimization
        #[arg(long)]
        skip_optimize: bool,
    },

    /// Start development mode
    Dev {
        /// Entry point directory
        #[arg(short, long, default_value = "pages")]
        entry: PathBuf,

        /// Port for dev server
        #[arg(short, long, default_value = "3000")]
        port: u16,

        /// Enable verbose logging
        #[arg(short, long)]
        verbose: bool,
    },

    /// Compile styles
    Style {
        /// Input file or directory
        input: PathBuf,

        /// Output file
        output: PathBuf,
    },

    /// Show version information
    Version,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        // New subcommands
        Commands::Forge(cmd) => cmd.execute().await,
        Commands::Tools(cmd) => cmd.execute().await,
        Commands::Branch(cmd) => cmd.execute().await,
        Commands::Config(cmd) => cmd.execute().await,
        Commands::Cache(cmd) => cmd.execute().await,

        // Existing commands
        Commands::New { name, template } => {
            forge::init(&name, &template)?;
            Ok(())
        }
        Commands::Build {
            entry,
            output,
            verbose,
            skip_optimize,
        } => {
            www::cmd::build(entry, output, verbose, skip_optimize).await?;
            Ok(())
        }
        Commands::Dev {
            entry,
            port,
            verbose,
        } => {
            www::cmd::dev(entry, port, verbose).await?;
            Ok(())
        }
        Commands::Style { input, output } => {
            style::compile(input, output).map_err(|e| anyhow::anyhow!("{}", e))?;
            Ok(())
        }
        Commands::Version => {
            print_version();
            Ok(())
        }
    }
}

fn print_version() {
    use console::style;
    
    println!();
    println!(
        "  {} {}",
        style("DX CLI").cyan().bold(),
        style(env!("CARGO_PKG_VERSION")).dim()
    );
    println!();
    println!("  Components:");
    println!("    Forge:      {}", forge::VERSION);
    println!("    Serializer: {}", dx_serializer::zero::VERSION);
    println!();
}
