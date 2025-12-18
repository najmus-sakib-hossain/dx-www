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

## 📄 License

Licensed under either of:

- Apache License, Version 2.0
- MIT license

at your option.

---

Made with ❤️ by the DX Forge team
