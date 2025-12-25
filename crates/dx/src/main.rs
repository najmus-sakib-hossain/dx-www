//! DX CLI - The Binary-First Development Experience
//!
//! A modern, high-performance CLI for the DX development platform.
//! Provides unified control over all dx-* tools with a clean, Vercel-like UX.
//!
//! ## Hardening Features
//!
//! This CLI includes comprehensive hardening for production use:
//! - Crash reporting with diagnostic information (Requirement 1.5)
//! - Resource management with cleanup on exit (Requirement 9.7)
//! - Graceful signal handling with child process termination (Requirement 1.6, 9.5)
//! - Verbose/quiet/debug logging modes (Requirement 10.1, 10.2, 10.6)
//! - Offline mode detection (Requirement 3.7, 11.4)

// Allow dead_code for API completeness
#![allow(dead_code)]

use std::sync::{Arc, OnceLock};

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
use ui::logger::{Logger, StructuredLogger};
use utils::crash::CrashReporter;
use utils::network::NetworkClient;
use utils::resource::ResourceManager;
use utils::signal;

/// Global resource manager for cleanup on exit (thread-safe)
static GLOBAL_RESOURCE_MANAGER: OnceLock<Arc<ResourceManager>> = OnceLock::new();

/// Get the global resource manager
#[allow(dead_code)]
fn get_resource_manager() -> Option<Arc<ResourceManager>> {
    GLOBAL_RESOURCE_MANAGER.get().cloned()
}

#[tokio::main]
async fn main() {
    // Initialize the resource manager first (Requirement 9.7)
    let resource_manager = Arc::new(ResourceManager::default());
    let _ = GLOBAL_RESOURCE_MANAGER.set(resource_manager.clone());

    // Install crash reporter with resource manager integration (Requirement 1.5)
    CrashReporter::install(Some(resource_manager.clone()));

    // Initialize the base logger
    Logger::init();

    // Parse CLI arguments
    let cli = Cli::parse();

    // Initialize structured logger based on CLI flags (Requirement 10.1, 10.2, 10.6)
    let verbose = cli.verbose;
    let quiet = cli.quiet;
    let debug = std::env::var("DX_DEBUG").map(|v| v == "1").unwrap_or(false);
    
    StructuredLogger::with_settings(verbose, quiet, debug)
        .with_file_logging()
        .install();

    // Setup enhanced signal handler with resource cleanup (Requirement 1.6, 9.5)
    let rm_for_signal = resource_manager.clone();
    if let Err(e) = signal::setup_signal_handlers(move || {
        // Clean up resources on Ctrl+C
        rm_for_signal.cleanup();
        ui::theme::Theme::new().print_cancelled();
    }) {
        ui::logger::warn(&format!("Failed to setup signal handlers: {}", e));
    }

    // Check for offline mode and skip update checks if offline (Requirement 3.2, 3.7, 11.4)
    if NetworkClient::is_offline() {
        if verbose {
            ui::logger::info("Running in offline mode - update checks skipped");
        }
    }

    // Run the CLI
    let result = cli.run().await;

    // Clean up resources before exit
    resource_manager.cleanup();

    // Handle result
    if let Err(err) = result {
        ui::logger::error(&format!("{err:#}"));
        std::process::exit(1);
    }
}
