Please create a new crate called generate in the crates folder with this planning!!!

Here is the planning:
```markdown
# dx-workspace: Universal Development Environment Generator

This is a fantastic addition to the dx ecosystem! A unified workspace configuration system that works across all editors and cloud IDEs is exactly what developers need. Let me help you design this crate.

## Architecture Overview

```
crates/dx-workspace/
├── Cargo.toml
├── src/
│   ├── lib.rs                    # Public API
│   ├── config/
│   │   ├── mod.rs                # Configuration types
│   │   ├── workspace.rs          # dx-workspace.toml parser
│   │   ├── project.rs            # Project detection (Rust, Node, Python, etc.)
│   │   └── monorepo.rs           # Monorepo configuration
│   │
│   ├── editors/                   # Editor-specific generators
│   │   ├── mod.rs
│   │   ├── vscode.rs             # .vscode/ generation
│   │   ├── jetbrains.rs          # .idea/ generation
│   │   ├── zed.rs                # .zed/ generation
│   │   ├── neovim.rs             # nvim configs
│   │   ├── helix.rs              # helix configs
│   │   └── cursor.rs             # Cursor AI editor
│   │
│   ├── cloud/                     # Cloud IDE generators
│   │   ├── mod.rs
│   │   ├── codespaces.rs         # .devcontainer/
│   │   ├── gitpod.rs             # .gitpod.yml
│   │   ├── codesandbox.rs        # .codesandbox/
│   │   ├── firebase_studio.rs    # Firebase Studio (IDX)
│   │   ├── stackblitz.rs         # StackBlitz WebContainers
│   │   └── replit.rs             # .replit + replit.nix
│   │
│   ├── sync/                      # Configuration synchronization
│   │   ├── mod.rs
│   │   ├── watcher.rs            # File system watcher
│   │   └── generator.rs          # Auto-regeneration
│   │
│   └── binary/                    # Binary format (faster than JSON/YAML)
│       ├── mod.rs
│       └── workspace_format.rs   # .dxw binary workspace cache
│
├── templates/                     # Template files
│   ├── vscode/
│   ├── devcontainer/
│   └── gitpod/
│
└── tests/
    ├── editor_tests.rs
    ├── cloud_tests.rs
    └── monorepo_tests.rs
```

## Core Configuration Format

```toml
# dx-workspace.toml - The single source of truth

[workspace]
name = "my-awesome-project"
version = "1.0.0"
description = "A dx-powered application"

# Project type detection (auto-detected if not specified)
type = "monorepo"  # "single", "monorepo", "hybrid"

[workspace.members]
# Monorepo packages
packages = [
    "apps/*",
    "packages/*",
    "libs/*",
]

# Exclude patterns
exclude = ["**/node_modules", "**/target", "**/.git"]

# ═══════════════════════════════════════════════════════════════
# EDITOR CONFIGURATIONS
# ═══════════════════════════════════════════════════════════════

[editors]
# Which editors to generate configs for
targets = ["vscode", "zed", "cursor", "neovim"]

[editors.vscode]
# VS Code specific settings
extensions = [
    "rust-lang.rust-analyzer",
    "tamasfe.even-better-toml",
    "bradlc.vscode-tailwindcss",
    "esbenp.prettier-vscode",
]

[editors.vscode.settings]
"editor.formatOnSave" = true
"editor.defaultFormatter" = "esbenp.prettier-vscode"
"rust-analyzer.cargo.features" = "all"

[editors.vscode.tasks]
# Custom tasks
[[editors.vscode.tasks.task]]
label = "dx build"
command = "dx"
args = ["build", "--release"]
group = "build"

[editors.zed]
extensions = ["rust", "toml"]

[editors.zed.settings]
"tab_size" = 4
"format_on_save" = "on"

# ═══════════════════════════════════════════════════════════════
# CLOUD IDE CONFIGURATIONS
# ═══════════════════════════════════════════════════════════════

[cloud]
# Which cloud IDEs to support
targets = ["codespaces", "gitpod", "codesandbox", "firebase-studio", "replit"]

[cloud.common]
# Shared settings across all cloud IDEs
node_version = "22"
rust_version = "1.83"
ports = [3000, 5173, 8080]

[cloud.codespaces]
# GitHub Codespaces / Devcontainer
image = "mcr.microsoft.com/devcontainers/rust:1"
features = [
    "ghcr.io/devcontainers/features/node:1",
    "ghcr.io/devcontainers/features/github-cli:1",
]
postCreateCommand = "cargo build && dx setup"

[cloud.gitpod]
# Gitpod configuration
image = "gitpod/workspace-rust"
init = "cargo build"
command = "dx dev"

[[cloud.gitpod.ports]]
port = 3000
visibility = "public"
onOpen = "notify"

[cloud.codesandbox]
# CodeSandbox configuration
template = "rust"
start_command = "dx dev"
compile_command = "dx build"

[cloud.firebase-studio]
# Firebase Studio (IDX) configuration
workspace_type = "custom"
nix_packages = ["rustc", "cargo", "nodejs_22"]
preview_port = 3000

[cloud.replit]
# Replit configuration
language = "rust"
run = "dx dev"
compile = "dx build --release"

[[cloud.replit.nix.deps]]
pkgs = ["rustc", "cargo", "pkg-config", "openssl"]

[cloud.stackblitz]
# StackBlitz WebContainers
start_script = "dev"
install_command = "npm install"

# ═══════════════════════════════════════════════════════════════
# MONOREPO CONFIGURATION (Better than Turborepo!)
# ═══════════════════════════════════════════════════════════════

[monorepo]
# Enable advanced monorepo features
enabled = true

# Caching strategy
[monorepo.cache]
# Content-addressed caching (like Turborepo but faster)
enabled = true
backend = "local"  # "local", "remote", "s3", "gcs"
directory = ".dx-cache"
max_size = "10GB"

# Remote cache for CI/CD
[monorepo.cache.remote]
enabled = false
endpoint = "https://cache.example.com"
team_id = "${DX_TEAM_ID}"
token = "${DX_CACHE_TOKEN}"

# Task pipeline (DAG-based like Turborepo)
[monorepo.pipeline]

[monorepo.pipeline.build]
# Build depends on build of dependencies
depends_on = ["^build"]
outputs = ["target/**", "dist/**", ".next/**"]
cache = true

[monorepo.pipeline.test]
depends_on = ["build"]
outputs = []
cache = true

[monorepo.pipeline.lint]
depends_on = []  # Can run in parallel
outputs = []
cache = true

[monorepo.pipeline.dev]
depends_on = ["^build"]
cache = false
persistent = true  # Long-running task

[monorepo.pipeline.typecheck]
depends_on = ["^build"]
outputs = []
cache = true

# Package-specific overrides
[monorepo.packages."apps/web"]
# Override pipeline for specific package
[monorepo.packages."apps/web".pipeline.build]
outputs = [".next/**"]
env = ["NEXT_PUBLIC_*"]

[monorepo.packages."packages/ui"]
[monorepo.packages."packages/ui".pipeline.build]
outputs = ["dist/**"]

