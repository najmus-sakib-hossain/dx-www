//! DX Forge Daemon - Binary Dawn Edition
//!
//! A persistent daemon that orchestrates all DX tools with:
//! - Dual-nature watchers (LSP + FileSystem)
//! - Background task processing
//! - Tool lifecycle management
//! - R2 cloud sync
//! - VS Code extension integration
//!
//! Architecture:
//! ```text
//! ┌──────────────────────────────────────────────────────────────────┐
//! │                     FORGE DAEMON (Binary Dawn)                    │
//! ├──────────────────────────────────────────────────────────────────┤
//! │  ┌─────────────────────┐     ┌─────────────────────────────┐    │
//! │  │   LSP Watcher       │     │   FileSystem Watcher        │    │
//! │  │   (Primary)         │     │   (Fallback)                │    │
//! │  │   - VS Code events  │     │   - notify-debouncer        │    │
//! │  │   - Semantic info   │     │   - Recursive watch         │    │
//! │  └─────────┬───────────┘     └──────────────┬──────────────┘    │
//! │            │                                │                    │
//! │            └──────────────┬─────────────────┘                    │
//! │                           ▼                                      │
//! │  ┌────────────────────────────────────────────────────────────┐ │
//! │  │              UNIFIED CHANGE STREAM                          │ │
//! │  │  (Deduplication + Pattern Detection + Priority Queue)       │ │
//! │  └───────────────────────────┬────────────────────────────────┘ │
//! │                              ▼                                   │
//! │  ┌────────────────────────────────────────────────────────────┐ │
//! │  │              TOOL ORCHESTRATOR                              │ │
//! │  │  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐          │ │
//! │  │  │ bundler │ │ style   │ │ test    │ │ www     │ ...      │ │
//! │  │  └─────────┘ └─────────┘ └─────────┘ └─────────┘          │ │
//! │  └───────────────────────────┬────────────────────────────────┘ │
//! │                              ▼                                   │
//! │  ┌────────────────────────────────────────────────────────────┐ │
//! │  │              BACKGROUND WORKER POOL                         │ │
//! │  │  - Cache warming    - R2 sync     - Pattern analysis       │ │
//! │  │  - Package prefetch - Cleanup     - Metrics                │ │
//! │  └────────────────────────────────────────────────────────────┘ │
//! └──────────────────────────────────────────────────────────────────┘
//! ```

pub mod core;
pub mod lsp;
pub mod state;
pub mod worker;
pub mod server;
pub mod lsp_server;

pub use core::{ForgeDaemon, DaemonConfig, DaemonState, DaemonEvent};
pub use lsp::{LspBridge, LspMessage, LspNotification};
pub use state::{DaemonStateManager, ToolState, ProjectState};
pub use worker::{WorkerPool, WorkerTask, TaskPriority};
pub use server::{DaemonServer, IpcCommand, IpcResponse};
pub use lsp_server::{LspServer, LspServerState, LspRequest, LspResponse};
