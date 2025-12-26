# Requirements Document

## Introduction

This document specifies the requirements for unifying the DX ecosystem into a cohesive platform where the `dx` CLI serves as the central control interface for all DX tools, the `forge` crate operates as a persistent daemon with VCS-like file watching capabilities, and the VS Code extension (renamed from "dx-serializer" to "dx") provides IDE integration. The system uses dummy tool instances initially to establish the architecture, with real tool integration to follow.

## Glossary

- **DX_CLI**: The unified command-line interface (`dx`) that provides access to all DX tools and controls the Forge daemon
- **Forge_Daemon**: A persistent background service that watches for file changes (VCS-like), orchestrates tool execution, and manages the traffic branch system
- **Traffic_Branch**: A safety system with three states (Green/Yellow/Red) that determines how file changes are applied
- **DX_Extension**: The VS Code extension (renamed from "dx-serializer" to "dx") that provides IDE integration with Forge
- **Dummy_Tool**: A placeholder tool implementation that simulates real tool behavior for architecture validation
- **LSP_Bridge**: The communication layer between the VS Code extension and the Forge daemon
- **Tool_Registry**: A registry that tracks all available DX tools and their states

## Requirements

### Requirement 1: Unified CLI Control

**User Story:** As a developer, I want a single CLI command (`dx`) that controls all DX tools, so that I can manage the entire DX ecosystem from one interface.

#### Acceptance Criteria

1. WHEN a user runs `dx forge start`, THE DX_CLI SHALL start the Forge daemon in the background
2. WHEN a user runs `dx forge stop`, THE DX_CLI SHALL gracefully stop the running Forge daemon
3. WHEN a user runs `dx forge status`, THE DX_CLI SHALL display the current daemon state, uptime, and tool statistics
4. WHEN a user runs `dx tools list`, THE DX_CLI SHALL display all registered tools with their current status
5. WHEN a user runs `dx tools run <tool-name>`, THE DX_CLI SHALL execute the specified tool through the Forge daemon
6. WHEN the Forge daemon is not running and a tool command is issued, THE DX_CLI SHALL display an error message suggesting to start the daemon first
7. THE DX_CLI SHALL provide a `--help` flag for all commands that displays usage information

### Requirement 2: Forge Daemon VCS-like File Watching

**User Story:** As a developer, I want Forge to watch my project files like a VCS system, so that changes are automatically detected and processed by the appropriate tools.

#### Acceptance Criteria

1. WHEN the Forge daemon starts, THE Forge_Daemon SHALL initialize dual watchers (LSP and FileSystem) for the project directory
2. WHEN a file is created, modified, or deleted, THE Forge_Daemon SHALL detect the change within 100ms debounce window
3. WHEN a file change is detected, THE Forge_Daemon SHALL determine which tools need to run based on file extension and patterns
4. WHILE the Forge daemon is running, THE Forge_Daemon SHALL maintain a log of all file changes with timestamps
5. WHEN multiple changes occur rapidly, THE Forge_Daemon SHALL debounce and batch them to prevent redundant tool executions
6. IF the Forge daemon crashes unexpectedly, THEN THE Forge_Daemon SHALL log the error and attempt graceful recovery on restart

### Requirement 3: Traffic Branch System

**User Story:** As a developer, I want a traffic branch system that controls how changes are applied, so that I can safely manage automatic updates versus manual review.

#### Acceptance Criteria

1. WHEN a change is classified as safe (Green branch), THE Forge_Daemon SHALL apply it automatically without user intervention
2. WHEN a change requires review (Yellow branch), THE Forge_Daemon SHALL queue it and notify the user for approval
3. WHEN a change is potentially dangerous (Red branch), THE Forge_Daemon SHALL block it and require explicit manual approval
4. WHEN a user runs `dx branch status`, THE DX_CLI SHALL display the current branch color and pending changes
5. WHEN a user runs `dx branch approve`, THE DX_CLI SHALL approve all pending Yellow branch changes
6. WHEN a user runs `dx branch reject`, THE DX_CLI SHALL reject all pending Yellow branch changes

### Requirement 4: Dummy Tool Integration

**User Story:** As a developer, I want dummy tool instances that simulate real tool behavior, so that I can validate the architecture before integrating actual tools.

#### Acceptance Criteria

1. THE Forge_Daemon SHALL register dummy implementations for: Bundler, Style, TestRunner, PackageManager, Serializer, and Www tools
2. WHEN a dummy tool is executed, THE Dummy_Tool SHALL log its invocation with input parameters and return a success result after a simulated delay
3. WHEN a dummy tool encounters a simulated error condition, THE Dummy_Tool SHALL return an appropriate error result
4. THE Dummy_Tool SHALL implement the same interface as real tools to ensure seamless future replacement
5. WHEN `dx tools list` is run, THE DX_CLI SHALL indicate which tools are dummy implementations versus real implementations

### Requirement 5: VS Code Extension Rename and Enhancement

**User Story:** As a developer, I want the VS Code extension renamed from "dx-serializer" to "dx" and enhanced to connect with Forge, so that I have full IDE integration with the DX ecosystem.

#### Acceptance Criteria

1. THE DX_Extension SHALL be renamed from "vscode-dx-serializer" to "vscode-dx" in package.json
2. THE DX_Extension SHALL maintain all existing serializer functionality after the rename
3. WHEN the extension activates, THE DX_Extension SHALL attempt to connect to the Forge daemon via LSP bridge
4. WHEN the Forge daemon is not running, THE DX_Extension SHALL display a notification offering to start it
5. WHEN a file change occurs in VS Code, THE DX_Extension SHALL notify the Forge daemon via the LSP bridge
6. THE DX_Extension SHALL display Forge daemon status in the VS Code status bar
7. THE DX_Extension SHALL provide commands to start/stop/restart the Forge daemon from the command palette

### Requirement 6: LSP Bridge Communication

**User Story:** As a developer, I want seamless communication between VS Code and Forge, so that IDE actions are reflected in the daemon and vice versa.

#### Acceptance Criteria

1. WHEN the DX_Extension connects to Forge, THE LSP_Bridge SHALL establish a WebSocket connection on a configurable port
2. WHEN a file is saved in VS Code, THE LSP_Bridge SHALL send a file change notification to the Forge daemon
3. WHEN the Forge daemon detects a tool completion, THE LSP_Bridge SHALL send a notification to the extension for display
4. IF the LSP connection is lost, THEN THE LSP_Bridge SHALL attempt reconnection with exponential backoff
5. WHEN the Forge daemon sends diagnostic information, THE DX_Extension SHALL display it in the VS Code Problems panel

### Requirement 7: Configuration Management

**User Story:** As a developer, I want centralized configuration for the DX ecosystem, so that I can customize behavior across all tools.

#### Acceptance Criteria

1. THE DX_CLI SHALL read configuration from the `dx` file (no extension) in the project root
2. WHEN no `dx` file exists, THE DX_CLI SHALL use sensible defaults
3. THE DX_CLI SHALL support environment variable overrides for all configuration options
4. WHEN configuration is invalid, THE DX_CLI SHALL display a clear error message indicating the problem
5. THE DX_CLI SHALL provide a `dx config show` command that displays the current effective configuration
6. THE DX_CLI SHALL provide a `dx config init` command that creates a default `dx` configuration file
