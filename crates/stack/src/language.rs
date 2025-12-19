//! Language definitions and their stack requirements
//!
//! This module defines all supported programming languages and
//! specifies which stack components each language requires.

use crate::capability::{StackCapability, StackCapabilitySet};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Supported programming languages in the DX ecosystem.
///
/// Each language has different requirements for development tooling
/// based on its ecosystem characteristics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    /// JavaScript - needs full stack (fragmented ecosystem)
    JavaScript,

    /// TypeScript - same as JavaScript (superset)
    TypeScript,

    /// Python - needs runtime, package manager, test runner
    Python,

    /// Rust - needs nothing (cargo is complete)
    Rust,

    /// Go - needs nothing (go toolchain is complete)
    Go,

    /// C - needs bundler (build system), compatibility, test runner
    C,

    /// C++ - needs bundler (build system), compatibility, test runner
    Cpp,

    /// Zig - needs minimal tooling (zig build is comprehensive)
    Zig,

    /// Ruby - needs runtime, package manager, test runner
    Ruby,

    /// Java - needs bundler (build), package manager
    Java,

    /// Kotlin - same requirements as Java
    Kotlin,

    /// Swift - needs minimal tooling (SwiftPM handles most)
    Swift,

    /// Elixir - needs runtime, package manager, test runner
    Elixir,
}

impl Language {
    /// Returns the capabilities required for this language's stack.
    ///
    /// Languages with mature unified toolchains (like Rust's cargo or Go's go)
    /// return empty capability sets since they don't need DX tooling.
    pub fn required_capabilities(&self) -> StackCapabilitySet {
        match self {
            // JavaScript/TypeScript: Full stack needed (fragmented ecosystem)
            Language::JavaScript | Language::TypeScript => StackCapabilitySet::all(),

            // Python: Needs most components (pip, poetry, pytest ecosystem)
            Language::Python => StackCapabilitySet::from_iter([
                StackCapability::Runtime,
                StackCapability::PackageManager,
                StackCapability::Monorepo,
                StackCapability::Compatibility,
                StackCapability::TestRunner,
            ]),

            // Ruby: Similar to Python
            Language::Ruby => StackCapabilitySet::from_iter([
                StackCapability::Runtime,
                StackCapability::PackageManager,
                StackCapability::TestRunner,
            ]),

            // C/C++: Needs build system, compatibility layer, testing
            Language::C | Language::Cpp => StackCapabilitySet::from_iter([
                StackCapability::Bundler, // Build system
                StackCapability::Compatibility,
                StackCapability::TestRunner,
            ]),

            // Java/Kotlin: Build + package management
            Language::Java | Language::Kotlin => StackCapabilitySet::from_iter([
                StackCapability::Bundler, // Gradle/Maven replacement
                StackCapability::PackageManager,
                StackCapability::TestRunner,
            ]),

            // Elixir: Mix handles a lot, but some components useful
            Language::Elixir => StackCapabilitySet::from_iter([
                StackCapability::Runtime,
                StackCapability::PackageManager,
                StackCapability::TestRunner,
            ]),

            // Rust: cargo handles everything - no DX stack needed
            Language::Rust => StackCapabilitySet::empty(),

            // Go: go toolchain handles everything - no DX stack needed
            Language::Go => StackCapabilitySet::empty(),

            // Zig: zig build is comprehensive - minimal needs
            Language::Zig => StackCapabilitySet::empty(),

            // Swift: SwiftPM handles most - minimal needs
            Language::Swift => StackCapabilitySet::empty(),
        }
    }

    /// Returns whether this language needs a DX stack at all.
    ///
    /// Languages with unified toolchains (Rust, Go, Zig) return false.
    pub fn needs_stack(&self) -> bool {
        !self.required_capabilities().is_empty()
    }

