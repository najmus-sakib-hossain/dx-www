This is a very big task so please create planing and then create tasklist and then complete all todos one by one systemitically and your agent mode will take so much question and tokens so please do as much works as you can possibily can in one go but make sure that don't try to do too so we will get errors, just try to do as efficiently and systemitically as you can!!!

Please create a new rust crate at crates folder called "dx" where unite all dx crates so that we can access all dx tools via that dx cli with these features:

```markdown
# Modern DX CLI - Complete Implementation

Here's a comprehensive, modern CLI for your dx project with a Vercel-like aesthetic clean and systemized structure:

## Project Structure

```
crates/dx/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── cli.rs
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── init.rs
│   │   ├── dev.rs
│   │   ├── build.rs
│   │   ├── run.rs
│   │   ├── bundle.rs
│   │   ├── install.rs
│   │   ├── test.rs
│   │   ├── style.rs
│   │   ├── forge.rs
│   │   ├── info.rs
│   │   └── clean.rs
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── theme.rs
│   │   ├── spinner.rs
│   │   ├── table.rs
│   │   ├── progress.rs
│   │   └── logger.rs
│   ├── config/
│   │   ├── mod.rs
│   │   └── dx_config.rs
│   └── utils/
│       ├── mod.rs
│       ├── paths.rs
│       └── system.rs
```

## Cargo.toml

```toml
[package]
name = "dx"
version = "0.1.0"
edition = "2024"
authors = ["DX Team"]
description = "The Binary-First Development Experience CLI"
license = "MIT OR Apache-2.0"
repository = "https://github.com/aspect-build/dx"
keywords = ["cli", "build-tool", "bundler", "runtime", "webassembly"]
categories = ["command-line-utilities", "development-tools", "wasm"]

[[bin]]
name = "dx"
path = "src/main.rs"

[dependencies]
# CLI Framework
clap = { version = "4.5", features = ["derive", "env", "string", "wrap_help"] }

# Terminal UI
console = "0.15"
indicatif = { version = "0.17", features = ["rayon", "tokio"] }
dialoguer = { version = "0.11", features = ["fuzzy-select"] }
owo-colors = { version = "4.0", features = ["supports-colors"] }

# Async Runtime
tokio = { version = "1.40", features = ["full", "parking_lot"] }

# Configuration
toml = "0.8"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# File System
walkdir = "2.5"
notify = { version = "6.1", default-features = false, features = ["macos_kqueue"] }
ignore = "0.4"
glob = "0.3"

# Utilities
anyhow = "1.0"
thiserror = "2.0"
chrono = { version = "0.4", features = ["serde"] }
human_bytes = "0.4"
humantime = "2.1"
ctrlc = "3.4"
which = "6.0"
home = "0.5"
open = "5.3"
url = "2.5"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }

[target.'cfg(unix)'.dependencies]
libc = "0.2"

[target.'cfg(windows)'.dependencies]
winapi = { version = "0.3", features = ["processthreadsapi"] }

[dev-dependencies]
assert_cmd = "2.0"
predicates = "3.1"
tempfile = "3.12"

[features]
default = []
self-update = []  # For future self-update functionality
```

## src/main.rs

```rust
//! DX CLI - The Binary-First Development Experience
//! 
//! A modern, high-performance CLI for the DX development platform.
//! Provides unified control over all dx-* tools with a clean, Vercel-like UX.

use anyhow::Result;
use clap::Parser;

mod cli;
mod commands;
mod config;
mod ui;
mod utils;

use cli::Cli;
use ui::logger::Logger;

#[tokio::main]
async fn main() {
    // Initialize the logger with colors disabled if not a TTY
    Logger::init();

    // Parse CLI arguments
    let cli = Cli::parse();

    // Handle Ctrl+C gracefully
    ctrlc::set_handler(move || {
        ui::theme::Theme::new().print_cancelled();
        std::process::exit(130);
    })
    .expect("Error setting Ctrl+C handler");

    // Run the CLI
    if let Err(err) = cli.run().await {
        ui::logger::error(&format!("{err:#}"));
        std::process::exit(1);
    }
}
```

## src/cli.rs

```rust
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

    /// Run dx-forge build orchestration
    Forge(commands::forge::ForgeArgs),

    /// Display system and project information
    Info(commands::info::InfoArgs),

    /// Clean build artifacts and caches
    Clean(commands::clean::CleanArgs),

    /// Upgrade DX to the latest version
    #[command(hide = true)]
    Upgrade(commands::UpgradeArgs),
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
```

## src/ui/mod.rs

```rust
//! UI module - Terminal styling and output utilities

pub mod logger;
pub mod progress;
pub mod spinner;
pub mod table;
pub mod theme;
```

## src/ui/theme.rs

```rust
//! Theme definitions for consistent CLI styling

use console::{style, Style, Term};
use owo_colors::OwoColorize;

/// The DX CLI theme
pub struct Theme {
    term: Term,
    pub primary: Style,
    pub secondary: Style,
    pub success: Style,
    pub warning: Style,
    pub error: Style,
    pub dim: Style,
    pub bold: Style,
    pub highlight: Style,
}

impl Default for Theme {
    fn default() -> Self {
        Self::new()
    }
}

impl Theme {
    pub fn new() -> Self {
        Self {
            term: Term::stderr(),
            primary: Style::new().cyan(),
            secondary: Style::new().magenta(),
            success: Style::new().green(),
            warning: Style::new().yellow(),
            error: Style::new().red(),
            dim: Style::new().dim(),
            bold: Style::new().bold(),
            highlight: Style::new().cyan().bold(),
        }
    }

    /// Print the DX header/logo
    pub fn print_header(&self) {
        let version = env!("CARGO_PKG_VERSION");
        
        eprintln!();
        eprintln!(
            "  {}  {}",
            "▲".cyan().bold(),
            format!("DX v{version}").bright_white().bold()
        );
        eprintln!();
    }

    /// Print a section header
    pub fn print_section(&self, title: &str) {
        eprintln!(
            "  {} {}",
            "│".bright_black(),
            title.bright_white().bold()
        );
    }

    /// Print an info line
    pub fn print_info(&self, label: &str, value: &str) {
        eprintln!(
            "  {} {}: {}",
            "│".bright_black(),
            label.bright_black(),
            value.white()
        );
    }

    /// Print a success message
    pub fn print_success(&self, message: &str) {
        eprintln!();
        eprintln!(
            "  {} {}",
            "✓".green().bold(),
            message.white()
        );
    }

    /// Print a warning message
    pub fn print_warning(&self, message: &str) {
        eprintln!(
            "  {} {}",
            "⚠".yellow().bold(),
            message.yellow()
        );
    }

    /// Print an error message
    pub fn print_error(&self, message: &str) {
        eprintln!();
        eprintln!(
            "  {} {}",
            "✕".red().bold(),
            message.red()
        );
    }

    /// Print a step in a process
    pub fn print_step(&self, step: usize, total: usize, message: &str) {
        eprintln!(
            "  {} {} {}",
            format!("[{step}/{total}]").bright_black(),
            "→".cyan(),
            message.white()
        );
    }

    /// Print a command hint
    pub fn print_hint(&self, command: &str) {
        eprintln!();
        eprintln!(
            "  {} Run {} to get started",
            "→".cyan(),
            format!("`{command}`").cyan().bold()
        );
    }

    /// Print a link
    pub fn print_link(&self, label: &str, url: &str) {
        eprintln!(
            "  {} {}: {}",
            "│".bright_black(),
            label.bright_black(),
            url.cyan().underline()
        );
    }

    /// Print a divider line
    pub fn print_divider(&self) {
        eprintln!(
            "  {}",
            "─".repeat(48).bright_black()
        );
    }

    /// Print an empty line with the sidebar
    pub fn print_empty(&self) {
        eprintln!("  {}", "│".bright_black());
    }

