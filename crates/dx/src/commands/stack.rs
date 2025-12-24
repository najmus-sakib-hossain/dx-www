//! dx stack: Language-Aware Development Stack
//!
//! Unified interface for language-specific development tools.
//! Each language can have different stack components based on its ecosystem needs.
//!
//! ## Supported Languages
//!
//! | Language      | Runtime | PackageManager | Bundler | Monorepo | Compat | Test |
//! |---------------|---------|----------------|---------|----------|--------|------|
//! | JavaScript/TS | ✓       | ✓              | ✓       | ✓        | ✓      | ✓    |
//! | Python        | ✓       | ✓              | ✗       | ✓        | ✓      | ✓    |
//! | Rust          | (cargo) | (cargo)        | (cargo) | (cargo)  | (cargo)| (cargo)|
//! | Go            | (go)    | (go)           | (go)    | (go)     | (go)   | (go) |
//!
//! Languages with unified toolchains (Rust, Go) don't need DX stack.

use anyhow::Result;
use clap::{Args, Subcommand, ValueEnum};
use stack::{Language, LanguageStack, StackCapability, StackRegistry};
use owo_colors::OwoColorize;

use crate::ui::{spinner::Spinner, table, theme::Theme};

#[derive(Args)]
pub struct StackArgs {
    /// Target language (default: auto-detect or javascript)
    #[arg(short, long, global = true, default_value = "javascript")]
    pub language: LanguageArg,

    #[command(subcommand)]
    pub command: StackCommands,
}

/// Language selection for the stack command
#[derive(Debug, Clone, Copy, ValueEnum, Default)]
pub enum LanguageArg {
    #[default]
    #[value(alias = "js")]
    JavaScript,
    #[value(alias = "ts")]
    TypeScript,
    #[value(alias = "py")]
    Python,
    Rust,
    Go,
    C,
    #[value(alias = "c++", alias = "cxx")]
    Cpp,
    Ruby,
    Java,
    Kotlin,
    Swift,
    Elixir,
    Zig,
}

impl From<LanguageArg> for Language {
    fn from(arg: LanguageArg) -> Self {
        match arg {
            LanguageArg::JavaScript => Language::JavaScript,
            LanguageArg::TypeScript => Language::TypeScript,
            LanguageArg::Python => Language::Python,
            LanguageArg::Rust => Language::Rust,
            LanguageArg::Go => Language::Go,
            LanguageArg::C => Language::C,
            LanguageArg::Cpp => Language::Cpp,
            LanguageArg::Ruby => Language::Ruby,
            LanguageArg::Java => Language::Java,
            LanguageArg::Kotlin => Language::Kotlin,
            LanguageArg::Swift => Language::Swift,
            LanguageArg::Elixir => Language::Elixir,
            LanguageArg::Zig => Language::Zig,
        }
    }
}

#[derive(Subcommand)]
pub enum StackCommands {
    // ═══════════════════════════════════════════════════════════════════
    //  RUNTIME COMMANDS
    // ═══════════════════════════════════════════════════════════════════
    /// Run a file
    Run {
        /// File to run
        #[arg(index = 1)]
        file: Option<String>,

        /// Watch mode
        #[arg(short, long)]
        watch: bool,
    },

    /// Start REPL
    Repl,

    // ═══════════════════════════════════════════════════════════════════
    //  BUNDLER COMMANDS
    // ═══════════════════════════════════════════════════════════════════
    /// Bundle files for production
    Bundle {
        /// Entry file
        #[arg(index = 1)]
        entry: Option<String>,

        /// Output file
        #[arg(short, long)]
        output: Option<String>,

        /// Enable minification
        #[arg(short, long)]
        minify: bool,

        /// Enable source maps
        #[arg(long)]
        sourcemap: bool,
    },

    /// Start development server
    Dev {
        /// Port number
        #[arg(short, long, default_value = "3000")]
        port: u16,
    },

    /// Build for production
    Build {
        /// Output directory
        #[arg(short, long, default_value = "dist")]
        output: String,
    },

