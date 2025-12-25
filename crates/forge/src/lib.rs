//! # DX Forge - Production-Ready VCS and Orchestration Engine
//!
//! Forge is the orchestration backbone for the DX tools ecosystem, providing:
//! - Content-addressable storage with SHA-256 blob hashing
//! - Git-compatible versioning with traffic branch safety system
//! - Dual-watcher architecture (LSP + File System monitoring)
//! - Tool orchestration with priority-based execution and dependency resolution
//! - Component injection for zero-bloat dependency management
//! - Semantic versioning with dependency resolution
//! - Pattern detection for dx-tools (dxButton, dxiIcon, dxfRoboto, etc.)
//! - R2 component caching and injection
//! - Production error handling with retry logic
//!
//! ## Architecture Overview
//!
//! Forge eliminates node_modules bloat by detecting code patterns via LSP,
//! injecting only needed components directly into user files, and coordinating
//! DX tool execution with traffic branch safety logic.
//!
//! ### Core Components
//!
//! - **Orchestrator**: Coordinates tool execution with lifecycle hooks, circular dependency detection
//! - **Dual-Watcher**: Monitors LSP + file system changes with pattern detection
//! - **Traffic Branch System**: Green (auto), Yellow (merge), Red (manual) for safe updates
//! - **Storage Layer**: Content-addressable blobs with R2 cloud sync
//! - **Version Manager**: Semantic versioning with compatibility checking
//! - **Pattern Detector**: Identifies dx-tool patterns in source code
//! - **Injection Manager**: Fetches and caches components from R2 storage
//!
//! ## Quick Start - Tool Development
//!
//! ```rust,no_run
//! use dx_forge::{DxTool, ExecutionContext, ToolOutput, Orchestrator};
//! use anyhow::Result;
//!
//! struct MyDxTool;
//!
//! impl DxTool for MyDxTool {
//!     fn name(&self) -> &str { "dx-mytool" }
//!     fn version(&self) -> &str { "1.0.0" }
//!     fn priority(&self) -> u32 { 50 }
//!     
//!     fn execute(&mut self, _ctx: &ExecutionContext) -> Result<ToolOutput> {
//!         // Your tool logic here
//!         Ok(ToolOutput::success())
//!     }
//! }
//!
//! fn main() -> Result<()> {
//!     let mut orchestrator = Orchestrator::new(".")?;
//!     orchestrator.register_tool(Box::new(MyDxTool))?;
//!     let _outputs = orchestrator.execute_all()?;
//!     Ok(())
//! }
//! ```
//!
//! ## Quick Start - Change Detection
//!
//! ```rust,no_run
//! use dx_forge::{DualWatcher, FileChange};
//! use anyhow::Result;
//! use std::path::PathBuf;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let mut watcher = DualWatcher::new()?;
//!     let project_root = PathBuf::from(".");
//!
//!     // Start watching for changes
//!     watcher.start(&project_root).await?;
//!
//!     // Subscribe to the unified change stream
//!     let mut rx = watcher.receiver();
//!
//!     while let Ok(change) = rx.recv().await {
//!         println!("Change detected: {:?} ({:?})", change.path, change.source);
//!     }
//!
//!     Ok(())
//! }
//! ```

// Core modules
pub mod context;
pub mod crdt;
pub mod server;
pub mod storage;
pub mod sync;

// Core library - NEW unified API
pub mod core;

// ========================================================================
// The 132 Eternal API Functions (v0.1.0)
// ========================================================================
pub mod api;

// Legacy watcher module (for CLI compatibility)
#[path = "watcher_legacy/mod.rs"]
pub mod watcher_legacy;

// Production orchestration modules (v1.0.0)
pub mod orchestrator;
pub mod watcher;

// DX Tools support modules
pub mod error;
pub mod injection;
pub mod patterns;
pub mod version;

// Phase 5 modules
pub mod auto_update;
pub mod cache;
pub mod profiler;

// DX Tool Cache Management (v2.0.0)
pub mod dx_cache;
pub mod dx_executor;
pub mod serializer_tool;

// Binary Dawn: Sovereign Orchestration Engine
pub mod sovereign;

// Forge Daemon - Binary Dawn Edition (v2.0.0)
pub mod daemon;

// Platform-native I/O abstraction layer
pub mod platform_io;

// ========================================================================
// Primary Public API - Forge Unified Interface
// ========================================================================

pub use core::{
    EditorInfo, EditorType, Forge, ForgeConfig, GeneratedFileInfo, LifecycleEvent, OutputStrategy,
    ToolId, ToolStatus,
};

// ========================================================================
// Re-export orchestration types (public API)
// ========================================================================

pub use orchestrator::{
    Conflict, DxTool, ExecutionContext, Orchestrator, OrchestratorConfig, ToolOutput,
    TrafficAnalyzer, TrafficBranch,
};

