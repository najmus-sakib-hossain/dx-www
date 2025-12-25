//! DX CLI - The Binary-First Development Experience
//!
//! A modern, high-performance CLI for the DX development platform.
//! Provides unified control over all dx-* tools with a clean, Vercel-like UX.

// Allow dead_code for API completeness
#![allow(dead_code)]

use clap::Parser;

mod cli;
mod commands;
mod config;
pub mod io;
pub mod prompts;
mod templates;
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