# ═══════════════════════════════════════════════════════════════
# SHARED CONFIGURATIONS
# ═══════════════════════════════════════════════════════════════

[shared]
# Configurations shared across all packages

[shared.typescript]
# Shared TypeScript configuration
strict = true
target = "ES2024"
module = "ESNext"
paths = { "@/*" = ["./src/*"] }

[shared.eslint]
extends = ["@dx/eslint-config"]
rules = { "no-console" = "warn" }

[shared.prettier]
tabWidth = 2
semi = true
singleQuote = true
trailingComma = "all"

[shared.rust]
# Shared Rust configuration
edition = "2024"
lto = true
codegen_units = 1

[shared.env]
# Environment variables (encrypted secrets support)
NODE_ENV = "development"
RUST_LOG = "info"

[shared.env.secrets]
# These are encrypted and stored in .dx-secrets
DATABASE_URL = "encrypted:..."
API_KEY = "encrypted:..."

# ═══════════════════════════════════════════════════════════════
# DX-SPECIFIC FEATURES
# ═══════════════════════════════════════════════════════════════

[dx]
# dx framework specific settings
runtime = "dx-js-runtime"
bundler = "dx-js-bundler"
style = "dx-style"

[dx.build]
minify = true
sourcemap = true
target = ["chrome100", "firefox100", "safari16"]

[dx.dev]
port = 3000
hot_reload = true
open_browser = true
```

## Core Implementation

### lib.rs - Public API

```rust
//! dx-workspace: Universal Development Environment Generator
//!
//! Generate configurations for any editor or cloud IDE from a single source of truth.
//! Superior monorepo orchestration with binary caching.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod config;
pub mod editors;
pub mod cloud;
pub mod monorepo;
pub mod sync;
pub mod binary;

use std::path::{Path, PathBuf};
use thiserror::Error;

/// Result type for dx-workspace operations
pub type Result<T> = std::result::Result<T, WorkspaceError>;

/// Errors that can occur during workspace operations
#[derive(Error, Debug)]
pub enum WorkspaceError {
    #[error("Configuration file not found: {0}")]
    ConfigNotFound(PathBuf),
    
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("TOML parse error: {0}")]
    TomlParse(#[from] toml::de::Error),
    
    #[error("Circular dependency detected in package: {0}")]
    CircularDependency(String),
    
    #[error("Package not found: {0}")]
    PackageNotFound(String),
    
    #[error("Task failed: {0}")]
    TaskFailed(String),
    
    #[error("Cache error: {0}")]
    CacheError(String),
}

/// Main workspace manager
pub struct Workspace {
    /// Root directory of the workspace
    root: PathBuf,
    /// Parsed configuration
    config: config::WorkspaceConfig,
    /// Dependency graph for monorepo
    graph: Option<monorepo::DependencyGraph>,
    /// Build cache
    cache: Option<monorepo::BuildCache>,
}

impl Workspace {
    /// Load a workspace from a directory
    ///
    /// Searches for `dx-workspace.toml` in the given directory and parent directories.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let config_path = Self::find_config(path)?;
        let root = config_path.parent().unwrap().to_path_buf();
        
        let config_content = std::fs::read_to_string(&config_path)?;
        let config: config::WorkspaceConfig = toml::from_str(&config_content)?;
        
        let graph = if config.monorepo.enabled {
            Some(monorepo::DependencyGraph::build(&root, &config)?)
        } else {
            None
        };
        
        let cache = if config.monorepo.cache.enabled {
            Some(monorepo::BuildCache::new(&root, &config.monorepo.cache)?)
        } else {
            None
        };
        
        Ok(Self { root, config, graph, cache })
    }
    
    /// Initialize a new workspace in the given directory
    pub fn init<P: AsRef<Path>>(path: P, template: WorkspaceTemplate) -> Result<Self> {
        let path = path.as_ref();
        std::fs::create_dir_all(path)?;
        
        let config = template.to_config();
        let config_content = toml::to_string_pretty(&config)
            .map_err(|e| WorkspaceError::InvalidConfig(e.to_string()))?;
        
        std::fs::write(path.join("dx-workspace.toml"), config_content)?;
        
        Self::load(path)
    }
    
    /// Generate all editor configurations
    pub fn generate_editor_configs(&self) -> Result<GenerationReport> {
        let mut report = GenerationReport::default();
        
        for target in &self.config.editors.targets {
            match target.as_str() {
                "vscode" => {
                    let files = editors::vscode::generate(&self.root, &self.config)?;
                    report.add_files("vscode", files);
                }
                "zed" => {
                    let files = editors::zed::generate(&self.root, &self.config)?;
                    report.add_files("zed", files);
                }
                "cursor" => {
                    let files = editors::cursor::generate(&self.root, &self.config)?;
                    report.add_files("cursor", files);
                }
                "neovim" => {
                    let files = editors::neovim::generate(&self.root, &self.config)?;
                    report.add_files("neovim", files);
                }
                "helix" => {
                    let files = editors::helix::generate(&self.root, &self.config)?;
                    report.add_files("helix", files);
                }
                "jetbrains" => {
                    let files = editors::jetbrains::generate(&self.root, &self.config)?;
                    report.add_files("jetbrains", files);
                }
                _ => {
                    report.add_warning(format!("Unknown editor target: {}", target));
                }
            }
        }
        
        Ok(report)
    }
    
    /// Generate all cloud IDE configurations
    pub fn generate_cloud_configs(&self) -> Result<GenerationReport> {
        let mut report = GenerationReport::default();
        
        for target in &self.config.cloud.targets {
            match target.as_str() {
                "codespaces" | "devcontainer" => {
                    let files = cloud::codespaces::generate(&self.root, &self.config)?;
                    report.add_files("codespaces", files);
                }
                "gitpod" => {
                    let files = cloud::gitpod::generate(&self.root, &self.config)?;
                    report.add_files("gitpod", files);
                }
                "codesandbox" => {
                    let files = cloud::codesandbox::generate(&self.root, &self.config)?;
                    report.add_files("codesandbox", files);
                }
                "firebase-studio" | "idx" => {
                    let files = cloud::firebase_studio::generate(&self.root, &self.config)?;
                    report.add_files("firebase-studio", files);
                }
                "replit" => {
                    let files = cloud::replit::generate(&self.root, &self.config)?;
                    report.add_files("replit", files);
                }
                "stackblitz" => {
                    let files = cloud::stackblitz::generate(&self.root, &self.config)?;
                    report.add_files("stackblitz", files);
                }
                _ => {
                    report.add_warning(format!("Unknown cloud target: {}", target));
                }
            }
        }
        
        Ok(report)
    }
    
    /// Generate all configurations (editors + cloud)
    pub fn generate_all(&self) -> Result<GenerationReport> {
        let mut report = self.generate_editor_configs()?;
        let cloud_report = self.generate_cloud_configs()?;
        report.merge(cloud_report);
        Ok(report)
    }
    
    /// Run a task in the monorepo (like turborepo)
    pub async fn run_task(&self, task_name: &str, options: TaskOptions) -> Result<TaskResult> {
        let graph = self.graph.as_ref()
            .ok_or_else(|| WorkspaceError::InvalidConfig("Monorepo not enabled".into()))?;
        
        let scheduler = monorepo::Scheduler::new(graph, self.cache.as_ref());
        scheduler.run(task_name, options).await
    }
    
    /// Get the dependency graph
    pub fn dependency_graph(&self) -> Option<&monorepo::DependencyGraph> {
        self.graph.as_ref()
    }
    
    /// Watch for configuration changes and auto-regenerate
    pub async fn watch(&self) -> Result<()> {
        sync::Watcher::new(&self.root, &self.config)?.watch().await
    }
    
    fn find_config(start_path: &Path) -> Result<PathBuf> {
        let mut current = start_path.to_path_buf();
        
        loop {
            let config_path = current.join("dx-workspace.toml");
            if config_path.exists() {
                return Ok(config_path);
            }
            
            if !current.pop() {
                return Err(WorkspaceError::ConfigNotFound(
                    start_path.join("dx-workspace.toml")
                ));
            }
        }
    }
}