    /// Print ready message for dev server
    pub fn print_ready(&self, url: &str, time_ms: u64) {
        eprintln!();
        eprintln!(
            "  {} Ready in {}",
            "✓".green().bold(),
            format!("{time_ms}ms").cyan().bold()
        );
        eprintln!();
        eprintln!(
            "  {} Local:   {}",
            "→".cyan(),
            url.cyan().bold().underline()
        );
        eprintln!();
    }

    /// Print cancelled message
    pub fn print_cancelled(&self) {
        eprintln!();
        eprintln!(
            "  {} Cancelled",
            "○".bright_black()
        );
        eprintln!();
    }

    /// Print build stats
    pub fn print_build_stats(&self, duration_ms: u64, bundle_size: &str, files: usize) {
        eprintln!();
        self.print_divider();
        eprintln!(
            "  {} Built in {} │ {} │ {} files",
            "✓".green().bold(),
            format!("{duration_ms}ms").cyan().bold(),
            bundle_size.magenta().bold(),
            files.to_string().white().bold()
        );
        self.print_divider();
        eprintln!();
    }

    /// Print test results
    pub fn print_test_results(&self, passed: usize, failed: usize, skipped: usize, duration_ms: u64) {
        eprintln!();
        self.print_divider();
        
        let status = if failed == 0 {
            "PASS".green().bold()
        } else {
            "FAIL".red().bold()
        };

        eprintln!(
            "  {} {} {} passed {} {} failed {} {} skipped {} in {}",
            status,
            "│".bright_black(),
            passed.to_string().green().bold(),
            "│".bright_black(),
            failed.to_string().if_supports_color(owo_colors::Stream::Stderr, |t| {
                if failed > 0 { t.red().bold() } else { t.bright_black() }
            }),
            "│".bright_black(),
            skipped.to_string().bright_black(),
            "│".bright_black(),
            format!("{duration_ms}ms").cyan()
        );
        
        self.print_divider();
        eprintln!();
    }

    /// Get terminal width
    pub fn width(&self) -> usize {
        self.term.size().1 as usize
    }

    /// Clear the terminal
    pub fn clear(&self) {
        let _ = self.term.clear_screen();
    }
}

/// ASCII art logo for splash screens (kept minimal)
pub const LOGO_SMALL: &str = r"
   ██████╗ ██╗  ██╗
   ██╔══██╗╚██╗██╔╝
   ██║  ██║ ╚███╔╝ 
   ██║  ██║ ██╔██╗ 
   ██████╔╝██╔╝ ██╗
   ╚═════╝ ╚═╝  ╚═╝
";
```

## src/ui/spinner.rs

```rust
//! Spinner and loading indicators

use indicatif::{ProgressBar, ProgressStyle};
use owo_colors::OwoColorize;
use std::time::Duration;

/// A modern spinner for async operations
pub struct Spinner {
    pb: ProgressBar,
    message: String,
}

impl Spinner {
    /// Create a new spinner with a message
    pub fn new(message: &str) -> Self {
        let pb = ProgressBar::new_spinner();
        
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(&[
                    "   ◐",
                    "   ◓",
                    "   ◑",
                    "   ◒",
                ])
                .template("  {spinner:.cyan} {msg}")
                .unwrap(),
        );

        pb.set_message(message.to_string());
        pb.enable_steady_tick(Duration::from_millis(80));

        Self {
            pb,
            message: message.to_string(),
        }
    }

    /// Create a spinner with dots style
    pub fn dots(message: &str) -> Self {
        let pb = ProgressBar::new_spinner();
        
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(&[
                    "   ⠋",
                    "   ⠙",
                    "   ⠹",
                    "   ⠸",
                    "   ⠼",
                    "   ⠴",
                    "   ⠦",
                    "   ⠧",
                    "   ⠇",
                    "   ⠏",
                ])
                .template("  {spinner:.cyan} {msg}")
                .unwrap(),
        );

        pb.set_message(message.to_string());
        pb.enable_steady_tick(Duration::from_millis(80));

        Self {
            pb,
            message: message.to_string(),
        }
    }

    /// Update the spinner message
    pub fn set_message(&self, message: &str) {
        self.pb.set_message(message.to_string());
    }

    /// Mark spinner as successful
    pub fn success(self, message: &str) {
        self.pb.finish_and_clear();
        eprintln!(
            "  {} {}",
            "✓".green().bold(),
            message.white()
        );
    }

    /// Mark spinner as failed
    pub fn error(self, message: &str) {
        self.pb.finish_and_clear();
        eprintln!(
            "  {} {}",
            "✕".red().bold(),
            message.red()
        );
    }

    /// Mark spinner as warning
    pub fn warn(self, message: &str) {
        self.pb.finish_and_clear();
        eprintln!(
            "  {} {}",
            "⚠".yellow().bold(),
            message.yellow()
        );
    }

    /// Finish the spinner without a status
    pub fn finish(self) {
        self.pb.finish_and_clear();
    }

    /// Stop the spinner without clearing (for intermediate output)
    pub fn suspend<F, R>(&self, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        self.pb.suspend(f)
    }
}

impl Drop for Spinner {
    fn drop(&mut self) {
        self.pb.finish_and_clear();
    }
}

/// Multi-spinner for parallel operations
pub struct MultiSpinner {
    spinners: Vec<Spinner>,
}

impl MultiSpinner {
    pub fn new() -> Self {
        Self {
            spinners: Vec::new(),
        }
    }

    pub fn add(&mut self, message: &str) -> usize {
        let spinner = Spinner::dots(message);
        self.spinners.push(spinner);
        self.spinners.len() - 1
    }

    pub fn finish_one(&mut self, index: usize, message: &str) {
        if let Some(spinner) = self.spinners.get(index) {
            spinner.set_message(message);
        }
    }
}
```

## src/ui/progress.rs

```rust
//! Progress bars for long-running operations

use indicatif::{ProgressBar, ProgressStyle, MultiProgress};
use owo_colors::OwoColorize;
use std::time::Duration;

/// A styled progress bar
pub struct Progress {
    pb: ProgressBar,
}

impl Progress {
    /// Create a new progress bar with a total count
    pub fn new(total: u64, message: &str) -> Self {
        let pb = ProgressBar::new(total);
        
        pb.set_style(
            ProgressStyle::default_bar()
                .template("  {msg}\n  [{bar:40.cyan/bright_black}] {pos}/{len} {eta}")
                .unwrap()
                .progress_chars("━━─"),
        );

        pb.set_message(message.to_string());

        Self { pb }
    }

    /// Create a byte-based progress bar (for downloads)
    pub fn bytes(total: u64, message: &str) -> Self {
        let pb = ProgressBar::new(total);
        
        pb.set_style(
            ProgressStyle::default_bar()
                .template("  {msg}\n  [{bar:40.cyan/bright_black}] {bytes}/{total_bytes} ({bytes_per_sec})")
                .unwrap()
                .progress_chars("━━─"),
        );

        pb.set_message(message.to_string());

        Self { pb }
    }

    /// Increment the progress
    pub fn inc(&self, delta: u64) {
        self.pb.inc(delta);
    }

    /// Set the current position
    pub fn set_position(&self, pos: u64) {
        self.pb.set_position(pos);
    }

    /// Update the message
    pub fn set_message(&self, message: &str) {
        self.pb.set_message(message.to_string());
    }

    /// Finish the progress bar with a success message
    pub fn finish_success(self, message: &str) {
        self.pb.finish_and_clear();
        eprintln!(
            "  {} {}",
            "✓".green().bold(),
            message.white()
        );
    }

    /// Finish the progress bar with an error
    pub fn finish_error(self, message: &str) {
        self.pb.finish_and_clear();
        eprintln!(
            "  {} {}",
            "✕".red().bold(),
            message.red()
        );
    }

    /// Get the inner progress bar for advanced usage
    pub fn inner(&self) -> &ProgressBar {
        &self.pb
    }
}

/// Progress tracker for multiple parallel operations
pub struct MultiProgressTracker {
    mp: MultiProgress,
    bars: Vec<ProgressBar>,
}

impl MultiProgressTracker {
    pub fn new() -> Self {
        Self {
            mp: MultiProgress::new(),
            bars: Vec::new(),
        }
    }

