use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "dx")]
#[command(about = "The Unified Core - CLI & Orchestrator for the dx ecosystem", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Dx project
    New {
        /// Project name
        name: String,

        /// Use template (minimal, counter, todomvc)
        #[arg(short, long, default_value = "minimal")]
        template: String,
    },

    /// Build the project into optimized .dxb artifacts
    Build {
        /// Entry point directory (default: pages)
        #[arg(short, long, default_value = "pages")]
        entry: PathBuf,

        /// Output directory (default: dist)
        #[arg(short, long, default_value = "dist")]
        output: PathBuf,

        /// Enable verbose logging
        #[arg(short, long)]
        verbose: bool,

        /// Skip WASM optimization (faster builds)
        #[arg(long)]
        skip_optimize: bool,
    },

    /// Start development mode with hot-swap
    Dev {
        /// Entry point directory (default: pages)
        #[arg(short, long, default_value = "pages")]
        entry: PathBuf,

        /// Port for dev server (default: 3000)
        #[arg(short, long, default_value = "3000")]
        port: u16,

        /// Enable verbose logging
        #[arg(short, long)]
        verbose: bool,
    },

    /// Compile styles (standalone)
    Style {
        /// Input file or directory
        input: PathBuf,

        /// Output file
        output: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { name, template } => {
            // Forward to dx-forge
            dx_forge::init(&name, &template)?;
        }
        Commands::Build {
            entry,
            output,
            verbose,
            skip_optimize,
        } => {
            dx_www::cmd::build(entry, output, verbose, skip_optimize).await?;
        }
        Commands::Dev {
            entry,
            port,
            verbose,
        } => {
            dx_www::cmd::dev(entry, port, verbose).await?;
        }
        Commands::Style { input, output } => {
            dx_style::compile(input, output)?;
        }
    }

    Ok(())
}
