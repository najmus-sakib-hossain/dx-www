//! # dx-stack: Language-Aware Development Stack
//!
//! A unified abstraction for language-specific development tools.
//! Each language can have different stack components based on its ecosystem needs.
//!
//! ## Stack Components
//!
//! | Component      | Description                           | JS/TS | Python | Rust | Go | C/C++ |
//! |----------------|---------------------------------------|-------|--------|------|----|-------|
//! | Runtime        | Code execution engine                 | ✓     | ✓      | ✗    | ✗  | ✗     |
//! | PackageManager | Dependency management                 | ✓     | ✓      | ✗    | ✗  | ✗     |
//! | Bundler        | Code bundling/compilation             | ✓     | ✗      | ✗    | ✗  | ✓     |
//! | Monorepo       | Multi-package workspace management    | ✓     | ✓      | ✗    | ✗  | ✗     |
//! | Compatibility  | Cross-version/platform compatibility  | ✓     | ✓      | ✗    | ✗  | ✓     |
//! | TestRunner     | Test execution framework              | ✓     | ✓      | ✗    | ✗  | ✓     |
//!
//! ## Why Languages Need Different Components
//!
//! - **Rust**: Uses `cargo` for everything (build, test, package) - no DX stack needed
//! - **Go**: Uses `go` toolchain - minimal DX stack needed  
//! - **JavaScript/TypeScript**: Fragmented ecosystem - full DX stack needed
//! - **Python**: Multiple tools (pip, poetry, pytest) - DX stack provides unified interface
//! - **C/C++**: Build systems vary wildly - DX stack provides consistency
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                     dx stack <language>                      │
//! ├─────────────────────────────────────────────────────────────┤
//! │  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────────────┐ │
//! │  │ Runtime │  │ Bundler │  │  Test   │  │ Package Manager │ │
//! │  └────┬────┘  └────┬────┘  └────┬────┘  └────────┬────────┘ │
//! │       │            │            │                │          │
//! │  ┌────┴────────────┴────────────┴────────────────┴────┐     │
//! │  │              Language Stack Implementation          │     │
//! │  │         (e.g., JavaScriptStack, PythonStack)        │     │
//! │  └─────────────────────────────────────────────────────┘     │
//! └─────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Example Usage
//!
//! ```rust,ignore
//! use dx_stack::{Language, StackRegistry, StackCapability};
//!
//! // Get the JavaScript stack
//! let js_stack = StackRegistry::get(Language::JavaScript)?;
//!
//! // Check what capabilities are available
//! if js_stack.has_capability(StackCapability::Runtime) {
//!     js_stack.runtime().run("index.ts")?;
//! }
//!
//! // Rust has no DX stack - cargo handles everything
//! assert!(StackRegistry::get(Language::Rust).is_none());
//! ```

mod capability;
mod component;
mod error;
mod language;
mod registry;

// Language-specific implementations
pub mod languages;

pub use capability::{StackCapability, StackCapabilitySet};
pub use component::{
    Bundler, Compatibility, Monorepo, PackageManager, Runtime, StackComponent, TestRunner,
};
pub use error::{StackError, StackResult};
pub use language::Language;
pub use registry::StackRegistry;

/// The main trait for a language development stack.
///
/// Each language that needs DX tooling implements this trait
/// to provide unified access to its development tools.
pub trait LanguageStack: Send + Sync {
    /// Returns the language this stack supports
    fn language(&self) -> Language;

    /// Returns the set of capabilities this stack provides
    fn capabilities(&self) -> StackCapabilitySet;

    /// Check if a specific capability is available
    fn has_capability(&self, cap: StackCapability) -> bool {
        self.capabilities().contains(cap)
    }

    /// Get the runtime component (if available)
    fn runtime(&self) -> Option<&dyn Runtime> {
        None
    }

    /// Get the package manager component (if available)
    fn package_manager(&self) -> Option<&dyn PackageManager> {
        None
    }

    /// Get the bundler component (if available)
    fn bundler(&self) -> Option<&dyn Bundler> {
        None
    }

    /// Get the monorepo component (if available)
    fn monorepo(&self) -> Option<&dyn Monorepo> {
        None
    }

    /// Get the compatibility component (if available)
    fn compatibility(&self) -> Option<&dyn Compatibility> {
        None
    }

    /// Get the test runner component (if available)
    fn test_runner(&self) -> Option<&dyn TestRunner> {
        None
    }

    /// Get stack version information
    fn version(&self) -> &str;

    /// Get a human-readable description
    fn description(&self) -> &str;
}

/// Version information for the dx-stack crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_capabilities() {
        // JavaScript should need all components
        let js_caps = Language::JavaScript.required_capabilities();
        assert!(js_caps.contains(StackCapability::Runtime));
        assert!(js_caps.contains(StackCapability::PackageManager));
        assert!(js_caps.contains(StackCapability::Bundler));
        assert!(js_caps.contains(StackCapability::TestRunner));

        // Rust should need nothing (cargo handles it)
        let rust_caps = Language::Rust.required_capabilities();
        assert!(rust_caps.is_empty());
    }
}