    /// Add a new progress bar
    pub fn add(&mut self, total: u64, message: &str) -> usize {
        let pb = self.mp.add(ProgressBar::new(total));
        
        pb.set_style(
            ProgressStyle::default_bar()
                .template("  {prefix:.cyan} [{bar:30.cyan/bright_black}] {pos}/{len}")
                .unwrap()
                .progress_chars("━━─"),
        );

        pb.set_prefix(message.to_string());
        
        self.bars.push(pb);
        self.bars.len() - 1
    }

    /// Increment a specific progress bar
    pub fn inc(&self, index: usize, delta: u64) {
        if let Some(pb) = self.bars.get(index) {
            pb.inc(delta);
        }
    }

    /// Finish a specific progress bar
    pub fn finish(&self, index: usize) {
        if let Some(pb) = self.bars.get(index) {
            pb.finish();
        }
    }

    /// Finish all progress bars
    pub fn finish_all(self) {
        for pb in self.bars {
            pb.finish_and_clear();
        }
    }
}

impl Default for MultiProgressTracker {
    fn default() -> Self {
        Self::new()
    }
}
```

## src/ui/table.rs

```rust
//! Table formatting for CLI output

use owo_colors::OwoColorize;

/// A simple table formatter for CLI output
pub struct Table {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    column_widths: Vec<usize>,
}

impl Table {
    pub fn new(headers: Vec<&str>) -> Self {
        let headers: Vec<String> = headers.into_iter().map(String::from).collect();
        let column_widths = headers.iter().map(|h| h.len()).collect();
        
        Self {
            headers,
            rows: Vec::new(),
            column_widths,
        }
    }

    pub fn add_row(&mut self, row: Vec<&str>) {
        let row: Vec<String> = row.into_iter().map(String::from).collect();
        
        // Update column widths
        for (i, cell) in row.iter().enumerate() {
            if i < self.column_widths.len() {
                self.column_widths[i] = self.column_widths[i].max(cell.len());
            }
        }
        
        self.rows.push(row);
    }

    pub fn print(&self) {
        // Print header
        eprint!("  ");
        for (i, header) in self.headers.iter().enumerate() {
            let width = self.column_widths.get(i).copied().unwrap_or(0);
            eprint!("{:width$}  ", header.bright_black().bold(), width = width);
        }
        eprintln!();

        // Print separator
        eprint!("  ");
        for width in &self.column_widths {
            eprint!("{:─<width$}  ", "", width = *width);
        }
        eprintln!();

        // Print rows
        for row in &self.rows {
            eprint!("  ");
            for (i, cell) in row.iter().enumerate() {
                let width = self.column_widths.get(i).copied().unwrap_or(0);
                eprint!("{:width$}  ", cell.white(), width = width);
            }
            eprintln!();
        }
    }
}

/// Print a key-value list
pub fn print_kv_list(items: &[(&str, &str)]) {
    let max_key_len = items.iter().map(|(k, _)| k.len()).max().unwrap_or(0);
    
    for (key, value) in items {
        eprintln!(
            "  {} {:width$}  {}",
            "│".bright_black(),
            format!("{key}:").bright_black(),
            value.white(),
            width = max_key_len + 1
        );
    }
}

/// Print a file tree
pub fn print_file_tree(files: &[(&str, &str)]) {
    eprintln!();
    for (i, (path, info)) in files.iter().enumerate() {
        let prefix = if i == files.len() - 1 { "└──" } else { "├──" };
        eprintln!(
            "  {} {} {}",
            prefix.bright_black(),
            path.white(),
            format!("({info})").bright_black()
        );
    }
    eprintln!();
}
```

## src/ui/logger.rs

```rust
//! Logging utilities

use owo_colors::OwoColorize;
use tracing_subscriber::{fmt, EnvFilter};

/// Initialize the logger
pub fn init() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("warn"));

    fmt()
        .with_env_filter(filter)
        .with_target(false)
        .without_time()
        .init();
}

/// Log an info message
pub fn info(message: &str) {
    eprintln!(
        "  {} {}",
        "→".cyan(),
        message.white()
    );
}

/// Log a success message
pub fn success(message: &str) {
    eprintln!(
        "  {} {}",
        "✓".green().bold(),
        message.white()
    );
}

/// Log a warning message
pub fn warn(message: &str) {
    eprintln!(
        "  {} {}",
        "⚠".yellow().bold(),
        message.yellow()
    );
}

/// Log an error message
pub fn error(message: &str) {
    eprintln!();
    eprintln!(
        "  {} {}",
        "Error:".red().bold(),
        message.red()
    );
    eprintln!();
}

/// Log a debug message (only in verbose mode)
pub fn debug(message: &str) {
    eprintln!(
        "  {} {}",
        "debug".bright_black(),
        message.bright_black()
    );
}

/// Log a step in a process
pub fn step(number: usize, message: &str) {
    eprintln!(
        "  {} {}",
        format!("{number}.").cyan().bold(),
        message.white()
    );
}

/// Log a list item
pub fn list_item(message: &str) {
    eprintln!(
        "  {} {}",
        "•".bright_black(),
        message.white()
    );
}

/// Log a code block / command
pub fn code(command: &str) {
    eprintln!();
    eprintln!("  {}", format!("$ {command}").cyan());
    eprintln!();
}
```

## src/commands/mod.rs

```rust
//! Command implementations

pub mod build;
pub mod bundle;
pub mod clean;
pub mod dev;
pub mod forge;
pub mod info;
pub mod init;
pub mod install;
pub mod run;
pub mod style;
pub mod test;
pub mod upgrade;

// Re-export common types
pub use upgrade::UpgradeArgs;
```

## src/commands/init.rs

```rust
//! Initialize a new DX project

use anyhow::{Context, Result};
use clap::Args;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use std::fs;
use std::path::PathBuf;

use crate::ui::{spinner::Spinner, theme::Theme};

#[derive(Args)]
pub struct InitArgs {
    /// Project name (defaults to current directory)
    #[arg(index = 1)]
    pub name: Option<String>,

    /// Template to use
    #[arg(short, long, value_parser = ["counter", "dashboard", "hackernews", "minimal", "api"])]
    pub template: Option<String>,

    /// Skip interactive prompts
    #[arg(short = 'y', long)]
    pub yes: bool,

    /// Install dependencies after init
    #[arg(long, default_value = "true")]
    pub install: bool,

    /// Initialize git repository
    #[arg(long, default_value = "true")]
    pub git: bool,

    /// Directory to create project in
    #[arg(short, long)]
    pub dir: Option<PathBuf>,
}

pub async fn run(args: InitArgs, theme: &Theme) -> Result<()> {
    let (project_name, template) = if args.yes {
        // Non-interactive mode
        let name = args.name.unwrap_or_else(|| "dx-app".to_string());
        let template = args.template.unwrap_or_else(|| "counter".to_string());
        (name, template)
    } else {
        // Interactive mode
        interactive_init(&args)?
    };

    let project_dir = args.dir.unwrap_or_else(|| PathBuf::from(&project_name));

    // Create project
    let spinner = Spinner::new(&format!("Creating {} project...", project_name));

    // Check if directory exists
    if project_dir.exists() {
        spinner.error(&format!("Directory '{}' already exists", project_dir.display()));
        return Ok(());
    }

    // Create directory structure
    create_project_structure(&project_dir, &project_name, &template)?;

    spinner.success(&format!("Created project '{}'", project_name));

    // Initialize git
    if args.git {
        let git_spinner = Spinner::dots("Initializing git repository...");
        if init_git(&project_dir).is_ok() {
            git_spinner.success("Initialized git repository");
        } else {
            git_spinner.warn("Could not initialize git (git not found)");
        }
    }

    // Install dependencies
    if args.install {
        let install_spinner = Spinner::dots("Installing dependencies...");
        // Simulate install - in real implementation, call dx-js-package-manager
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        install_spinner.success("Installed dependencies");
    }

    // Print success message and next steps
    theme.print_success(&format!("Project '{}' created successfully!", project_name));
    
    eprintln!();
    theme.print_section("Next steps:");
    eprintln!();
    
    if project_dir != PathBuf::from(".") {
        crate::ui::logger::step(1, &format!("cd {}", project_dir.display()));
    }
    crate::ui::logger::step(2, "dx dev");
    
    eprintln!();
    theme.print_link("Docs", "https://dx.dev/docs");
    eprintln!();

    Ok(())
}