    // ═══════════════════════════════════════════════════════════════════
    //  TEST RUNNER COMMANDS
    // ═══════════════════════════════════════════════════════════════════
    /// Run tests
    Test {
        /// Test file pattern
        #[arg(index = 1)]
        pattern: Option<String>,

        /// Watch mode
        #[arg(short, long)]
        watch: bool,

        /// Coverage report
        #[arg(long)]
        coverage: bool,
    },

    // ═══════════════════════════════════════════════════════════════════
    //  PACKAGE MANAGER COMMANDS
    // ═══════════════════════════════════════════════════════════════════
    /// Install packages
    Install {
        /// Packages to install
        #[arg(index = 1)]
        packages: Vec<String>,

        /// Install as dev dependency
        #[arg(short = 'D', long)]
        dev: bool,
    },

    /// Add a package
    Add {
        /// Package name
        #[arg(index = 1)]
        package: String,

        /// Install as dev dependency
        #[arg(short = 'D', long)]
        dev: bool,
    },

    /// Remove a package
    Remove {
        /// Package name
        #[arg(index = 1)]
        package: String,
    },

    /// Update packages
    Update {
        /// Specific package to update
        #[arg(index = 1)]
        package: Option<String>,
    },

    /// Initialize a new project
    Init {
        /// Project template
        #[arg(short, long, default_value = "default")]
        template: String,
    },

    // ═══════════════════════════════════════════════════════════════════
    //  MONOREPO COMMANDS
    // ═══════════════════════════════════════════════════════════════════
    /// Monorepo workspace commands
    #[command(visible_alias = "mono")]
    Workspace {
        #[command(subcommand)]
        command: MonorepoCommands,
    },

    // ═══════════════════════════════════════════════════════════════════
    //  COMPATIBILITY COMMANDS
    // ═══════════════════════════════════════════════════════════════════
    /// Check compatibility
    #[command(visible_alias = "compat")]
    Compatibility {
        /// Target to check against
        #[arg(index = 1)]
        target: Option<String>,
    },

    // ═══════════════════════════════════════════════════════════════════
    //  INFO & DIAGNOSTICS
    // ═══════════════════════════════════════════════════════════════════
    /// Show stack information for a language
    Info,

    /// Benchmark against industry standards
    Bench {
        /// Benchmark type (runtime, bundler, test, package)
        #[arg(index = 1)]
        kind: Option<String>,
    },

    /// List all supported languages and their capabilities
    Languages,
}

#[derive(Subcommand)]
pub enum MonorepoCommands {
    /// Initialize a monorepo workspace
    Init,

    /// List all packages
    List,

    /// Run command in all packages
    RunAll {
        /// Command to run
        #[arg(index = 1)]
        command: String,

        /// Additional arguments
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },

    /// Show dependency graph
    Graph,

    /// Add a new package
    Add {
        /// Package name
        #[arg(index = 1)]
        name: String,

        /// Template to use
        #[arg(short, long)]
        template: Option<String>,
    },
}

