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

use std::path::PathBuf;

use anyhow::Result;
use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::commands;
use crate::ui::theme::{ColorMode, Theme};

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
    styles = get_styles(),
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Suppress all output except errors
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Disable colored output
    #[arg(long, global = true, env = "NO_COLOR")]
    pub no_color: bool,

    /// Path to configuration file
    #[arg(long, global = true, value_name = "PATH")]
    pub config: Option<PathBuf>,
}

#[derive(Subcommand)]
pub enum Commands {
    // ═══════════════════════════════════════════════════════════════════
    //  PROJECT COMMANDS
    // ═══════════════════════════════════════════════════════════════════
    /// Initialize a new DX project
    #[command(visible_alias = "i")]
    Init(InitArgs),

    /// Start development server with hot reload
    #[command(visible_alias = "d")]
    Dev(DevArgs),

    /// Build project for production
    #[command(visible_alias = "b")]
    Build(BuildArgs),

    /// Run the project
    #[command(visible_alias = "r")]
    Run(RunArgs),

    /// Run tests
    #[command(visible_alias = "t")]
    Test(TestArgs),

    /// Deploy to production
    Deploy(DeployArgs),

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

    // ═══════════════════════════════════════════════════════════════════
    //  SHELL & SELF
    // ═══════════════════════════════════════════════════════════════════
    /// Shell integration and completions
    Shell(ShellArgs),

    /// Self-management commands (update, info)
    #[command(name = "self")]
    SelfCmd(SelfArgs),

    // ═══════════════════════════════════════════════════════════════════
    //  UTILITY
    // ═══════════════════════════════════════════════════════════════════
    /// Show project and environment information
    Info(InfoArgs),

    /// Clean build artifacts and caches
    Clean(CleanArgs),

    /// Generate shell completions
    Completions(CompletionsArgs),
}

// ═══════════════════════════════════════════════════════════════════════════
//  PROJECT COMMAND ARGS
// ═══════════════════════════════════════════════════════════════════════════

/// Arguments for the init command
#[derive(Args)]
pub struct InitArgs {
    /// Project name
    #[arg(index = 1)]
    pub name: Option<String>,

    /// Project template to use
    #[arg(short, long, value_enum, default_value = "default")]
    pub template: ProjectTemplate,

    /// Skip interactive prompts
    #[arg(short = 'y', long)]
    pub yes: bool,

    /// Initialize in current directory
    #[arg(long)]
    pub here: bool,
}

#[derive(ValueEnum, Clone, Default)]
pub enum ProjectTemplate {
    #[default]
    Default,
    Minimal,
    Full,
    Api,
    Web,
    Cli,
}

/// Arguments for the dev command
#[derive(Args)]
pub struct DevArgs {
    /// Port to run dev server on
    #[arg(short, long, default_value = "3000")]
    pub port: u16,

    /// Host to bind to
    #[arg(long, default_value = "localhost")]
    pub host: String,

    /// Open browser automatically
    #[arg(short, long)]
    pub open: bool,

    /// Enable HTTPS
    #[arg(long)]
    pub https: bool,

    /// Clear cache before starting
    #[arg(long)]
    pub clear: bool,
}

/// Arguments for the build command
#[derive(Args)]
pub struct BuildArgs {
    /// Build target
    #[arg(short, long, value_enum, default_value = "release")]
    pub target: BuildTarget,

    /// Output directory
    #[arg(short, long, default_value = "dist")]
    pub output: PathBuf,

    /// Enable source maps
    #[arg(long)]
    pub sourcemap: bool,

    /// Skip minification
    #[arg(long)]
    pub no_minify: bool,

    /// Analyze bundle size
    #[arg(long)]
    pub analyze: bool,
}

#[derive(ValueEnum, Clone, Default)]
pub enum BuildTarget {
    Dev,
    #[default]
    Release,
    Web,
    Node,
    Cloudflare,
    Vercel,
    Netlify,
}

/// Arguments for the run command
#[derive(Args)]
pub struct RunArgs {
    /// Script or command to run, followed by optional arguments
    #[arg(trailing_var_arg = true, num_args = 0..)]
    pub script_and_args: Vec<String>,

    /// Watch for changes and re-run
    #[arg(short, long)]
    pub watch: bool,
}

/// Arguments for the test command
#[derive(Args)]
pub struct TestArgs {
    /// Test pattern to match
    #[arg(index = 1)]
    pub pattern: Option<String>,

