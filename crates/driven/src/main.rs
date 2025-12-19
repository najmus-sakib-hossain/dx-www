//! Driven CLI - AI Development Orchestrator
//!
//! Command-line interface for managing AI coding rules across editors.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// AI Development Orchestrator
#[derive(Parser)]
#[command(name = "driven")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Verbose output
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize Driven in the current project
    Init {
        /// Run in interactive mode
        #[arg(short, long)]
        interactive: bool,
    },
    /// Synchronize rules to all editors
    Sync {
        /// Watch for changes
        #[arg(short, long)]
        watch: bool,
    },
    /// Convert rules between formats
    Convert {
        /// Input file
        input: PathBuf,
        /// Output file
        output: PathBuf,
        /// Target editor format
        #[arg(short, long)]
        editor: Option<String>,
    },
    /// Manage templates
    Template {
        #[command(subcommand)]
        action: TemplateAction,
    },
    /// Analyze project for context
    Analyze {
        /// Generate context rules
        #[arg(short, long)]
        context: bool,
        /// Index the codebase
        #[arg(short, long)]
        index: bool,
    },
    /// Validate rules
    Validate {
        /// Rules file to validate
        #[arg(default_value = ".driven/rules.md")]
        file: PathBuf,
        /// Strict mode (fail on warnings)
        #[arg(short, long)]
        strict: bool,
    },
}

#[derive(Subcommand)]
enum TemplateAction {
    /// List available templates
    List,
    /// Search templates
    Search {
        /// Search query
        query: String,
    },
    /// Apply a template
    Apply {
        /// Template name
        name: String,
    },
}

fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let cli = Cli::parse();
    let project_root = std::env::current_dir()?;

    match cli.command {
        Commands::Init { interactive } => {
            driven::cli::InitCommand::run(&project_root, interactive)?;
        }
        Commands::Sync { watch } => {
            if watch {
                driven::cli::SyncCommand::watch(&project_root)?;
            } else {
                driven::cli::SyncCommand::run(&project_root)?;
            }
        }
        Commands::Convert { input, output, editor } => {
            let editor = editor.and_then(|e| match e.to_lowercase().as_str() {
                "cursor" => Some(driven::Editor::Cursor),
                "copilot" => Some(driven::Editor::Copilot),
                "windsurf" => Some(driven::Editor::Windsurf),
                "claude" => Some(driven::Editor::Claude),
                "aider" => Some(driven::Editor::Aider),
                "cline" => Some(driven::Editor::Cline),
                _ => None,
            });
            driven::cli::ConvertCommand::run(&input, &output, editor)?;
        }
        Commands::Template { action } => match action {
            TemplateAction::List => {
                driven::cli::TemplateCommand::list()?;
            }
            TemplateAction::Search { query } => {
                driven::cli::TemplateCommand::search(&query)?;
            }
            TemplateAction::Apply { name } => {
                driven::cli::TemplateCommand::apply(&project_root, &name)?;
            }
        },
        Commands::Analyze { context, index } => {
            if context {
                let output = project_root.join(".driven/context.md");
                driven::cli::AnalyzeCommand::generate_context(&project_root, &output)?;
            } else if index {
                driven::cli::AnalyzeCommand::index(&project_root)?;
            } else {
                driven::cli::AnalyzeCommand::run(&project_root)?;
            }
        }
        Commands::Validate { file, strict } => {
            if strict {
                driven::cli::ValidateCommand::run_strict(&file)?;
            } else {
                driven::cli::ValidateCommand::run(&file)?;
            }
        }
    }

    Ok(())
}