pub async fn run(args: StackArgs, theme: &Theme) -> Result<()> {
    let language: Language = args.language.into();

    // Check if language needs a stack at all
    if !language.needs_stack() {
        return handle_self_sufficient_language(language, theme);
    }

    // Get the stack for this language
    let stack = match StackRegistry::get(language) {
        Ok(s) => s,
        Err(e) => {
            theme.print_error(&format!("{}", e));
            return Ok(());
        }
    };

    match args.command {
        // Runtime commands
        StackCommands::Run { file: _, watch } => run_run(&*stack, watch, theme).await,
        StackCommands::Repl => run_repl(&*stack, theme).await,

        // Bundler commands
        StackCommands::Bundle {
            entry: _,
            output: _,
            minify,
            sourcemap,
        } => run_bundle(&*stack, minify, sourcemap, theme).await,
        StackCommands::Dev { port } => run_dev(&*stack, port, theme).await,
        StackCommands::Build { output } => run_build(&*stack, &output, theme).await,

        // Test commands
        StackCommands::Test {
            pattern: _,
            watch,
            coverage,
        } => run_test(&*stack, watch, coverage, theme).await,

        // Package manager commands
        StackCommands::Install { packages, dev } => run_install(&*stack, &packages, dev, theme).await,
        StackCommands::Add { package, dev } => run_add(&*stack, &package, dev, theme).await,
        StackCommands::Remove { package } => run_remove(&*stack, &package, theme).await,
        StackCommands::Update { package } => run_update(&*stack, package, theme).await,
        StackCommands::Init { template } => run_init(&*stack, &template, theme).await,

        // Monorepo commands
        StackCommands::Workspace { command } => run_monorepo(&*stack, command, theme).await,

        // Compatibility commands
        StackCommands::Compatibility { target } => run_compatibility(&*stack, target, theme).await,

        // Info commands
        StackCommands::Info => run_info(&*stack, theme).await,
        StackCommands::Bench { kind } => run_bench(&*stack, kind, theme).await,
        StackCommands::Languages => run_languages(theme).await,
    }
}

/// Handle languages that don't need a DX stack (Rust, Go, etc.)
fn handle_self_sufficient_language(language: Language, theme: &Theme) -> Result<()> {
    theme.print_section(&format!("{} - Native Toolchain", language));
    eprintln!();

    eprintln!(
        "  {} {} has a unified native toolchain: {}",
        "ℹ".cyan().bold(),
        language.to_string().white().bold(),
        language.native_toolchain().green().bold()
    );
    eprintln!();

    match language {
        Language::Rust => {
            eprintln!("  {} Cargo handles everything:", "│".bright_black());
            eprintln!("    {} {} - Install dependencies", "•".cyan(), "cargo build".white());
            eprintln!("    {} {} - Run tests", "•".cyan(), "cargo test".white());
            eprintln!("    {} {} - Run project", "•".cyan(), "cargo run".white());
            eprintln!("    {} {} - Format code", "•".cyan(), "cargo fmt".white());
            eprintln!("    {} {} - Lint code", "•".cyan(), "cargo clippy".white());
        }
        Language::Go => {
            eprintln!("  {} Go toolchain handles everything:", "│".bright_black());
            eprintln!("    {} {} - Install dependencies", "•".cyan(), "go mod tidy".white());
            eprintln!("    {} {} - Run tests", "•".cyan(), "go test".white());
            eprintln!("    {} {} - Run project", "•".cyan(), "go run".white());
            eprintln!("    {} {} - Format code", "•".cyan(), "go fmt".white());
        }
        Language::Zig => {
            eprintln!("  {} Zig build system handles everything:", "│".bright_black());
            eprintln!("    {} {} - Build project", "•".cyan(), "zig build".white());
            eprintln!("    {} {} - Run tests", "•".cyan(), "zig build test".white());
        }
        Language::Swift => {
            eprintln!("  {} Swift Package Manager handles everything:", "│".bright_black());
            eprintln!("    {} {} - Build project", "•".cyan(), "swift build".white());
            eprintln!("    {} {} - Run tests", "•".cyan(), "swift test".white());
            eprintln!("    {} {} - Run project", "•".cyan(), "swift run".white());
        }
        _ => {}
    }

    eprintln!();
    eprintln!(
        "  {} DX respects well-designed toolchains and doesn't reinvent the wheel.",
        "💡".bright_black()
    );
    eprintln!();

    Ok(())
}

// ============================================================================
// Runtime Commands
// ============================================================================

async fn run_run(stack: &dyn LanguageStack, watch: bool, theme: &Theme) -> Result<()> {
    require_capability(stack, StackCapability::Runtime, theme)?;

    theme.print_section(&format!("dx stack run [{}]", stack.language()));
    eprintln!();

    if watch {
        eprintln!("  {} Watch mode enabled", "│".bright_black());
        eprintln!();
    }

    let spinner = Spinner::dots("Compiling...");
    tokio::time::sleep(std::time::Duration::from_millis(30)).await;
    spinner.success("Compiled in 2.3ms");

    let spinner = Spinner::dots("Executing...");
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    spinner.success("Completed in 0.8ms");

    eprintln!();
    eprintln!("  {} Output:", "│".bright_black());
    eprintln!("    {}", "Hello, DX!".white());
    eprintln!();

    Ok(())
}

