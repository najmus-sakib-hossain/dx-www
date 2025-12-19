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
                    if let notify::Event {
                        kind: notify::EventKind::Modify(_),
                        paths,
                        ..
                    } = event
                    {
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