/// Template for initializing a new workspace
#[derive(Debug, Clone, Copy)]
pub enum WorkspaceTemplate {
    /// Single package Rust project
    RustSingle,
    /// Rust workspace (multiple crates)
    RustWorkspace,
    /// Node.js monorepo
    NodeMonorepo,
    /// Full-stack dx project
    DxFullStack,
    /// Minimal configuration
    Minimal,
}

impl WorkspaceTemplate {
    fn to_config(self) -> config::WorkspaceConfig {
        match self {
            Self::RustSingle => config::templates::rust_single(),
            Self::RustWorkspace => config::templates::rust_workspace(),
            Self::NodeMonorepo => config::templates::node_monorepo(),
            Self::DxFullStack => config::templates::dx_fullstack(),
            Self::Minimal => config::templates::minimal(),
        }
    }
}

/// Options for running tasks
#[derive(Debug, Clone, Default)]
pub struct TaskOptions {
    /// Only run for specific packages
    pub filter: Option<Vec<String>>,
    /// Skip packages matching pattern
    pub ignore: Option<Vec<String>>,
    /// Maximum parallelism (default: CPU count)
    pub concurrency: Option<usize>,
    /// Continue on error
    pub continue_on_error: bool,
    /// Force run (ignore cache)
    pub force: bool,
    /// Dry run (show what would be executed)
    pub dry_run: bool,
    /// Output mode
    pub output: OutputMode,
}

/// Output mode for task execution
#[derive(Debug, Clone, Copy, Default)]
pub enum OutputMode {
    /// Stream output as it happens
    #[default]
    Stream,
    /// Buffer and show on completion
    Buffered,
    /// Only show errors
    ErrorsOnly,
    /// No output
    Silent,
}

/// Result of task execution
#[derive(Debug)]
pub struct TaskResult {
    /// Tasks that succeeded
    pub succeeded: Vec<String>,
    /// Tasks that failed
    pub failed: Vec<TaskFailure>,
    /// Tasks that were skipped (cached)
    pub cached: Vec<String>,
    /// Total execution time
    pub duration: std::time::Duration,
}

/// Information about a failed task
#[derive(Debug)]
pub struct TaskFailure {
    /// Package name
    pub package: String,
    /// Task name
    pub task: String,
    /// Exit code
    pub exit_code: Option<i32>,
    /// Error message
    pub error: String,
}

/// Report of generated files
#[derive(Debug, Default)]
pub struct GenerationReport {
    /// Files generated per target
    pub files: std::collections::HashMap<String, Vec<PathBuf>>,
    /// Warnings during generation
    pub warnings: Vec<String>,
}

impl GenerationReport {
    fn add_files(&mut self, target: &str, files: Vec<PathBuf>) {
        self.files.insert(target.to_string(), files);
    }
    
    fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }
    
    fn merge(&mut self, other: Self) {
        for (key, value) in other.files {
            self.files.insert(key, value);
        }
        self.warnings.extend(other.warnings);
    }
    
    /// Get total number of files generated
    pub fn total_files(&self) -> usize {
        self.files.values().map(|v| v.len()).sum()
    }
}
```

### VS Code Generator

```rust
//! VS Code configuration generator

use crate::{config::WorkspaceConfig, Result};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};

/// Generate VS Code configuration files
pub fn generate(root: &Path, config: &WorkspaceConfig) -> Result<Vec<PathBuf>> {
    let vscode_dir = root.join(".vscode");
    std::fs::create_dir_all(&vscode_dir)?;
    
    let mut files = Vec::new();
    
    // settings.json
    files.push(generate_settings(&vscode_dir, config)?);
    
    // extensions.json
    files.push(generate_extensions(&vscode_dir, config)?);
    
    // tasks.json
    files.push(generate_tasks(&vscode_dir, config)?);
    
    // launch.json
    files.push(generate_launch(&vscode_dir, config)?);
    
    // Multi-root workspace file (for monorepos)
    if config.monorepo.enabled {
        files.push(generate_workspace_file(root, config)?);
    }
    
    Ok(files)
}

fn generate_settings(vscode_dir: &Path, config: &WorkspaceConfig) -> Result<PathBuf> {
    let vscode_config = &config.editors.vscode;
    
    let mut settings: Value = json!({
        // Default dx settings
        "editor.formatOnSave": true,
        "editor.tabSize": 2,
        "files.trimTrailingWhitespace": true,
        "files.insertFinalNewline": true,
        
        // Rust settings
        "rust-analyzer.cargo.features": "all",
        "rust-analyzer.checkOnSave.command": "clippy",
        "rust-analyzer.inlayHints.chainingHints.enable": true,
        "rust-analyzer.inlayHints.parameterHints.enable": true,
        
        // File associations
        "files.associations": {
            "*.dxw": "toml",
            "dx-workspace.toml": "toml",
            "*.tsx": "typescriptreact"
        },
        
        // Search exclusions (for monorepos)
        "search.exclude": {
            "**/node_modules": true,
            "**/target": true,
            "**/.dx-cache": true,
            "**/dist": true
        },
        
        // File watcher exclusions
        "files.watcherExclude": {
            "**/target/**": true,
            "**/node_modules/**": true,
            "**/.dx-cache/**": true
        }
    });
    
    // Merge user settings
    if let Some(obj) = settings.as_object_mut() {
        for (key, value) in &vscode_config.settings {
            obj.insert(key.clone(), value.clone());
        }
    }
    
    // Add shared configurations
    if let Some(shared) = &config.shared {
        if let Some(prettier) = &shared.prettier {
            if let Some(obj) = settings.as_object_mut() {
                obj.insert("prettier.tabWidth".into(), json!(prettier.tab_width));
                obj.insert("prettier.semi".into(), json!(prettier.semi));
                obj.insert("prettier.singleQuote".into(), json!(prettier.single_quote));
            }
        }
    }
    
    let path = vscode_dir.join("settings.json");
    write_json(&path, &settings)?;
    Ok(path)
}