fn interactive_init(args: &InitArgs) -> Result<(String, String)> {
    let theme = ColorfulTheme::default();

    eprintln!();
    
    // Get project name
    let name: String = if let Some(ref name) = args.name {
        name.clone()
    } else {
        Input::with_theme(&theme)
            .with_prompt("  Project name")
            .default("dx-app".to_string())
            .interact_text()?
    };

    // Select template
    let templates = ["counter", "dashboard", "hackernews", "minimal", "api"];
    let template_descriptions = [
        "counter      - Simple counter example (recommended for beginners)",
        "dashboard    - SaaS dashboard with charts and tables",
        "hackernews   - Hacker News clone (real-world example)",
        "minimal      - Bare minimum setup",
        "api          - API server only (no frontend)",
    ];

    let template = if let Some(ref t) = args.template {
        t.clone()
    } else {
        let selection = Select::with_theme(&theme)
            .with_prompt("  Select a template")
            .items(&template_descriptions)
            .default(0)
            .interact()?;
        
        templates[selection].to_string()
    };

    Ok((name, template))
}

fn create_project_structure(dir: &PathBuf, name: &str, template: &str) -> Result<()> {
    fs::create_dir_all(dir)?;
    fs::create_dir_all(dir.join("src"))?;
    fs::create_dir_all(dir.join("public"))?;

    // Create dx.toml
    let config = format!(
        r#"[project]
name = "{name}"
version = "0.1.0"

[build]
target = "browser"
minify = true

[dev]
port = 3000
open = true

[runtime]
jsx = "dx"
typescript = true
"#
    );
    fs::write(dir.join("dx.toml"), config)?;

    // Create main file based on template
    let main_content = match template {
        "counter" => include_str!("../templates/counter.tsx"),
        "minimal" => include_str!("../templates/minimal.tsx"),
        _ => include_str!("../templates/counter.tsx"),
    };
    fs::write(dir.join("src/main.tsx"), main_content)?;

    // Create index.html
    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{name}</title>
</head>
<body>
    <div id="root"></div>
    <script type="module" src="/src/main.tsx"></script>
</body>
</html>
"#
    );
    fs::write(dir.join("index.html"), html)?;

    // Create .gitignore
    let gitignore = r#"# Build output
dist/
.dx/

# Dependencies
node_modules/
dx_modules/

# IDE
.idea/
*.swp
*.swo
.DS_Store

# Logs
*.log
"#;
    fs::write(dir.join(".gitignore"), gitignore)?;

    Ok(())
}

fn init_git(dir: &PathBuf) -> Result<()> {
    std::process::Command::new("git")
        .args(["init"])
        .current_dir(dir)
        .output()
        .context("Failed to initialize git")?;
    Ok(())
}
```

## src/commands/dev.rs

```rust
//! Development server with hot reload

use anyhow::Result;
use clap::Args;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use owo_colors::OwoColorize;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};

use crate::ui::{spinner::Spinner, theme::Theme};

#[derive(Args)]
pub struct DevArgs {
    /// Port to run the dev server on
    #[arg(short, long, default_value = "3000")]
    pub port: u16,

    /// Host to bind to
    #[arg(long, default_value = "localhost")]
    pub host: String,

    /// Open browser automatically
    #[arg(long)]
    pub open: bool,

    /// Enable HTTPS
    #[arg(long)]
    pub https: bool,

    /// Directory to serve (defaults to current directory)
    #[arg(index = 1)]
    pub dir: Option<PathBuf>,

    /// Disable hot reload
    #[arg(long)]
    pub no_hmr: bool,
}

pub async fn run(args: DevArgs, theme: &Theme) -> Result<()> {
    let start = Instant::now();
    let dir = args.dir.unwrap_or_else(|| PathBuf::from("."));

    // Validate project directory
    if !dir.join("dx.toml").exists() && !dir.join("package.json").exists() {
        theme.print_warning("No dx.toml or package.json found. Running in static mode.");
    }

    let spinner = Spinner::dots("Starting development server...");

    // Simulate build
    tokio::time::sleep(Duration::from_millis(200)).await;

    spinner.finish();

    let protocol = if args.https { "https" } else { "http" };
    let url = format!("{}://{}:{}", protocol, args.host, args.port);
    
    theme.print_ready(&url, start.elapsed().as_millis() as u64);

    // Print watching message
    eprintln!(
        "  {} Watching for changes in {}",
        "○".bright_black(),
        dir.display().to_string().cyan()
    );
    eprintln!();

    // Open browser if requested
    if args.open {
        let _ = open::that(&url);
    }

    // Set up file watcher
    if !args.no_hmr {
        let (tx, rx) = channel();
        
        let mut watcher = RecommendedWatcher::new(
            move |res| {
                if let Ok(event) = res {
                    let _ = tx.send(event);
                }
            },
            Config::default(),
        )?;

        watcher.watch(&dir, RecursiveMode::Recursive)?;

        // Watch for changes
        loop {
            match rx.recv_timeout(Duration::from_secs(1)) {
                Ok(event) => {
                    if let notify::Event { kind: notify::EventKind::Modify(_), paths, .. } = event {
                        for path in paths {
                            let relative = path.strip_prefix(&dir).unwrap_or(&path);
                            eprintln!(
                                "  {} Rebuilt in {}",
                                "↻".cyan(),
                                relative.display().to_string().bright_black()
                            );
                        }
                    }
                }
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                    // Keep running
                }
                Err(_) => break,
            }
        }
    } else {
        // Just keep the server running
        loop {
            tokio::time::sleep(Duration::from_secs(60)).await;
        }
    }

    Ok(())
}
```

## src/commands/build.rs

```rust
//! Production build command

use anyhow::Result;
use clap::Args;
use owo_colors::OwoColorize;
use std::path::PathBuf;
use std::time::Instant;

use crate::ui::{progress::Progress, spinner::Spinner, table, theme::Theme};

#[derive(Args)]
pub struct BuildArgs {
    /// Output directory
    #[arg(short, long, default_value = "dist")]
    pub out_dir: PathBuf,

    /// Enable minification
    #[arg(long, default_value = "true")]
    pub minify: bool,

    /// Enable source maps
    #[arg(long)]
    pub sourcemap: bool,

    /// Target environment
    #[arg(long, default_value = "browser", value_parser = ["browser", "node", "wasm"])]
    pub target: String,

    /// Enable watch mode
    #[arg(short, long)]
    pub watch: bool,

    /// Analyze bundle size
    #[arg(long)]
    pub analyze: bool,
}

