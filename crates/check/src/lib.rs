//! # dx-check
//!
//! **The binary-first linter that killed ESLint and Biome.**
//!
//! ## Performance Targets
//!
//! - **100-200x faster** than ESLint
//! - **5-15x faster** than Biome
//! - **<5ms** latency for any single file operation
//! - **<100MB** memory for million-line codebases
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────┐
//! │                         DX CHECK ARCHITECTURE                        │
//! ├─────────────────────────────────────────────────────────────────────┤
//! │                                                                     │
//! │   Source Files ──► SIMD Scanner ──► Parser ──► Binary AST Cache    │
//! │                         │              │              │             │
//! │                         ▼              ▼              ▼             │
//! │                   Quick Reject    AST Teleport    Cache Hit?        │
//! │                         │              │              │             │
//! │                         └──────────────┼──────────────┘             │
//! │                                        ▼                            │
//! │                              Binary Rule Fusion Engine              │
//! │                              (Single AST Traversal)                 │
//! │                                        │                            │
//! │                                        ▼                            │
//! │                              Binary Diagnostics                     │
//! │                                        │                            │
//! │                              ┌─────────┴─────────┐                  │
//! │                              ▼                   ▼                  │
//! │                           Terminal            Binary LSP            │
//! │                                                                     │
//! └─────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Key Features
//!
//! 1. **Binary Rule Fusion Engine** - All rules execute in ONE AST traversal
//! 2. **SIMD Pattern Scanner** - 32-64 bytes scanned simultaneously
//! 3. **Persistent Binary AST Cache** - Zero parsing for unchanged files
//! 4. **Thread-Per-Core Reactor** - 95%+ parallel efficiency
//! 5. **Binary LSP Protocol** - 10-33x faster IDE communication
//!
//! ## Usage
//!
//! ```rust,ignore
//! use dx_check::{Checker, CheckerConfig};
//!
//! let checker = Checker::new(CheckerConfig::default());
//! let diagnostics = checker.check_path("./src")?;
//!
//! for diagnostic in diagnostics {
//!     println!("{}", diagnostic);
//! }
//! ```

#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

pub mod cache;
pub mod cli;
pub mod config;
pub mod diagnostics;
pub mod engine;
pub mod fix;
pub mod project;
pub mod reactor;
pub mod rules;
pub mod scanner;

// Re-exports
pub use cache::AstCache;
pub use config::CheckerConfig;
pub use diagnostics::{Diagnostic, DiagnosticSeverity, Fix, Span};
pub use engine::{Checker, CheckResult};
pub use fix::FixEngine;
pub use project::ProjectProfile;
pub use reactor::LintReactor;
pub use rules::{Rule, RuleId, RuleRegistry};
pub use scanner::PatternScanner;

/// Dx Check version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Maximum files per second target
pub const TARGET_FILES_PER_SECOND: u32 = 50_000;

/// Maximum latency for single file operations (microseconds)
pub const TARGET_LATENCY_US: u32 = 5_000;

/// Frame budget for interactive operations (microseconds)
pub const FRAME_BUDGET_US: u32 = 4_000;

/// Prelude for convenient imports
pub mod prelude {
    pub use crate::cache::AstCache;
    pub use crate::config::CheckerConfig;
    pub use crate::diagnostics::{Diagnostic, DiagnosticSeverity, Fix, Span};
    pub use crate::engine::{CheckResult, Checker};
    pub use crate::fix::FixEngine;
    pub use crate::project::ProjectProfile;
    pub use crate::reactor::LintReactor;
    pub use crate::rules::{Rule, RuleId, RuleRegistry};
    pub use crate::scanner::PatternScanner;
}