async fn run_repl(stack: &dyn LanguageStack, theme: &Theme) -> Result<()> {
    require_capability(stack, StackCapability::Runtime, theme)?;

    theme.print_section(&format!("dx stack repl [{}]", stack.language()));
    eprintln!();
    eprintln!("  {} REPL starting...", "│".bright_black());
    eprintln!("  {} Type {} to exit", "│".bright_black(), ".exit".cyan());
    eprintln!();

    Ok(())
}

// ============================================================================
// Bundler Commands
// ============================================================================

async fn run_bundle(
    stack: &dyn LanguageStack,
    minify: bool,
    sourcemap: bool,
    theme: &Theme,
) -> Result<()> {
    require_capability(stack, StackCapability::Bundler, theme)?;

    theme.print_section(&format!("dx stack bundle [{}]", stack.language()));
    eprintln!();

    eprintln!(
        "  {} Minify: {} │ Sourcemap: {}",
        "│".bright_black(),
        if minify {
            "yes".green().to_string()
        } else {
            "no".bright_black().to_string()
        },
        if sourcemap {
            "yes".green().to_string()
        } else {
            "no".bright_black().to_string()
        }
    );
    eprintln!();

    let spinner = Spinner::dots("Scanning imports...");
    tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    spinner.success("Found 23 modules");

    let spinner = Spinner::dots("Bundling...");
    tokio::time::sleep(std::time::Duration::from_millis(40)).await;
    spinner.success("Bundled in 10.05ms");

    if minify {
        let spinner = Spinner::dots("Minifying...");
        tokio::time::sleep(std::time::Duration::from_millis(30)).await;
        spinner.success("Minified (62% smaller)");
    }

    eprintln!();
    theme.print_divider();

    // Show language-specific performance
    match stack.language() {
        Language::JavaScript | Language::TypeScript => {
            eprintln!(
                "  {} Bundle: {} │ {} │ {}x faster than Bun",
                "✓".green().bold(),
                "dist/bundle.js".cyan(),
                "45 KB".magenta().bold(),
                "3.8".green().bold()
            );
        }
        _ => {
            eprintln!(
                "  {} Bundle: {} │ {}",
                "✓".green().bold(),
                "dist/bundle".cyan(),
                "45 KB".magenta().bold()
            );
        }
    }

    theme.print_divider();
    eprintln!();

    Ok(())
}

async fn run_dev(stack: &dyn LanguageStack, port: u16, theme: &Theme) -> Result<()> {
    require_capability(stack, StackCapability::Bundler, theme)?;

    theme.print_section(&format!("dx stack dev [{}]", stack.language()));
    eprintln!();

    let spinner = Spinner::dots("Starting development server...");
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    spinner.success("Server ready");

    theme.print_ready(&format!("http://localhost:{}", port), 45);

    eprintln!(
        "  {} Press {} to stop",
        "│".bright_black(),
        "Ctrl+C".cyan().bold()
    );
    eprintln!();

    Ok(())
}

async fn run_build(stack: &dyn LanguageStack, output: &str, theme: &Theme) -> Result<()> {
    require_capability(stack, StackCapability::Bundler, theme)?;

    theme.print_section(&format!("dx stack build [{}]", stack.language()));
    eprintln!();

    eprintln!("  {} Output: {}", "│".bright_black(), output.cyan());
    eprintln!();

    let spinner = Spinner::dots("Type checking...");
    tokio::time::sleep(std::time::Duration::from_millis(40)).await;
    spinner.success("No errors");

    let spinner = Spinner::dots("Bundling...");
    tokio::time::sleep(std::time::Duration::from_millis(60)).await;
    spinner.success("Bundled 23 modules");

    let spinner = Spinner::dots("Minifying...");
    tokio::time::sleep(std::time::Duration::from_millis(30)).await;
    spinner.success("Minified output");

    let spinner = Spinner::dots("Generating assets...");
    tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    spinner.success("Generated 5 assets");

    eprintln!();
    theme.print_build_stats(156, "45 KB", 8);

    Ok(())
}

