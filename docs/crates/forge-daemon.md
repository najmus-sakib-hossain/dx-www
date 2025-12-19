# DX Forge Daemon - Binary Dawn Edition

## Overview

The **Forge Daemon** is a persistent background service that orchestrates all DX tools with:
- **Dual Watcher Architecture** (LSP + FileSystem)
- **Background Worker Pool** for async tasks
- **Tool Orchestration** with dependency resolution
- **R2 Cloud Sync** for shared team cache
- **VS Code Extension Integration** via LSP

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                     FORGE DAEMON (Binary Dawn)                   │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────────────┐     ┌─────────────────────────────┐   │
│  │   LSP Watcher       │     │   FileSystem Watcher        │   │
│  │   (Primary)         │     │   (Fallback)                │   │
│  │   - VS Code events  │     │   - notify-debouncer        │   │
│  │   - Port 9527       │     │   - Recursive watch         │   │
│  └─────────┬───────────┘     └──────────────┬──────────────┘   │
│            │                                │                   │
│            └──────────────┬─────────────────┘                   │
│                           ▼                                     │
│  ┌────────────────────────────────────────────────────────────┐│
│  │              UNIFIED CHANGE STREAM                          ││
│  │  (Deduplication + Pattern Detection + Priority Queue)       ││
│  └───────────────────────────┬────────────────────────────────┘│
│                              ▼                                  │
│  ┌────────────────────────────────────────────────────────────┐│
│  │              TOOL ORCHESTRATOR                              ││
│  │  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐          ││
│  │  │ bundler │ │ style   │ │ test    │ │ www     │ ...      ││
│  │  └─────────┘ └─────────┘ └─────────┘ └─────────┘          ││
│  └───────────────────────────┬────────────────────────────────┘│
│                              ▼                                  │
│  ┌────────────────────────────────────────────────────────────┐│
│  │              BACKGROUND WORKER POOL                         ││
│  │  - Cache warming    - R2 sync     - Pattern analysis       ││
│  │  - Package prefetch - Cleanup     - Project indexing       ││
│  └────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────┘
```

## Quick Start

### Start the Daemon

```rust
use forge::{ForgeDaemon, DaemonConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = DaemonConfig {
        project_root: ".".into(),
        enable_lsp_watcher: true,
        enable_fs_watcher: true,
        auto_run_tools: true,
        ..Default::default()
    };
    
    let daemon = ForgeDaemon::new(config)?;
    daemon.start().await?;  // Blocks until Ctrl+C
    
    Ok(())
}
```

### Use the Builder

```rust
use forge::daemon::core::ForgeDaemonBuilder;

let daemon = ForgeDaemonBuilder::new("./my-project")
    .enable_lsp(true)
    .enable_fs_watcher(true)
    .workers(4)
    .enable_r2("dx-team-cache")
    .auto_run_tools(true)
    .build()?;

daemon.start().await?;
```

## Dual Watcher Architecture

### LSP Watcher (Primary)

The LSP watcher receives file changes from code editors **before they hit disk**:

- Faster response times (no disk I/O)
- Semantic understanding of changes
- Content available immediately
- Pattern detection on save

VS Code extension connects to port **9527** and sends:
- `textDocument/didChange` - Content changes
- `textDocument/didSave` - File saved

### FileSystem Watcher (Fallback)

Falls back to `notify-debouncer` for:

- Editors without LSP integration
- External file changes (git, scripts)
- Build tool outputs
- Recursive directory watching

## Background Worker Pool

Async task processing for non-blocking operations:

```rust
use forge::{WorkerPool, WorkerTask, TaskPriority};

let pool = WorkerPool::new(4);

// Queue tasks with priority
pool.queue_with_priority(
    WorkerTask::WarmCache { tool: "bundler".to_string() },
    TaskPriority::High,
).await;

// Queue multiple tasks
pool.queue_many(vec![
    WorkerTask::SyncToR2 { tool: "style".to_string(), paths: vec![] },
    WorkerTask::AnalyzePatterns { paths: vec!["src".to_string()] },
]).await;