fn generate_extensions(vscode_dir: &Path, config: &WorkspaceConfig) -> Result<PathBuf> {
    let vscode_config = &config.editors.vscode;
    
    // Default recommended extensions
    let mut recommendations = vec![
        "rust-lang.rust-analyzer".to_string(),
        "tamasfe.even-better-toml".to_string(),
        "serayuzgur.crates".to_string(),
        "usernamehw.errorlens".to_string(),
    ];
    
    // Add TypeScript extensions if needed
    if has_typescript_files(&config) {
        recommendations.extend([
            "dbaeumer.vscode-eslint".to_string(),
            "esbenp.prettier-vscode".to_string(),
        ]);
    }
    
    // Add user-specified extensions
    recommendations.extend(vscode_config.extensions.clone());
    
    // Deduplicate
    recommendations.sort();
    recommendations.dedup();
    
    let extensions = json!({
        "recommendations": recommendations,
        "unwantedRecommendations": []
    });
    
    let path = vscode_dir.join("extensions.json");
    write_json(&path, &extensions)?;
    Ok(path)
}

fn generate_tasks(vscode_dir: &Path, config: &WorkspaceConfig) -> Result<PathBuf> {
    let mut tasks_list = vec![
        json!({
            "label": "dx build",
            "type": "shell",
            "command": "dx",
            "args": ["build"],
            "group": "build",
            "problemMatcher": ["$rustc"]
        }),
        json!({
            "label": "dx dev",
            "type": "shell",
            "command": "dx",
            "args": ["dev"],
            "group": "none",
            "isBackground": true,
            "problemMatcher": []
        }),
        json!({
            "label": "dx test",
            "type": "shell",
            "command": "dx",
            "args": ["test"],
            "group": "test",
            "problemMatcher": ["$rustc"]
        }),
        json!({
            "label": "cargo check",
            "type": "shell",
            "command": "cargo",
            "args": ["check", "--workspace"],
            "group": "build",
            "problemMatcher": ["$rustc"]
        }),
    ];
    
    // Add monorepo tasks
    if config.monorepo.enabled {
        for (task_name, _) in &config.monorepo.pipeline {
            tasks_list.push(json!({
                "label": format!("dx run {}", task_name),
                "type": "shell",
                "command": "dx",
                "args": ["run", task_name],
                "group": if task_name == "build" { "build" } else { "none" },
                "problemMatcher": []
            }));
        }
    }
    
    // Add user-defined tasks
    for task in &config.editors.vscode.tasks {
        tasks_list.push(task.clone());
    }
    
    let tasks = json!({
        "version": "2.0.0",
        "tasks": tasks_list
    });
    
    let path = vscode_dir.join("tasks.json");
    write_json(&path, &tasks)?;
    Ok(path)
}

fn generate_launch(vscode_dir: &Path, config: &WorkspaceConfig) -> Result<PathBuf> {
    let launch = json!({
        "version": "0.2.0",
        "configurations": [
            {
                "name": "Debug Binary",
                "type": "lldb",
                "request": "launch",
                "program": "${workspaceFolder}/target/debug/${workspaceFolderBasename}",
                "args": [],
                "cwd": "${workspaceFolder}",
                "preLaunchTask": "cargo check"
            },
            {
                "name": "Debug Tests",
                "type": "lldb",
                "request": "launch",
                "cargo": {
                    "args": ["test", "--no-run", "--lib"],
                    "filter": {
                        "kind": "lib"
                    }
                },
                "cwd": "${workspaceFolder}"
            },
            {
                "name": "dx-js-runtime Debug",
                "type": "lldb",
                "request": "launch",
                "program": "${workspaceFolder}/target/debug/dx-js-runtime",
                "args": ["${file}"],
                "cwd": "${workspaceFolder}"
            }
        ]
    });
    
    let path = vscode_dir.join("launch.json");
    write_json(&path, &launch)?;
    Ok(path)
}

fn generate_workspace_file(root: &Path, config: &WorkspaceConfig) -> Result<PathBuf> {
    let mut folders = vec![json!({"path": "."})];
    
    // Add package folders for better navigation
    for pattern in &config.workspace.members.packages {
        if let Some(base) = pattern.strip_suffix("/*") {
            folders.push(json!({
                "name": format!("📦 {}", base),
                "path": base
            }));
        }
    }
    
    let workspace = json!({
        "folders": folders,
        "settings": {
            "files.exclude": {
                "target": true,
                "node_modules": true
            }
        }
    });
    
    let path = root.join(format!("{}.code-workspace", config.workspace.name));
    write_json(&path, &workspace)?;
    Ok(path)
}

fn write_json(path: &Path, value: &Value) -> Result<()> {
    let content = serde_json::to_string_pretty(value)
        .map_err(|e| crate::WorkspaceError::InvalidConfig(e.to_string()))?;
    std::fs::write(path, content)?;
    Ok(())
}

fn has_typescript_files(config: &WorkspaceConfig) -> bool {
    // Heuristic: check if any dx-specific TypeScript features are enabled
    config.dx.is_some()
}
```

### GitHub Codespaces / Devcontainer Generator

```rust
//! GitHub Codespaces and devcontainer configuration generator

use crate::{config::WorkspaceConfig, Result};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};

/// Generate devcontainer configuration
pub fn generate(root: &Path, config: &WorkspaceConfig) -> Result<Vec<PathBuf>> {
    let devcontainer_dir = root.join(".devcontainer");
    std::fs::create_dir_all(&devcontainer_dir)?;
    
    let mut files = Vec::new();
    
    // devcontainer.json
    files.push(generate_devcontainer_json(&devcontainer_dir, config)?);
    
    // Dockerfile (if custom)
    if config.cloud.codespaces.dockerfile.is_some() {
        files.push(generate_dockerfile(&devcontainer_dir, config)?);
    }
    
    // docker-compose.yml (for complex setups)
    if needs_compose(config) {
        files.push(generate_compose(&devcontainer_dir, config)?);
    }
    
    Ok(files)
}

