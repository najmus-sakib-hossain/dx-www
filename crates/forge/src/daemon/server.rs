//! Forge Daemon IPC Server
//!
//! Handles CLI and extension connections via Unix sockets (or TCP on Windows).
//! Also provides WebSocket server for VS Code extension integration.

use crate::daemon::{DaemonEvent, DaemonState};
use crate::tools::{ToolRegistry, ToolStatus};
use crate::dx_cache::DxToolId;
use anyhow::Result;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::broadcast;
use futures::{SinkExt, StreamExt};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite;

#[cfg(unix)]
use tokio::net::{UnixListener, UnixStream};

use tokio::net::{TcpListener, TcpStream};

// ============================================================================
// IPC PROTOCOL
// ============================================================================

/// Commands from CLI/Extension to Daemon
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "command")]
pub enum IpcCommand {
    GetStatus,
    ListTools,
    RunTool { name: String, args: Vec<String> },
    EnableTool { name: String },
    DisableTool { name: String },
    GetBranchStatus,
    ApproveChange { id: String },
    ApproveAllPending,
    RejectChange { id: String },
    RejectAllPending,
    GetBranchHistory { limit: usize },
    Shutdown { force: bool },
    Ping,
    // Extension-specific
    FileChanged { path: String, change_type: String },
}

