//! dx-py CLI - Ultra-fast Python package manager
//!
//! A high-performance Python package manager that is 5-50x faster than uv.

use clap::{Parser, Subcommand};
use clap_complete::Shell;

mod commands;

use commands::{
    add, build, cache, completions, init, install, lock, publish, python, remove, run, sync, tool,
};

/// Ultra-fast Python package manager
#[derive(Parser)]
#[command(name = "dx-py")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new Python project
    Init {
        /// Python version to use
        #[arg(long)]
        python: Option<String>,
        /// Project name
        #[arg(long)]
        name: Option<String>,
        /// Project directory (defaults to current directory)
        #[arg(default_value = ".")]
        path: String,
    },

    /// Add dependencies to the project
    Add {
        /// Packages to add (e.g., "requests>=2.0")
        packages: Vec<String>,
        /// Add as development dependency
        #[arg(short = 'D', long)]
        dev: bool,
        /// Add as optional dependency group
        #[arg(long)]
        optional: Option<String>,
    },

    /// Remove dependencies from the project
    Remove {
        /// Packages to remove
        packages: Vec<String>,
        /// Remove from development dependencies
        #[arg(short = 'D', long)]
        dev: bool,
    },

    /// Generate lock file from dependencies
    Lock {
        /// Update all packages to latest versions
        #[arg(long)]
        upgrade: bool,
    },

    /// Install packages from lock file
    Sync {
        /// Install development dependencies
        #[arg(long)]
        dev: bool,
        /// Install optional dependency groups
        #[arg(long)]
        extras: Vec<String>,
        /// Show verbose output with cache statistics
        #[arg(short, long)]
        verbose: bool,
    },

    /// Lock and sync (convenience command)
    Install {
        /// Install development dependencies
        #[arg(long)]
        dev: bool,
        /// Install optional dependency groups
        #[arg(long)]
        extras: Vec<String>,
        /// Show verbose output with cache statistics
        #[arg(short, long)]
        verbose: bool,
    },

    /// Run a command in the virtual environment
    Run {
        /// Command and arguments to run
        #[arg(trailing_var_arg = true)]
        command: Vec<String>,
    },

    /// Python version management
    Python {
        #[command(subcommand)]
        command: PythonCommands,
    },

    /// Global tool management (pipx replacement)
    Tool {
        #[command(subcommand)]
        command: ToolCommands,
    },

    /// Build package for distribution
    Build {
        /// Output directory
        #[arg(short, long, default_value = "dist")]
        output: String,
        /// Build wheel only
        #[arg(long)]
        wheel: bool,
        /// Build sdist only
        #[arg(long)]
        sdist: bool,
    },

    /// Publish package to PyPI
    Publish {
        /// Repository URL
        #[arg(long)]
        repository: Option<String>,
        /// API token
        #[arg(long)]
        token: Option<String>,
        /// Distribution files to upload
        #[arg(default_value = "dist/*")]
        files: String,
    },

    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },

    /// Cache management
    Cache {
        #[command(subcommand)]
        command: CacheCommands,
    },
}

#[derive(Subcommand)]
enum PythonCommands {
    /// Install a Python version
    Install {
        /// Python version to install (e.g., "3.12.0")
        version: String,
    },
    /// List installed Python versions
    List,
    /// Pin Python version for the current project
    Pin {
        /// Python version to pin
        version: String,
    },
    /// Show which Python would be used
    Which,
}

#[derive(Subcommand)]
enum ToolCommands {
    /// Install a tool globally
    Install {
        /// Tool package name
        name: String,
        /// Python version to use
        #[arg(long)]
        python: Option<String>,
    },
    /// Run a tool ephemerally (without installing)
    Run {
        /// Tool package name
        name: String,
        /// Arguments to pass to the tool
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// List installed tools
    List,
    /// Uninstall a tool
    Uninstall {
        /// Tool name to uninstall
        name: String,
    },
}

#[derive(Subcommand)]
enum CacheCommands {
    /// Clean cached data
    Clean {
        /// Clear layout cache
        #[arg(long)]
        layouts: bool,
        /// Clear package store
        #[arg(long)]
        store: bool,
        /// Clear all caches
        #[arg(long)]
        all: bool,
    },
    /// Show cache statistics
    Stats,
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Init { python, name, path } => init::run(&path, name.as_deref(), python.as_deref()),
        Commands::Add { packages, dev, optional } => add::run(&packages, dev, optional.as_deref()),
        Commands::Remove { packages, dev } => remove::run(&packages, dev),
        Commands::Lock { upgrade } => lock::run(upgrade),
        Commands::Sync { dev, extras, verbose } => sync::run(dev, &extras, verbose),
        Commands::Install { dev, extras, verbose } => install::run(dev, &extras, verbose),
        Commands::Run { command } => run::run(&command),
        Commands::Python { command } => match command {
            PythonCommands::Install { version } => python::install(&version),
            PythonCommands::List => python::list(),
            PythonCommands::Pin { version } => python::pin(&version),
            PythonCommands::Which => python::which(),
        },
        Commands::Tool { command } => match command {
            ToolCommands::Install { name, python } => tool::install(&name, python.as_deref()),
            ToolCommands::Run { name, args } => tool::run(&name, &args),
            ToolCommands::List => tool::list(),
            ToolCommands::Uninstall { name } => tool::uninstall(&name),
        },
        Commands::Build { output, wheel, sdist } => build::run(&output, wheel, sdist),
        Commands::Publish { repository, token, files } => {
            publish::run(repository.as_deref(), token.as_deref(), &files)
        }
        Commands::Completions { shell } => completions::run(shell),
        Commands::Cache { command } => match command {
            CacheCommands::Clean { layouts, store, all } => cache::clean(layouts, store, all),
            CacheCommands::Stats => cache::stats(),
        },
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