pub async fn run(args: BuildArgs, theme: &Theme) -> Result<()> {
    let start = Instant::now();

    theme.print_section("Building for production...");
    eprintln!();

    // Step 1: Analyze source files
    let spinner = Spinner::dots("Analyzing source files...");
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    spinner.success("Found 12 source files");

    // Step 2: Transform TypeScript/TSX
    let spinner = Spinner::dots("Transforming TypeScript...");
    tokio::time::sleep(std::time::Duration::from_millis(150)).await;
    spinner.success("Transformed 12 files");

    // Step 3: Bundle with dx-js-bundler
    let spinner = Spinner::dots("Bundling with dx-js-bundler...");
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    spinner.success("Bundled in 10.05ms (3.8x faster than Bun)");

    // Step 4: Generate binary CSS
    let spinner = Spinner::dots("Generating binary CSS...");
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    spinner.success("Generated B-CSS (98% smaller)");

    // Step 5: Compile to WASM (if applicable)
    if args.target == "wasm" || args.target == "browser" {
        let spinner = Spinner::dots("Compiling to WebAssembly...");
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        spinner.success("Compiled WASM binary");
    }

    // Step 6: Optimize and minify
    if args.minify {
        let spinner = Spinner::dots("Minifying output...");
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        spinner.success("Minified all assets");
    }

    // Print output files
    eprintln!();
    theme.print_section("Output files:");
    table::print_file_tree(&[
        ("dist/index.html", "1.2 KB"),
        ("dist/main.wasm", "23.3 KB"),
        ("dist/main.js", "0.5 KB"),
        ("dist/styles.bcss", "2.1 KB"),
        ("dist/assets/", "12 files"),
    ]);

    // Print build stats
    let duration = start.elapsed().as_millis() as u64;
    theme.print_build_stats(duration, "27.1 KB", 5);

    if args.analyze {
        eprintln!();
        theme.print_section("Bundle Analysis:");
        eprintln!();
        
        let mut table = table::Table::new(vec!["Module", "Size", "Gzip"]);
        table.add_row(vec!["main.wasm", "23.3 KB", "8.2 KB"]);
        table.add_row(vec!["main.js", "0.5 KB", "0.3 KB"]);
        table.add_row(vec!["styles.bcss", "2.1 KB", "0.8 KB"]);
        table.add_row(vec!["runtime", "1.0 KB", "0.4 KB"]);
        table.print();
    }

    theme.print_hint(&format!("dx preview --dir {}", args.out_dir.display()));

    Ok(())
}
```

## src/commands/run.rs

```rust
//! Run files with dx-js-runtime

use anyhow::{Context, Result};
use clap::Args;
use owo_colors::OwoColorize;
use std::path::PathBuf;
use std::time::Instant;

use crate::ui::{spinner::Spinner, theme::Theme};

#[derive(Args)]
pub struct RunArgs {
    /// File to run
    #[arg(index = 1, required = true)]
    pub file: PathBuf,

    /// Arguments to pass to the script
    #[arg(trailing_var_arg = true)]
    pub args: Vec<String>,

    /// Enable watch mode
    #[arg(short, long)]
    pub watch: bool,

    /// Show timing information
    #[arg(long)]
    pub timing: bool,
}

pub async fn run(args: RunArgs, theme: &Theme) -> Result<()> {
    let start = Instant::now();

    // Validate file exists
    if !args.file.exists() {
        theme.print_error(&format!("File not found: {}", args.file.display()));
        return Ok(());
    }

    let extension = args.file.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    // Show what we're running
    eprintln!(
        "  {} Running {}",
        "▲".cyan().bold(),
        args.file.display().to_string().white()
    );
    eprintln!();

    // Simulate running with dx-js-runtime
    let compile_start = Instant::now();
    
    // In real implementation, this would call dx-js-runtime
    // For demo, we simulate the output
    
    if args.timing {
        let compile_time = compile_start.elapsed().as_micros();
        eprintln!(
            "  {} Compiled in {}",
            "→".bright_black(),
            format!("{}µs", compile_time).cyan()
        );
    }

    // Simulate script output
    eprintln!("  {} {}", "│".bright_black(), "Hello from dx-js-runtime!".white());
    eprintln!("  {} {}", "│".bright_black(), format!("Running: {}", args.file.display()).bright_black());
    
    if !args.args.is_empty() {
        eprintln!(
            "  {} Args: {}",
            "│".bright_black(),
            args.args.join(" ").bright_black()
        );
    }

    if args.timing {
        let total_time = start.elapsed().as_millis();
        eprintln!();
        eprintln!(
            "  {} Finished in {} (10.59x faster than Bun)",
            "✓".green().bold(),
            format!("{}ms", total_time).cyan().bold()
        );
    }

    Ok(())
}
```

## src/commands/bundle.rs

```rust
//! Bundle files with dx-js-bundler

use anyhow::Result;
use clap::Args;
use owo_colors::OwoColorize;
use std::path::PathBuf;
use std::time::Instant;

use crate::ui::{spinner::Spinner, theme::Theme, table};

#[derive(Args)]
pub struct BundleArgs {
    /// Entry point file
    #[arg(index = 1, required = true)]
    pub entry: PathBuf,

    /// Output file
    #[arg(short, long)]
    pub out: Option<PathBuf>,

    /// Bundle format
    #[arg(long, default_value = "esm", value_parser = ["esm", "cjs", "iife"])]
    pub format: String,

    /// Enable minification
    #[arg(long)]
    pub minify: bool,

    /// External packages (don't bundle)
    #[arg(long)]
    pub external: Vec<String>,

    /// Enable Fusion mode (pre-compiled .dxm modules)
    #[arg(long)]
    pub fusion: bool,

    /// Enable source maps
    #[arg(long)]
    pub sourcemap: bool,
}

pub async fn run(args: BundleArgs, theme: &Theme) -> Result<()> {
    let start = Instant::now();

    // Validate entry point
    if !args.entry.exists() {
        theme.print_error(&format!("Entry point not found: {}", args.entry.display()));
        return Ok(());
    }

    let out_file = args.out.unwrap_or_else(|| {
        let stem = args.entry.file_stem().unwrap().to_str().unwrap();
        PathBuf::from(format!("{}.bundle.js", stem))
    });

    theme.print_section("dx-js-bundler");
    eprintln!();

    let mode = if args.fusion {
        "Fusion mode (0.7ms target)"
    } else {
        "Standard mode"
    };

    eprintln!(
        "  {} Entry:  {}",
        "│".bright_black(),
        args.entry.display().to_string().white()
    );
    eprintln!(
        "  {} Output: {}",
        "│".bright_black(),
        out_file.display().to_string().white()
    );
    eprintln!(
        "  {} Mode:   {}",
        "│".bright_black(),
        mode.cyan()
    );
    eprintln!();

    // Bundling steps
    let spinner = Spinner::dots("Resolving dependencies...");
    tokio::time::sleep(std::time::Duration::from_millis(30)).await;
    spinner.success("Resolved 24 modules");

    let spinner = Spinner::dots("SIMD parsing (AVX2)...");
    tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    spinner.success("Parsed in 0.6ms");

    let spinner = Spinner::dots("Transforming modules...");
    tokio::time::sleep(std::time::Duration::from_millis(30)).await;
    spinner.success("Transformed TypeScript + JSX");

    if args.minify {
        let spinner = Spinner::dots("Minifying output...");
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        spinner.success("Minified (Terser-compatible)");
    }

    let spinner = Spinner::dots("Writing bundle...");
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    spinner.success(&format!("Wrote {}", out_file.display()));

    let duration = start.elapsed().as_micros() as f64 / 1000.0;
    
    eprintln!();
    theme.print_divider();
    eprintln!(
        "  {} Bundled in {} │ 3.8x faster than Bun",
        "✓".green().bold(),
        format!("{:.2}ms", duration).cyan().bold()
    );
    theme.print_divider();
    eprintln!();

    // Bundle stats
    theme.print_section("Bundle stats:");
    eprintln!();
    
    let mut table = table::Table::new(vec!["Metric", "Value"]);
    table.add_row(vec!["Modules", "24"]);
    table.add_row(vec!["Output size", "45.2 KB"]);
    table.add_row(vec!["Gzip size", "12.8 KB"]);
    table.add_row(vec!["Tree-shaken", "8 modules"]);
    table.print();

    Ok(())
}
```

## src/commands/install.rs

```rust
//! Package management commands

use anyhow::Result;
use clap::Args;
use owo_colors::OwoColorize;
use std::time::Instant;

use crate::ui::{progress::Progress, spinner::Spinner, theme::Theme, table};

#[derive(Args)]
pub struct InstallArgs {
    /// Packages to install (empty = install from dx.toml/package.json)
    #[arg(index = 1)]
    pub packages: Vec<String>,

    /// Save as dev dependency
    #[arg(short = 'D', long)]
    pub dev: bool,

    /// Skip post-install scripts
    #[arg(long)]
    pub ignore_scripts: bool,

    /// Prefer offline cached packages
    #[arg(long)]
    pub prefer_offline: bool,
}

