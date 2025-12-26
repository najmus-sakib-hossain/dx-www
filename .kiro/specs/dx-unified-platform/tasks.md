# Implementation Tasks

## Task 1: Restructure DX CLI with Subcommand Architecture
- [x] 1.1: Create CLI module structure (`src/commands/forge.rs`, `src/commands/tools.rs`, `src/commands/branch.rs`, `src/commands/config.rs`)
- [x] 1.2: Implement `ForgeCommands` enum with Start, Stop, Status, Restart, Logs variants
- [x] 1.3: Implement `ToolsCommands` enum with List, Run, Enable, Disable variants
- [x] 1.4: Implement `BranchCommands` enum with Status, Approve, Reject, History variants
- [x] 1.5: Implement `ConfigCommands` enum with Show, Init variants
- [x] 1.6: Update main.rs to use new subcommand structure while preserving existing commands

## Task 2: Implement Daemon Client for CLI-Daemon Communication
- [x] 2.1: Create `src/daemon_client.rs` with DaemonClient struct
- [x] 2.2: Implement Unix socket connection logic (Windows named pipe fallback)
- [x] 2.3: Define DaemonCommand and DaemonResponse enums for IPC protocol
- [x] 2.4: Implement `is_daemon_running()` check function
- [x] 2.5: Implement `send_command()` for synchronous command execution
- [x] 2.6: Add connection retry logic with timeout

## Task 3: Enhance Forge Daemon with IPC Server
- [x] 3.1: Create `src/daemon/server.rs` with DaemonServer struct
- [x] 3.2: Implement Unix socket listener for CLI connections
- [x] 3.3: Create `src/daemon/ipc.rs` with IPC message handling
- [x] 3.4: Implement CommandHandler trait and default implementation
- [x] 3.5: Add daemon PID file management for process tracking
- [x] 3.6: Implement graceful shutdown handling via IPC

## Task 4: Implement Dummy Tool System
- [x] 4.1: Create `src/tools/dummy.rs` with DummyTool struct
- [x] 4.2: Implement DxToolExecutable trait for DummyTool
- [x] 4.3: Create `create_dummy_tools()` factory function for all 6 tools (Bundler, Style, TestRunner, PackageManager, Serializer, Www)
- [x] 4.4: Add `is_dummy` flag to ToolInfo struct
- [x] 4.5: Register dummy tools in Forge daemon on startup
- [x] 4.6: Add simulated delay and optional failure rate to dummy tools

## Task 5: Implement Traffic Branch System in CLI
- [x] 5.1: Create `src/commands/branch.rs` with branch command handlers
- [x] 5.2: Implement `dx branch status` to display current branch color and pending changes
- [x] 5.3: Implement `dx branch approve` to approve pending Yellow changes
- [x] 5.4: Implement `dx branch reject` to reject pending Yellow changes
- [x] 5.5: Implement `dx branch history` to show recent branch decisions
- [x] 5.6: Add colored terminal output for branch status (Green/Yellow/Red)

## Task 6: Implement Configuration System
- [x] 6.1: Create `src/config.rs` with DxConfig, ForgeConfig, ToolsConfig structs
- [x] 6.2: Implement `dx` file parser using dx_serializer
- [x] 6.3: Implement default configuration fallback
- [x] 6.4: Add environment variable override support
- [x] 6.5: Implement `dx config show` command
- [x] 6.6: Implement `dx config init` command to create default `dx` file

## Task 7: Rename VS Code Extension from dx-serializer to dx
- [x] 7.1: Update package.json: name to "vscode-dx", displayName to "DX"
- [x] 7.2: Update package.json: description to reflect full DX ecosystem
- [x] 7.3: Update README.md with new extension name and features
- [x] 7.4: Update all internal references from "dx-serializer" to "dx"
- [x] 7.5: Add new keywords: "forge", "tools", "orchestration"
- [ ] 7.6: Update extension icon/branding if needed

## Task 8: Add Forge Integration to VS Code Extension
- [x] 8.1: Create `src/forge/client.ts` with ForgeClient class
- [x] 8.2: Implement WebSocket connection to Forge daemon LSP port
- [x] 8.3: Create `src/forge/statusBar.ts` with ForgeStatusBar class
- [x] 8.4: Add status bar item showing daemon connection status
- [x] 8.5: Implement reconnection logic with exponential backoff
- [x] 8.6: Register Forge-related commands in command palette

## Task 9: Implement Extension Commands for Forge Control
- [x] 9.1: Add "DX: Start Forge Daemon" command
- [x] 9.2: Add "DX: Stop Forge Daemon" command
- [x] 9.3: Add "DX: Restart Forge Daemon" command
- [x] 9.4: Add "DX: Show Forge Status" command
- [x] 9.5: Add "DX: Show Tool List" command
- [x] 9.6: Update extension.ts to initialize Forge client on activation

## Task 10: Implement LSP Bridge in Forge
- [x] 10.1: Create `src/daemon/lsp_server.rs` with WebSocket server
- [x] 10.2: Define LSP message protocol for extension communication
- [x] 10.3: Implement file change notification from extension to daemon
- [x] 10.4: Implement tool completion notification from daemon to extension
- [x] 10.5: Implement diagnostic forwarding to extension
- [x] 10.6: Add connection management for multiple extension instances

## Task 11: Wire Up CLI Commands to Daemon
- [x] 11.1: Implement `dx forge start` - spawn daemon process, wait for ready
- [x] 11.2: Implement `dx forge stop` - send shutdown command via IPC
- [x] 11.3: Implement `dx forge status` - query daemon state and display
- [x] 11.4: Implement `dx tools list` - query tool registry and display
- [x] 11.5: Implement `dx tools run <tool>` - trigger tool execution via IPC
- [x] 11.6: Add error handling for daemon-not-running scenarios

## Task 12: Integration Testing and Verification
- [x] 12.1: Test CLI can start/stop daemon correctly
- [x] 12.2: Test dummy tools execute and return results
- [x] 12.3: Test VS Code extension connects to running daemon
- [x] 12.4: Test file changes trigger appropriate tool execution
- [x] 12.5: Test traffic branch approve/reject workflow
- [x] 12.6: Test configuration loading from `dx` file
