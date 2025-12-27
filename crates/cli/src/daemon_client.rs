//! Daemon Client for CLI-Daemon Communication
//!
//! Provides IPC communication with the Forge daemon.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

#[cfg(unix)]
use std::os::unix::net::UnixStream;

#[cfg(windows)]
use std::net::TcpStream;

// ============================================================================
// IPC PROTOCOL TYPES
// ============================================================================

/// Commands sent to the daemon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DaemonCommand {
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
}

/// Responses from the daemon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DaemonResponse {
    Status(DaemonStatus),
    ToolList(Vec<ToolInfo>),
    ToolResult(ToolResult),
    BranchStatus(BranchStatus),
    BranchHistory(Vec<BranchHistoryEntry>),
    Count(usize),
    Success,
    Error(String),
    Pong,
}

/// Daemon status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonStatus {
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

/// Tool information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub status: String,
    pub is_dummy: bool,
    pub last_run: Option<String>,
    pub run_count: u64,
    pub error_count: u64,
}

/// Tool execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub success: bool,
    pub warm_start: bool,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub output: Option<String>,
    pub error: Option<String>,
}

/// Branch status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchStatus {
    pub current_color: String,
    pub pending_changes: Vec<PendingChange>,
    pub auto_approved: u64,
    pub manual_approved: u64,
    pub rejected: u64,
}

/// Pending change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingChange {
    pub id: String,
    pub path: String,
    pub change_type: String,
    pub color: String,
    pub timestamp: String,
    pub tool: Option<String>,
}

/// Branch history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchHistoryEntry {
    pub timestamp: String,
    pub path: String,
    pub color: String,
    pub action: String,
}

// ============================================================================
// DAEMON CLIENT
// ============================================================================

/// Client for communicating with the Forge daemon
pub struct DaemonClient {
    #[cfg(unix)]
    stream: UnixStream,
    #[cfg(windows)]
    stream: TcpStream,
}

impl DaemonClient {
    /// Check if the daemon is running
    pub fn is_daemon_running() -> bool {
        // Check PID file first
        if let Some(pid) = Self::read_pid_file() {
            // Verify process is actually running
            #[cfg(unix)]
            {
                use std::process::Command;
                let output = Command::new("kill")
                    .arg("-0")
                    .arg(pid.to_string())
                    .output();
                if let Ok(o) = output {
                    if o.status.success() {
                        return true;
                    }
                }
            }
            #[cfg(windows)]
            {
                use std::process::Command;
                let output = Command::new("tasklist")
                    .arg("/FI")
                    .arg(format!("PID eq {}", pid))
                    .output();
                if let Ok(o) = output {
                    let stdout = String::from_utf8_lossy(&o.stdout);
                    if stdout.contains(&pid.to_string()) {
                        return true;
                    }
                }
            }
        }

        // Try to connect as fallback
        Self::connect().is_ok()
    }

    /// Connect to the daemon
    pub fn connect() -> Result<Self> {
        #[cfg(unix)]
        {
            let socket_path = Self::socket_path();
            let stream = UnixStream::connect(&socket_path)
                .map_err(|e| anyhow!("Failed to connect to daemon at {}: {}", socket_path.display(), e))?;
            stream.set_read_timeout(Some(std::time::Duration::from_secs(30)))?;
            stream.set_write_timeout(Some(std::time::Duration::from_secs(10)))?;
            Ok(Self { stream })
        }

        #[cfg(windows)]
        {
            let port = Self::ipc_port();
            let stream = TcpStream::connect(format!("127.0.0.1:{}", port))
                .map_err(|e| anyhow!("Failed to connect to daemon on port {}: {}", port, e))?;
            stream.set_read_timeout(Some(std::time::Duration::from_secs(30)))?;
            stream.set_write_timeout(Some(std::time::Duration::from_secs(10)))?;
            Ok(Self { stream })
        }
    }

    /// Send a command and receive response
    fn send(&mut self, cmd: DaemonCommand) -> Result<DaemonResponse> {
        let json = serde_json::to_string(&cmd)?;
        writeln!(self.stream, "{}", json)?;
        self.stream.flush()?;

        let mut reader = BufReader::new(&self.stream);
        let mut response = String::new();
        reader.read_line(&mut response)?;

        let resp: DaemonResponse = serde_json::from_str(&response)?;
        Ok(resp)
    }

    /// Get daemon status
    pub fn get_status(&mut self) -> Result<DaemonStatus> {
        match self.send(DaemonCommand::GetStatus)? {
            DaemonResponse::Status(status) => Ok(status),
            DaemonResponse::Error(e) => Err(anyhow!("{}", e)),
            _ => Err(anyhow!("Unexpected response")),
        }
    }

    /// List all tools
    pub fn list_tools(&mut self) -> Result<Vec<ToolInfo>> {
        match self.send(DaemonCommand::ListTools)? {
            DaemonResponse::ToolList(tools) => Ok(tools),
            DaemonResponse::Error(e) => Err(anyhow!("{}", e)),
            _ => Err(anyhow!("Unexpected response")),
        }
    }

