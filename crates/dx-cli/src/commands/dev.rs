/**
 * `dx dev` - Development server with hot reload
 * 
 * Orchestrates:
 * 1. File watching (notify) → triggers rebuild
 * 2. dx-compiler → compiles TSX to binary
 * 3. dx-server → serves the binary over HTTP
 * 4. WebSocket → pushes updates to client
 */

use anyhow::{Context, Result};
use console::style;
use tokio::sync::broadcast;
use tracing::{info, warn};

use crate::config::ProjectConfig;

pub async fn execute(port: u16, host: &str, open_browser: bool) -> Result<()> {
    println!("{}", style("🚀 Starting development server...").bold());
    println!();

    // Load project configuration
    let config = ProjectConfig::load(".")
        .with_context(|| "Failed to load dx config. Run 'dx new' to create a project.")?;

    info!("Project: {} v{}", config.name(), config.version());

    // Create shutdown channel
    let (shutdown_tx, _) = broadcast::channel::<()>(1);

    // Initial build
    println!("{}", style("📦 Building application...").bold());
    let build_result = compile_project(&config).await?;
    println!("{} Initial build complete ({:.2}ms)", 
        style("✓").green(), 
        build_result.duration_ms
    );
    println!("  {} Runtime: {}", style("→").cyan(), build_result.runtime);
    println!("  {} Size: {} bytes", style("→").cyan(), build_result.size);
    println!();

    // Start file watcher
    let config_clone = config.clone();
    let shutdown_rx = shutdown_tx.subscribe();
    let watcher_handle = tokio::spawn(async move {
        watch_and_rebuild(config_clone, shutdown_rx).await
    });

    // Start HTTP server
    // Convert localhost to 127.0.0.1 for proper parsing
    let bind_host = if host == "localhost" { "127.0.0.1" } else { host };
    let addr = format!("{}:{}", bind_host, port);
    println!("{}", style(format!("🌐 Server running at http://{}:{}", host, port)).green().bold());
    println!("{}", style("   Press Ctrl+C to stop").dim());
    println!();

    let addr_clone = addr.clone();
    let server_handle = tokio::spawn(async move {
        if let Err(e) = start_server(&addr_clone).await {
            warn!("Server error: {}", e);
        }
    });

    // Open browser if requested
    if open_browser {
        let url = format!("http://{}", addr);
        if let Err(e) = open::that(&url) {
            warn!("Failed to open browser: {}", e);
        }
    }

    // Wait for Ctrl+C
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            println!("\n{}", style("⏹️  Shutting down...").yellow());
            let _ = shutdown_tx.send(());
        }
        _ = server_handle => {
            warn!("Server stopped unexpectedly");
        }
        _ = watcher_handle => {
            warn!("File watcher stopped unexpectedly");
        }
    }

    println!("{}", style("✨ Shutdown complete").green());
    Ok(())
}

/// Compile the project using dx-compiler
async fn compile_project(config: &ProjectConfig) -> Result<BuildResult> {
    use std::path::Path;

    // Determine entry point
    let entry = Path::new("src/App.tsx");
    if !entry.exists() {
        anyhow::bail!("Entry file not found: {}", entry.display());
    }

    // Determine output directory (.dx-cache for dev builds)
    let output = Path::new(".dx-cache");

    // Compile using dx-compiler
    let compile_result = dx_compiler::compile_tsx(entry, output, false)?;

    // Determine runtime string
    let runtime = format!(
        "auto ({})",
        match compile_result.runtime_variant {
            dx_compiler::analyzer::RuntimeVariant::Micro => "micro",
            dx_compiler::analyzer::RuntimeVariant::Macro => "macro",
        }
    );

    Ok(BuildResult {
        runtime,
        size: compile_result.total_size as usize,
        duration_ms: compile_result.compile_time_ms as f64,
    })
}

/// Watch files and trigger rebuilds
async fn watch_and_rebuild(
    config: ProjectConfig,
    mut shutdown: broadcast::Receiver<()>,
) -> Result<()> {
    use notify::RecursiveMode;
    use notify_debouncer_mini::{new_debouncer, DebouncedEventKind};
    use std::time::Duration;

    let (tx, mut rx) = tokio::sync::mpsc::channel(100);

    // Create file watcher
    let mut debouncer = new_debouncer(
        Duration::from_millis(500),
        move |res: Result<Vec<notify_debouncer_mini::DebouncedEvent>, _>| {
            if let Ok(events) = res {
                for event in events {
                    if let DebouncedEventKind::Any = event.kind {
                        // Check if it's a .tsx file
                        if event.path.extension().and_then(|s| s.to_str()) == Some("tsx") {
                            let _ = tx.blocking_send(event.path.clone());
                        }
                    }
                }
            }
        },
    )?;

    // Watch src/ directory
    debouncer
        .watcher()
        .watch(std::path::Path::new("src"), RecursiveMode::Recursive)?;

    info!("👀 Watching src/ for changes...");

    loop {
        tokio::select! {
            Some(path) = rx.recv() => {
                println!("\n{} File changed: {}", 
                    style("🔄").cyan(), 
                    path.display()
                );
                println!("{}", style("📦 Rebuilding...").bold());
                
                match compile_project(&config).await {
                    Ok(result) => {
                        println!("{} Build complete ({:.2}ms)", 
                            style("✓").green(), 
                            result.duration_ms
                        );
                        println!("  {} Size: {} bytes\n", 
                            style("→").cyan(), 
                            result.size
                        );
                    }
                    Err(e) => {
                        println!("{} Build failed: {}\n", 
                            style("✗").red(), 
                            e
                        );
                    }
                }
            }
            _ = shutdown.recv() => {
                break;
            }
        }
    }

    Ok(())
}

/// Start the HTTP server using dx-server
async fn start_server(addr: &str) -> Result<()> {
    use std::net::SocketAddr;
    use std::path::Path;

    info!("HTTP server started at {}", addr);

    // Create server state and load artifacts
    let state = dx_server::ServerState::new();
    
    // Set project directory (current directory)
    let current_dir = std::env::current_dir()?;
    state.set_project_dir(current_dir);
    
    // Load compiled artifacts from .dx-cache
    let cache_path = Path::new(".dx-cache");
    if cache_path.exists() {
        if let Err(e) = state.load_artifacts(cache_path) {
            warn!("Failed to load artifacts: {}", e);
        }
    }

    // Parse address
    let socket_addr: SocketAddr = addr.parse()
        .with_context(|| format!("Invalid address: {}", addr))?;

    // Start dx-server
    dx_server::serve(socket_addr, state).await
        .map_err(|e| anyhow::anyhow!("Server error: {}", e))?;

    Ok(())
}

#[derive(Debug)]
struct BuildResult {
    runtime: String,
    size: usize,
    duration_ms: f64,
}