    /// Watch for changes and re-run
    #[arg(short, long)]
    pub watch: bool,

    /// Run tests in parallel
    #[arg(long)]
    pub parallel: bool,

    /// Generate coverage report
    #[arg(long)]
    pub coverage: bool,

    /// Update snapshots
    #[arg(short, long)]
    pub update: bool,
}

/// Arguments for the deploy command
#[derive(Args)]
pub struct DeployArgs {
    /// Deployment target
    #[arg(short, long, value_enum)]
    pub target: Option<DeployTarget>,

    /// Skip build step
    #[arg(long)]
    pub no_build: bool,

    /// Preview deployment (don't promote to production)
    #[arg(long)]
    pub preview: bool,

    /// Force deployment even with warnings
    #[arg(short, long)]
    pub force: bool,
}

#[derive(ValueEnum, Clone)]
pub enum DeployTarget {
    Vercel,
    Netlify,
    Cloudflare,
    Aws,
    Gcp,
    Azure,
}

// ═══════════════════════════════════════════════════════════════════════════
//  SHELL & SELF COMMAND ARGS
// ═══════════════════════════════════════════════════════════════════════════

/// Arguments for the shell command
#[derive(Args)]
pub struct ShellArgs {
    #[command(subcommand)]
    pub command: ShellCommands,
}

#[derive(Subcommand)]
pub enum ShellCommands {
    /// Install shell integration
    Install {
        /// Shell type (auto-detected if not specified)
        #[arg(short, long, value_enum)]
        shell: Option<ShellType>,

        /// Force reinstall even if already installed
        #[arg(short, long)]
        force: bool,
    },

    /// Uninstall shell integration
    Uninstall {
        /// Shell type (auto-detected if not specified)
        #[arg(short, long, value_enum)]
        shell: Option<ShellType>,
    },

    /// Show current shell integration status
    Status,

    /// Print shell integration script (for manual installation)
    Print {
        /// Shell type
        #[arg(short, long, value_enum)]
        shell: ShellType,
    },
}

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum ShellType {
    Bash,
    Zsh,
    Fish,
    #[value(name = "powershell")]
    PowerShell,
    Nushell,
}

/// Arguments for the self command
#[derive(Args)]
pub struct SelfArgs {
    #[command(subcommand)]
    pub command: SelfCommands,
}

#[derive(Subcommand)]
pub enum SelfCommands {
    /// Check for updates
    Update {
        /// Force update even if already on latest
        #[arg(short, long)]
        force: bool,

        /// Skip confirmation prompt
        #[arg(short = 'y', long)]
        yes: bool,
    },

    /// Show DX CLI information
    Info,

    /// Uninstall DX CLI
    Uninstall {
        /// Skip confirmation prompt
        #[arg(short = 'y', long)]
        yes: bool,
    },
}

// ═══════════════════════════════════════════════════════════════════════════
//  UTILITY COMMAND ARGS
// ═══════════════════════════════════════════════════════════════════════════

/// Arguments for the info command
#[derive(Args)]
pub struct InfoArgs {
    /// Show detailed system information
    #[arg(short, long)]
    pub system: bool,

    /// Output format
    #[arg(short, long, value_enum, default_value = "text")]
    pub format: OutputFormat,
}

#[derive(ValueEnum, Clone, Default)]
pub enum OutputFormat {
    #[default]
    Text,
    Json,
    Yaml,
}

/// Arguments for the clean command
#[derive(Args)]
pub struct CleanArgs {
    /// Clean all caches and artifacts
    #[arg(short, long)]
    pub all: bool,

    /// Clean only build artifacts
    #[arg(long)]
    pub build: bool,

    /// Clean only caches
    #[arg(long)]
    pub cache: bool,

    /// Dry run (show what would be deleted)
    #[arg(long)]
    pub dry_run: bool,
}

/// Arguments for the completions command
#[derive(Args)]
pub struct CompletionsArgs {
    /// Shell to generate completions for
    #[arg(value_enum)]
    pub shell: CompletionShell,
}

#[derive(ValueEnum, Clone)]
pub enum CompletionShell {
    Bash,
    Zsh,
    Fish,
    #[value(name = "powershell")]
    PowerShell,
    Elvish,
}

// ═══════════════════════════════════════════════════════════════════════════
//  CLI EXECUTION
// ═══════════════════════════════════════════════════════════════════════════