    /// Run a specific tool
    pub fn run_tool(&mut self, name: &str, args: Vec<String>) -> Result<ToolResult> {
        match self.send(DaemonCommand::RunTool {
            name: name.to_string(),
            args,
        })? {
            DaemonResponse::ToolResult(result) => Ok(result),
            DaemonResponse::Error(e) => Err(anyhow!("{}", e)),
            _ => Err(anyhow!("Unexpected response")),
        }
    }

    /// Enable a tool
    pub fn enable_tool(&mut self, name: &str) -> Result<()> {
        match self.send(DaemonCommand::EnableTool {
            name: name.to_string(),
        })? {
            DaemonResponse::Success => Ok(()),
            DaemonResponse::Error(e) => Err(anyhow!("{}", e)),
            _ => Err(anyhow!("Unexpected response")),
        }
    }

    /// Disable a tool
    pub fn disable_tool(&mut self, name: &str) -> Result<()> {
        match self.send(DaemonCommand::DisableTool {
            name: name.to_string(),
        })? {
            DaemonResponse::Success => Ok(()),
            DaemonResponse::Error(e) => Err(anyhow!("{}", e)),
            _ => Err(anyhow!("Unexpected response")),
        }
    }

    /// Get branch status
    pub fn get_branch_status(&mut self) -> Result<BranchStatus> {
        match self.send(DaemonCommand::GetBranchStatus)? {
            DaemonResponse::BranchStatus(status) => Ok(status),
            DaemonResponse::Error(e) => Err(anyhow!("{}", e)),
            _ => Err(anyhow!("Unexpected response")),
        }
    }

    /// Approve a specific change
    pub fn approve_change(&mut self, id: &str) -> Result<()> {
        match self.send(DaemonCommand::ApproveChange { id: id.to_string() })? {
            DaemonResponse::Success => Ok(()),
            DaemonResponse::Error(e) => Err(anyhow!("{}", e)),
            _ => Err(anyhow!("Unexpected response")),
        }
    }

    /// Approve all pending changes
    pub fn approve_all_pending(&mut self) -> Result<usize> {
        match self.send(DaemonCommand::ApproveAllPending)? {
            DaemonResponse::Count(n) => Ok(n),
            DaemonResponse::Error(e) => Err(anyhow!("{}", e)),
            _ => Err(anyhow!("Unexpected response")),
        }
    }

    /// Reject a specific change
    pub fn reject_change(&mut self, id: &str) -> Result<()> {
        match self.send(DaemonCommand::RejectChange { id: id.to_string() })? {
            DaemonResponse::Success => Ok(()),
            DaemonResponse::Error(e) => Err(anyhow!("{}", e)),
            _ => Err(anyhow!("Unexpected response")),
        }
    }

    /// Reject all pending changes
    pub fn reject_all_pending(&mut self) -> Result<usize> {
        match self.send(DaemonCommand::RejectAllPending)? {
            DaemonResponse::Count(n) => Ok(n),
            DaemonResponse::Error(e) => Err(anyhow!("{}", e)),
            _ => Err(anyhow!("Unexpected response")),
        }
    }

    /// Get branch history
    pub fn get_branch_history(&mut self, limit: usize) -> Result<Vec<BranchHistoryEntry>> {
        match self.send(DaemonCommand::GetBranchHistory { limit })? {
            DaemonResponse::BranchHistory(history) => Ok(history),
            DaemonResponse::Error(e) => Err(anyhow!("{}", e)),
            _ => Err(anyhow!("Unexpected response")),
        }
    }

    /// Send shutdown command
    pub fn send_shutdown(&mut self, force: bool) -> Result<()> {
        match self.send(DaemonCommand::Shutdown { force })? {
            DaemonResponse::Success => Ok(()),
            DaemonResponse::Error(e) => Err(anyhow!("{}", e)),
            _ => Err(anyhow!("Unexpected response")),
        }
    }

    /// Get socket path (Unix)
    #[cfg(unix)]
    pub fn socket_path() -> PathBuf {
        let runtime_dir = std::env::var("XDG_RUNTIME_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| std::env::temp_dir());
        runtime_dir.join("dx-forge.sock")
    }

    /// Get IPC port (Windows)
    #[cfg(windows)]
    pub fn ipc_port() -> u16 {
        std::env::var("DX_FORGE_IPC_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(9877)
    }

    /// Get PID file path
    pub fn pid_path() -> PathBuf {
        let runtime_dir = std::env::var("XDG_RUNTIME_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| std::env::temp_dir());
        runtime_dir.join("dx-forge.pid")
    }

    /// Get log file path
    pub fn log_path() -> PathBuf {
        let cache_dir = dirs::cache_dir().unwrap_or_else(std::env::temp_dir);
        cache_dir.join("dx").join("forge.log")
    }

    /// Read PID from file
    pub fn read_pid_file() -> Option<u32> {
        let pid_path = Self::pid_path();
        std::fs::read_to_string(pid_path)
            .ok()
            .and_then(|s| s.trim().parse().ok())
    }
}
