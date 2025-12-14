/**
 * dx-cli: The Command-Line Orchestrator
 *
 * This is the entry point that connects:
 * - dx-compiler (TSX → Binary)
 * - dx-server (Binary → HTTP)
 * - dx-client (HTTP → Rendered App)
 *
 * Commands:
 * - `dx new <name>` - Scaffold a new project
 * - `dx dev` - Development server with hot reload
 * - `dx build` - Production build
 */
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use console::style;
use std::path::PathBuf;

mod commands;
mod config;
mod scaffold;
mod watch;

use commands::{build, dev, new_project};

/// dx-www: The Binary Web Runtime
///
/// Build web applications that compile to WebAssembly and render
/// via the HTIP protocol, achieving 338 bytes (Micro) to 7.5 KB (Macro)
/// bundle sizes with zero-parse, zero-GC, zero-hydration architecture.
#[derive(Parser)]
#[command(name = "dx")]
#[command(version, about, long_about = None)]
#[command(author = "Dx-WWW Team")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Set working directory
    #[arg(short = 'C', long, global = true)]
    directory: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new dx-www project
    New {
        /// Project name
        name: String,

        /// Target directory (default: current directory)
        #[arg(short, long)]
        path: Option<PathBuf>,

        /// Use template (counter, dashboard, hackernews)
        #[arg(short, long, default_value = "counter")]
        template: String,
    },

    /// Start development server with hot reload
    Dev {
        /// Port to bind server (default: 3000)
        #[arg(short, long, default_value = "3000")]
        port: u16,

        /// Host to bind server (default: localhost)
        #[arg(long, default_value = "localhost")]
        host: String,

        /// Open browser automatically
        #[arg(short, long)]
        open: bool,
    },

    /// Build for production
    Build {
        /// Enable release optimizations
        #[arg(short, long)]
        release: bool,

        /// Output directory (default: dist/)
        #[arg(short, long, default_value = "dist")]
        output: PathBuf,

        /// Skip WASM optimization (faster builds)
        #[arg(long)]
        skip_optimize: bool,
    },

    /// Show project information
    Info,

    /// Clean build artifacts
    Clean,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    init_tracing(cli.verbose);

    // Change directory if specified
    if let Some(dir) = cli.directory {
        std::env::set_current_dir(&dir)
            .with_context(|| format!("Failed to change directory to {:?}", dir))?;
    }

    // Print banner
    print_banner();

    // Execute command
    let result = match cli.command {
        Commands::New {
            name,
            path,
            template,
        } => new_project::execute(&name, path, &template).await,
        Commands::Dev { port, host, open } => dev::execute(port, &host, open).await,
        Commands::Build {
            release,
            output,
            skip_optimize,
        } => build::execute(release, output, skip_optimize).await,
        Commands::Info => show_info().await,
        Commands::Clean => clean_artifacts().await,
    };

    // Handle errors with pretty formatting
    if let Err(e) = result {
        eprintln!("\n{} {}", style("Error:").red().bold(), e);
        for cause in e.chain().skip(1) {
            eprintln!("  {} {}", style("Caused by:").red(), cause);
        }
        std::process::exit(1);
    }

    Ok(())
}

/// Initialize tracing/logging
fn init_tracing(verbose: bool) {
    use tracing_subscriber::{fmt, EnvFilter};

    let filter = if verbose {
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"))
    } else {
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"))
    };

    fmt().with_env_filter(filter).with_target(false).without_time().init();
}

/// Print the dx banner
fn print_banner() {
    println!(
        "{}",
        style(
            "
██████╗ ██╗  ██╗      ██╗    ██╗██╗    ██╗██╗    ██╗
██╔══██╗╚██╗██╔╝      ██║    ██║██║    ██║██║    ██║
██║  ██║ ╚███╔╝ █████╗██║ █╗ ██║██║ █╗ ██║██║ █╗ ██║
██║  ██║ ██╔██╗ ╚════╝██║███╗██║██║███╗██║██║███╗██║
██████╔╝██╔╝ ██╗      ╚███╔███╔╝╚███╔███╔╝╚███╔███╔╝
╚═════╝ ╚═╝  ╚═╝       ╚══╝╚══╝  ╚══╝╚══╝  ╚══╝╚══╝
    "
        )
        .cyan()
    );
    println!("{}", style("The Binary Web Runtime").cyan().dim());
    println!("{}\n", style("v0.4.0").dim());
}

/// Show project information
async fn show_info() -> Result<()> {
    use crate::config::ProjectConfig;

    let config = ProjectConfig::load(".")?;

    println!("{}", style("📦 Project Information").bold());
    println!();
    println!("  {}: {}", style("Name").bold(), config.name());
    println!("  {}: {}", style("Version").bold(), config.version());
    println!("  {}: {}", style("Runtime").bold(), config.runtime());
    println!();
    println!("{}", style("📊 Build Status").bold());
    println!();

    let dist_exists = std::path::Path::new("dist").exists();
    if dist_exists {
        println!("  {} Built artifacts in dist/", style("✓").green());

        // Show sizes
        if let Ok(metadata) = std::fs::metadata("dist/app.dxb") {
            println!("  {} app.dxb: {} bytes", style("•").dim(), metadata.len());
        }
    } else {
        println!("  {} No build artifacts found", style("○").dim());
        println!("  {} Run {} to build", style("→").cyan(), style("dx build").bold());
    }

    Ok(())
}

/// Clean build artifacts
async fn clean_artifacts() -> Result<()> {
    use std::fs;

    println!("{}", style("🧹 Cleaning build artifacts...").bold());

    let dirs = vec!["dist", "target/dx"];

    for dir in dirs {
        if std::path::Path::new(dir).exists() {
            fs::remove_dir_all(dir).with_context(|| format!("Failed to remove {}", dir))?;
            println!("  {} Removed {}", style("✓").green(), dir);
        }
    }

    println!("\n{}", style("✨ Clean complete").green().bold());
    Ok(())
}