fn generate_devcontainer_json(devcontainer_dir: &Path, config: &WorkspaceConfig) -> Result<PathBuf> {
    let codespaces = &config.cloud.codespaces;
    let common = &config.cloud.common;
    
    // Base image selection
    let image = codespaces.image.as_deref()
        .unwrap_or("mcr.microsoft.com/devcontainers/rust:1-bookworm");
    
    let mut devcontainer = json!({
        "name": &config.workspace.name,
        "image": image,
        
        // Features (dev container features)
        "features": {
            // Node.js
            "ghcr.io/devcontainers/features/node:1": {
                "version": &common.node_version
            },
            // GitHub CLI
            "ghcr.io/devcontainers/features/github-cli:1": {},
            // Common utilities
            "ghcr.io/devcontainers/features/common-utils:2": {
                "installZsh": true,
                "configureZshAsDefaultShell": true
            }
        },
        
        // VS Code customizations
        "customizations": {
            "vscode": {
                "extensions": [
                    "rust-lang.rust-analyzer",
                    "tamasfe.even-better-toml",
                    "serayuzgur.crates",
                    "usernamehw.errorlens",
                    "fill-labs.dependi"
                ],
                "settings": {
                    "rust-analyzer.cargo.features": "all",
                    "rust-analyzer.checkOnSave.command": "clippy",
                    "terminal.integrated.defaultProfile.linux": "zsh"
                }
            },
            "codespaces": {
                "openFiles": ["README.md", "dx-workspace.toml"]
            }
        },
        
        // Port forwarding
        "forwardPorts": &common.ports,
        "portsAttributes": generate_port_attributes(&common.ports),
        
        // Lifecycle scripts
        "postCreateCommand": codespaces.post_create_command.as_deref()
            .unwrap_or("cargo build && echo '✅ Development environment ready!'"),
        "postStartCommand": "echo '🚀 dx workspace started'",
        
        // Container settings
        "containerEnv": {
            "RUST_BACKTRACE": "1",
            "CARGO_TERM_COLOR": "always"
        },
        
        // Performance optimizations
        "mounts": [
            // Persistent cargo cache
            "source=dx-cargo-cache,target=/usr/local/cargo/registry,type=volume",
            // Persistent target cache
            "source=${localWorkspaceFolderBasename}-target,target=${containerWorkspaceFolder}/target,type=volume"
        ],
        
        // Host requirements
        "hostRequirements": {
            "cpus": 4,
            "memory": "8gb",
            "storage": "32gb"
        }
    });
    
    // Add user-specified features
    for feature in &codespaces.features {
        if let Some(obj) = devcontainer.get_mut("features").and_then(|f| f.as_object_mut()) {
            obj.insert(feature.clone(), json!({}));
        }
    }
    
    let path = devcontainer_dir.join("devcontainer.json");
    let content = serde_json::to_string_pretty(&devcontainer)
        .map_err(|e| crate::WorkspaceError::InvalidConfig(e.to_string()))?;
    std::fs::write(&path, content)?;
    Ok(path)
}

fn generate_port_attributes(ports: &[u16]) -> Value {
    let mut attrs = serde_json::Map::new();
    
    for port in ports {
        let label = match port {
            3000 => "Web App",
            5173 => "Vite Dev Server",
            8080 => "API Server",
            5432 => "PostgreSQL",
            6379 => "Redis",
            _ => "Service",
        };
        
        attrs.insert(
            port.to_string(),
            json!({
                "label": label,
                "onAutoForward": "notify"
            })
        );
    }
    
    Value::Object(attrs)
}

fn generate_dockerfile(devcontainer_dir: &Path, config: &WorkspaceConfig) -> Result<PathBuf> {
    let codespaces = &config.cloud.codespaces;
    let common = &config.cloud.common;
    
    let dockerfile = format!(r#"# dx development container
FROM mcr.microsoft.com/devcontainers/rust:1-bookworm

# Install additional tools
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    cmake \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js
RUN curl -fsSL https://deb.nodesource.com/setup_{node_version}.x | bash - \
    && apt-get install -y nodejs

# Install dx CLI
RUN cargo install dx-cli || true

# Install wasm target
RUN rustup target add wasm32-unknown-unknown

# Pre-build dependencies for faster startup
WORKDIR /workspace
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main() {{}}' > src/main.rs && cargo build --release && rm -rf src target

EXPOSE {ports}

CMD ["zsh"]
"#,
        node_version = common.node_version,
        ports = common.ports.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(" ")
    );
    
    let path = devcontainer_dir.join("Dockerfile");
    std::fs::write(&path, dockerfile)?;
    Ok(path)
}

fn generate_compose(devcontainer_dir: &Path, config: &WorkspaceConfig) -> Result<PathBuf> {
    let compose = format!(r#"version: '3.8'

services:
  app:
    build:
      context: ..
      dockerfile: .devcontainer/Dockerfile
    volumes:
      - ..:/workspace:cached
      - dx-cargo-cache:/usr/local/cargo/registry
      - dx-target-cache:/workspace/target
    ports:
      - "3000:3000"
      - "5173:5173"
      - "8080:8080"
    environment:
      - RUST_BACKTRACE=1
      - CARGO_TERM_COLOR=always
    command: sleep infinity

  # Database for development
  postgres:
    image: postgres:16
    restart: unless-stopped
    volumes:
      - postgres-data:/var/lib/postgresql/data
    environment:
      POSTGRES_USER: dx
      POSTGRES_PASSWORD: dx_dev_password
      POSTGRES_DB: dx_dev
    ports:
      - "5432:5432"

  # Redis for caching
  redis:
    image: redis:7-alpine
    restart: unless-stopped
    ports:
      - "6379:6379"

volumes:
  dx-cargo-cache:
  dx-target-cache:
  postgres-data:
"#);
    
    let path = devcontainer_dir.join("docker-compose.yml");
    std::fs::write(&path, compose)?;
    Ok(path)
}

fn needs_compose(config: &WorkspaceConfig) -> bool {
    // Use compose if the project needs database or other services
    config.cloud.codespaces.use_compose.unwrap_or(false)
}
```

### Monorepo Scheduler (Better than Turborepo!)

```rust
//! Monorepo task scheduler with content-addressed caching
//!
//! Key advantages over Turborepo:
//! - Binary caching (faster serialization)
//! - Rust-native parallelism (tokio)
//! - SIMD-accelerated hashing (Blake3)
//! - Zero-copy cache retrieval

use crate::{config::WorkspaceConfig, Result, TaskOptions, TaskResult, TaskFailure, WorkspaceError};
use blake3::Hasher;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::algo::toposort;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::{Semaphore, RwLock};

/// Dependency graph for monorepo packages
pub struct DependencyGraph {
    /// Package name -> Node index
    nodes: HashMap<String, NodeIndex>,
    /// The actual graph
    graph: DiGraph<Package, ()>,
    /// Topological order (cached)
    topo_order: Vec<NodeIndex>,
}

#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub path: PathBuf,
    pub dependencies: Vec<String>,
    pub dev_dependencies: Vec<String>,
}

impl DependencyGraph {
    /// Build the dependency graph from workspace configuration
    pub fn build(root: &Path, config: &WorkspaceConfig) -> Result<Self> {
        let mut graph = DiGraph::new();
        let mut nodes = HashMap::new();
        
        // Discover all packages
        let packages = discover_packages(root, &config.workspace.members.packages)?;
        
        // Create nodes
        for package in &packages {
            let idx = graph.add_node(package.clone());
            nodes.insert(package.name.clone(), idx);
        }
        
        // Create edges (dependencies)
        for package in &packages {
            let from_idx = nodes[&package.name];
            
            for dep in &package.dependencies {
                if let Some(&to_idx) = nodes.get(dep) {
                    graph.add_edge(from_idx, to_idx, ());
                }
            }
        }
        
        // Compute topological order (detects cycles)
        let topo_order = toposort(&graph, None)
            .map_err(|cycle| {
                let pkg = &graph[cycle.node_id()];
                WorkspaceError::CircularDependency(pkg.name.clone())
            })?;
        
        Ok(Self { nodes, graph, topo_order })
    }
    
    /// Get packages in build order
    pub fn build_order(&self) -> Vec<&Package> {
        self.topo_order.iter()
            .rev() // Dependencies first
            .map(|&idx| &self.graph[idx])
            .collect()
    }
    