    /// Returns the file extensions associated with this language.
    pub fn extensions(&self) -> &'static [&'static str] {
        match self {
            Language::JavaScript => &["js", "mjs", "cjs", "jsx"],
            Language::TypeScript => &["ts", "mts", "cts", "tsx"],
            Language::Python => &["py", "pyw", "pyi"],
            Language::Rust => &["rs"],
            Language::Go => &["go"],
            Language::C => &["c", "h"],
            Language::Cpp => &["cpp", "cc", "cxx", "hpp", "hxx", "h"],
            Language::Zig => &["zig"],
            Language::Ruby => &["rb", "erb"],
            Language::Java => &["java"],
            Language::Kotlin => &["kt", "kts"],
            Language::Swift => &["swift"],
            Language::Elixir => &["ex", "exs"],
        }
    }

    /// Returns the primary native toolchain for this language.
    ///
    /// This is what DX would delegate to if it doesn't provide custom tooling.
    pub fn native_toolchain(&self) -> &'static str {
        match self {
            Language::JavaScript | Language::TypeScript => "node/npm",
            Language::Python => "python/pip",
            Language::Rust => "cargo",
            Language::Go => "go",
            Language::C => "gcc/clang",
            Language::Cpp => "g++/clang++",
            Language::Zig => "zig",
            Language::Ruby => "ruby/gem",
            Language::Java => "java/maven",
            Language::Kotlin => "kotlin/gradle",
            Language::Swift => "swift/swiftpm",
            Language::Elixir => "elixir/mix",
        }
    }

    /// Returns all supported languages
    pub fn all() -> &'static [Language] {
        &[
            Language::JavaScript,
            Language::TypeScript,
            Language::Python,
            Language::Rust,
            Language::Go,
            Language::C,
            Language::Cpp,
            Language::Zig,
            Language::Ruby,
            Language::Java,
            Language::Kotlin,
            Language::Swift,
            Language::Elixir,
        ]
    }

    /// Returns languages that need DX stack tooling
    pub fn stackable() -> Vec<Language> {
        Self::all().iter().copied().filter(|l| l.needs_stack()).collect()
    }

    /// Returns languages that have native complete toolchains
    pub fn self_sufficient() -> Vec<Language> {
        Self::all().iter().copied().filter(|l| !l.needs_stack()).collect()
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Language::JavaScript => write!(f, "JavaScript"),
            Language::TypeScript => write!(f, "TypeScript"),
            Language::Python => write!(f, "Python"),
            Language::Rust => write!(f, "Rust"),
            Language::Go => write!(f, "Go"),
            Language::C => write!(f, "C"),
            Language::Cpp => write!(f, "C++"),
            Language::Zig => write!(f, "Zig"),
            Language::Ruby => write!(f, "Ruby"),
            Language::Java => write!(f, "Java"),
            Language::Kotlin => write!(f, "Kotlin"),
            Language::Swift => write!(f, "Swift"),
            Language::Elixir => write!(f, "Elixir"),
        }
    }
}

impl FromStr for Language {
    type Err = crate::error::StackError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "javascript" | "js" => Ok(Language::JavaScript),
            "typescript" | "ts" => Ok(Language::TypeScript),
            "python" | "py" => Ok(Language::Python),
            "rust" | "rs" => Ok(Language::Rust),
            "go" | "golang" => Ok(Language::Go),
            "c" => Ok(Language::C),
            "cpp" | "c++" | "cxx" => Ok(Language::Cpp),
            "zig" => Ok(Language::Zig),
            "ruby" | "rb" => Ok(Language::Ruby),
            "java" => Ok(Language::Java),
            "kotlin" | "kt" => Ok(Language::Kotlin),
            "swift" => Ok(Language::Swift),
            "elixir" | "ex" => Ok(Language::Elixir),
            _ => Err(crate::error::StackError::UnknownLanguage(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_javascript_needs_full_stack() {
        let caps = Language::JavaScript.required_capabilities();
        assert!(caps.contains(StackCapability::Runtime));
        assert!(caps.contains(StackCapability::PackageManager));
        assert!(caps.contains(StackCapability::Bundler));
        assert!(caps.contains(StackCapability::Monorepo));
        assert!(caps.contains(StackCapability::Compatibility));
        assert!(caps.contains(StackCapability::TestRunner));
    }

    #[test]
    fn test_rust_needs_nothing() {
        let caps = Language::Rust.required_capabilities();
        assert!(caps.is_empty());
        assert!(!Language::Rust.needs_stack());
    }

    #[test]
    fn test_go_needs_nothing() {
        let caps = Language::Go.required_capabilities();
        assert!(caps.is_empty());
        assert!(!Language::Go.needs_stack());
    }

    #[test]
    fn test_language_from_str() {
        assert_eq!("js".parse::<Language>().unwrap(), Language::JavaScript);
        assert_eq!("typescript".parse::<Language>().unwrap(), Language::TypeScript);
        assert_eq!("python".parse::<Language>().unwrap(), Language::Python);
        assert_eq!("rust".parse::<Language>().unwrap(), Language::Rust);
        assert_eq!("c++".parse::<Language>().unwrap(), Language::Cpp);
    }

    #[test]
    fn test_stackable_languages() {
        let stackable = Language::stackable();
        assert!(stackable.contains(&Language::JavaScript));
        assert!(stackable.contains(&Language::Python));
        assert!(!stackable.contains(&Language::Rust));
        assert!(!stackable.contains(&Language::Go));
    }
}