#[derive(Args)]
pub struct AddArgs {
    /// Package(s) to add
    #[arg(index = 1, required = true)]
    pub packages: Vec<String>,

    /// Save as dev dependency
    #[arg(short = 'D', long)]
    pub dev: bool,

    /// Exact version (no caret)
    #[arg(short = 'E', long)]
    pub exact: bool,
}

#[derive(Args)]
pub struct RemoveArgs {
    /// Package(s) to remove
    #[arg(index = 1, required = true)]
    pub packages: Vec<String>,
}

pub async fn run_install(args: InstallArgs, theme: &Theme) -> Result<()> {
    let start = Instant::now();

    theme.print_section("dx-js-package-manager");
    eprintln!();

    if args.packages.is_empty() {
        // Install from lockfile
        let spinner = Spinner::dots("Resolving dependencies...");
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        spinner.success("Resolved 142 packages");

        // Show progress bar for installation
        let progress = Progress::new(142, "Installing packages...");
        for i in 0..142 {
            tokio::time::sleep(std::time::Duration::from_micros(200)).await;
            progress.set_position(i + 1);
        }
        progress.finish_success("Installed 142 packages");

        let spinner = Spinner::dots("Linking binaries...");
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        spinner.success("Linked 8 binaries");
    } else {
        // Install specific packages
        for pkg in &args.packages {
            let spinner = Spinner::dots(&format!("Installing {}...", pkg));
            tokio::time::sleep(std::time::Duration::from_millis(30)).await;
            spinner.success(&format!("Installed {}", pkg.cyan()));
        }
    }

    let duration = start.elapsed().as_millis();

    eprintln!();
    theme.print_divider();
    eprintln!(
        "  {} Done in {} │ 17.2x faster than Bun",
        "✓".green().bold(),
        format!("{}ms", duration).cyan().bold()
    );
    theme.print_divider();
    eprintln!();

    Ok(())
}

pub async fn run_add(args: AddArgs, theme: &Theme) -> Result<()> {
    let start = Instant::now();

    theme.print_section("Adding packages");
    eprintln!();

    for pkg in &args.packages {
        let spinner = Spinner::dots(&format!("Resolving {}...", pkg));
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        
        // Simulate version resolution
        let version = "1.0.0";
        spinner.success(&format!("{} → {}", pkg.cyan(), format!("v{}", version).bright_black()));
    }

    let spinner = Spinner::dots("Updating lockfile...");
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    spinner.success("Updated dx-lock.dxl");

    let dep_type = if args.dev { "devDependencies" } else { "dependencies" };
    
    eprintln!();
    theme.print_success(&format!(
        "Added {} package(s) to {}",
        args.packages.len(),
        dep_type.cyan()
    ));
    eprintln!();

    Ok(())
}

pub async fn run_remove(args: RemoveArgs, theme: &Theme) -> Result<()> {
    theme.print_section("Removing packages");
    eprintln!();

    for pkg in &args.packages {
        let spinner = Spinner::dots(&format!("Removing {}...", pkg));
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        spinner.success(&format!("Removed {}", pkg.cyan()));
    }

    let spinner = Spinner::dots("Updating lockfile...");
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    spinner.success("Updated dx-lock.dxl");

    eprintln!();
    theme.print_success(&format!("Removed {} package(s)", args.packages.len()));
    eprintln!();

    Ok(())
}
```

## src/commands/test.rs

```rust
//! Test runner command

use anyhow::Result;
use clap::Args;
use owo_colors::OwoColorize;
use std::path::PathBuf;
use std::time::Instant;

use crate::ui::{spinner::Spinner, theme::Theme};

#[derive(Args)]
pub struct TestArgs {
    /// Test files or directories (defaults to **/*.test.{ts,tsx,js,jsx})
    #[arg(index = 1)]
    pub patterns: Vec<String>,

    /// Run tests matching this pattern
    #[arg(short = 't', long)]
    pub filter: Option<String>,

    /// Update snapshots
    #[arg(short = 'u', long)]
    pub update_snapshots: bool,

    /// Enable watch mode
    #[arg(short, long)]
    pub watch: bool,

    /// Run tests in parallel
    #[arg(long, default_value = "true")]
    pub parallel: bool,

    /// Coverage report
    #[arg(long)]
    pub coverage: bool,

    /// Bail on first failure
    #[arg(long)]
    pub bail: bool,
}

pub async fn run(args: TestArgs, theme: &Theme) -> Result<()> {
    let start = Instant::now();

    theme.print_section("dx-js-test-runner");
    eprintln!();

    // Discover tests
    let spinner = Spinner::dots("Discovering tests...");
    tokio::time::sleep(std::time::Duration::from_millis(30)).await;
    spinner.success("Found 48 test files");

    // Run tests
    let spinner = Spinner::dots("Running tests...");
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    spinner.finish();

    // Simulate test output
    let tests = [
        ("src/utils.test.ts", "PASS", 5),
        ("src/parser.test.ts", "PASS", 12),
        ("src/compiler.test.ts", "PASS", 8),
        ("src/runtime.test.ts", "PASS", 23),
    ];

    for (file, status, count) in tests {
        let status_colored = if status == "PASS" {
            "PASS".green().bold()
        } else {
            "FAIL".red().bold()
        };
        
        eprintln!(
            "  {} {} {} ({})",
            status_colored,
            file.white(),
            format!("• {} tests", count).bright_black(),
            "12ms".bright_black()
        );
    }

    let duration = start.elapsed().as_millis() as u64;
    
    theme.print_test_results(48, 0, 2, duration);

    if args.coverage {
        eprintln!();
        theme.print_section("Coverage Report:");
        eprintln!();
        eprintln!(
            "  {} Statements: {} │ Branches: {} │ Functions: {} │ Lines: {}",
            "│".bright_black(),
            "94.2%".green(),
            "87.5%".yellow(),
            "91.8%".green(),
            "93.1%".green()
        );
        eprintln!();
    }

    eprintln!(
        "  {} 26x faster than Bun's test runner",
        "→".cyan()
    );
    eprintln!();

    Ok(())
}
```

## src/commands/style.rs

```rust
//! dx-style commands

use anyhow::Result;
use clap::{Args, Subcommand};
use owo_colors::OwoColorize;

use crate::ui::{spinner::Spinner, theme::Theme, table};

#[derive(Args)]
pub struct StyleArgs {
    #[command(subcommand)]
    pub command: StyleCommands,
}

#[derive(Subcommand)]
pub enum StyleCommands {
    /// Build binary CSS from source
    Build {
        /// Input CSS file
        #[arg(short, long)]
        input: Option<String>,
        
        /// Output .bcss file
        #[arg(short, long)]
        output: Option<String>,
    },
    
    /// Analyze CSS usage
    Analyze {
        /// Source directory
        #[arg(index = 1)]
        dir: Option<String>,
    },
    
    /// Show style statistics
    Stats,
}

