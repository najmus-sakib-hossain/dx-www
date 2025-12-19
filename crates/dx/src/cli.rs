//! CLI definition and command routing

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
    /// Initialize a new DX project
    #[command(visible_alias = "create", visible_alias = "new")]
    Init(commands::init::InitArgs),

    /// Start the development server with hot reload
    #[command(visible_alias = "serve")]
    Dev(commands::dev::DevArgs),

    /// Build for production
    Build(commands::build::BuildArgs),

    /// Run a file with dx-js-runtime
    Run(commands::run::RunArgs),

    /// Bundle files with dx-js-bundler
    Bundle(commands::bundle::BundleArgs),

    /// Install dependencies
    #[command(visible_alias = "i")]
    Install(commands::install::InstallArgs),

    /// Add a dependency
    Add(commands::install::AddArgs),

    /// Remove a dependency
    #[command(visible_alias = "rm", visible_alias = "uninstall")]
    Remove(commands::install::RemoveArgs),

    /// Run tests with dx-js-test-runner
    #[command(visible_alias = "t")]
    Test(commands::test::TestArgs),

    /// Work with dx-style (Binary CSS)
    Style(commands::style::StyleArgs),

    /// Manage dx-media (Image/Video optimization)
    Media(commands::media::MediaArgs),

    /// Manage dx-font (Font subsetting)
    Font(commands::font::FontArgs),

    /// Manage dx-icon (SVG icon system)
    Icon(commands::icon::IconArgs),

    /// Run dx-forge build orchestration
    Forge(commands::forge::ForgeArgs),

    /// Display system and project information
    Info(commands::info::InfoArgs),

    /// Clean build artifacts and caches
    Clean(commands::clean::CleanArgs),

    /// Upgrade DX to the latest version
    #[command(hide = true)]
    Upgrade(commands::upgrade::UpgradeArgs),
}

impl Cli {
    pub async fn run(self) -> Result<()> {
        let theme = Theme::new();

        if !self.quiet {
            theme.print_header();
        }

        match self.command {
            Commands::Init(args) => commands::init::run(args, &theme).await,
            Commands::Dev(args) => commands::dev::run(args, &theme).await,
            Commands::Build(args) => commands::build::run(args, &theme).await,
            Commands::Run(args) => commands::run::run(args, &theme).await,
            Commands::Bundle(args) => commands::bundle::run(args, &theme).await,
            Commands::Install(args) => commands::install::run_install(args, &theme).await,
            Commands::Add(args) => commands::install::run_add(args, &theme).await,
            Commands::Remove(args) => commands::install::run_remove(args, &theme).await,
            Commands::Test(args) => commands::test::run(args, &theme).await,
            Commands::Style(args) => commands::style::run(args, &theme).await,
            Commands::Media(args) => commands::media::run(args, &theme).await,
            Commands::Font(args) => commands::font::run(args, &theme).await,
            Commands::Icon(args) => commands::icon::run(args, &theme).await,
            Commands::Forge(args) => commands::forge::run(args, &theme).await,
            Commands::Info(args) => commands::info::run(args, &theme).await,
            Commands::Clean(args) => commands::clean::run(args, &theme).await,
            Commands::Upgrade(args) => commands::upgrade::run(args, &theme).await,
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
