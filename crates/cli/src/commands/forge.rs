//! Forge Daemon Control Commands
//!
//! Commands for managing the Forge daemon lifecycle.

use anyhow::Result;
use clap::Subcommand;

use crate::daemon_client::DaemonClient;

/// Forge daemon control commands
#[derive(Subcommand, Debug)]
pub enum ForgeCommands {
    /// Start the Forge daemon
    Start {
        /// Run daemon in background (detached)
        #[arg(short, long, default_value = "true")]
        detach: bool,

        /// Port for LSP/WebSocket connections
        #[arg(short, long, default_value = "9876")]
        port: u16,

        /// Enable verbose logging
        #[arg(short, long)]
        verbose: bool,
    },

    /// Stop the running Forge daemon
    Stop {
        /// Force stop without graceful shutdown
        #[arg(short, long)]
        force: bool,
    },

    /// Show Forge daemon status
    Status,

    /// Restart the Forge daemon
    Restart {
        /// Port for LSP/WebSocket connections
        #[arg(short, long, default_value = "9876")]
        port: u16,
    },

    /// Show daemon logs
    Logs {
        /// Follow log output (like tail -f)
        #[arg(short, long)]
        follow: bool,

        /// Number of lines to show
        #[arg(short = 'n', long, default_value = "50")]
        lines: usize,
    },
}

impl ForgeCommands {
    pub async fn execute(self) -> Result<()> {
        match self {
            ForgeCommands::Start { detach, port, verbose } => {
                start_daemon(detach, port, verbose).await
            }
            ForgeCommands::Stop { force } => stop_daemon(force).await,
            ForgeCommands::Status => show_status().await,
            ForgeCommands::Restart { port } => restart_daemon(port).await,
            ForgeCommands::Logs { follow, lines } => show_logs(follow, lines).await,
        }
    }
}

/// Start the Forge daemon
async fn start_daemon(detach: bool, port: u16, verbose: bool) -> Result<()> {
    use console::style;

    // Check if daemon is already running
    if DaemonClient::is_daemon_running() {
        println!(
            "{} Forge daemon is already running",
            style("⚠").yellow()
        );
        println!("  Use {} to check status", style("dx forge status").cyan());
        return Ok(());
    }

    println!(
        "{} Starting Forge daemon on port {}...",
        style("⚡").cyan(),
        port
    );

    if detach {
        // Spawn daemon as background process
        let exe = std::env::current_exe()?;
        let mut cmd = std::process::Command::new(exe);
        cmd.arg("forge")
            .arg("start")
            .arg("--detach=false")
            .arg("--port")
            .arg(port.to_string());

        if verbose {
            cmd.arg("--verbose");
        }

        // Detach from terminal
        #[cfg(unix)]
        {
            use std::os::unix::process::CommandExt;
            cmd.process_group(0);
        }

        cmd.stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()?;

        // Wait a moment for daemon to start
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        if DaemonClient::is_daemon_running() {
            println!(
                "{} Forge daemon started successfully",
                style("✓").green()
            );
            println!("  LSP port: {}", style(port).cyan());
        } else {
            println!(
                "{} Failed to start daemon. Check logs with {}",
                style("✗").red(),
                style("dx forge logs").cyan()
            );
        }
    } else {
        // Run daemon in foreground
        run_daemon_foreground(port, verbose).await?;
    }

    Ok(())
}

/// Run daemon in foreground (non-detached mode)
async fn run_daemon_foreground(port: u16, verbose: bool) -> Result<()> {
    use forge::{DaemonConfig, ForgeDaemon};

    let config = DaemonConfig {
        project_root: std::env::current_dir()?,
        enable_lsp_watcher: true,
        enable_fs_watcher: true,
        debounce_ms: 100,
        worker_count: num_cpus::get(),
        enable_r2_sync: false,
        r2_bucket: None,
        auto_run_tools: true,
        watch_patterns: vec![
            "**/*.ts".to_string(),
            "**/*.tsx".to_string(),
            "**/*.js".to_string(),
            "**/*.jsx".to_string(),
            "**/*.css".to_string(),
            "**/*.rs".to_string(),
            "**/*.dx".to_string(),
            "**/dx".to_string(),
        ],
        ignore_patterns: vec![
            "**/node_modules/**".to_string(),
            "**/.dx/**".to_string(),
            "**/target/**".to_string(),
            "**/dist/**".to_string(),
            "**/.git/**".to_string(),
        ],
        max_concurrent_tools: 4,
        tool_timeout_ms: 60_000,
    };

    if verbose {
        println!("Config: {:?}", config);
    }

    let daemon = ForgeDaemon::new(config)?;
    daemon.start().await?;

    Ok(())
}