pub async fn run(args: StyleArgs, theme: &Theme) -> Result<()> {
    match args.command {
        StyleCommands::Build { input, output } => {
            theme.print_section("dx-style: Binary CSS Compiler");
            eprintln!();

            let spinner = Spinner::dots("Parsing CSS...");
            tokio::time::sleep(std::time::Duration::from_millis(30)).await;
            spinner.success("Parsed 1,248 rules");

            let spinner = Spinner::dots("Generating integer class IDs...");
            tokio::time::sleep(std::time::Duration::from_millis(20)).await;
            spinner.success("Generated 892 unique IDs");

            let spinner = Spinner::dots("Building binary stylesheet...");
            tokio::time::sleep(std::time::Duration::from_millis(20)).await;
            spinner.success("Output: styles.bcss (2.1 KB)");

            eprintln!();
            theme.print_divider();
            eprintln!(
                "  {} 98% smaller │ 80x faster application",
                "✓".green().bold()
            );
            theme.print_divider();
            eprintln!();
        }
        
        StyleCommands::Analyze { dir } => {
            theme.print_section("dx-style: Usage Analysis");
            eprintln!();

            let spinner = Spinner::dots("Scanning source files...");
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            spinner.success("Scanned 45 files");

            eprintln!();
            
            let mut table = table::Table::new(vec!["Class", "Usage", "Size Impact"]);
            table.add_row(vec!["flex", "124", "+8 bytes"]);
            table.add_row(vec!["p-4", "89", "+12 bytes"]);
            table.add_row(vec!["text-white", "67", "+16 bytes"]);
            table.add_row(vec!["bg-gray-900", "45", "+20 bytes"]);
            table.print();

            eprintln!();
            theme.print_info("Unused classes", "23 (can be purged)");
            theme.print_info("Potential savings", "1.2 KB");
            eprintln!();
        }
        
        StyleCommands::Stats => {
            theme.print_section("dx-style: Statistics");
            eprintln!();

            table::print_kv_list(&[
                ("Total rules", "1,248"),
                ("Unique classes", "892"),
                ("Binary size", "2.1 KB"),
                ("Equivalent CSS", "98.2 KB"),
                ("Compression ratio", "98%"),
                ("Lookup time", "< 1µs"),
            ]);
            eprintln!();
        }
    }

    Ok(())
}
```

## src/commands/forge.rs

```rust
//! dx-forge build orchestration

use anyhow::Result;
use clap::Args;
use owo_colors::OwoColorize;
use std::time::Instant;

use crate::ui::{spinner::Spinner, theme::Theme};

#[derive(Args)]
pub struct ForgeArgs {
    /// Build configuration
    #[arg(short, long, default_value = "release")]
    pub config: String,

    /// Target platform
    #[arg(long, value_parser = ["web", "node", "cloudflare", "vercel", "netlify"])]
    pub target: Option<String>,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Dry run (don't write files)
    #[arg(long)]
    pub dry_run: bool,
}

pub async fn run(args: ForgeArgs, theme: &Theme) -> Result<()> {
    let start = Instant::now();

    theme.print_section("dx-forge: Build Orchestration");
    eprintln!();

    eprintln!(
        "  {} Configuration: {}",
        "│".bright_black(),
        args.config.cyan()
    );
    
    if let Some(ref target) = args.target {
        eprintln!(
            "  {} Target: {}",
            "│".bright_black(),
            target.cyan()
        );
    }
    eprintln!();

    // Build pipeline
    let steps = [
        ("Validating configuration", 20),
        ("Type checking", 80),
        ("Compiling TypeScript", 100),
        ("Building WASM modules", 150),
        ("Bundling assets", 60),
        ("Generating binary CSS", 30),
        ("Optimizing output", 40),
    ];

    for (step, delay) in steps {
        let spinner = Spinner::dots(step);
        tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
        spinner.success(step);
    }

    if let Some(ref target) = args.target {
        let spinner = Spinner::dots(&format!("Adapting for {}...", target));
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        spinner.success(&format!("Configured for {} deployment", target));
    }

    let duration = start.elapsed().as_millis();

    eprintln!();
    theme.print_build_stats(duration as u64, "156 KB", 12);

    if args.dry_run {
        theme.print_warning("Dry run - no files were written");
        eprintln!();
    }

    Ok(())
}
```

## src/commands/info.rs

```rust
//! Display system and project information

use anyhow::Result;
use clap::Args;
use owo_colors::OwoColorize;
use std::env;

use crate::ui::{table, theme::Theme};

#[derive(Args)]
pub struct InfoArgs {
    /// Show only system information
    #[arg(long)]
    pub system: bool,

    /// Show only project information
    #[arg(long)]
    pub project: bool,

    /// Output as JSON
    #[arg(long)]
    pub json: bool,
}

pub async fn run(args: InfoArgs, theme: &Theme) -> Result<()> {
    if !args.project {
        theme.print_section("System Information");
        eprintln!();

        let os = env::consts::OS;
        let arch = env::consts::ARCH;
        let cpus = std::thread::available_parallelism()
            .map(|p| p.get())
            .unwrap_or(1);

        table::print_kv_list(&[
            ("Platform", &format!("{} ({})", os, arch)),
            ("CPU Cores", &cpus.to_string()),
            ("DX Version", env!("CARGO_PKG_VERSION")),
            ("Rust", &rustc_version()),
        ]);
        eprintln!();
    }

    if !args.system {
        theme.print_section("DX Components");
        eprintln!();

        let components = [
            ("dx-js-runtime", "10.59x faster than Bun", "✓"),
            ("dx-js-bundler", "3.8x faster than Bun", "✓"),
            ("dx-js-package-manager", "17.2x faster than Bun", "✓"),
            ("dx-js-test-runner", "26x faster than Bun", "✓"),
            ("dx-style", "98% smaller CSS", "✓"),
            ("dx-serializer", "37% better than TOON", "✓"),
            ("dx-www", "338 bytes runtime", "✓"),
        ];

        for (name, perf, status) in components {
            let status_icon = if status == "✓" {
                "✓".green().bold()
            } else {
                "○".bright_black()
            };
            
            eprintln!(
                "  {} {} {} {}",
                status_icon,
                name.white(),
                "─".bright_black(),
                perf.cyan()
            );
        }
        eprintln!();
    }

    if std::path::Path::new("dx.toml").exists() && !args.system {
        theme.print_section("Project");
        eprintln!();
        
        // Would parse dx.toml in real implementation
        table::print_kv_list(&[
            ("Name", "my-dx-app"),
            ("Version", "0.1.0"),
            ("Entry", "src/main.tsx"),
            ("Target", "browser"),
        ]);
        eprintln!();
    }

    Ok(())
}

fn rustc_version() -> String {
    std::process::Command::new("rustc")
        .arg("--version")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}
```

## src/commands/clean.rs

```rust
//! Clean build artifacts and caches

use anyhow::Result;
use clap::Args;
use owo_colors::OwoColorize;
use std::path::PathBuf;

use crate::ui::{spinner::Spinner, theme::Theme};

#[derive(Args)]
pub struct CleanArgs {
    /// Clean only build output (dist/)
    #[arg(long)]
    pub dist: bool,

    /// Clean only cache (.dx/)
    #[arg(long)]
    pub cache: bool,

    /// Clean only dependencies (dx_modules/)
    #[arg(long)]
    pub modules: bool,

    /// Clean everything
    #[arg(long)]
    pub all: bool,

    /// Don't ask for confirmation
    #[arg(short = 'y', long)]
    pub yes: bool,
}

pub async fn run(args: CleanArgs, theme: &Theme) -> Result<()> {
    theme.print_section("Cleaning project");
    eprintln!();

    let mut cleaned = Vec::new();
    let mut total_size = 0u64;

    let clean_all = args.all || (!args.dist && !args.cache && !args.modules);

    if args.dist || clean_all {
        if PathBuf::from("dist").exists() {
            let spinner = Spinner::dots("Cleaning dist/...");
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            // In real impl: fs::remove_dir_all("dist")?;
            spinner.success("Cleaned dist/");
            cleaned.push("dist/");
            total_size += 1024 * 1024; // Simulated
        }
    }

    if args.cache || clean_all {
        if PathBuf::from(".dx").exists() {
            let spinner = Spinner::dots("Cleaning .dx/ cache...");
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            spinner.success("Cleaned .dx/");
            cleaned.push(".dx/");
            total_size += 512 * 1024;
        }
    }

    if args.modules || clean_all {
        if PathBuf::from("dx_modules").exists() {
            let spinner = Spinner::dots("Cleaning dx_modules/...");
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            spinner.success("Cleaned dx_modules/");
            cleaned.push("dx_modules/");
            total_size += 50 * 1024 * 1024;
        }
    }

    if cleaned.is_empty() {
        theme.print_info("Nothing to clean", "Project is already clean");
    } else {
        eprintln!();
        theme.print_success(&format!(
            "Cleaned {} director{}, freed {}",
            cleaned.len(),
            if cleaned.len() == 1 { "y" } else { "ies" },
            human_bytes::human_bytes(total_size as f64)
        ));
    }

    eprintln!();

    Ok(())
}
```

## src/commands/upgrade.rs

```rust
//! Self-upgrade command (hidden)

use anyhow::Result;
use clap::Args;

use crate::ui::theme::Theme;

#[derive(Args)]
pub struct UpgradeArgs {
    /// Upgrade to a specific version
    #[arg(long)]
    pub version: Option<String>,