    /// Get packages that depend on the given package
    pub fn dependents(&self, package: &str) -> Vec<&Package> {
        if let Some(&idx) = self.nodes.get(package) {
            self.graph.neighbors_directed(idx, petgraph::Direction::Incoming)
                .map(|idx| &self.graph[idx])
                .collect()
        } else {
            vec![]
        }
    }
    
    /// Get packages that the given package depends on
    pub fn dependencies(&self, package: &str) -> Vec<&Package> {
        if let Some(&idx) = self.nodes.get(package) {
            self.graph.neighbors_directed(idx, petgraph::Direction::Outgoing)
                .map(|idx| &self.graph[idx])
                .collect()
        } else {
            vec![]
        }
    }
}

/// Content-addressed build cache (like Turborepo but faster)
pub struct BuildCache {
    /// Cache directory
    cache_dir: PathBuf,
    /// In-memory index for O(1) lookups
    index: RwLock<HashMap<CacheKey, CacheEntry>>,
    /// Maximum cache size
    max_size: u64,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct CacheKey {
    /// Package name
    pub package: String,
    /// Task name
    pub task: String,
    /// Content hash of inputs
    pub input_hash: [u8; 32],
}

#[derive(Debug, Clone)]
pub struct CacheEntry {
    /// Path to cached outputs
    pub outputs: Vec<PathBuf>,
    /// When this was cached
    pub timestamp: std::time::SystemTime,
    /// Size in bytes
    pub size: u64,
}

impl BuildCache {
    pub fn new(root: &Path, config: &crate::config::CacheConfig) -> Result<Self> {
        let cache_dir = root.join(&config.directory);
        std::fs::create_dir_all(&cache_dir)?;
        
        // Load existing index
        let index_path = cache_dir.join("index.dxc");
        let index = if index_path.exists() {
            load_cache_index(&index_path)?
        } else {
            HashMap::new()
        };
        
        Ok(Self {
            cache_dir,
            index: RwLock::new(index),
            max_size: parse_size(&config.max_size),
        })
    }
    
    /// Check if a task result is cached
    pub async fn get(&self, key: &CacheKey) -> Option<CacheEntry> {
        let index = self.index.read().await;
        index.get(key).cloned()
    }
    
    /// Store task outputs in cache
    pub async fn put(&self, key: CacheKey, outputs: Vec<PathBuf>) -> Result<()> {
        // Calculate hash of outputs
        let hash_str = hex::encode(&key.input_hash[..8]);
        let cache_path = self.cache_dir.join(&key.package).join(&key.task).join(&hash_str);
        std::fs::create_dir_all(&cache_path)?;
        
        // Copy outputs to cache
        let mut total_size = 0u64;
        for output in &outputs {
            if output.exists() {
                let dest = cache_path.join(output.file_name().unwrap());
                if output.is_dir() {
                    copy_dir_recursive(output, &dest)?;
                } else {
                    std::fs::copy(output, &dest)?;
                }
                total_size += dir_size(&dest);
            }
        }
        
        // Update index
        let entry = CacheEntry {
            outputs: outputs.clone(),
            timestamp: std::time::SystemTime::now(),
            size: total_size,
        };
        
        let mut index = self.index.write().await;
        index.insert(key, entry);
        
        // Persist index
        save_cache_index(&self.cache_dir.join("index.dxc"), &index)?;
        
        Ok(())
    }
    
    /// Restore cached outputs
    pub async fn restore(&self, key: &CacheKey, target_dir: &Path) -> Result<bool> {
        let entry = match self.get(key).await {
            Some(e) => e,
            None => return Ok(false),
        };
        
        let hash_str = hex::encode(&key.input_hash[..8]);
        let cache_path = self.cache_dir.join(&key.package).join(&key.task).join(&hash_str);
        
        if !cache_path.exists() {
            return Ok(false);
        }
        
        // Restore outputs
        for output in &entry.outputs {
            let cached = cache_path.join(output.file_name().unwrap());
            let dest = target_dir.join(output);
            
            if cached.exists() {
                if let Some(parent) = dest.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                if cached.is_dir() {
                    copy_dir_recursive(&cached, &dest)?;
                } else {
                    std::fs::copy(&cached, &dest)?;
                }
            }
        }
        
        Ok(true)
    }
    
    /// Compute input hash for a package/task combination
    pub fn compute_hash(&self, package: &Package, task: &str, pipeline: &crate::config::PipelineTask) -> [u8; 32] {
        let mut hasher = Hasher::new();
        
        // Hash package name and task
        hasher.update(package.name.as_bytes());
        hasher.update(task.as_bytes());
        
        // Hash all source files
        if let Ok(files) = glob_files(&package.path, &["src/**/*", "Cargo.toml", "package.json"]) {
            for file in files {
                if let Ok(content) = std::fs::read(&file) {
                    hasher.update(&content);
                }
            }
        }
        
        // Hash environment variables that affect the build
        for env_var in &pipeline.env {
            if let Ok(value) = std::env::var(env_var.trim_end_matches('*')) {
                hasher.update(env_var.as_bytes());
                hasher.update(value.as_bytes());
            }
        }
        
        *hasher.finalize().as_bytes()
    }
}

/// Task scheduler for parallel execution
pub struct Scheduler<'a> {
    graph: &'a DependencyGraph,
    cache: Option<&'a BuildCache>,
}

impl<'a> Scheduler<'a> {
    pub fn new(graph: &'a DependencyGraph, cache: Option<&'a BuildCache>) -> Self {
        Self { graph, cache }
    }
    
    /// Run a task across all packages (respecting dependencies)
    pub async fn run(&self, task: &str, options: TaskOptions) -> Result<TaskResult> {
        let start = std::time::Instant::now();
        
        let concurrency = options.concurrency.unwrap_or_else(num_cpus::get);
        let semaphore = Arc::new(Semaphore::new(concurrency));
        
        let packages = self.graph.build_order();
        let filter = options.filter.as_ref();
        let ignore = options.ignore.as_ref();
        
        let succeeded = Arc::new(RwLock::new(Vec::new()));
        let failed = Arc::new(RwLock::new(Vec::new()));
        let cached = Arc::new(RwLock::new(Vec::new()));
        
        // Track completed packages for dependency checking
        let completed: Arc<RwLock<HashSet<String>>> = Arc::new(RwLock::new(HashSet::new()));
        
        let mut handles = Vec::new();
        
        for package in packages {
            // Apply filters
            if let Some(filter) = filter {
                if !filter.iter().any(|f| package.name.contains(f)) {
                    continue;
                }
            }
            
            if let Some(ignore) = ignore {
                if ignore.iter().any(|i| package.name.contains(i)) {
                    continue;
                }
            }
            
            let package = package.clone();
            let task = task.to_string();
            let semaphore = Arc::clone(&semaphore);
            let completed = Arc::clone(&completed);
            let succeeded = Arc::clone(&succeeded);
            let failed = Arc::clone(&failed);
            let cached = Arc::clone(&cached);
            let force = options.force;
            let dry_run = options.dry_run;
            
            let handle = tokio::spawn(async move {
                // Wait for dependencies to complete
                loop {
                    let completed_read = completed.read().await;
                    let deps_complete = package.dependencies.iter()
                        .all(|dep| completed_read.contains(dep));
                    drop(completed_read);
                    
                    if deps_complete {
                        break;
                    }
                    
                    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                }
                
                // Acquire semaphore permit
                let _permit = semaphore.acquire().await.unwrap();
                
                // Check cache (unless force)
                if !force && !dry_run {
                    // TODO: Check cache here
                }
                
                // Dry run mode
                if dry_run {
                    println!("Would run: {} in {}", task, package.name);
                    completed.write().await.insert(package.name.clone());
                    succeeded.write().await.push(package.name.clone());
                    return;
                }
                
                // Execute task
                println!("▶ {}:{}", package.name, task);
                
                let result = run_task_in_package(&package, &task).await;
                
                match result {
                    Ok(_) => {
                        println!("✓ {}:{}", package.name, task);
                        succeeded.write().await.push(package.name.clone());
                    }
                    Err(e) => {
                        println!("✗ {}:{} - {}", package.name, task, e);
                        failed.write().await.push(TaskFailure {
                            package: package.name.clone(),
                            task: task.clone(),
                            exit_code: None,
                            error: e.to_string(),
                        });
                    }
                }
                
                completed.write().await.insert(package.name);
            });
            
            handles.push(handle);
        }
        
        // Wait for all tasks
        for handle in handles {
            handle.await.map_err(|e| WorkspaceError::TaskFailed(e.to_string()))?;
        }
        
        Ok(TaskResult {
            succeeded: Arc::try_unwrap(succeeded).unwrap().into_inner(),
            failed: Arc::try_unwrap(failed).unwrap().into_inner(),
            cached: Arc::try_unwrap(cached).unwrap().into_inner(),
            duration: start.elapsed(),
        })
    }
}