/// Stop the Forge daemon
async fn stop_daemon(force: bool) -> Result<()> {
    use console::style;

    if !DaemonClient::is_daemon_running() {
        println!(
            "{} Forge daemon is not running",
            style("ℹ").blue()
        );
        return Ok(());
    }

    println!(
        "{} Stopping Forge daemon{}...",
        style("⏹").yellow(),
        if force { " (force)" } else { "" }
    );

    match DaemonClient::connect() {
        Ok(mut client) => {
            client.send_shutdown(force)?;
            println!(
                "{} Forge daemon stopped",
                style("✓").green()
            );
        }
        Err(e) => {
            if force {
                // Force kill via PID file
                if let Some(pid) = DaemonClient::read_pid_file() {
                    #[cfg(unix)]
                    {
                        use std::process::Command;
                        Command::new("kill")
                            .arg("-9")
                            .arg(pid.to_string())
                            .output()?;
                    }
                    #[cfg(windows)]
                    {
                        use std::process::Command;
                        Command::new("taskkill")
                            .arg("/F")
                            .arg("/PID")
                            .arg(pid.to_string())
                            .output()?;
                    }
                    println!(
                        "{} Forge daemon force killed",
                        style("✓").green()
                    );
                } else {
                    println!(
                        "{} Could not find daemon PID: {}",
                        style("✗").red(),
                        e
                    );
                }
            } else {
                println!(
                    "{} Could not connect to daemon: {}",
                    style("✗").red(),
                    e
                );
                println!(
                    "  Try {} to force stop",
                    style("dx forge stop --force").cyan()
                );
            }
        }
    }

    Ok(())
}

/// Show daemon status
async fn show_status() -> Result<()> {
    use console::style;

    if !DaemonClient::is_daemon_running() {
        println!(
            "{} Forge daemon is {} running",
            style("●").red(),
            style("not").red()
        );
        println!(
            "  Start with: {}",
            style("dx forge start").cyan()
        );
        return Ok(());
    }

    match DaemonClient::connect() {
        Ok(mut client) => {
            let status = client.get_status()?;
            
            println!(
                "{} Forge daemon is {}",
                style("●").green(),
                style("running").green()
            );
            println!();
            println!("  Uptime:         {}", format_duration(status.uptime_seconds));
            println!("  Files changed:  {}", status.files_changed);
            println!("  Tools executed: {}", status.tools_executed);
            println!("  Cache hits:     {}", status.cache_hits);
            println!("  Errors:         {}", status.errors);
            println!();
            println!("  LSP events:     {}", status.lsp_events);
            println!("  FS events:      {}", status.fs_events);
        }
        Err(e) => {
            println!(
                "{} Daemon appears running but cannot connect: {}",
                style("⚠").yellow(),
                e
            );
        }
    }

    Ok(())
}

/// Restart the daemon
async fn restart_daemon(port: u16) -> Result<()> {
    use console::style;

    println!(
        "{} Restarting Forge daemon...",
        style("🔄").cyan()
    );

    // Stop if running
    if DaemonClient::is_daemon_running() {
        stop_daemon(false).await?;
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    // Start fresh
    start_daemon(true, port, false).await?;

    Ok(())
}

/// Show daemon logs
async fn show_logs(follow: bool, lines: usize) -> Result<()> {
    use console::style;

    let log_path = DaemonClient::log_path();

    if !log_path.exists() {
        println!(
            "{} No log file found at {}",
            style("ℹ").blue(),
            log_path.display()
        );
        return Ok(());
    }

    if follow {
        println!(
            "{} Following logs (Ctrl+C to stop)...",
            style("📜").cyan()
        );
        // Use tail -f equivalent
        #[cfg(unix)]
        {
            use std::process::Command;
            Command::new("tail")
                .arg("-f")
                .arg("-n")
                .arg(lines.to_string())
                .arg(&log_path)
                .status()?;
        }
        #[cfg(windows)]
        {
            use std::process::Command;
            Command::new("powershell")
                .arg("-Command")
                .arg(format!(
                    "Get-Content -Path '{}' -Tail {} -Wait",
                    log_path.display(),
                    lines
                ))
                .status()?;
        }
    } else {
        let content = std::fs::read_to_string(&log_path)?;
        let log_lines: Vec<&str> = content.lines().collect();
        let start = log_lines.len().saturating_sub(lines);
        
        for line in &log_lines[start..] {
            println!("{}", line);
        }
    }

    Ok(())
}

/// Format duration in human-readable form
fn format_duration(seconds: u64) -> String {
    if seconds < 60 {
        format!("{}s", seconds)
    } else if seconds < 3600 {
        format!("{}m {}s", seconds / 60, seconds % 60)
    } else {
        format!("{}h {}m", seconds / 3600, (seconds % 3600) / 60)
    }
}
