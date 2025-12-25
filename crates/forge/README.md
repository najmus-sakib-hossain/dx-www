# DX Forge v0.1.3

[![Crates.io](https://img.shields.io/crates/v/dx-forge.svg)](https://crates.io/crates/dx-forge)
[![Documentation](https://docs.rs/dx-forge/badge.svg)](https://docs.rs/dx-forge)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

## The Future of Developer Tooling

DX Forge is a production-ready orchestration engine and VCS for developer experience (DX) tools. It provides **132 eternal API functions** for building the next generation of development tools with zero-bloat component injection, traffic-branch safety, and offline-first architecture.

## ✨ Key Features

- **🎯 132 Immutable API Functions** - Complete, stable, professional API (v0.1.0 → forever)
- **🚦 Traffic Branch Safety** - Green/Yellow/Red conflict resolution prevents breaking changes
- **✨ Configuration Magic** - One-keystroke config injection with full templates
- **📦 Cart-Based Discovery** - Users discover features through intuitive shopping UX
- **🔌 Offline-First** - Works completely offline with binary caching
- **🎼 Smart Orchestration** - Priority-based execution with dependency resolution
- **📡 Event-Driven** - Global event bus for observability and extensibility
- **🌳 Git-Compatible VCS** - Content-addressable storage with snapshots
- **🔍 Dual-Watcher** - LSP + file system monitoring with pattern detection
- **⚡ Triple-Path Reactivity** - Realtime, debounced, and idle execution paths

## 🚀 Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
dx-forge = "0.1.0"
```

### Basic Usage

```rust
use dx_forge::*;

fn main() -> anyhow::Result<()> {
    // Initialize forge
    initialize_forge()?;
    
    // Register your tool
    struct MyTool;
    impl DxTool for MyTool {
        fn name(&self) -> &str { "my-tool" }
        fn version(&self) -> &str { "1.0.0" }
        fn priority(&self) -> u32 { 50 }
        
        fn execute(&mut self, ctx: &ExecutionContext) -> anyhow::Result<ToolOutput> {
            // Your tool logic here
            Ok(ToolOutput::success())
        }
    }
    
    register_tool(Box::new(MyTool))?;
    
    // Execute pipeline
    execute_pipeline("default")?;
    
    // Shutdown gracefully
    shutdown_forge()?;
    
    Ok(())
}
```

## 📚 The 132 Eternal Functions

DX Forge provides a comprehensive, immutable API organized into 14 categories:

### Core APIs (4 functions)

- `initialize_forge()`, `register_tool()`, `get_tool_context()`, `shutdown_forge()`

### Version Governance (6 functions)

- `declare_tool_version()`, `enforce_exact_version()`, `current_forge_version()`, `activate_package_variant()`, etc.

### Pipeline Execution (7 functions)

- `execute_pipeline()`, `execute_tool_immediately()`, `suspend_pipeline_execution()`, etc.

### Reactivity Engine (5 functions)

- `trigger_realtime_event()`, `trigger_debounced_event()`, `trigger_idle_event()`, etc.

### File Application & Branching (15 functions)

- `apply_changes()`, `preview_proposed_changes()`, `submit_branching_vote()`, etc.

### Event Bus (10 functions)

- `publish_event()`, `subscribe_to_event_stream()`, `emit_tool_started_event()`, etc.

### Configuration System (17 functions)

- `inject_full_config_section_at_cursor()` ⭐, `inject_style_tooling_config()`, etc.

### CI/CD & Workspace (8 functions)

- `trigger_ci_cd_pipeline()`, `detect_workspace_root()`, etc.

### .dx/ Directory (10 functions)

- `commit_current_dx_state()`, `cache_tool_offline_binary()`, etc.

### Offline-First (5 functions)

- `detect_offline_mode()`, `download_missing_tool_binaries()`, etc.

### Cart System (8 functions)

- `stage_item_in_cart()`, `commit_entire_cart()`, etc.

### Package Management (8 functions)

- `install_package_with_variant()`, `search_dx_package_registry()`, etc.

### Code Governance (5 functions)

- `mark_code_region_as_dx_generated()`, `claim_full_ownership_of_file()`, etc.

### DX Experience (26 functions)

- `open_file_and_reveal_location()`, `trigger_ai_powered_suggestion()`, etc.

📖 **Full API Documentation**: [docs/API_QUICK_REFERENCE.md](docs/API_QUICK_REFERENCE.md)

## 🎯 Why DX Forge?

### The Problem

Traditional developer tools suffer from:

- Bloated `node_modules` directories
- Breaking changes in updates
- Poor offline support
- Manual configuration hell
- Fragmented tool ecosystems

### The Solution

DX Forge provides:

- **Zero-bloat component injection** - Only inject what's needed
- **Traffic-branch safety** - Automatic conflict prevention
- **Offline-first architecture** - Works anywhere, anytime
- **Configuration magic** - One-click complete configs
- **Unified orchestration** - All tools work together

## 🏗️ Architecture

```text
┌─────────────────────────────────────────────┐
│           DX Forge Core Engine              │
├─────────────────────────────────────────────┤
│  • Global Lifecycle Management              │
│  • Version Governance & Enforcement         │
│  • Pipeline Orchestration                   │
│  • Triple-Path Reactivity Engine            │
└─────────────────────────────────────────────┘
                    │
        ┌───────────┼───────────┐
        │           │           │
┌───────▼─────┐ ┌──▼──────┐ ┌─▼────────────┐
│ Branching   │ │ Event   │ │ Config       │
│ Safety      │ │ Bus     │ │ Magic        │
│ System      │ │         │ │ Injection    │
└─────────────┘ └─────────┘ └──────────────┘
        │           │           │
┌───────▼───────────▼───────────▼────────────┐
│        Tool Ecosystem Layer                │
│  • dx-style   • dx-ui      • dx-auth       │
│  • dx-icons   • dx-fonts   • dx-media      │
└────────────────────────────────────────────┘
```

## 📦 What's Inside

- **Orchestrator**: Priority-based tool execution with dependency resolution
- **Dual-Watcher**: LSP + file system monitoring with debouncing
- **Traffic Branch System**: Green/Yellow/Red conflict resolution
- **Storage Layer**: Content-addressable blobs with R2 cloud sync
- **Version Manager**: Semantic versioning with compatibility checking
- **Pattern Detector**: Identifies DX tool patterns in source code
- **Injection Manager**: Fetches and caches components from storage
- **Event Bus**: Global event streaming for observability

## ⚡ Platform-Native I/O

DX Forge automatically uses the most performant I/O backend for your platform:

| Platform | Backend | Features |
|----------|---------|----------|
| Linux (5.1+) | io_uring | Batch submit, SQPOLL, zero-copy |
| macOS | kqueue | VNode events, timer events |
| Windows | IOCP | Overlapped I/O, directory changes |
| Fallback | tokio | Cross-platform compatibility |

### Usage

```rust
use dx_forge::{create_platform_io, PlatformIO, WriteOp};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Automatically selects the best backend
    let io = create_platform_io();
    println!("Using backend: {}", io.backend_name());
    
    // Single file operations
    io.write_all(Path::new("file.txt"), b"Hello").await?;
    let content = io.read_all(Path::new("file.txt")).await?;
    
    // Batch operations (optimized for io_uring)
    let ops = vec![
        WriteOp::new("a.txt", b"Content A".to_vec()),
        WriteOp::new("b.txt", b"Content B".to_vec()),
    ];
    io.batch_write(&ops).await?;
    
    Ok(())
}
```

### Configuration

```rust
use dx_forge::ForgeConfig;

let config = ForgeConfig::new(".")
    .with_max_file_handles(2048)      // Default: 1024
    .with_operation_timeout(30);       // Seconds
```

### Metrics & Observability

```rust
use dx_forge::MetricsCollector;

let metrics = MetricsCollector::new();
// ... perform operations ...

let stats = metrics.export_json();
println!("Operations: {}", stats["operations_total"]);
println!("Cache hit rate: {}%", stats["cache_hit_rate"]);
println!("P99 latency: {}μs", stats["io_latency_p99_us"]);
```

## 🔧 Configuration Magic Example

The killer feature - instant config injection:

```rust
// User types "style:" in dx.toml
let config = inject_full_config_section_at_cursor("style")?;

// Instantly expands to:
/*
[style]
# Style tooling configuration
processor = "tailwind"  # tailwind | css | scss
autoprefixer = true
minify = true

[style.tailwind]
config = "tailwind.config.js"
content = ["./src/**/*.{js,ts,jsx,tsx}"]
*/
```

## 🚦 Traffic Branch Safety

Automatic conflict prevention with color-coded safety:

```rust
let changes = vec![/* file changes */];

// Apply with automatic branching safety
let applied = apply_changes(changes)?;

// Or preview first
let preview = preview_proposed_changes(changes)?;
// Shows: 🟢 Safe files, 🟡 Review needed, 🔴 Manual resolution
```

## 📊 Status

- ✅ **API**: 132/132 functions implemented
- ✅ **Tests**: All core tests passing
- ✅ **Documentation**: Complete
- ✅ **Build**: Successful
- ✅ **Package**: Ready for publication

## 📖 Documentation

- [API Quick Reference](docs/API_QUICK_REFERENCE.md) - Fast lookup for all functions
- [API Implementation Status](docs/API_IMPLEMENTATION_STATUS.md) - Complete function listing
- [DX Tools Integration Guide](docs/DX_TOOLS_INTEGRATION_GUIDE.md) - Building tools with Forge
- [Architecture Overview](docs/DOCS.md) - Deep dive into internals

## 🤝 Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📄 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
- MIT license ([LICENSE-MIT](LICENSE) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

at your option.

## 🌟 The Vision

DX Forge is not just a library. It's the foundation for the next generation of developer tools. A world where:

- Tools work together seamlessly
- Updates never break your workflow
- Configuration is instant and intuitive
- Everything works offline
- The developer experience is delightful

**This is forge. The future has a name: dx.**

---

Made with ❤️ by the DX Forge team
