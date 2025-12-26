# Design Document: DX Unified Platform

## Overview

This design document describes the architecture for unifying the DX ecosystem into a cohesive platform. The system consists of three main components:

1. **DX CLI** (`dx`) - The unified command-line interface that controls all DX tools
2. **Forge Daemon** - A persistent background service with VCS-like file watching
3. **DX Extension** - The VS Code extension (renamed from "dx-serializer") with Forge integration

The architecture follows a hub-and-spoke model where Forge acts as the central orchestration hub, with the CLI and extension as primary interfaces.

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           DX UNIFIED PLATFORM                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌──────────────────┐                    ┌──────────────────────────────┐   │
│  │    DX CLI        │                    │     VS Code Extension        │   │
│  │  ┌────────────┐  │                    │  ┌────────────────────────┐  │   │
│  │  │ dx forge   │  │                    │  │ Status Bar Integration │  │   │
│  │  │ dx tools   │  │                    │  │ Command Palette        │  │   │
│  │  │ dx branch  │  │                    │  │ Serializer (existing)  │  │   │
│  │  │ dx config  │  │                    │  │ Forge Commands         │  │   │
│  │  │ dx chat*   │  │                    │  └────────────────────────┘  │   │
│  │  └────────────┘  │                    └──────────────┬───────────────┘   │
│  └────────┬─────────┘                                   │                    │
│           │                                             │                    │
│           │  IPC/Socket                    WebSocket/LSP│                    │
│           │                                             │                    │
│           └─────────────────┬───────────────────────────┘                    │
│                             ▼                                                │
│  ┌──────────────────────────────────────────────────────────────────────┐   │
│  │                      FORGE DAEMON                                     │   │
│  │  ┌─────────────────────────────────────────────────────────────────┐ │   │
│  │  │                    DUAL WATCHER SYSTEM                           │ │   │
│  │  │  ┌─────────────────┐         ┌─────────────────────────────┐   │ │   │
│  │  │  │  LSP Watcher    │         │  FileSystem Watcher         │   │ │   │
│  │  │  │  (Primary)      │         │  (Fallback)                 │   │ │   │
│  │  │  └────────┬────────┘         └──────────────┬──────────────┘   │ │   │
│  │  │           └──────────────┬──────────────────┘                   │ │   │
│  │  │                          ▼                                      │ │   │
│  │  │              UNIFIED CHANGE STREAM                              │ │   │
│  │  └─────────────────────────┬───────────────────────────────────────┘ │   │
│  │                            ▼                                          │   │
│  │  ┌─────────────────────────────────────────────────────────────────┐ │   │
│  │  │                  TRAFFIC BRANCH SYSTEM                          │ │   │
│  │  │  ┌─────────┐    ┌─────────┐    ┌─────────┐                     │ │   │
│  │  │  │ GREEN   │    │ YELLOW  │    │  RED    │                     │ │   │
│  │  │  │ (Auto)  │    │ (Review)│    │ (Block) │                     │ │   │
│  │  │  └─────────┘    └─────────┘    └─────────┘                     │ │   │
│  │  └─────────────────────────┬───────────────────────────────────────┘ │   │
│  │                            ▼                                          │   │
│  │  ┌─────────────────────────────────────────────────────────────────┐ │   │
│  │  │                    TOOL REGISTRY                                 │ │   │
│  │  │  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐  │ │   │
│  │  │  │Bundler* │ │ Style*  │ │TestRun* │ │PkgMgr*  │ │Serial*  │  │ │   │
│  │  │  └─────────┘ └─────────┘ └─────────┘ └─────────┘ └─────────┘  │ │   │
│  │  │  (* = Dummy implementation initially)                          │ │   │
│  │  └─────────────────────────────────────────────────────────────────┘ │   │
│  └──────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Components and Interfaces

### 1. DX CLI Component

The CLI is restructured to use a modular subcommand architecture:

```rust
// crates/dx-cli/src/main.rs
pub struct DxCli {
    config: DxConfig,
    daemon_client: Option<DaemonClient>,
}

pub enum Commands {
    // Forge daemon control
    Forge(ForgeCommands),
    // Tool management
    Tools(ToolsCommands),
    // Traffic branch control
    Branch(BranchCommands),
    // Configuration
    Config(ConfigCommands),
    // Existing commands
    New { name: String, template: String },
    Build { ... },
    Dev { ... },
    Style { ... },
}

pub enum ForgeCommands {
    Start { detach: bool, port: u16 },
    Stop { force: bool },
    Status,
    Restart,
    Logs { follow: bool, lines: usize },
}

pub enum ToolsCommands {
    List,
    Run { tool: String, args: Vec<String> },
    Enable { tool: String },
    Disable { tool: String },
}

pub enum BranchCommands {
    Status,
    Approve { all: bool },
    Reject { all: bool },
    History { limit: usize },
}
```

### 2. Daemon Client Interface