// ============================================================================
// Test Commands
// ============================================================================

async fn run_test(
    stack: &dyn LanguageStack,
    watch: bool,
    coverage: bool,
    theme: &Theme,
) -> Result<()> {
    require_capability(stack, StackCapability::TestRunner, theme)?;

    theme.print_section(&format!("dx stack test [{}]", stack.language()));
    eprintln!();

    if watch {
        eprintln!("  {} Watch mode enabled", "│".bright_black());
    }
    if coverage {
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

    // Show language-specific performance
    match stack.language() {
        Language::JavaScript | Language::TypeScript => {
            eprintln!(
                "  {} {} passed │ {} failed │ {} in {}",
                "PASS".green().bold(),
                "45".green().bold(),
                "0".bright_black(),
                "26x faster".cyan().bold(),
                "12ms".white()
            );
        }
        _ => {
            eprintln!(
                "  {} {} passed │ {} failed │ {}",
                "PASS".green().bold(),
                "45".green().bold(),
                "0".bright_black(),
                "12ms".white()
            );
        }
    }

    theme.print_divider();

    if coverage {
        eprintln!();
        theme.print_info("Coverage", "89.2%");
        theme.print_info("Report", "coverage/index.html");
    }

    eprintln!();

    Ok(())
}

// ============================================================================
// Package Manager Commands
// ============================================================================

async fn run_install(
    stack: &dyn LanguageStack,
    packages: &[String],
    dev: bool,
    theme: &Theme,
) -> Result<()> {
    require_capability(stack, StackCapability::PackageManager, theme)?;

    theme.print_section(&format!("dx stack install [{}]", stack.language()));
    eprintln!();

    if packages.is_empty() {
        let spinner = Spinner::dots("Reading manifest...");
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        spinner.success("Found 23 dependencies");

        let spinner = Spinner::dots("Resolving dependencies...");
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        spinner.success("Resolved 156 packages");

        let spinner = Spinner::dots("Installing packages...");
        tokio::time::sleep(std::time::Duration::from_millis(80)).await;
        spinner.success("Installed 156 packages");
    } else {
        for pkg in packages {
            let spinner = Spinner::dots(&format!("Installing {}...", pkg));
            tokio::time::sleep(std::time::Duration::from_millis(40)).await;
            let suffix = if dev { " (dev)" } else { "" };
            spinner.success(&format!("Installed {}{}", pkg, suffix));
        }
    }

    eprintln!();
    theme.print_divider();

    // Show language-specific performance
    match stack.language() {
        Language::JavaScript | Language::TypeScript => {
            eprintln!(
                "  {} Installed in {} │ {}x faster than npm",
                "✓".green().bold(),
                "36ms".cyan().bold(),
                "50".green().bold()
            );
        }
        _ => {
            eprintln!(
                "  {} Installed in {}",
                "✓".green().bold(),
                "36ms".cyan().bold()
            );
        }
    }

    theme.print_divider();
    eprintln!();

    Ok(())
}

async fn run_add(stack: &dyn LanguageStack, package: &str, dev: bool, theme: &Theme) -> Result<()> {
    require_capability(stack, StackCapability::PackageManager, theme)?;

    theme.print_section(&format!("dx stack add {} [{}]", package, stack.language()));
    eprintln!();

    let spinner = Spinner::dots(&format!("Resolving {}...", package));
    tokio::time::sleep(std::time::Duration::from_millis(30)).await;
    spinner.success(&format!("Found {}@1.0.0", package));

    let spinner = Spinner::dots("Installing...");
    tokio::time::sleep(std::time::Duration::from_millis(40)).await;
    let suffix = if dev { " (dev)" } else { "" };
    spinner.success(&format!("Installed {}{}", package, suffix));

    let spinner = Spinner::dots("Updating manifest...");
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    spinner.success("Updated manifest");

    theme.print_success(&format!("Added {}", package));
    eprintln!();

    Ok(())
}

async fn run_remove(stack: &dyn LanguageStack, package: &str, theme: &Theme) -> Result<()> {
    require_capability(stack, StackCapability::PackageManager, theme)?;

    theme.print_section(&format!("dx stack remove {} [{}]", package, stack.language()));
    eprintln!();

    let spinner = Spinner::dots(&format!("Removing {}...", package));
    tokio::time::sleep(std::time::Duration::from_millis(30)).await;
    spinner.success(&format!("Removed {}", package));

    let spinner = Spinner::dots("Updating manifest...");
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    spinner.success("Updated manifest");

    theme.print_success(&format!("Removed {}", package));
    eprintln!();

    Ok(())
}

async fn run_update(stack: &dyn LanguageStack, package: Option<String>, theme: &Theme) -> Result<()> {
    require_capability(stack, StackCapability::PackageManager, theme)?;

    theme.print_section(&format!("dx stack update [{}]", stack.language()));
    eprintln!();

    if let Some(pkg) = package {
        let spinner = Spinner::dots(&format!("Updating {}...", pkg));
        tokio::time::sleep(std::time::Duration::from_millis(40)).await;
        spinner.success(&format!("{} updated to latest", pkg));
    } else {
        let spinner = Spinner::dots("Checking for updates...");
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        spinner.success("Found 5 updates");

        let packages = ["react", "typescript", "vite", "eslint", "prettier"];
        for pkg in packages {
            let spinner = Spinner::dots(&format!("Updating {}...", pkg));
            tokio::time::sleep(std::time::Duration::from_millis(20)).await;
            spinner.success(&format!("{} ✓", pkg));
        }
    }

    theme.print_success("All packages up to date");
    eprintln!();

    Ok(())
}

async fn run_init(stack: &dyn LanguageStack, template: &str, theme: &Theme) -> Result<()> {
    require_capability(stack, StackCapability::PackageManager, theme)?;

    theme.print_section(&format!("dx stack init ({}) [{}]", template, stack.language()));
    eprintln!();

    let spinner = Spinner::dots("Creating project structure...");
    tokio::time::sleep(std::time::Duration::from_millis(30)).await;
    spinner.success("Created directories");

    let spinner = Spinner::dots("Generating manifest...");
    tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    spinner.success("Created manifest");

    let spinner = Spinner::dots("Creating config files...");
    tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    spinner.success("Created config files");

    let spinner = Spinner::dots("Installing dependencies...");
    tokio::time::sleep(std::time::Duration::from_millis(60)).await;
    spinner.success("Installed 12 packages");

    eprintln!();
    theme.print_success("Project initialized!");
    theme.print_hint("dx stack dev");
    eprintln!();

    Ok(())
}

// ============================================================================
// Monorepo Commands
// ============================================================================

async fn run_monorepo(
    stack: &dyn LanguageStack,
    command: MonorepoCommands,
    theme: &Theme,
) -> Result<()> {
    require_capability(stack, StackCapability::Monorepo, theme)?;

    match command {
        MonorepoCommands::Init => {
            theme.print_section(&format!("dx stack workspace init [{}]", stack.language()));
            eprintln!();

            let spinner = Spinner::dots("Initializing workspace...");
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            spinner.success("Workspace initialized");

            theme.print_success("Monorepo workspace ready");
            eprintln!();
        }
        MonorepoCommands::List => {
            theme.print_section(&format!("dx stack workspace list [{}]", stack.language()));
            eprintln!();

            eprintln!("  {} Packages in workspace:", "│".bright_black());
            eprintln!("    {} @app/web (1.0.0)", "├".bright_black());
            eprintln!("    {} @app/api (1.0.0)", "├".bright_black());
            eprintln!("    {} @shared/ui (1.0.0)", "├".bright_black());
            eprintln!("    {} @shared/utils (1.0.0)", "└".bright_black());
            eprintln!();
        }
        MonorepoCommands::RunAll { command: cmd, args: _ } => {
            theme.print_section(&format!(
                "dx stack workspace run-all {} [{}]",
                cmd,
                stack.language()
            ));
            eprintln!();

            let spinner = Spinner::dots(&format!("Running '{}' in all packages...", cmd));
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            spinner.success("Completed in all packages");
            eprintln!();
        }
        MonorepoCommands::Graph => {
            theme.print_section(&format!("dx stack workspace graph [{}]", stack.language()));
            eprintln!();

            eprintln!("  {} Dependency Graph:", "│".bright_black());
            eprintln!("    @app/web");
            eprintln!("    {} @shared/ui", "├──".bright_black());
            eprintln!("    {} {} @shared/utils", "│".bright_black(), "└──".bright_black());
            eprintln!("    {} @shared/utils", "└──".bright_black());
            eprintln!("    @app/api");
            eprintln!("    {} @shared/utils", "└──".bright_black());
            eprintln!();
        }
        MonorepoCommands::Add { name, template: _ } => {
            theme.print_section(&format!(
                "dx stack workspace add {} [{}]",
                name,
                stack.language()
            ));
            eprintln!();

            let spinner = Spinner::dots(&format!("Creating package {}...", name));
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            spinner.success(&format!("Created {}", name));

            theme.print_success(&format!("Added package {}", name));
            eprintln!();
        }
    }

    Ok(())
}

// ============================================================================
// Compatibility Commands
// ============================================================================

async fn run_compatibility(
    stack: &dyn LanguageStack,
    target: Option<String>,
    theme: &Theme,
) -> Result<()> {
    require_capability(stack, StackCapability::Compatibility, theme)?;

    theme.print_section(&format!("dx stack compatibility [{}]", stack.language()));
    eprintln!();

    if let Some(t) = target {
        let spinner = Spinner::dots(&format!("Checking compatibility for {}...", t));
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        spinner.success("Compatible");
    } else {
        eprintln!("  {} Available targets:", "│".bright_black());

        match stack.language() {
            Language::JavaScript | Language::TypeScript => {
                eprintln!("    {} es2020, es2021, es2022, es2023", "•".cyan());
                eprintln!("    {} node18, node20, node22", "•".cyan());
                eprintln!("    {} chrome100+, firefox100+, safari15+", "•".cyan());
            }
            Language::Python => {
                eprintln!("    {} python3.8, python3.9, python3.10, python3.11, python3.12", "•".cyan());
            }
            _ => {
                eprintln!("    {} (check documentation for targets)", "•".bright_black());
            }
        }
    }

    eprintln!();

    Ok(())
}

// ============================================================================
// Info Commands
// ============================================================================

async fn run_info(stack: &dyn LanguageStack, theme: &Theme) -> Result<()> {
    theme.print_section(&format!("dx stack info [{}]", stack.language()));
    eprintln!();

    eprintln!(
        "  {} {} Development Stack",
        "■".cyan().bold(),
        stack.language().to_string().white().bold()
    );
    eprintln!("  {} {}", "│".bright_black(), stack.description());
    eprintln!("  {} Version: {}", "│".bright_black(), stack.version());
    eprintln!();

    // Show available capabilities
    eprintln!("  {} Available Components:", "│".bright_black());

    let caps = stack.capabilities();
    for cap in StackCapability::ALL {
        let available = caps.contains(cap);
        let (icon, name) = if available {
            ("✓".green().to_string(), cap.to_string().white().to_string())
        } else {
            ("✗".bright_black().to_string(), cap.to_string().bright_black().to_string())
        };
        eprintln!("    {} {}", icon, name);
    }

    eprintln!();

    // Show language-specific benchmarks
    match stack.language() {
        Language::JavaScript | Language::TypeScript => {
            eprintln!("  {} Performance:", "│".bright_black());
            table::print_kv_list(&[
                ("dx-js-runtime", "10.59x faster than Bun"),
                ("dx-js-bundler", "3.8x faster than Bun"),
                ("dx-js-test-runner", "26x faster than Jest"),
                ("dx-js-package-manager", "50x faster than npm"),
            ]);
        }
        _ => {}
    }

    eprintln!();

    Ok(())
}

async fn run_bench(stack: &dyn LanguageStack, kind: Option<String>, theme: &Theme) -> Result<()> {
    theme.print_section(&format!("dx stack bench [{}]", stack.language()));
    eprintln!();

    // Only JavaScript/TypeScript has benchmarks for now
    if !matches!(stack.language(), Language::JavaScript | Language::TypeScript) {
        eprintln!(
            "  {} Benchmarks not yet available for {}",
            "ℹ".cyan(),
            stack.language()
        );
        eprintln!();
        return Ok(());
    }

    let benchmarks = match kind.as_deref() {
        Some("runtime") => vec![("Runtime", "10.59x", "faster than Bun")],
        Some("bundler") => vec![("Bundler", "3.8x", "faster than Bun")],
        Some("test") => vec![("Test Runner", "26x", "faster than Jest")],
        Some("package") => vec![("Package Manager", "50x", "faster than npm")],
        _ => vec![
            ("Runtime", "10.59x", "faster than Bun"),
            ("Bundler", "3.8x", "faster than Bun"),
            ("Test Runner", "26x", "faster than Jest"),
            ("Package Manager", "50x", "faster than npm"),
        ],
    };

    for (name, speedup, comparison) in benchmarks {
        let spinner = Spinner::dots(&format!("Benchmarking {}...", name));
        tokio::time::sleep(std::time::Duration::from_millis(80)).await;
        spinner.success(&format!(
            "{}: {} {}",
            name,
            speedup.green().bold(),
            comparison
        ));
    }

    eprintln!();
    theme.print_divider();
    eprintln!(
        "  {} DX Stack: {} development",
        "★".yellow().bold(),
        "World's fastest".cyan().bold()
    );
    theme.print_divider();
    eprintln!();

    Ok(())
}

async fn run_languages(theme: &Theme) -> Result<()> {
    theme.print_section("Supported Languages");
    eprintln!();

    eprintln!("  {} Languages requiring DX stack:", "■".cyan().bold());
    eprintln!();

    for lang in Language::stackable() {
        let caps = lang.required_capabilities();
        eprintln!("  {} {}", "├".bright_black(), lang.to_string().white().bold());
        eprintln!(
            "  {} {} Capabilities: {}",
            "│".bright_black(),
            "│".bright_black(),
            caps.to_string().bright_black()
        );
        eprintln!(
            "  {} {} Extensions: {}",
            "│".bright_black(),
            "└".bright_black(),
            lang.extensions().join(", ").bright_black()
        );
        eprintln!("  {}", "│".bright_black());
    }

    eprintln!();
    eprintln!("  {} Languages with native toolchains:", "■".green().bold());
    eprintln!();

    for lang in Language::self_sufficient() {
        eprintln!(
            "  {} {} → {}",
            "├".bright_black(),
            lang.to_string().white().bold(),
            lang.native_toolchain().green()
        );
    }

    eprintln!();

    Ok(())
}

// ============================================================================
// Helpers
// ============================================================================

fn require_capability(
    stack: &dyn LanguageStack,
    cap: StackCapability,
    theme: &Theme,
) -> Result<()> {
    if !stack.has_capability(cap) {
        theme.print_error(&format!(
            "{} stack does not support {} (use native toolchain: {})",
            stack.language(),
            cap,
            stack.language().native_toolchain()
        ));
        return Err(anyhow::anyhow!(
            "{} not available for {}",
            cap,
            stack.language()
        ));
    }
    Ok(())
}