// Wait for completion
pool.wait_for_completion().await;
```

### Task Types

| Task | Description |
|------|-------------|
| `WarmCache` | Pre-load cache for tool |
| `SyncToR2` | Upload to R2 cloud |
| `PullFromR2` | Download from R2 cloud |
| `AnalyzePatterns` | Detect DX patterns in code |
| `PrefetchPackage` | Pre-download npm package |
| `CleanCache` | Remove old cache entries |
| `BuildCache` | Cache tool outputs |
| `IndexProject` | Index project files |

### Priority Levels

```rust
pub enum TaskPriority {
    Critical = 0,   // Run immediately
    High = 1,       // Run soon
    Normal = 2,     // Default
    Low = 3,        // Run when idle
    Background = 4, // Spare cycles
}
```

## LSP Bridge

VS Code extension integration:

```rust
use forge::{LspBridge, LspNotification};

let bridge = LspBridge::default();
bridge.start().await?;

// Broadcast to all clients
bridge.notify_tool_started("bundler").await;
bridge.notify_tool_completed("bundler", 100, true).await;

// Custom notifications
bridge.broadcast(LspNotification::new(
    "dx/patternDetected",
    serde_json::json!({ "patterns": [...] }),
)).await;
```

### DX Notifications

| Method | Description |
|--------|-------------|
| `dx/toolStarted` | Tool execution started |
| `dx/toolCompleted` | Tool execution completed |
| `dx/patternDetected` | DX patterns found in file |
| `dx/cacheStatus` | Cache status update |

## Configuration

```rust
pub struct DaemonConfig {
    pub project_root: PathBuf,
    pub enable_lsp_watcher: bool,    // Default: true
    pub enable_fs_watcher: bool,     // Default: true
    pub debounce_ms: u64,            // Default: 100
    pub worker_count: usize,         // Default: CPU count
    pub enable_r2_sync: bool,        // Default: false
    pub r2_bucket: Option<String>,
    pub auto_run_tools: bool,        // Default: true
    pub max_concurrent_tools: usize, // Default: 4
    pub tool_timeout_ms: u64,        // Default: 60000
    pub watch_patterns: Vec<String>,
    pub ignore_patterns: Vec<String>,
}
```

## Event Subscription

```rust
let daemon = ForgeDaemon::new(config)?;
let mut events = daemon.subscribe();

tokio::spawn(async move {
    while let Ok(event) = events.recv().await {
        match event {
            DaemonEvent::Started => println!("Daemon started"),
            DaemonEvent::FileChanged(change) => {
                println!("File changed: {:?}", change.path);
            }
            DaemonEvent::ToolStarted(tool) => {
                println!("Tool started: {:?}", tool);
            }
            DaemonEvent::ToolCompleted(tool, result) => {
                println!("Tool completed: {:?} ({}ms)", tool, result.duration_ms);
            }
            DaemonEvent::Stopped => break,
            _ => {}
        }
    }
});

daemon.start().await?;
```

## VS Code Extension

### Installation

```bash
cd integrations/vscode
npm install
npm run compile
```

### Commands

| Command | Description |
|---------|-------------|
| `DX: Start Forge Daemon` | Connect to daemon |
| `DX: Stop Forge Daemon` | Disconnect |
| `DX: Run Tool` | Run specific tool |
| `DX: Warm Cache` | Pre-warm all caches |
| `DX: Clear Cache` | Clear all caches |
| `DX: Show Statistics` | Show daemon stats |

### Settings

```json
{
  "dx.daemonPort": 9527,
  "dx.autoStart": true,
  "dx.enableLspWatcher": true,
  "dx.enableFsWatcher": true,
  "dx.autoRunTools": true
}
```

## Run Demo

```bash
cargo run --example forge_daemon_demo -p forge
```

## License

MIT OR Apache-2.0
