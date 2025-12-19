//! # dx-compat-compile
//!
//! Single executable compilation compatibility layer.

#![warn(missing_docs)]

mod error;

pub use error::{CompileError, CompileResult};

/// Target platform for compilation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    /// Linux x64
    LinuxX64,
    /// Linux ARM64
    LinuxArm64,
    /// macOS x64
    MacosX64,
    /// macOS ARM64
    MacosArm64,
    /// Windows x64
    WindowsX64,
}

impl Target {
    /// Get the target triple.
    pub fn triple(&self) -> &'static str {
        match self {
            Target::LinuxX64 => "x86_64-unknown-linux-gnu",
            Target::LinuxArm64 => "aarch64-unknown-linux-gnu",
            Target::MacosX64 => "x86_64-apple-darwin",
            Target::MacosArm64 => "aarch64-apple-darwin",
            Target::WindowsX64 => "x86_64-pc-windows-msvc",
        }
    }
}
