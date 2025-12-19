# DX Forge: Tool Orchestration with .dx Cache System

## Overview

**DX Forge** is the orchestration backbone for all DX tools, providing:
- `.dx` folder structure with 16 tool-specific cache directories
- Blake3 content-addressable storage for fast lookups
- Warm start caching (10x faster than cold builds)
- R2 cloud sync for shared team cache
- Dependency-ordered tool execution

## .dx Folder Structure

```
project/
├── dx                    # Universal config file (like package.json)
├── .dx/                  # DX cache directory (like node_modules/)
│   ├── cache/            # Global cache
│   ├── forge/            # Forge orchestrator state
│   ├── bundler/          # JS/TS bundler cache
│   ├── node_modules/     # npm package cache (replaces node_modules/)
│   ├── test/             # Test runner cache
│   ├── style/            # Binary CSS cache
│   ├── icon/             # SVG icon cache
│   ├── font/             # Font cache
│   ├── media/            # Image/video cache
│   ├── i18n/             # Internationalization cache
│   ├── ui/               # UI component cache
│   ├── serializer/       # Serializer cache
│   ├── generator/        # Code generator cache
│   ├── driven/           # AI task cache
│   ├── workspace/        # Monorepo workspace cache
│   └── www/              # HTIP framework cache
```

## Quick Start

```rust
use forge::{
    DxToolExecutor, DxToolId, BundlerTool, PackageManagerTool,
    StyleTool, TestRunnerTool, ToolConfig,
};

fn main() -> anyhow::Result<()> {
    // 1. Create executor (auto-creates .dx folder)
    let mut executor = DxToolExecutor::new("./my-project")?;
    
    // 2. Register tools
    executor.register(PackageManagerTool);
    executor.register(BundlerTool);
    executor.register(StyleTool);
    executor.register(TestRunnerTool);
    
    // 3. Configure tools
    executor.configure(DxToolId::Bundler, ToolConfig {
        enabled: true,
        cache_enabled: true,
        r2_sync: true,
        timeout_ms: 30_000,
        ..Default::default()
    });
    
    // 4. Warm up cache (10x faster on subsequent runs)
    let warm_starts = executor.warm_up()?;
    
    // 5. Execute all tools in dependency order
    let results = executor.execute_all()?;
    
    Ok(())
}
```

## Warm Start Performance

After the initial cold build, DX Forge caches all tool outputs for instant warm starts:

| Tool            | Cold     | Warm      | Speedup |
|-----------------|----------|-----------|---------|
| package-manager | 620ms    | 36ms      | **17.2x** |
| bundler         | 100ms    | 10ms      | **10.0x** |
| style           | 50ms     | 5ms       | **10.0x** |
| test            | 200ms    | 20ms      | **10.0x** |

## R2 Cloud Sync

Enable shared team cache via Cloudflare R2:

```bash
# Set environment variables
export DX_R2_BUCKET="dx-team-cache"
export R2_ACCOUNT_ID="your-account-id"
export R2_ACCESS_KEY_ID="your-key-id"
export R2_SECRET_ACCESS_KEY="your-secret-key"
```

```rust
// Sync caches to R2
executor.sync_to_r2().await?;

// Pull caches from R2 (new team member)
executor.cache().pull_from_r2(DxToolId::NodeModules).await?;
```

## Creating Custom Tools

Implement the `DxToolExecutable` trait:

```rust
use forge::{DxToolExecutable, DxToolId, DxExecutionContext, ToolResult};

struct MyCustomTool;

impl DxToolExecutable for MyCustomTool {
    fn id(&self) -> DxToolId {
        DxToolId::Generator  // Use appropriate ID
    }

    fn execute(&self, ctx: &DxExecutionContext) -> anyhow::Result<ToolResult> {
        let warm = ctx.has_warm_cache(self.id());
        
        // Your tool logic here
        
        Ok(ToolResult {
            tool: "my-custom-tool".to_string(),
            success: true,
            duration_ms: 100,
            warm_start: warm,
            cache_hits: 0,
            cache_misses: 0,
            output_files: vec![],
            errors: vec![],
        })
    }

    fn should_run(&self, ctx: &DxExecutionContext) -> bool {
        ctx.config(self.id()).enabled
    }

    fn dependencies(&self) -> &[DxToolId] {
        &[DxToolId::NodeModules]  // Runs after package install
    }

    fn build_cache(&self, ctx: &DxExecutionContext, result: &ToolResult) -> anyhow::Result<()> {
        for output in &result.output_files {
            if output.exists() {
                let content = std::fs::read(output)?;
                ctx.cache.cache_content(self.id(), output, &content)?;
            }
        }
        Ok(())
    }
}
```

## Cache API

```rust
use forge::{DxToolCacheManager, DxToolId};

let cache = DxToolCacheManager::new("./project")?;

// Cache content with Blake3 hash
let entry = cache.cache_content(
    DxToolId::Bundler,
    &PathBuf::from("src/main.js"),
    b"content",
)?;

// Retrieve cached content
let content = cache.get_cached_content(DxToolId::Bundler, &entry.hash)?;

// Check if cached
let exists = cache.is_cached(&hash);

// Clear tool cache
cache.clear_tool_cache(DxToolId::Bundler)?;

// Clear all caches
cache.clear_all()?;
```

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                      DxToolExecutor                              │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │                 DxToolCacheManager                        │   │
│  │  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐        │   │
│  │  │ bundler │ │ style   │ │ test    │ │ www     │  ...   │   │
│  │  └────┬────┘ └────┬────┘ └────┬────┘ └────┬────┘        │   │
│  │       │           │           │           │              │   │
│  │       └───────────┴───────────┴───────────┘              │   │
│  │                       │                                   │   │
│  │                ┌──────┴──────┐                           │   │
│  │                │ Blake3 Hash │                           │   │
│  │                └──────┬──────┘                           │   │
│  │                       │                                   │   │
│  │           ┌───────────┼───────────┐                      │   │
│  │           ▼           ▼           ▼                      │   │
│  │    ┌──────────┐ ┌──────────┐ ┌──────────┐               │   │
│  │    │  Disk    │ │  Memory  │ │    R2    │               │   │
│  │    │  Cache   │ │  LRU     │ │  Cloud   │               │   │
│  │    └──────────┘ └──────────┘ └──────────┘               │   │
│  └──────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
```

## API Reference

### DxToolId (16 tools)

| ID | Folder | Description |
|----|--------|-------------|
| `Cache` | `cache/` | Global cache |
| `Forge` | `forge/` | Orchestrator state |
| `Bundler` | `bundler/` | JS/TS bundler |
| `NodeModules` | `node_modules/` | npm packages |
| `Test` | `test/` | Test runner |
| `Style` | `style/` | Binary CSS |
| `Icon` | `icon/` | SVG icons |
| `Font` | `font/` | Fonts |
| `Media` | `media/` | Images/videos |
| `I18n` | `i18n/` | Internationalization |
| `Ui` | `ui/` | UI components |
| `Serializer` | `serializer/` | Serializer |
| `Generator` | `generator/` | Code generator |
| `Driven` | `driven/` | AI tasks |
| `Workspace` | `workspace/` | Monorepo |
| `Www` | `www/` | HTIP framework |

### ToolConfig

```rust
pub struct ToolConfig {
    pub enabled: bool,       // Run this tool
    pub parallel: bool,      // Run in parallel with others
    pub cache_enabled: bool, // Enable caching
    pub r2_sync: bool,       // Sync to R2 cloud
    pub timeout_ms: u64,     // Execution timeout
}
```

## Run Demo

```bash
cargo run --example dx_orchestrator_demo -p forge
```

## License

MIT OR Apache-2.0