pub use watcher::{ChangeKind, ChangeSource, DualWatcher, FileChange, FileWatcher, LspWatcher};

// ========================================================================
// Re-export storage types
// ========================================================================

pub use context::{ComponentStateManager, UpdateResult};
pub use crdt::{Operation, OperationType, Position};
pub use storage::{Database, OperationLog};

// ========================================================================
// Re-export DX tools support types
// ========================================================================

pub use error::{
    categorize_error, with_retry, EnhancedError, EnhancedResult, ErrorCategory, RetryPolicy,
    ToEnhanced,
};
pub use injection::{CacheStats, ComponentMetadata, InjectionManager};
pub use patterns::{DxToolType, PatternDetector, PatternMatch};
pub use version::{
    Branch, FileSnapshot, Snapshot, SnapshotDiff, SnapshotId, SnapshotManager, ToolInfo,
    ToolRegistry, ToolSource, ToolState, Version, VersionReq,
};

// ========================================================================
// Binary Dawn: Sovereign Orchestration Engine exports
// ========================================================================

pub use sovereign::{
    BackgroundTask, BackgroundWorker, DxForge, DxToolDefinition,
    Orchestrator as SovereignOrchestrator, ToolStatus as SovereignToolStatus, TrafficLight,
    TrafficManager,
};

// ========================================================================
// DX Tool Cache Management exports (v2.0.0)
// ========================================================================

pub use dx_cache::{
    CacheEntry, CacheStats as DxCacheStats, DxToolCacheManager, DxToolId, SyncResult,
    WarmStartResult,
};
pub use dx_executor::{
    BundlerTool, DxToolExecutable, DxToolExecutor, PackageManagerTool, StyleTool, TestRunnerTool,
    ToolConfig, ToolResult, ExecutionContext as DxExecutionContext,
};

// ========================================================================
// Forge Daemon exports (v2.0.0) - Binary Dawn Edition
// ========================================================================

pub use daemon::{
    ForgeDaemon, DaemonConfig, DaemonState, DaemonEvent,
    LspBridge, LspMessage, LspNotification,
    DaemonStateManager, ToolState as DaemonToolState, ProjectState,
    WorkerPool, WorkerTask, TaskPriority,
};

// ========================================================================
// Platform-native I/O exports
// ========================================================================

pub use platform_io::{
    create_platform_io, EventStream, FallbackBackend, FileEvent, FileEventKind,
    IoBackend, Platform, PlatformIO, PlatformInfo, WriteOp,
};

// ========================================================================
// Legacy exports (deprecated in favor of new Forge API)
// ========================================================================

#[deprecated(since = "1.0.0", note = "use `Forge` instead")]
pub use watcher::DualWatcher as ForgeWatcher;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// ========================================================================
// Re-export The 132 Eternal API Functions
// ========================================================================

// Core Lifecycle & System Orchestration (4 functions)
pub use api::lifecycle::{get_tool_context, initialize_forge, register_tool, shutdown_forge};

// Version Governance & Package Identity (6 functions)
pub use api::version::{
    activate_package_variant, current_forge_version, declare_tool_version, enforce_exact_version,
    query_active_package_variant, require_forge_minimum,
};

// Pipeline Execution & Orchestration (7 functions)
pub use api::pipeline::{
    execute_pipeline, execute_tool_immediately, get_resolved_execution_order,
    restart_current_pipeline, resume_pipeline_execution, suspend_pipeline_execution,
    temporarily_override_pipeline_order,
};

// Triple-Path Reactivity Engine (5 functions)
pub use api::reactivity::{
    begin_batch_operation, end_batch_operation, trigger_debounced_event, trigger_idle_event,
    trigger_realtime_event,
};

// Safe File Application & Branching Decision Engine (15 functions)
pub use api::branching::{
    apply_changes, apply_changes_force_unchecked, apply_changes_with_preapproved_votes,
    automatically_accept_green_conflicts, automatically_reject_red_conflicts,
    is_change_guaranteed_safe, issue_immediate_veto, preview_proposed_changes,
    prompt_review_for_yellow_conflicts, query_predicted_branch_color,
    register_permanent_branching_voter, reset_branching_engine_state,
    revert_most_recent_application, submit_branching_vote, BranchColor, BranchingVote,
};
// Note: FileChange is already exported from watcher module

// Global Event Bus & Observability (9 functions)
pub use api::events::{
    emit_magical_config_injection, emit_package_installation_begin,
    emit_package_installation_success, emit_pipeline_completed_event, emit_pipeline_started_event,
    emit_security_violation_detected, emit_tool_completed_event, emit_tool_started_event,
    publish_event, subscribe_to_event_stream, ForgeEvent,
};

