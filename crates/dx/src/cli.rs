//! CLI definition and command routing
//!
//! DX CLI: The 10 Core Tools
//! ========================
//!
//! ASSET TOOLS:
//!   style      - Binary CSS (B-CSS) compiler
//!   media      - Image/video optimization (WebP, AVIF)
//!   font       - Font subsetting and WOFF2 optimization
//!   icon       - SVG icon system with binary encoding
//!
//! INFRASTRUCTURE:
//!   forge      - Package manager + orchestrator for all dx-* crates
//!   serializer - World-record data format (DX ∞)
//!   stack      - Unified JS/TS development stack
//!
//! DEVELOPMENT:
//!   driven     - AI agents control
//!   generator  - Code generation tools
//!   workspace  - Code editors + preinstall and setup

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::commands;
use crate::ui::theme::Theme;

/// DX - The Binary-First Development Experience
///
/// Build faster. Ship smaller. Zero compromise.
#[derive(Parser)]
#[command(
    name = "dx",
    author,
    version,
    about = "The Binary-First Development Experience",
    long_about = None,
    propagate_version = true,
    arg_required_else_help = true,
    styles = get_styles(),
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Suppress all output except errors
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Disable colored output
    #[arg(long, global = true, env = "NO_COLOR")]
    pub no_color: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    // ═══════════════════════════════════════════════════════════════════
    //  ASSET TOOLS
    // ═══════════════════════════════════════════════════════════════════
    /// Binary CSS (B-CSS) compiler - 98% smaller, 80x faster
    #[command(visible_alias = "css")]
    Style(commands::style::StyleArgs),

    /// Image/video optimization - WebP, AVIF, responsive srcsets
    #[command(visible_alias = "img")]
    Media(commands::media::MediaArgs),

    /// Font subsetting and WOFF2 optimization
    Font(commands::font::FontArgs),

    /// SVG icon system with binary encoding and sprite generation
    Icon(commands::icon::IconArgs),

    // ═══════════════════════════════════════════════════════════════════
    //  INFRASTRUCTURE
    // ═══════════════════════════════════════════════════════════════════
    /// Package manager + orchestrator for all dx-* crates
    #[command(visible_alias = "f")]
    Forge(commands::forge::ForgeArgs),

    /// World-record data format (DX ∞) - 73% smaller, 4x faster
    #[command(visible_alias = "ser", visible_alias = "data")]
    Serializer(commands::serializer::SerializerArgs),

    /// Unified JS/TS development stack (runtime, bundler, test, pkg)
    #[command(visible_alias = "js", visible_alias = "ts")]
    Stack(commands::stack::StackArgs),

    // ═══════════════════════════════════════════════════════════════════
    //  DEVELOPMENT
    // ═══════════════════════════════════════════════════════════════════
    /// AI agents control - review, refactor, test generation
    #[command(visible_alias = "ai")]
    Driven(commands::driven::DrivenArgs),

    /// Code generation tools - components, APIs, forms, CRUD
    #[command(visible_alias = "gen", visible_alias = "g")]
    Generator(commands::generator::GeneratorArgs),

    /// Code editors + preinstall and setup
    #[command(visible_alias = "ws", visible_alias = "ide")]
    Workspace(commands::workspace::WorkspaceArgs),
}

impl Cli {
    pub async fn run(self) -> Result<()> {
        let theme = Theme::new();

        if !self.quiet {
            theme.print_header();
        }

        match self.command {
            // Asset Tools
            Commands::Style(args) => commands::style::run(args, &theme).await,
            Commands::Media(args) => commands::media::run(args, &theme).await,
            Commands::Font(args) => commands::font::run(args, &theme).await,
            Commands::Icon(args) => commands::icon::run(args, &theme).await,

            // Infrastructure
            Commands::Forge(args) => commands::forge::run(args, &theme).await,
            Commands::Serializer(args) => commands::serializer::run(args, &theme).await,
            Commands::Stack(args) => commands::stack::run(args, &theme).await,

            // Development
            Commands::Driven(args) => commands::driven::run(args, &theme).await,
            Commands::Generator(args) => commands::generator::run(args, &theme).await,
            Commands::Workspace(args) => commands::workspace::run(args, &theme).await,
        }
    }
}

/// Custom clap styles for a modern look
fn get_styles() -> clap::builder::Styles {
    use clap::builder::styling::*;

    Styles::styled()
        .header(AnsiColor::Cyan.on_default().bold())
        .usage(AnsiColor::Cyan.on_default().bold())
        .literal(AnsiColor::White.on_default().bold())
        .placeholder(AnsiColor::BrightBlack.on_default())
        .valid(AnsiColor::Green.on_default())
        .invalid(AnsiColor::Red.on_default())
        .error(AnsiColor::Red.on_default().bold())
}