async fn run_task_in_package(package: &Package, task: &str) -> Result<()> {
    use tokio::process::Command;
    
    // Determine the command based on package type
    let (cmd, args) = if package.path.join("Cargo.toml").exists() {
        // Rust package
        match task {
            "build" => ("cargo", vec!["build", "--release"]),
            "test" => ("cargo", vec!["test"]),
            "lint" => ("cargo", vec!["clippy", "--", "-D", "warnings"]),
            "check" => ("cargo", vec!["check"]),
            _ => ("dx", vec!["run", task]),
        }
    } else if package.path.join("package.json").exists() {
        // Node package
        match task {
            "build" => ("npm", vec!["run", "build"]),
            "test" => ("npm", vec!["test"]),
            "lint" => ("npm", vec!["run", "lint"]),
            "dev" => ("npm", vec!["run", "dev"]),
            _ => ("npm", vec!["run", task]),
        }
    } else {
        // Generic
        ("dx", vec!["run", task])
    };
    
    let status = Command::new(cmd)
        .args(&args)
        .current_dir(&package.path)
        .status()
        .await?;
    
    if status.success() {
        Ok(())
    } else {
        Err(WorkspaceError::TaskFailed(format!(
            "Command exited with status: {}",
            status.code().unwrap_or(-1)
        )))
    }
}

// Helper functions
fn discover_packages(root: &Path, patterns: &[String]) -> Result<Vec<Package>> {
    let mut packages = Vec::new();
    
    for pattern in patterns {
        for entry in glob::glob(&root.join(pattern).to_string_lossy())
            .map_err(|e| WorkspaceError::InvalidConfig(e.to_string()))?
        {
            let path = entry.map_err(|e| WorkspaceError::Io(e.into_error()))?;
            
            if let Some(package) = parse_package(&path)? {
                packages.push(package);
            }
        }
    }
    
    Ok(packages)
}

fn parse_package(path: &Path) -> Result<Option<Package>> {
    // Check for Cargo.toml
    let cargo_toml = path.join("Cargo.toml");
    if cargo_toml.exists() {
        let content = std::fs::read_to_string(&cargo_toml)?;
        let doc: toml::Value = toml::from_str(&content)?;
        
        let name = doc.get("package")
            .and_then(|p| p.get("name"))
            .and_then(|n| n.as_str())
            .unwrap_or_else(|| path.file_name().unwrap().to_str().unwrap());
        
        let dependencies = doc.get("dependencies")
            .and_then(|d| d.as_table())
            .map(|t| t.keys().cloned().collect())
            .unwrap_or_default();
        
        return Ok(Some(Package {
            name: name.to_string(),
            path: path.to_path_buf(),
            dependencies,
            dev_dependencies: vec![],
        }));
    }
    
    // Check for package.json
    let package_json = path.join("package.json");
    if package_json.exists() {
        let content = std::fs::read_to_string(&package_json)?;
        let doc: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| WorkspaceError::InvalidConfig(e.to_string()))?;
        
        let name = doc.get("name")
            .and_then(|n| n.as_str())
            .unwrap_or_else(|| path.file_name().unwrap().to_str().unwrap());
        
        let dependencies = doc.get("dependencies")
            .and_then(|d| d.as_object())
            .map(|o| o.keys().cloned().collect())
            .unwrap_or_default();
        
        let dev_dependencies = doc.get("devDependencies")
            .and_then(|d| d.as_object())
            .map(|o| o.keys().cloned().collect())
            .unwrap_or_default();
        
        return Ok(Some(Package {
            name: name.to_string(),
            path: path.to_path_buf(),
            dependencies,
            dev_dependencies,
        }));
    }
    
    Ok(None)
}

fn parse_size(size_str: &str) -> u64 {
    let size_str = size_str.to_uppercase();
    let (num, multiplier) = if size_str.ends_with("GB") {
        (size_str.trim_end_matches("GB"), 1024 * 1024 * 1024)
    } else if size_str.ends_with("MB") {
        (size_str.trim_end_matches("MB"), 1024 * 1024)
    } else if size_str.ends_with("KB") {
        (size_str.trim_end_matches("KB"), 1024)
    } else {
        (size_str.as_str(), 1)
    };
    
    num.trim().parse::<u64>().unwrap_or(10 * 1024 * 1024 * 1024) * multiplier
}

fn load_cache_index(path: &Path) -> Result<HashMap<CacheKey, CacheEntry>> {
    // Binary format for fast loading
    let data = std::fs::read(path)?;
    bincode::decode_from_slice(&data, bincode::config::standard())
        .map(|(index, _)| index)
        .map_err(|e| WorkspaceError::CacheError(e.to_string()))
}

fn save_cache_index(path: &Path, index: &HashMap<CacheKey, CacheEntry>) -> Result<()> {
    let data = bincode::encode_to_vec(index, bincode::config::standard())
        .map_err(|e| WorkspaceError::CacheError(e.to_string()))?;
    std::fs::write(path, data)?;
    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

fn dir_size(path: &Path) -> u64 {
    if path.is_file() {
        return std::fs::metadata(path).map(|m| m.len()).unwrap_or(0);
    }
    
    walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| e.metadata().ok())
        .map(|m| m.len())
        .sum()
}

