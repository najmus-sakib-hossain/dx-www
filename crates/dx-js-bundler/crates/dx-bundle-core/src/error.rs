//! Error types for DX JS Bundler

use thiserror::Error;
use std::path::PathBuf;

/// Bundler error types
#[derive(Error, Debug)]
pub enum BundleError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Module not found: {path}")]
    ModuleNotFound { path: PathBuf },
    
    #[error("Parse error at {file}:{line}:{column}: {message}")]
    ParseError {
        file: PathBuf,
        line: u32,
        column: u32,
        message: String,
    },
    
    #[error("Transform error: {message}")]
    TransformError { message: String },
    
    #[error("Circular dependency detected: {chain}")]
    CircularDependency { chain: String },
    
    #[error("Arena exhausted: requested {requested} bytes, {available} available")]
    ArenaExhausted { requested: usize, available: usize },
    
    #[error("Cache corruption: {message}")]
    CacheCorruption { message: String },
    
    #[error("Invalid binary format: expected {expected}, got {got}")]
    InvalidFormat { expected: String, got: String },
    
    #[error("Resolution failed for '{specifier}' from '{from}'")]
    ResolutionFailed { specifier: String, from: PathBuf },
    
    #[error("Unsupported feature: {feature}")]
    UnsupportedFeature { feature: String },
}

impl BundleError {
    pub fn module_not_found(path: impl Into<PathBuf>) -> Self {
        Self::ModuleNotFound { path: path.into() }
    }
    
    pub fn parse_error(file: impl Into<PathBuf>, line: u32, column: u32, message: impl Into<String>) -> Self {
        Self::ParseError {
            file: file.into(),
            line,
            column,
            message: message.into(),
        }
    }
    
    pub fn transform_error(message: impl Into<String>) -> Self {
        Self::TransformError { message: message.into() }
    }
    
    pub fn arena_exhausted(requested: usize, available: usize) -> Self {
        Self::ArenaExhausted { requested, available }
    }
    
    pub fn cache_corruption(message: impl Into<String>) -> Self {
        Self::CacheCorruption { message: message.into() }
    }
}

/// Result type alias
pub type BundleResult<T> = Result<T, BundleError>;
