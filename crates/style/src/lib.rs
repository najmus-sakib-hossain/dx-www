pub mod binary;
pub mod cache;
pub mod config;
pub mod core;
pub mod datasource;
pub mod generator;
pub mod header;
pub mod parser;
pub mod platform;
pub mod telemetry;

use std::path::Path;

/// Style compilation error
#[derive(Debug)]
pub enum StyleError {
    /// IO error
    Io(std::io::Error),
    /// Parse error
    Parse(String),
}

impl std::fmt::Display for StyleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StyleError::Io(e) => write!(f, "IO error: {}", e),
            StyleError::Parse(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for StyleError {}

impl From<std::io::Error> for StyleError {
    fn from(e: std::io::Error) -> Self {
        StyleError::Io(e)
    }
}

/// Compile styles from input path to output path.
///
/// This is the main entry point for the style compiler CLI.
pub fn compile(input: impl AsRef<Path>, output: impl AsRef<Path>) -> Result<(), StyleError> {
    let _input = input.as_ref();
    let _output = output.as_ref();

    // TODO: Implement full style compilation pipeline
    // For now, this is a placeholder that will be expanded
    // to handle the complete dx-style compilation

    Ok(())
}
