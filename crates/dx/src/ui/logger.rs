//! Logging utilities

use owo_colors::OwoColorize;
use tracing_subscriber::{fmt, EnvFilter};

/// Logger initialization helper
pub struct Logger;

impl Logger {
    /// Initialize the logger
    pub fn init() {
        let filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn"));

        let _ = fmt()
            .with_env_filter(filter)
            .with_target(false)
            .without_time()
            .try_init();
    }
}

/// Log an info message
#[allow(dead_code)]
pub fn info(message: &str) {
    eprintln!("  {} {}", "→".cyan(), message.white());
}

/// Log a success message
#[allow(dead_code)]
pub fn success(message: &str) {
    eprintln!("  {} {}", "✓".green().bold(), message.white());
}

/// Log a warning message
#[allow(dead_code)]
pub fn warn(message: &str) {
    eprintln!("  {} {}", "⚠".yellow().bold(), message.yellow());
}

/// Log an error message
pub fn error(message: &str) {
    eprintln!();
    eprintln!("  {} {}", "Error:".red().bold(), message.red());
    eprintln!();
}

/// Log a debug message (only in verbose mode)
#[allow(dead_code)]
pub fn debug(message: &str) {
    eprintln!("  {} {}", "debug".bright_black(), message.bright_black());
}

/// Log a step in a process
pub fn step(number: usize, message: &str) {
    eprintln!("  {} {}", format!("{number}.").cyan().bold(), message.white());
}

/// Log a list item
#[allow(dead_code)]
pub fn list_item(message: &str) {
    eprintln!("  {} {}", "•".bright_black(), message.white());
}

/// Log a code block / command
#[allow(dead_code)]
pub fn code(command: &str) {
    eprintln!();
    eprintln!("  {}", format!("$ {command}").cyan());
    eprintln!();
}