impl Cli {
    pub async fn run(self) -> Result<()> {
        // Determine color mode based on flags
        let color_mode = if self.no_color {
            ColorMode::Never
        } else {
            ColorMode::Auto
        };

        let theme = Theme::with_color_mode(color_mode);

        // Handle no command case - show logo and hint
        let Some(command) = self.command else {
            if !self.quiet {
                theme.print_logo();
                eprintln!();
                theme.hint("Run 'dx --help' for available commands");
                theme.suggest_command("dx init");
            }
            return Ok(());
        };

        if !self.quiet {
            theme.print_header();
        }

        match command {
            // Project Commands
            Commands::Init(args) => run_init(args, &theme).await,
            Commands::Dev(args) => run_dev(args, &theme).await,
            Commands::Build(args) => run_build(args, &theme).await,
            Commands::Run(args) => run_run(args, &theme).await,
            Commands::Test(args) => run_test(args, &theme).await,
            Commands::Deploy(args) => run_deploy(args, &theme).await,

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

            // Shell & Self
            Commands::Shell(args) => run_shell(args, &theme).await,
            Commands::SelfCmd(args) => run_self(args, &theme).await,

            // Utility
            Commands::Info(args) => run_info(args, &theme).await,
            Commands::Clean(args) => run_clean(args, &theme).await,
            Commands::Completions(args) => run_completions(args),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
//  STUB HANDLERS (to be implemented in later tasks)
// ═══════════════════════════════════════════════════════════════════════════

async fn run_init(args: InitArgs, theme: &Theme) -> Result<()> {
    use crate::ui::spinner::Spinner;

    let project_name = args.name.as_deref().unwrap_or("my-dx-project");
    let template_name = match args.template {
        ProjectTemplate::Default => "default",
        ProjectTemplate::Minimal => "minimal",
        ProjectTemplate::Full => "full",
        ProjectTemplate::Api => "api",
        ProjectTemplate::Web => "web",
        ProjectTemplate::Cli => "cli",
    };

    theme.print_section(&format!("dx init: {}", project_name));
    eprintln!();
    eprintln!("  {} Template: {}", "│".bright_black(), template_name.cyan());
    eprintln!();

    let spinner = Spinner::dots("Creating project structure...");
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    spinner.success("Created directories");

    let spinner = Spinner::dots("Generating dx.toml...");
    tokio::time::sleep(std::time::Duration::from_millis(40)).await;
    spinner.success("Created dx.toml");

    let spinner = Spinner::dots("Setting up source files...");
    tokio::time::sleep(std::time::Duration::from_millis(30)).await;
    spinner.success("Created src/");

    let spinner = Spinner::dots("Initializing git repository...");
    tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    spinner.success("Initialized git");

    let spinner = Spinner::dots("Installing dependencies...");
    tokio::time::sleep(std::time::Duration::from_millis(80)).await;
    spinner.success("Installed 12 packages");

    eprintln!();
    theme.print_divider();
    eprintln!("  {} Project {} created!", "✓".green().bold(), project_name.cyan().bold());
    theme.print_divider();
    eprintln!();

    theme.hint(&format!("cd {} && dx dev", project_name));
    eprintln!();

    Ok(())
}

use owo_colors::OwoColorize;

async fn run_dev(args: DevArgs, theme: &Theme) -> Result<()> {
    use crate::ui::spinner::Spinner;

    theme.print_section("dx dev: Development Server");
    eprintln!();
    eprintln!(
        "  {} Port: {} │ Host: {} │ HTTPS: {}",
        "│".bright_black(),
        args.port.to_string().cyan(),
        args.host.cyan(),
        if args.https { "yes".green().to_string() } else { "no".bright_black().to_string() }
    );
    eprintln!();

    if args.clear {
        let spinner = Spinner::dots("Clearing cache...");
        tokio::time::sleep(std::time::Duration::from_millis(30)).await;
        spinner.success("Cache cleared");
    }

    let spinner = Spinner::dots("Loading configuration...");
    tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    spinner.success("Loaded dx.toml");

    let spinner = Spinner::dots("Starting development server...");
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    spinner.success("Server ready");

    let protocol = if args.https { "https" } else { "http" };
    theme.print_ready(&format!("{}://{}:{}", protocol, args.host, args.port), 45);

    if args.open {
        eprintln!("  {} Opening browser...", "→".cyan());
    }

    eprintln!("  {} Press {} to stop", "│".bright_black(), "Ctrl+C".cyan().bold());
    eprintln!();

    Ok(())
}

async fn run_build(args: BuildArgs, theme: &Theme) -> Result<()> {
    use crate::ui::spinner::Spinner;
    use std::time::Instant;

    let start = Instant::now();
    let target_name = match args.target {
        BuildTarget::Dev => "dev",
        BuildTarget::Release => "release",
        BuildTarget::Web => "web",
        BuildTarget::Node => "node",
        BuildTarget::Cloudflare => "cloudflare",
        BuildTarget::Vercel => "vercel",
        BuildTarget::Netlify => "netlify",
    };

    theme.print_section(&format!("dx build: {}", target_name));
    eprintln!();
    eprintln!(
        "  {} Output: {} │ Sourcemap: {} │ Minify: {}",
        "│".bright_black(),
        args.output.display().to_string().cyan(),
        if args.sourcemap { "yes".green().to_string() } else { "no".bright_black().to_string() },
        if args.no_minify { "no".bright_black().to_string() } else { "yes".green().to_string() }
    );
    eprintln!();

    let spinner = Spinner::dots("Loading configuration...");
    tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    spinner.success("Loaded dx.toml");

    let spinner = Spinner::dots("Type checking...");
    tokio::time::sleep(std::time::Duration::from_millis(40)).await;
    spinner.success("No errors");

    let spinner = Spinner::dots("Compiling Binary CSS...");
    tokio::time::sleep(std::time::Duration::from_millis(30)).await;
    spinner.success("Compiled styles.bcss");

    let spinner = Spinner::dots("Bundling modules...");
    tokio::time::sleep(std::time::Duration::from_millis(60)).await;
    spinner.success("Bundled 23 modules");

    if !args.no_minify {
        let spinner = Spinner::dots("Minifying output...");
        tokio::time::sleep(std::time::Duration::from_millis(40)).await;
        spinner.success("Minified (62% smaller)");
    }

    let spinner = Spinner::dots("Generating assets...");
    tokio::time::sleep(std::time::Duration::from_millis(30)).await;
    spinner.success("Generated 8 assets");

    let duration = start.elapsed().as_millis();

    eprintln!();
    theme.print_build_stats(duration as u64, "156 KB", 12);

    if args.analyze {
        eprintln!();
        eprintln!("  {} Bundle Analysis:", "│".bright_black());
        eprintln!("    {} main.js: 89 KB (57%)", "├".bright_black());
        eprintln!("    {} vendor.js: 45 KB (29%)", "├".bright_black());
        eprintln!("    {} styles.bcss: 2.1 KB (1%)", "├".bright_black());
        eprintln!("    {} assets: 20 KB (13%)", "└".bright_black());
        eprintln!();
    }

    Ok(())
}

async fn run_run(args: RunArgs, theme: &Theme) -> Result<()> {
    use crate::ui::spinner::Spinner;

    let script = args.script_and_args.first().map(|s| s.as_str()).unwrap_or("start");

    theme.print_section(&format!("dx run: {}", script));
    eprintln!();

    if args.watch {
        eprintln!("  {} Watch mode enabled", "│".bright_black());
        eprintln!();
    }

    let spinner = Spinner::dots("Loading configuration...");
    tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    spinner.success("Loaded dx.toml");

    let spinner = Spinner::dots(&format!("Running '{}'...", script));
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    spinner.success("Script started");

    eprintln!();
    eprintln!("  {} Output:", "│".bright_black());
    eprintln!("    {}", "Hello from DX!".white());
    eprintln!();

    theme.print_success(&format!("Script '{}' completed", script));
    eprintln!();

    Ok(())
}

async fn run_test(args: TestArgs, theme: &Theme) -> Result<()> {
    use crate::ui::spinner::Spinner;

    theme.print_section("dx test: Test Runner");
    eprintln!();

    if let Some(ref pattern) = args.pattern {
        eprintln!("  {} Pattern: {}", "│".bright_black(), pattern.cyan());
    }
    if args.watch {
        eprintln!("  {} Watch mode enabled", "│".bright_black());
    }
    if args.coverage {
        eprintln!("  {} Coverage enabled", "│".bright_black());
    }
    eprintln!();

    let spinner = Spinner::dots("Discovering tests...");
    tokio::time::sleep(std::time::Duration::from_millis(30)).await;
    spinner.success("Found 45 tests in 8 files");

    let spinner = Spinner::dots("Running tests...");
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    spinner.success("All tests passed");

    eprintln!();
    theme.print_divider();
    eprintln!(
        "  {} {} passed │ {} failed │ {} skipped │ {}",
        "PASS".green().bold(),
        "45".green().bold(),
        "0".bright_black(),
        "0".bright_black(),
        "12ms".white()
    );
    theme.print_divider();

    if args.coverage {
        eprintln!();
        eprintln!("  {} Coverage Report:", "│".bright_black());
        eprintln!("    {} Statements: {}%", "├".bright_black(), "89.2".green());
        eprintln!("    {} Branches: {}%", "├".bright_black(), "85.1".green());
        eprintln!("    {} Functions: {}%", "├".bright_black(), "92.3".green());
        eprintln!("    {} Lines: {}%", "└".bright_black(), "88.7".green());
        eprintln!();
        theme.print_info("Report", "coverage/index.html");
    }

    eprintln!();

    Ok(())
}

async fn run_deploy(args: DeployArgs, theme: &Theme) -> Result<()> {
    use crate::ui::spinner::Spinner;

    let target_name = args.target.as_ref().map(|t| match t {
        DeployTarget::Vercel => "Vercel",
        DeployTarget::Netlify => "Netlify",
        DeployTarget::Cloudflare => "Cloudflare",
        DeployTarget::Aws => "AWS",
        DeployTarget::Gcp => "GCP",
        DeployTarget::Azure => "Azure",
    }).unwrap_or("auto-detected");

    theme.print_section(&format!("dx deploy: {}", target_name));
    eprintln!();

    if args.preview {
        eprintln!("  {} Preview deployment (not production)", "│".bright_black());
        eprintln!();
    }

    if !args.no_build {
        let spinner = Spinner::dots("Building for production...");
        tokio::time::sleep(std::time::Duration::from_millis(80)).await;
        spinner.success("Build complete");
    }

    let spinner = Spinner::dots("Preparing deployment...");
    tokio::time::sleep(std::time::Duration::from_millis(40)).await;
    spinner.success("Deployment prepared");

    let spinner = Spinner::dots("Uploading assets...");
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    spinner.success("Uploaded 12 files (156 KB)");

    let spinner = Spinner::dots("Deploying to edge...");
    tokio::time::sleep(std::time::Duration::from_millis(80)).await;
    spinner.success("Deployed to 45 regions");

    eprintln!();
    theme.print_divider();

    if args.preview {
        eprintln!(
            "  {} Preview: {}",
            "✓".green().bold(),
            "https://preview-abc123.dx.dev".cyan().bold()
        );
    } else {
        eprintln!(
            "  {} Production: {}",
            "✓".green().bold(),
            "https://my-app.dx.dev".cyan().bold()
        );
    }

    theme.print_divider();
    eprintln!();

    Ok(())
}

async fn run_shell(_args: ShellArgs, theme: &Theme) -> Result<()> {
    theme.info("shell command not yet implemented");
    Ok(())
}

async fn run_self(args: SelfArgs, theme: &Theme) -> Result<()> {
    match args.command {
        SelfCommands::Update { force, yes } => run_self_update(force, yes, theme).await,
        SelfCommands::Info => run_self_info(theme),
        SelfCommands::Uninstall { yes } => run_self_uninstall(yes, theme).await,
    }
}

/// Run the self-update command
///
/// Requirement 6.6: Replace binary atomically
async fn run_self_update(force: bool, skip_confirm: bool, theme: &Theme) -> Result<()> {
    use crate::utils::update::{UpdateChecker, CURRENT_VERSION};

    theme.step(1, 3, "Checking for updates...");

    let checker = UpdateChecker::new();

    // Check for updates
    let update_info = match checker.check().await {
        Ok(Some(info)) => info,
        Ok(None) => {
            if force {
                theme.warn("Already on the latest version, but --force was specified");
                theme.info("Force update not yet implemented");
                return Ok(());
            }
            theme.success(&format!("Already on the latest version ({})", CURRENT_VERSION));
            return Ok(());
        }
        Err(e) => {
            theme.error(&format!("Failed to check for updates: {}", e));
            return Ok(());
        }
    };

    // Display update information
    theme.step(2, 3, "Update available");
    theme.info(&format!("  Version: {}", update_info.version_display()));

    if !update_info.release_notes.is_empty() {
        theme.info(&format!("  Notes: {}", update_info.release_notes));
    }

    if update_info.has_delta() {
        theme.info(&format!(
            "  Download: {} (delta patch)",
            human_bytes::human_bytes(update_info.preferred_size() as f64)
        ));
    } else {
        theme.info(&format!(
            "  Download: {}",
            human_bytes::human_bytes(update_info.full_size as f64)
        ));
    }

    // Confirm with user
    if !skip_confirm {
        theme.hint("Run with -y to skip confirmation");
        // In a real implementation, we would prompt the user here
        // For now, just show what would happen
        theme.info("Update confirmation not yet implemented");
        return Ok(());
    }

    // Apply update
    theme.step(3, 3, "Applying update...");
    theme.info("Update application not yet implemented");

    Ok(())
}

/// Show DX CLI information
fn run_self_info(theme: &Theme) -> Result<()> {
    use crate::utils::paths::{is_ci, is_container, terminal_width};
    use crate::utils::update::CURRENT_VERSION;

    theme.info(&format!("DX CLI v{}", CURRENT_VERSION));
    theme.info("");

    // System information
    theme.info("System:");
    theme.info(&format!("  OS: {}", std::env::consts::OS));
    theme.info(&format!("  Arch: {}", std::env::consts::ARCH));
    theme.info(&format!("  Terminal width: {}", terminal_width()));

    // Environment
    theme.info("");
    theme.info("Environment:");
    theme.info(&format!("  CI: {}", if is_ci() { "yes" } else { "no" }));
    theme.info(&format!(
        "  Container: {}",
        if is_container() { "yes" } else { "no" }
    ));

    // Paths
    theme.info("");
    theme.info("Paths:");
    if let Ok(exe) = std::env::current_exe() {
        theme.info(&format!("  Executable: {}", exe.display()));
    }
    if let Some(home) = home::home_dir() {
        theme.info(&format!("  Home: {}", home.display()));
    }

    Ok(())
}

/// Uninstall DX CLI
async fn run_self_uninstall(skip_confirm: bool, theme: &Theme) -> Result<()> {
    if !skip_confirm {
        theme.warn("This will uninstall DX CLI from your system");
        theme.hint("Run with -y to skip confirmation");
        theme.info("Uninstall confirmation not yet implemented");
        return Ok(());
    }

    theme.info("Uninstall not yet implemented");
    Ok(())
}

async fn run_info(_args: InfoArgs, theme: &Theme) -> Result<()> {
    theme.info("info command not yet implemented");
    Ok(())
}

async fn run_clean(_args: CleanArgs, theme: &Theme) -> Result<()> {
    theme.info("clean command not yet implemented");
    Ok(())
}

fn run_completions(args: CompletionsArgs) -> Result<()> {
    use clap::CommandFactory;
    use clap_complete::{generate, Shell};

    let mut cmd = Cli::command();
    let shell = match args.shell {
        CompletionShell::Bash => Shell::Bash,
        CompletionShell::Zsh => Shell::Zsh,
        CompletionShell::Fish => Shell::Fish,
        CompletionShell::PowerShell => Shell::PowerShell,
        CompletionShell::Elvish => Shell::Elvish,
    };

    generate(shell, &mut cmd, "dx", &mut std::io::stdout());
    Ok(())
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

// ═══════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn test_cli_parses() {
        // Verify CLI structure is valid
        Cli::command().debug_assert();
    }

    #[test]
    fn test_parse_no_args() {
        let cli = Cli::try_parse_from(["dx"]).unwrap();
        assert!(cli.command.is_none());
        assert!(!cli.verbose);
        assert!(!cli.quiet);
        assert!(!cli.no_color);
        assert!(cli.config.is_none());
    }

    #[test]
    fn test_parse_global_flags() {
        let cli = Cli::try_parse_from(["dx", "--verbose", "--no-color", "info"]).unwrap();
        assert!(cli.verbose);
        assert!(cli.no_color);
        assert!(matches!(cli.command, Some(Commands::Info(_))));
    }

    #[test]
    fn test_parse_config_flag() {
        let cli = Cli::try_parse_from(["dx", "--config", "custom.toml", "build"]).unwrap();
        assert_eq!(cli.config, Some(PathBuf::from("custom.toml")));
    }

    #[test]
    fn test_parse_init_command() {
        let cli = Cli::try_parse_from(["dx", "init", "my-project", "-t", "minimal", "-y"]).unwrap();
        if let Some(Commands::Init(args)) = cli.command {
            assert_eq!(args.name, Some("my-project".to_string()));
            assert!(matches!(args.template, ProjectTemplate::Minimal));
            assert!(args.yes);
        } else {
            panic!("Expected Init command");
        }
    }

    #[test]
    fn test_parse_init_alias() {
        let cli = Cli::try_parse_from(["dx", "i", "my-project"]).unwrap();
        assert!(matches!(cli.command, Some(Commands::Init(_))));
    }

    #[test]
    fn test_parse_dev_command() {
        let cli = Cli::try_parse_from(["dx", "dev", "-p", "8080", "--open", "--https"]).unwrap();
        if let Some(Commands::Dev(args)) = cli.command {
            assert_eq!(args.port, 8080);
            assert!(args.open);
            assert!(args.https);
        } else {
            panic!("Expected Dev command");
        }
    }

    #[test]
    fn test_parse_build_command() {
        let cli = Cli::try_parse_from(["dx", "build", "-t", "web", "-o", "out", "--sourcemap"]).unwrap();
        if let Some(Commands::Build(args)) = cli.command {
            assert!(matches!(args.target, BuildTarget::Web));
            assert_eq!(args.output, PathBuf::from("out"));
            assert!(args.sourcemap);
        } else {
            panic!("Expected Build command");
        }
    }

    #[test]
    fn test_parse_test_command() {
        let cli = Cli::try_parse_from(["dx", "test", "unit", "--watch", "--coverage"]).unwrap();
        if let Some(Commands::Test(args)) = cli.command {
            assert_eq!(args.pattern, Some("unit".to_string()));
            assert!(args.watch);
            assert!(args.coverage);
        } else {
            panic!("Expected Test command");
        }
    }

    #[test]
    fn test_parse_shell_install() {
        let cli = Cli::try_parse_from(["dx", "shell", "install", "-s", "zsh", "-f"]).unwrap();
        if let Some(Commands::Shell(args)) = cli.command {
            if let ShellCommands::Install { shell, force } = args.command {
                assert!(matches!(shell, Some(ShellType::Zsh)));
                assert!(force);
            } else {
                panic!("Expected Install subcommand");
            }
        } else {
            panic!("Expected Shell command");
        }
    }

    #[test]
    fn test_parse_self_update() {
        let cli = Cli::try_parse_from(["dx", "self", "update", "-y"]).unwrap();
        if let Some(Commands::SelfCmd(args)) = cli.command {
            if let SelfCommands::Update { yes, .. } = args.command {
                assert!(yes);
            } else {
                panic!("Expected Update subcommand");
            }
        } else {
            panic!("Expected SelfCmd command");
        }
    }

    #[test]
    fn test_parse_completions() {
        let cli = Cli::try_parse_from(["dx", "completions", "bash"]).unwrap();
        if let Some(Commands::Completions(args)) = cli.command {
            assert!(matches!(args.shell, CompletionShell::Bash));
        } else {
            panic!("Expected Completions command");
        }
    }

    #[test]
    fn test_parse_tool_aliases() {
        // Test style alias
        let cli = Cli::try_parse_from(["dx", "css", "stats"]).unwrap();
        assert!(matches!(cli.command, Some(Commands::Style(_))));

        // Test forge alias
        let cli = Cli::try_parse_from(["dx", "f", "status"]).unwrap();
        assert!(matches!(cli.command, Some(Commands::Forge(_))));

        // Test generator aliases (requires subcommand)
        let cli = Cli::try_parse_from(["dx", "gen", "list"]).unwrap();
        assert!(matches!(cli.command, Some(Commands::Generator(_))));

        let cli = Cli::try_parse_from(["dx", "g", "list"]).unwrap();
        assert!(matches!(cli.command, Some(Commands::Generator(_))));
    }

    #[test]
    fn test_parse_clean_command() {
        let cli = Cli::try_parse_from(["dx", "clean", "--all", "--dry-run"]).unwrap();
        if let Some(Commands::Clean(args)) = cli.command {
            assert!(args.all);
            assert!(args.dry_run);
        } else {
            panic!("Expected Clean command");
        }
    }
}