// The One True Configuration System (16 functions)
pub use api::config::{
    auto_format_config_file, enable_live_config_watching, expand_config_placeholder,
    get_active_config_file_path, inject_authentication_config, inject_font_system_config,
    inject_full_config_section_at_cursor, inject_icon_system_config, inject_media_pipeline_config,
    inject_package_specific_config, inject_style_tooling_config, inject_ui_framework_config,
    jump_to_config_section, perform_config_schema_migration, provide_config_completion_suggestions,
    reload_configuration_manifest, validate_config_in_realtime,
};

// CI/CD & Workspace Orchestration (8 functions)
pub use api::cicd::{
    abort_running_ci_job, broadcast_change_to_workspace, detect_workspace_root,
    list_all_workspace_members, query_current_ci_status, register_ci_stage,
    synchronize_monorepo_workspace, trigger_ci_cd_pipeline,
};

// .dx/ Directory Management (10 functions)
pub use api::dx_directory::{
    cache_tool_offline_binary, checkout_dx_state, commit_current_dx_state,
    get_dx_binary_storage_path, get_dx_directory_path, list_dx_history, load_tool_offline_binary,
    pull_dx_state_from_remote, push_dx_state_to_remote, show_dx_state_diff,
};

// Offline-First Architecture (5 functions)
pub use api::offline::{
    detect_offline_mode, download_missing_tool_binaries, force_offline_operation,
    update_tool_binary_atomically, verify_binary_integrity_and_signature,
};

// Cart System (8 functions)
pub use api::cart::{
    clear_cart_completely, commit_cart_immediately, commit_entire_cart,
    export_cart_as_shareable_json, get_current_cart_contents, import_cart_from_json,
    remove_specific_cart_item, stage_item_in_cart, CartItem,
};

// Package Management (8 functions)
pub use api::packages::{
    fork_existing_variant, install_package_with_variant, list_all_installed_packages,
    pin_package_to_exact_version, publish_your_variant, search_dx_package_registry,
    uninstall_package_safely, update_package_intelligently, PackageInfo,
};

// Generated Code Governance (5 functions)
pub use api::codegen::{
    allow_safe_manual_edit_of_generated_code, claim_full_ownership_of_file, is_region_dx_generated,
    mark_code_region_as_dx_generated, release_ownership_of_file,
};

// Developer Experience & Editor Integration (26 functions)
pub use api::dx_experience::{
    apply_ai_generated_completion, apply_user_accepted_suggestion, await_editor_idle_state,
    create_watcher_ignored_scratch_file, display_dx_command_palette,
    display_inline_code_suggestion, dx_global_cache_directory, execute_full_security_audit,
    generate_comprehensive_project_report, log_structured_tool_action, open_dx_explorer_sidebar,
    open_embedded_dx_terminal, open_file_and_reveal_location, path_to_forge_manifest,
    project_root_directory, request_user_attention_flash, schedule_task_for_idle_time,
    show_onboarding_welcome_tour, trigger_ai_powered_suggestion, update_dx_status_bar_indicator,
};

// Testing forge logging
// test logging
// test event
// event2
// test edit

/// Initialize a new dx project.
///
/// This creates the project scaffolding using the specified template.
///
/// # Arguments
/// * `name` - The project name (will be used as the directory name)
/// * `template` - The template to use (e.g., "default", "minimal", "full")
///
/// # Errors
/// Returns an error if project creation fails.
pub fn init(name: &str, template: &str) -> anyhow::Result<()> {
    use std::fs;
    use std::path::Path;

    let project_path = Path::new(name);

    // Create project directory
    fs::create_dir_all(project_path)?;

    // Create src directory
    fs::create_dir_all(project_path.join("src"))?;
    fs::create_dir_all(project_path.join("src/pages"))?;
    fs::create_dir_all(project_path.join("src/components"))?;

    // Create dx.toml config
    let config_content = format!(
        r#"[project]
name = "{name}"
version = "0.1.0"
template = "{template}"

[build]
output = "dist"
"#
    );
    fs::write(project_path.join("dx.toml"), config_content)?;

    // Create main entry point based on template
    let main_content = match template {
        "minimal" => r#"// Minimal dx project
export default function App() {
    return <h1>Hello, dx!</h1>;
}
"#
        .to_string(),
        "full" => r#"// Full dx project with routing
import { Router, Route } from "dx/router";

export default function App() {
    return (
        <Router>
            <Route path="/" component={Home} />
            <Route path="/about" component={About} />
        </Router>
    );
}

function Home() {
    return <h1>Home</h1>;
}

function About() {
    return <h1>About</h1>;
}
"#
        .to_string(),
        _ => r#"// Default dx project
export default function App() {
    return (
        <main>
            <h1>Welcome to dx</h1>
            <p>Edit src/pages/index.dx to get started.</p>
        </main>
    );
}
"#
        .to_string(),
    };

    fs::write(project_path.join("src/pages/index.dx"), main_content)?;

    println!("✨ Created new dx project: {}", name);
    println!("📁 Template: {}", template);
    println!();
    println!("Next steps:");
    println!("  cd {}", name);
    println!("  dx dev");

    Ok(())
}
