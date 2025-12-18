# Design Document: Binary Dawn Architecture

## Overview

The Binary Dawn architecture transforms dx-forge from a VCS into a Sovereign Orchestration Engine. It introduces three subsystems:

1. **Sovereign Orchestrator** - Manages tool lifecycle (init, run, watch, kill)
2. **Shadow Worker** - Handles background tasks without blocking
3. **Traffic Branching System** - Revolutionary package manager with source injection

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                      DxForge                            │
│  (The God Object: Sovereign Engine Entry Point)         │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  ┌─────────────────┐  ┌─────────────────┐              │
│  │   Orchestrator  │  │ BackgroundWorker│              │
│  │                 │  │                 │              │
│  │ - tools map     │  │ - mpsc channel  │              │
│  │ - states map    │  │ - task queue    │              │
│  │ - ensure_running│  │ - async loop    │              │
│  │ - execute_tool  │  │                 │              │
│  └─────────────────┘  └─────────────────┘              │
│                                                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │              TrafficManager                      │   │
│  │                                                  │   │
│  │  - registry path                                 │   │
│  │  - install_package()                             │   │
│  │  - analyze_traffic_safety() → Green/Yellow/Red   │   │
│  └─────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
```

## Components and Interfaces

### 1. DxForge (Entry Point)

```rust
pub struct DxForge {
    orchestrator: Orchestrator,
    background_worker: BackgroundWorker,
    traffic_manager: TrafficManager,
}

impl DxForge {
    pub async fn new() -> Self;
    pub async fn run_pipeline(&self, command: &str) -> anyhow::Result<()>;
}
```

### 2. Orchestrator Module

```rust
pub enum ToolStatus {
    Stopped,
    Starting,
    Running(u32),  // PID
    Healthy,
    Degraded,
}

pub struct DxToolDefinition {
    pub name: String,
    pub binary_path: String,
    pub priority: u32,
    pub dependencies: Vec<String>,
}

pub struct Orchestrator {
    tools: HashMap<String, DxToolDefinition>,
    states: Arc<RwLock<HashMap<String, ToolStatus>>>,
}

impl Orchestrator {
    pub fn new() -> Self;
    pub fn register_tool(&mut self, tool: DxToolDefinition);
    pub async fn ensure_running(&self, tool_name: &str) -> anyhow::Result<()>;
    pub async fn execute_tool(&self, tool_name: &str, args: &[&str]) -> anyhow::Result<()>;
}
```

### 3. BackgroundWorker Module

```rust
pub enum BackgroundTask {
    CacheCurrentState,
    SyncToCloudflareR2,
    PrefetchPackage(String),
    AnalyzeCodebasePatterns,
}

pub struct BackgroundWorker {
    sender: mpsc::Sender<BackgroundTask>,
}

impl BackgroundWorker {
    pub fn new() -> Self;
    pub async fn enqueue(&self, task: BackgroundTask);
}
```

### 4. TrafficManager Module

```rust
pub enum TrafficLight {
    Green,   // Safe to overwrite
    Yellow,  // User modified, needs merge
    Red,     // Heavy modification, manual intervention
}

pub struct TrafficManager {
    registry: PathBuf,
}

impl TrafficManager {
    pub fn new() -> Self;
    pub async fn install_package(&self, package_name: &str, version: &str) -> anyhow::Result<()>;
    pub fn analyze_traffic_safety(&self, path: &Path, new_content: &str) -> TrafficLight;
    pub async fn install_all_dependencies(&self) -> anyhow::Result<()>;
}
```

## Data Models

### ToolStatus Enum
- `Stopped` - Tool is not running
- `Starting` - Tool is in startup phase
- `Running(u32)` - Tool is active with given PID
- `Healthy` - Tool is running and responding
- `Degraded` - Tool is running but experiencing issues

### TrafficLight Enum
- `Green` - File doesn't exist or matches exactly, safe to write
- `Yellow` - File differs, attempt 3-way merge
- `Red` - Heavy user modifications, preserve and create .new file

### BackgroundTask Enum
- `CacheCurrentState` - Snapshot codebase for rollback
- `SyncToCloudflareR2` - Upload artifacts to cloud
- `PrefetchPackage(String)` - Download package ahead of time
- `AnalyzeCodebasePatterns` - Pattern analysis task

## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system-essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

### Property 1: Tool Registration Round-Trip
*For any* DxToolDefinition, registering it with the Orchestrator and then querying for that tool should return a definition with identical name, binary_path, priority, and dependencies.
**Validates: Requirements 2.1**

### Property 2: Ensure Running Idempotence
*For any* tool in Running state, calling ensure_running multiple times should leave the tool in Running state without changing its PID.
**Validates: Requirements 2.3**

### Property 3: Stopped to Running Transition
*For any* tool in Stopped state, calling ensure_running should transition it to Running state.
**Validates: Requirements 2.2**

### Property 4: Tool Status Validity
*For any* registered tool, querying its status should return exactly one of: Stopped, Starting, Running(PID), Healthy, or Degraded.
**Validates: Requirements 2.5**

### Property 5: Background Task Non-Blocking
*For any* BackgroundTask, calling enqueue should return within a bounded time (< 10ms) regardless of task complexity.
**Validates: Requirements 3.5**

### Property 6: Traffic Safety Analysis
*For any* file path and content:
- If path doesn't exist → Green
- If path exists and content matches → Green  
- If path exists and content differs → Yellow or Red
**Validates: Requirements 4.2, 4.3, 4.4**

### Property 7: Green Signal Injection
*For any* file with TrafficLight::Green, the install operation should result in the file existing with the new content.
**Validates: Requirements 4.6**

## Error Handling

| Error Scenario | Handling Strategy |
|----------------|-------------------|
| Tool binary not found | Return error with path info |
| Tool fails to start | Set status to Degraded, log error |
| Background channel full | Log warning, drop oldest task |
| File permission denied | Return error, don't modify |
| Package not found | Return error with package name |

## Testing Strategy

### Property-Based Testing Library
We will use `proptest` for Rust property-based testing.

### Unit Tests
- Test DxForge initialization
- Test pipeline command routing
- Test tool registration and retrieval
- Test status transitions

### Property-Based Tests
Each correctness property will be implemented as a single property-based test:
- Configure minimum 100 iterations per property
- Tag each test with: `**Feature: binary-dawn, Property {N}: {description}**`

### Test Structure
```
crates/dx-forge/src/
├── sovereign/
│   ├── mod.rs
│   ├── orchestrator.rs
│   ├── background.rs
│   ├── traffic.rs
│   └── tests.rs  (property tests)
```