fn glob_files(dir: &Path, patterns: &[&str]) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    
    for pattern in patterns {
        let full_pattern = dir.join(pattern);
        for entry in glob::glob(&full_pattern.to_string_lossy())
            .map_err(|e| WorkspaceError::InvalidConfig(e.to_string()))?
        {
            if let Ok(path) = entry {
                if path.is_file() {
                    files.push(path);
                }
            }
        }
    }
    
    files.sort();
    Ok(files)
}
```

## CLI Integration

Add these commands to `dx-cli`:

```rust
// In dx-cli/src/commands/workspace.rs

use dx_workspace::{Workspace, WorkspaceTemplate, TaskOptions, OutputMode};

/// Workspace management commands
#[derive(clap::Subcommand)]
pub enum WorkspaceCommand {
    /// Initialize a new workspace
    Init {
        /// Template to use
        #[arg(short, long, default_value = "minimal")]
        template: String,
    },
    
    /// Generate editor configurations
    Generate {
        /// Only generate for specific targets
        #[arg(short, long)]
        target: Option<Vec<String>>,
    },
    
    /// Run a task across all packages
    Run {
        /// Task name
        task: String,
        
        /// Filter to specific packages
        #[arg(short, long)]
        filter: Option<Vec<String>>,
        
        /// Maximum parallel tasks
        #[arg(short, long)]
        concurrency: Option<usize>,
        
        /// Ignore cache
        #[arg(long)]
        force: bool,
        
        /// Dry run
        #[arg(long)]
        dry_run: bool,
    },
    
    /// Show dependency graph
    Graph {
        /// Output format (text, dot, json)
        #[arg(short, long, default_value = "text")]
        format: String,
    },
    
    /// Watch for changes and regenerate configs
    Watch,
    
    /// Show workspace info
    Info,
}

pub async fn handle(cmd: WorkspaceCommand) -> anyhow::Result<()> {
    match cmd {
        WorkspaceCommand::Init { template } => {
            let template = match template.as_str() {
                "rust" | "rust-single" => WorkspaceTemplate::RustSingle,
                "rust-workspace" => WorkspaceTemplate::RustWorkspace,
                "node" | "node-monorepo" => WorkspaceTemplate::NodeMonorepo,
                "dx" | "dx-fullstack" => WorkspaceTemplate::DxFullStack,
                _ => WorkspaceTemplate::Minimal,
            };
            
            let workspace = Workspace::init(".", template)?;
            println!("✅ Initialized dx-workspace");
            println!("   Run `dx workspace generate` to create editor configs");
        }
        
        WorkspaceCommand::Generate { target } => {
            let workspace = Workspace::load(".")?;
            let report = workspace.generate_all()?;
            
            println!("✅ Generated {} configuration files:", report.total_files());
            for (target, files) in &report.files {
                println!("   {}: {} files", target, files.len());
                for file in files {
                    println!("      - {}", file.display());
                }
            }
            
            if !report.warnings.is_empty() {
                println!("\n⚠️  Warnings:");
                for warning in &report.warnings {
                    println!("   - {}", warning);
                }
            }
        }
        
        WorkspaceCommand::Run { task, filter, concurrency, force, dry_run } => {
            let workspace = Workspace::load(".")?;
            
            let options = TaskOptions {
                filter,
                ignore: None,
                concurrency,
                continue_on_error: false,
                force,
                dry_run,
                output: OutputMode::Stream,
            };
            
            let result = workspace.run_task(&task, options).await?;
            
            println!("\n📊 Results:");
            println!("   ✓ Succeeded: {}", result.succeeded.len());
            println!("   ⚡ Cached: {}", result.cached.len());
            println!("   ✗ Failed: {}", result.failed.len());
            println!("   ⏱  Duration: {:?}", result.duration);
            
            if !result.failed.is_empty() {
                std::process::exit(1);
            }
        }
        
        WorkspaceCommand::Graph { format } => {
            let workspace = Workspace::load(".")?;
            
            if let Some(graph) = workspace.dependency_graph() {
                match format.as_str() {
                    "dot" => {
                        println!("digraph G {{");
                        for pkg in graph.build_order() {
                            for dep in graph.dependencies(&pkg.name) {
                                println!("  \"{}\" -> \"{}\";", pkg.name, dep.name);
                            }
                        }
                        println!("}}");
                    }
                    "json" => {
                        let packages: Vec<_> = graph.build_order().iter()
                            .map(|p| serde_json::json!({
                                "name": p.name,
                                "dependencies": graph.dependencies(&p.name).iter()
                                    .map(|d| &d.name).collect::<Vec<_>>()
                            }))
                            .collect();
                        println!("{}", serde_json::to_string_pretty(&packages)?);
                    }
                    _ => {
                        println!("📦 Dependency Graph (build order):\n");
                        for (i, pkg) in graph.build_order().iter().enumerate() {
                            let deps = graph.dependencies(&pkg.name);
                            if deps.is_empty() {
                                println!("{}. {} (no dependencies)", i + 1, pkg.name);
                            } else {
                                let dep_names: Vec<_> = deps.iter().map(|d| &d.name).collect();
                                println!("{}. {} → {:?}", i + 1, pkg.name, dep_names);
                            }
                        }
                    }
                }
            } else {
                println!("ℹ️  Monorepo not enabled. Add [monorepo] section to dx-workspace.toml");
            }
        }
        
        WorkspaceCommand::Watch => {
            let workspace = Workspace::load(".")?;
            println!("👀 Watching for changes...");
            workspace.watch().await?;
        }
        
        WorkspaceCommand::Info => {
            let workspace = Workspace::load(".")?;
            // Print workspace info
            println!("📁 dx-workspace info");
            // ... print details
        }
    }
    
    Ok(())
}
```

## Cargo.toml

```toml
[package]
name = "dx-workspace"
version = "0.1.0"
edition = "2024"
description = "Universal development environment generator for dx"
license = "MIT OR Apache-2.0"

[dependencies]
# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = { version = "0.8", features = ["preserve_order"] }
bincode = "2.0.0-rc.3"

# Async runtime
tokio = { version = "1.40", features = ["full"] }

# Graph algorithms
petgraph = "0.6"

# File system
walkdir = "2.4"
glob = "0.3"
notify = "6.1"

# Hashing (for cache)
blake3 = "1.5"
hex = "0.4"

# Error handling
thiserror = "1.0"
anyhow = "1.0"

# Utilities
num_cpus = "1.16"

[dev-dependencies]
tempfile = "3.10"
```

## Key Advantages Over Turborepo

| Feature | Turborepo | dx-workspace |
|---------|-----------|--------------|
| **Cache Format** | JSON | Binary (10x faster) |
| **Hashing** | SHA256 | Blake3 (SIMD, 3x faster) |
| **Parallelism** | Node.js workers | Tokio (native threads) |
| **Graph Analysis** | Custom JS | petgraph (battle-tested) |
| **Index Lookup** | O(n) JSON parse | O(1) binary lookup |
| **Memory Usage** | ~200MB Node heap | ~20MB Rust stack |
| **Cold Start** | ~500ms | ~10ms |
| **Editor Configs** | ❌ | ✅ (10+ editors) |
| **Cloud IDE Support** | ❌ | ✅ (6+ platforms) |

This design gives you a complete workspace management system that's faster than Turborepo, generates configs for all major editors and cloud IDEs, and integrates seamlessly with the dx ecosystem!
```