```rust
// crates/dx-cli/src/daemon_client.rs
pub struct DaemonClient {
    socket_path: PathBuf,
    connection: Option<UnixStream>,
}

impl DaemonClient {
    pub fn connect() -> Result<Self>;
    pub fn is_daemon_running() -> bool;
    pub fn send_command(&mut self, cmd: DaemonCommand) -> Result<DaemonResponse>;
    pub fn subscribe_events(&mut self) -> Result<EventStream>;
}

pub enum DaemonCommand {
    GetStatus,
    ListTools,
    RunTool(String),
    GetBranchStatus,
    ApprovePending,
    RejectPending,
    Shutdown,
}

pub enum DaemonResponse {
    Status(DaemonStatus),
    ToolList(Vec<ToolInfo>),
    ToolResult(ToolResult),
    BranchStatus(BranchStatus),
    Success,
    Error(String),
}
```

### 3. Forge Daemon Enhancements

```rust
// crates/forge/src/daemon/server.rs
pub struct DaemonServer {
    config: DaemonConfig,
    daemon: Arc<ForgeDaemon>,
    socket_path: PathBuf,
    lsp_port: u16,
}

impl DaemonServer {
    pub async fn start(&self) -> Result<()>;
    pub async fn handle_cli_connection(&self, stream: UnixStream);
    pub async fn handle_lsp_connection(&self, stream: TcpStream);
}

// crates/forge/src/daemon/ipc.rs
pub struct IpcServer {
    socket_path: PathBuf,
    command_handler: Arc<dyn CommandHandler>,
}

pub trait CommandHandler: Send + Sync {
    fn handle(&self, cmd: DaemonCommand) -> DaemonResponse;
}
```

### 4. Dummy Tool Implementation

```rust
// crates/forge/src/tools/dummy.rs
pub struct DummyTool {
    name: String,
    version: String,
    delay_ms: u64,
    fail_rate: f32,
}

impl DxToolExecutable for DummyTool {
    fn id(&self) -> DxToolId;
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn execute(&self, ctx: &ExecutionContext) -> Result<ToolResult>;
}

pub fn create_dummy_tools() -> Vec<Box<dyn DxToolExecutable>> {
    vec![
        Box::new(DummyTool::new("bundler", "0.1.0", 100, 0.0)),
        Box::new(DummyTool::new("style", "0.1.0", 50, 0.0)),
        Box::new(DummyTool::new("test-runner", "0.1.0", 200, 0.0)),
        Box::new(DummyTool::new("package-manager", "0.1.0", 150, 0.0)),
        Box::new(DummyTool::new("serializer", "0.1.0", 30, 0.0)),
        Box::new(DummyTool::new("www", "0.1.0", 80, 0.0)),
    ]
}
```

### 5. VS Code Extension Structure

```typescript
// extension/src/extension.ts (updated)
export async function activate(context: vscode.ExtensionContext) {
    // Initialize existing serializer functionality
    const serializer = new DxSerializer();
    
    // Initialize Forge connection
    const forgeClient = new ForgeClient();
    
    // Register status bar
    const statusBar = new ForgeStatusBar(forgeClient);
    
    // Register commands
    registerForgeCommands(context, forgeClient);
    
    // Attempt connection to daemon
    await forgeClient.connect();
}

// extension/src/forge/client.ts
export class ForgeClient {
    private ws: WebSocket | null = null;
    private reconnectAttempts = 0;
    
    async connect(): Promise<boolean>;
    async disconnect(): Promise<void>;
    async sendCommand(cmd: ForgeCommand): Promise<ForgeResponse>;
    onEvent(handler: (event: ForgeEvent) => void): void;
}

// extension/src/forge/statusBar.ts
export class ForgeStatusBar {
    private item: vscode.StatusBarItem;
    
    updateStatus(status: DaemonStatus): void;
    showConnecting(): void;
    showDisconnected(): void;
}
```

## Data Models

### Configuration Model

```rust
// crates/dx-cli/src/config.rs
#[derive(Debug, Serialize, Deserialize)]
pub struct DxConfig {
    pub project: ProjectConfig,
    pub forge: ForgeConfig,
    pub tools: ToolsConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForgeConfig {
    pub socket_path: Option<PathBuf>,
    pub lsp_port: u16,
    pub auto_start: bool,
    pub watch_patterns: Vec<String>,
    pub ignore_patterns: Vec<String>,
    pub debounce_ms: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolsConfig {
    pub enabled: Vec<String>,
    pub disabled: Vec<String>,
    pub custom: HashMap<String, CustomToolConfig>,
}
```

Note: Configuration is read from the `dx` file (no extension) in the project root, using the DX serializer format.

### Tool Registry Model

```rust
// crates/forge/src/tools/registry.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub status: ToolStatus,
    pub is_dummy: bool,
    pub last_run: Option<DateTime<Utc>>,
    pub run_count: u64,
    pub error_count: u64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ToolStatus {
    Ready,
    Running,
    Disabled,
    Error,
}
```

### Branch Status Model

```rust
// crates/forge/src/branch/status.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchStatus {
    pub current_color: BranchColor,
    pub pending_changes: Vec<PendingChange>,
    pub auto_approved: u64,
    pub manual_approved: u64,
    pub rejected: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingChange {
    pub id: String,
    pub path: PathBuf,
    pub change_type: ChangeType,
    pub color: BranchColor,
    pub timestamp: DateTime<Utc>,
    pub tool: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum BranchColor {
    Green,
    Yellow,
    Red,
}
```