    /// Check for updates without installing
    #[arg(long)]
    pub check: bool,
}

pub async fn run(args: UpgradeArgs, theme: &Theme) -> Result<()> {
    theme.print_section("Checking for updates...");
    eprintln!();

    // This would check GitHub releases in real implementation
    theme.print_info("Current version", env!("CARGO_PKG_VERSION"));
    theme.print_info("Latest version", env!("CARGO_PKG_VERSION"));
    eprintln!();

    theme.print_success("DX is up to date!");
    eprintln!();

    Ok(())
}
```

## src/config/mod.rs

```rust
//! Configuration module

pub mod dx_config;

pub use dx_config::DxConfig;
```

## src/config/dx_config.rs

```rust
//! DX configuration file parsing

use serde::{Deserialize, Serialize};
use std::path::Path;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DxConfig {
    pub project: ProjectConfig,
    
    #[serde(default)]
    pub build: BuildConfig,
    
    #[serde(default)]
    pub dev: DevConfig,
    
    #[serde(default)]
    pub runtime: RuntimeConfig,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProjectConfig {
    pub name: String,
    
    #[serde(default = "default_version")]
    pub version: String,
    
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildConfig {
    #[serde(default = "default_target")]
    pub target: String,
    
    #[serde(default = "default_true")]
    pub minify: bool,
    
    #[serde(default)]
    pub sourcemap: bool,
    
    #[serde(default = "default_out_dir")]
    pub out_dir: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DevConfig {
    #[serde(default = "default_port")]
    pub port: u16,
    
    #[serde(default)]
    pub open: bool,
    
    #[serde(default)]
    pub https: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RuntimeConfig {
    #[serde(default = "default_jsx")]
    pub jsx: String,
    
    #[serde(default = "default_true")]
    pub typescript: bool,
}

impl DxConfig {
    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: DxConfig = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn load_or_default() -> Self {
        let config_path = Path::new("dx.toml");
        if config_path.exists() {
            Self::load(config_path).unwrap_or_default()
        } else {
            Self::default()
        }
    }
}

// Default value functions
fn default_version() -> String { "0.1.0".to_string() }
fn default_target() -> String { "browser".to_string() }
fn default_out_dir() -> String { "dist".to_string() }
fn default_port() -> u16 { 3000 }
fn default_jsx() -> String { "dx".to_string() }
fn default_true() -> bool { true }

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            target: default_target(),
            minify: true,
            sourcemap: false,
            out_dir: default_out_dir(),
        }
    }
}

impl Default for DevConfig {
    fn default() -> Self {
        Self {
            port: default_port(),
            open: false,
            https: false,
        }
    }
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            jsx: default_jsx(),
            typescript: true,
        }
    }
}
```

## src/utils/mod.rs

```rust
//! Utility modules

pub mod paths;
pub mod system;
```

## src/utils/paths.rs

```rust
//! Path utilities

use std::path::{Path, PathBuf};

/// Find the project root by looking for dx.toml
pub fn find_project_root() -> Option<PathBuf> {
    let mut current = std::env::current_dir().ok()?;
    
    loop {
        if current.join("dx.toml").exists() {
            return Some(current);
        }
        
        if current.join("package.json").exists() {
            return Some(current);
        }
        
        if !current.pop() {
            return None;
        }
    }
}

/// Get the DX home directory (~/.dx)
pub fn dx_home() -> PathBuf {
    home::home_dir()
        .map(|h| h.join(".dx"))
        .unwrap_or_else(|| PathBuf::from(".dx"))
}

/// Get the cache directory
pub fn cache_dir() -> PathBuf {
    dx_home().join("cache")
}

/// Get the global bin directory
pub fn bin_dir() -> PathBuf {
    dx_home().join("bin")
}
```

## src/utils/system.rs

```rust
//! System utilities

use std::process::Command;

/// Check if a command exists on the system
pub fn command_exists(cmd: &str) -> bool {
    which::which(cmd).is_ok()
}

/// Get available memory in bytes
pub fn available_memory() -> Option<u64> {
    #[cfg(target_os = "linux")]
    {
        std::fs::read_to_string("/proc/meminfo")
            .ok()
            .and_then(|content| {
                content
                    .lines()
                    .find(|line| line.starts_with("MemAvailable:"))
                    .and_then(|line| {
                        line.split_whitespace()
                            .nth(1)
                            .and_then(|s| s.parse::<u64>().ok())
                            .map(|kb| kb * 1024)
                    })
            })
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("sysctl")
            .args(["-n", "hw.memsize"])
            .output()
            .ok()
            .and_then(|output| {
                String::from_utf8(output.stdout)
                    .ok()
                    .and_then(|s| s.trim().parse().ok())
            })
    }

    #[cfg(target_os = "windows")]
    {
        None // Would need Windows API
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        None
    }
}
```

## src/templates/counter.tsx (Template file embedded in binary)

```rust
// This would be in a templates directory and included via include_str!
// For now, create a simple module with the template content

// src/templates.rs
pub const COUNTER_TEMPLATE: &str = r#"import { useState } from 'dx';

export default function App() {
  const [count, setCount] = useState(0);

  return (
    <div class="min-h-screen bg-gray-900 text-white flex items-center justify-center">
      <div class="text-center">
        <h1 class="text-4xl font-bold mb-8">DX Counter</h1>
        <p class="text-6xl font-mono mb-8">{count}</p>
        <div class="space-x-4">
          <button
            onClick={() => setCount(count - 1)}
            class="px-6 py-3 bg-red-600 rounded-lg hover:bg-red-700"
          >
            -
          </button>
          <button
            onClick={() => setCount(count + 1)}
            class="px-6 py-3 bg-green-600 rounded-lg hover:bg-green-700"
          >
            +
          </button>
        </div>
      </div>
    </div>
  );
}
"#;

pub const MINIMAL_TEMPLATE: &str = r#"import { mount } from 'dx';

function App() {
  return <h1>Hello, DX!</h1>;
}

mount(<App />, document.getElementById('root'));
"#;
```

Then update the init.rs to use these:

```rust
// At the top of src/commands/init.rs, add:
mod templates {
    pub const COUNTER: &str = include_str!("../templates/counter.tsx");
    pub const MINIMAL: &str = include_str!("../templates/minimal.tsx");
}

// Or embed directly if templates are small
```

---

## CLI Output Preview

Here's what the CLI output looks like in practice:

```
  ▲  DX v0.1.0

  │ Building for production...

  ✓ Found 12 source files
  ✓ Transformed 12 files
  ✓ Bundled in 10.05ms (3.8x faster than Bun)
  ✓ Generated B-CSS (98% smaller)
  ✓ Compiled WASM binary
  ✓ Minified all assets

  │ Output files:

  ├── dist/index.html (1.2 KB)
  ├── dist/main.wasm (23.3 KB)
  ├── dist/main.js (0.5 KB)
  ├── dist/styles.bcss (2.1 KB)
  └── dist/assets/ (12 files)

  ────────────────────────────────────────────────
  ✓ Built in 156ms │ 27.1 KB │ 5 files
  ────────────────────────────────────────────────

  → Run `dx preview --dir dist` to get started
```

This CLI provides:

1. **Clean Visual Design** - Vercel-like aesthetic with subtle colors
2. **Consistent Branding** - The ▲ logo and cyan/magenta color scheme
3. **Progress Indicators** - Modern spinners with smooth animations
4. **Informative Output** - Shows performance comparisons to Bun
5. **Helpful Hints** - Suggests next commands
6. **Structured Information** - Tables and key-value lists
7. **Error Handling** - Clear error messages with visual distinction
8. **Fast UX** - Minimal dependencies, quick startup
```