/// Responses from Daemon to CLI/Extension
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum IpcResponse {
    Status(StatusResponse),
    ToolList { tools: Vec<ToolInfoResponse> },
    ToolResult(ToolResultResponse),
    BranchStatus(BranchStatusResponse),
    BranchHistory { entries: Vec<BranchHistoryEntry> },
    Count { count: usize },
    Success,
    Error { message: String },
    Pong,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusResponse {
    pub state: String,
    pub uptime_seconds: u64,
    pub files_changed: u64,
    pub tools_executed: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub errors: u64,
    pub lsp_events: u64,
    pub fs_events: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInfoResponse {
    pub id: String,
    pub name: String,
    pub version: String,
    pub status: String,
    pub is_dummy: bool,
    pub last_run: Option<String>,
    pub run_count: u64,
    pub error_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResultResponse {
    pub success: bool,
    pub warm_start: bool,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub output: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchStatusResponse {
    pub current_color: String,
    pub pending_changes: Vec<PendingChangeResponse>,
    pub auto_approved: u64,
    pub manual_approved: u64,
    pub rejected: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingChangeResponse {
    pub id: String,
    pub path: String,
    pub change_type: String,
    pub color: String,
    pub timestamp: String,
    pub tool: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchHistoryEntry {
    pub timestamp: String,
    pub path: String,
    pub color: String,
    pub action: String,
}

// ============================================================================
// DAEMON SERVER
// ============================================================================

/// The IPC server for the Forge daemon
pub struct DaemonServer {
    tool_registry: Arc<ToolRegistry>,
    shutdown: Arc<AtomicBool>,
    state: Arc<RwLock<DaemonState>>,
    stats: Arc<RwLock<ServerStats>>,
    event_tx: broadcast::Sender<DaemonEvent>,
    branch_state: Arc<RwLock<BranchState>>,
}

#[derive(Debug, Default)]
struct ServerStats {
    uptime_start: Option<std::time::Instant>,
    files_changed: u64,
    tools_executed: u64,
    cache_hits: u64,
    cache_misses: u64,
    errors: u64,
    lsp_events: u64,
    fs_events: u64,
}

#[derive(Debug, Default)]
struct BranchState {
    current_color: BranchColor,
    pending_changes: Vec<PendingChange>,
    auto_approved: u64,
    manual_approved: u64,
    rejected: u64,
    history: Vec<BranchHistoryEntry>,
}

#[derive(Debug, Clone, Copy, Default)]
enum BranchColor {
    #[default]
    Green,
    Yellow,
    Red,
}

#[derive(Debug, Clone)]
struct PendingChange {
    id: String,
    path: String,
    change_type: String,
    color: BranchColor,
    timestamp: chrono::DateTime<chrono::Utc>,
    tool: Option<String>,
}

impl DaemonServer {
    /// Create a new daemon server
    pub fn new() -> Self {
        let (event_tx, _) = broadcast::channel(1000);
        
        Self {
            tool_registry: Arc::new(ToolRegistry::with_dummy_tools()),
            shutdown: Arc::new(AtomicBool::new(false)),
            state: Arc::new(RwLock::new(DaemonState::Stopped)),
            stats: Arc::new(RwLock::new(ServerStats::default())),
            event_tx,
            branch_state: Arc::new(RwLock::new(BranchState::default())),
        }
    }

    /// Get the socket path
    #[cfg(unix)]
    pub fn socket_path() -> PathBuf {
        let runtime_dir = std::env::var("XDG_RUNTIME_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| std::env::temp_dir());
        runtime_dir.join("dx-forge.sock")
    }

    /// Get the IPC port (Windows)
    #[cfg(windows)]
    pub fn ipc_port() -> u16 {
        std::env::var("DX_FORGE_IPC_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(9877)
    }

    /// Get the WebSocket port for VS Code extension
    pub fn ws_port() -> u16 {
        std::env::var("DX_FORGE_WS_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(9876)
    }

    /// Get the PID file path
    pub fn pid_path() -> PathBuf {
        let runtime_dir = std::env::var("XDG_RUNTIME_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| std::env::temp_dir());
        runtime_dir.join("dx-forge.pid")
    }

    /// Write PID file
    fn write_pid_file() -> Result<()> {
        let pid = std::process::id();
        std::fs::write(Self::pid_path(), pid.to_string())?;
        Ok(())
    }

    /// Remove PID file
    fn remove_pid_file() {
        let _ = std::fs::remove_file(Self::pid_path());
    }

    /// Start the IPC server
    pub async fn start(&self) -> Result<()> {
        *self.state.write() = DaemonState::Starting;
        self.stats.write().uptime_start = Some(std::time::Instant::now());
        
        // Write PID file
        Self::write_pid_file()?;

        let ws_port = Self::ws_port();
        
        println!("╔══════════════════════════════════════════════════════════════╗");
        println!("║        ⚔️  FORGE DAEMON SERVER - Binary Dawn Edition          ║");
        println!("╠══════════════════════════════════════════════════════════════╣");
        println!("║  Tools Registered: {}                                         ║", self.tool_registry.count());
        println!("║  WebSocket Port: {}                                        ║", ws_port);
        println!("╚══════════════════════════════════════════════════════════════╝");

        // Start WebSocket server in background
        let ws_handler = self.clone_for_handler();
        let ws_shutdown = self.shutdown.clone();
        tokio::spawn(async move {
            if let Err(e) = ws_handler.start_websocket_server(ws_port, ws_shutdown).await {
                eprintln!("WebSocket server error: {}", e);
            }
        });

        #[cfg(unix)]
        {
            self.start_unix_server().await
        }

        #[cfg(windows)]
        {
            self.start_tcp_server().await
        }
    }

    #[cfg(unix)]
    async fn start_unix_server(&self) -> Result<()> {
        let socket_path = Self::socket_path();
        
        // Remove existing socket
        let _ = std::fs::remove_file(&socket_path);
        
        let listener = UnixListener::bind(&socket_path)?;
        println!("🔌 IPC Server listening on: {}", socket_path.display());
        
        *self.state.write() = DaemonState::Running;

        loop {
            if self.shutdown.load(Ordering::SeqCst) {
                break;
            }

            tokio::select! {
                result = listener.accept() => {
                    match result {
                        Ok((stream, _)) => {
                            let server = self.clone_for_handler();
                            tokio::spawn(async move {
                                if let Err(e) = server.handle_unix_connection(stream).await {
                                    eprintln!("Connection error: {}", e);
                                }
                            });
                        }
                        Err(e) => {
                            eprintln!("Accept error: {}", e);
                        }
                    }
                }
                _ = tokio::time::sleep(tokio::time::Duration::from_millis(100)) => {
                    // Check shutdown flag periodically
                }
            }
        }

        // Cleanup
        let _ = std::fs::remove_file(&socket_path);
        Self::remove_pid_file();
        *self.state.write() = DaemonState::Stopped;
        
        Ok(())
    }

    #[cfg(windows)]
    async fn start_tcp_server(&self) -> Result<()> {
        let port = Self::ipc_port();
        let addr = format!("127.0.0.1:{}", port);
        
        let listener = TcpListener::bind(&addr).await?;
        println!("🔌 IPC Server listening on: {}", addr);
        
        *self.state.write() = DaemonState::Running;

        loop {
            if self.shutdown.load(Ordering::SeqCst) {
                break;
            }

            tokio::select! {
                result = listener.accept() => {
                    match result {
                        Ok((stream, _)) => {
                            let server = self.clone_for_handler();
                            tokio::spawn(async move {
                                if let Err(e) = server.handle_tcp_connection(stream).await {
                                    eprintln!("Connection error: {}", e);
                                }
                            });
                        }
                        Err(e) => {
                            eprintln!("Accept error: {}", e);
                        }
                    }
                }
                _ = tokio::time::sleep(tokio::time::Duration::from_millis(100)) => {
                    // Check shutdown flag periodically
                }
            }
        }

        Self::remove_pid_file();
        *self.state.write() = DaemonState::Stopped;
        
        Ok(())
    }

    fn clone_for_handler(&self) -> DaemonServerHandler {
        DaemonServerHandler {
            tool_registry: self.tool_registry.clone(),
            shutdown: self.shutdown.clone(),
            state: self.state.clone(),
            stats: self.stats.clone(),
            event_tx: self.event_tx.clone(),
            branch_state: self.branch_state.clone(),
        }
    }

    /// Stop the server
    pub fn stop(&self) {
        self.shutdown.store(true, Ordering::SeqCst);
        *self.state.write() = DaemonState::ShuttingDown;
    }

    /// Subscribe to events
    pub fn subscribe(&self) -> broadcast::Receiver<DaemonEvent> {
        self.event_tx.subscribe()
    }
}

// ============================================================================
// CONNECTION HANDLER
// ============================================================================

struct DaemonServerHandler {
    tool_registry: Arc<ToolRegistry>,
    shutdown: Arc<AtomicBool>,
    state: Arc<RwLock<DaemonState>>,
    stats: Arc<RwLock<ServerStats>>,
    event_tx: broadcast::Sender<DaemonEvent>,
    branch_state: Arc<RwLock<BranchState>>,
}

impl DaemonServerHandler {
    /// Start WebSocket server for VS Code extension
    async fn start_websocket_server(&self, port: u16, shutdown: Arc<AtomicBool>) -> Result<()> {
        let addr = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(&addr).await?;
        println!("🌐 WebSocket Server listening on: ws://{}", addr);

        loop {
            if shutdown.load(Ordering::SeqCst) {
                break;
            }

            tokio::select! {
                result = listener.accept() => {
                    match result {
                        Ok((stream, addr)) => {
                            println!("📡 WebSocket connection from: {}", addr);
                            let handler = DaemonServerHandler {
                                tool_registry: self.tool_registry.clone(),
                                shutdown: self.shutdown.clone(),
                                state: self.state.clone(),
                                stats: self.stats.clone(),
                                event_tx: self.event_tx.clone(),
                                branch_state: self.branch_state.clone(),
                            };
                            tokio::spawn(async move {
                                if let Err(e) = handler.handle_websocket_connection(stream).await {
                                    eprintln!("WebSocket connection error: {}", e);
                                }
                            });
                        }
                        Err(e) => {
                            eprintln!("WebSocket accept error: {}", e);
                        }
                    }
                }
                _ = tokio::time::sleep(tokio::time::Duration::from_millis(100)) => {
                    // Check shutdown flag periodically
                }
            }
        }

        Ok(())
    }

    /// Handle a WebSocket connection
    async fn handle_websocket_connection(&self, stream: TcpStream) -> Result<()> {
        let ws_stream = accept_async(stream).await?;
        let (mut write, mut read) = ws_stream.split();

        while let Some(msg) = read.next().await {
            match msg {
                Ok(tungstenite::Message::Text(text)) => {
                    let response = self.handle_command(&text);
                    let response_json = serde_json::to_string(&response)?;
                    write.send(tungstenite::Message::Text(response_json.into())).await?;
                }
                Ok(tungstenite::Message::Ping(data)) => {
                    write.send(tungstenite::Message::Pong(data)).await?;
                }
                Ok(tungstenite::Message::Close(_)) => {
                    break;
                }
                Err(e) => {
                    eprintln!("WebSocket message error: {}", e);
                    break;
                }
                _ => {}
            }
        }

        Ok(())
    }

    #[cfg(unix)]
    async fn handle_unix_connection(&self, stream: UnixStream) -> Result<()> {
        let (reader, mut writer) = stream.into_split();
        let mut reader = BufReader::new(reader);
        let mut line = String::new();

        loop {
            line.clear();
            let n = reader.read_line(&mut line).await?;
            if n == 0 {
                break; // Connection closed
            }

            let response = self.handle_command(&line);
            let response_json = serde_json::to_string(&response)? + "\n";
            writer.write_all(response_json.as_bytes()).await?;
            writer.flush().await?;

            // Check for shutdown command
            if matches!(response, IpcResponse::Success) && line.contains("Shutdown") {
                break;
            }
        }

        Ok(())
    }

    #[cfg(windows)]
    async fn handle_tcp_connection(&self, stream: TcpStream) -> Result<()> {
        let (reader, mut writer) = stream.into_split();
        let mut reader = BufReader::new(reader);
        let mut line = String::new();

        loop {
            line.clear();
            let n = reader.read_line(&mut line).await?;
            if n == 0 {
                break;
            }

            let response = self.handle_command(&line);
            let response_json = serde_json::to_string(&response)? + "\n";
            writer.write_all(response_json.as_bytes()).await?;
            writer.flush().await?;

            if matches!(response, IpcResponse::Success) && line.contains("Shutdown") {
                break;
            }
        }

        Ok(())
    }

    fn handle_command(&self, line: &str) -> IpcResponse {
        let cmd: Result<IpcCommand, _> = serde_json::from_str(line.trim());
        
        match cmd {
            Ok(command) => self.execute_command(command),
            Err(e) => IpcResponse::Error {
                message: format!("Invalid command: {}", e),
            },
        }
    }

    fn execute_command(&self, cmd: IpcCommand) -> IpcResponse {
        match cmd {
            IpcCommand::Ping => IpcResponse::Pong,
            
            IpcCommand::GetStatus => {
                let stats = self.stats.read();
                let uptime = stats.uptime_start
                    .map(|s| s.elapsed().as_secs())
                    .unwrap_or(0);
                
                IpcResponse::Status(StatusResponse {
                    state: format!("{:?}", *self.state.read()),
                    uptime_seconds: uptime,
                    files_changed: stats.files_changed,
                    tools_executed: stats.tools_executed,
                    cache_hits: stats.cache_hits,
                    cache_misses: stats.cache_misses,
                    errors: stats.errors,
                    lsp_events: stats.lsp_events,
                    fs_events: stats.fs_events,
                })
            }
            
            IpcCommand::ListTools => {
                let tools = self.tool_registry.list();
                let tool_responses: Vec<ToolInfoResponse> = tools
                    .into_iter()
                    .map(|t| ToolInfoResponse {
                        id: t.id,
                        name: t.name,
                        version: t.version,
                        status: format!("{:?}", t.status),
                        is_dummy: t.is_dummy,
                        last_run: t.last_run.map(|dt| dt.to_rfc3339()),
                        run_count: t.run_count,
                        error_count: t.error_count,
                    })
                    .collect();
                
                IpcResponse::ToolList { tools: tool_responses }
            }
            
            IpcCommand::RunTool { name, args: _ } => {
                // Find tool by name
                let tool_id = match name.to_lowercase().as_str() {
                    "bundler" | "dx-bundler" => Some(DxToolId::Bundler),
                    "style" | "dx-style" => Some(DxToolId::Style),
                    "test-runner" | "dx-test-runner" | "test" => Some(DxToolId::Test),
                    "package-manager" | "dx-package-manager" | "node-modules" => Some(DxToolId::NodeModules),
                    "serializer" | "dx-serializer" => Some(DxToolId::Serializer),
                    "www" | "dx-www" => Some(DxToolId::Www),
                    _ => None,
                };

                match tool_id {
                    Some(id) => {
                        self.tool_registry.set_status(id, ToolStatus::Running);
                        
                        // Simulate execution (dummy tools)
                        std::thread::sleep(std::time::Duration::from_millis(100));
                        
                        self.tool_registry.record_execution(id, true);
                        self.tool_registry.set_status(id, ToolStatus::Ready);
                        self.stats.write().tools_executed += 1;
                        
                        IpcResponse::ToolResult(ToolResultResponse {
                            success: true,
                            warm_start: true,
                            cache_hits: 1,
                            cache_misses: 0,
                            output: Some(format!("Tool {} executed successfully", name)),
                            error: None,
                        })
                    }
                    None => IpcResponse::Error {
                        message: format!("Unknown tool: {}", name),
                    },
                }
            }
            
            IpcCommand::EnableTool { name } => {
                let tool_id = self.parse_tool_name(&name);
                match tool_id {
                    Some(id) => {
                        self.tool_registry.enable(id);
                        IpcResponse::Success
                    }
                    None => IpcResponse::Error {
                        message: format!("Unknown tool: {}", name),
                    },
                }
            }
            
            IpcCommand::DisableTool { name } => {
                let tool_id = self.parse_tool_name(&name);
                match tool_id {
                    Some(id) => {
                        self.tool_registry.disable(id);
                        IpcResponse::Success
                    }
                    None => IpcResponse::Error {
                        message: format!("Unknown tool: {}", name),
                    },
                }
            }
            
            IpcCommand::GetBranchStatus => {
                let state = self.branch_state.read();
                IpcResponse::BranchStatus(BranchStatusResponse {
                    current_color: format!("{:?}", state.current_color),
                    pending_changes: state.pending_changes.iter().map(|c| PendingChangeResponse {
                        id: c.id.clone(),
                        path: c.path.clone(),
                        change_type: c.change_type.clone(),
                        color: format!("{:?}", c.color),
                        timestamp: c.timestamp.to_rfc3339(),
                        tool: c.tool.clone(),
                    }).collect(),
                    auto_approved: state.auto_approved,
                    manual_approved: state.manual_approved,
                    rejected: state.rejected,
                })
            }
            
            IpcCommand::ApproveChange { id } => {
                let mut state = self.branch_state.write();
                if let Some(pos) = state.pending_changes.iter().position(|c| c.id == id) {
                    let change = state.pending_changes.remove(pos);
                    state.manual_approved += 1;
                    state.history.push(BranchHistoryEntry {
                        timestamp: chrono::Utc::now().to_rfc3339(),
                        path: change.path,
                        color: format!("{:?}", change.color),
                        action: "approved".to_string(),
                    });
                    IpcResponse::Success
                } else {
                    IpcResponse::Error {
                        message: format!("Change not found: {}", id),
                    }
                }
            }
            
            IpcCommand::ApproveAllPending => {
                let mut state = self.branch_state.write();
                let count = state.pending_changes.len();
                let changes: Vec<_> = state.pending_changes.drain(..).collect();
                for change in changes {
                    state.history.push(BranchHistoryEntry {
                        timestamp: chrono::Utc::now().to_rfc3339(),
                        path: change.path,
                        color: format!("{:?}", change.color),
                        action: "approved".to_string(),
                    });
                }
                state.manual_approved += count as u64;
                IpcResponse::Count { count }
            }
            
            IpcCommand::RejectChange { id } => {
                let mut state = self.branch_state.write();
                if let Some(pos) = state.pending_changes.iter().position(|c| c.id == id) {
                    let change = state.pending_changes.remove(pos);
                    state.rejected += 1;
                    state.history.push(BranchHistoryEntry {
                        timestamp: chrono::Utc::now().to_rfc3339(),
                        path: change.path,
                        color: format!("{:?}", change.color),
                        action: "rejected".to_string(),
                    });
                    IpcResponse::Success
                } else {
                    IpcResponse::Error {
                        message: format!("Change not found: {}", id),
                    }
                }
            }
            
            IpcCommand::RejectAllPending => {
                let mut state = self.branch_state.write();
                let count = state.pending_changes.len();
                let changes: Vec<_> = state.pending_changes.drain(..).collect();
                for change in changes {
                    state.history.push(BranchHistoryEntry {
                        timestamp: chrono::Utc::now().to_rfc3339(),
                        path: change.path,
                        color: format!("{:?}", change.color),
                        action: "rejected".to_string(),
                    });
                }
                state.rejected += count as u64;
                IpcResponse::Count { count }
            }
            
            IpcCommand::GetBranchHistory { limit } => {
                let state = self.branch_state.read();
                let entries: Vec<_> = state.history
                    .iter()
                    .rev()
                    .take(limit)
                    .cloned()
                    .collect();
                IpcResponse::BranchHistory { entries }
            }
            
            IpcCommand::Shutdown { force: _ } => {
                self.shutdown.store(true, Ordering::SeqCst);
                *self.state.write() = DaemonState::ShuttingDown;
                IpcResponse::Success
            }
            
            IpcCommand::FileChanged { path, change_type } => {
                self.stats.write().files_changed += 1;
                println!("📝 File changed: {} ({})", path, change_type);
                IpcResponse::Success
            }
        }
    }

    fn parse_tool_name(&self, name: &str) -> Option<DxToolId> {
        match name.to_lowercase().as_str() {
            "bundler" | "dx-bundler" => Some(DxToolId::Bundler),
            "style" | "dx-style" => Some(DxToolId::Style),
            "test-runner" | "dx-test-runner" | "test" => Some(DxToolId::Test),
            "package-manager" | "dx-package-manager" | "node-modules" => Some(DxToolId::NodeModules),
            "serializer" | "dx-serializer" => Some(DxToolId::Serializer),
            "www" | "dx-www" => Some(DxToolId::Www),
            _ => None,
        }
    }
}
