//! DX Configuration System
//!
//! Loads configuration from the `dx` file (no extension).

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Main DX configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DxConfig {
    pub project: ProjectConfig,
    pub forge: ForgeConfig,
    pub tools: ToolsConfig,
}

/// Project configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub version: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
}

/// Forge daemon configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeConfig {
    pub socket_path: Option<PathBuf>,
    pub lsp_port: u16,
    pub auto_start: bool,
    pub watch_patterns: Vec<String>,
    pub ignore_patterns: Vec<String>,
    pub debounce_ms: u64,
    pub max_concurrent_tools: usize,
}


/// Tools configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsConfig {
    pub enabled: Vec<String>,
    pub disabled: Vec<String>,
    pub custom: HashMap<String, CustomToolConfig>,
}

/// Custom tool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomToolConfig {
    pub path: PathBuf,
    pub args: Vec<String>,
}

impl Default for DxConfig {
    fn default() -> Self {
        Self {
            project: ProjectConfig::default(),
            forge: ForgeConfig::default(),
            tools: ToolsConfig::default(),
        }
    }
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            name: "dx-project".to_string(),
            version: "0.1.0".to_string(),
            title: None,
            description: None,
            author: None,
        }
    }
}

impl Default for ForgeConfig {
    fn default() -> Self {
        Self {
            socket_path: None,
            lsp_port: 9876,
            auto_start: false,
            watch_patterns: vec![
                "**/*.ts".to_string(),
                "**/*.tsx".to_string(),
                "**/*.js".to_string(),
                "**/*.jsx".to_string(),
                "**/*.css".to_string(),
                "**/*.rs".to_string(),
                "**/*.dx".to_string(),
                "**/dx".to_string(),
            ],
            ignore_patterns: vec![
                "**/node_modules/**".to_string(),
                "**/.dx/**".to_string(),
                "**/target/**".to_string(),
                "**/dist/**".to_string(),
                "**/.git/**".to_string(),
            ],
            debounce_ms: 100,
            max_concurrent_tools: 4,
        }
    }
}

impl Default for ToolsConfig {
    fn default() -> Self {
        Self {
            enabled: vec![],
            disabled: vec![],
            custom: HashMap::new(),
        }
    }
}


impl DxConfig {
    /// Load configuration from dx file
    pub fn load() -> Result<Self> {
        let config_path = Self::find_config_file()?;
        
        match config_path {
            Some(path) => Self::parse_dx_file(&path),
            None => Ok(Self::default()),
        }
    }

    /// Find dx config file by searching up directory tree
    fn find_config_file() -> Result<Option<PathBuf>> {
        let mut current = std::env::current_dir()?;

        loop {
            let config_path = current.join("dx");
            if config_path.exists() && config_path.is_file() {
                return Ok(Some(config_path));
            }

            match current.parent() {
                Some(parent) => current = parent.to_path_buf(),
                None => return Ok(None),
            }
        }
    }

    /// Parse dx file format
    fn parse_dx_file(path: &PathBuf) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let mut config = Self::default();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") {
                continue;
            }

            // Parse key|value format
            if let Some((key, value)) = line.split_once('|') {
                match key {
                    "nm" => config.project.name = value.to_string(),
                    "v" => config.project.version = value.to_string(),
                    "tt" => config.project.title = Some(value.to_string()),
                    "ds" => config.project.description = Some(value.to_string()),
                    "au" => config.project.author = Some(value.to_string()),
                    _ => {}
                }
            }
        }

        // Apply environment variable overrides
        config.apply_env_overrides();

        Ok(config)
    }

    /// Apply environment variable overrides
    fn apply_env_overrides(&mut self) {
        if let Ok(port) = std::env::var("DX_FORGE_PORT") {
            if let Ok(p) = port.parse() {
                self.forge.lsp_port = p;
            }
        }

        if let Ok(auto) = std::env::var("DX_FORGE_AUTO_START") {
            self.forge.auto_start = auto == "1" || auto.to_lowercase() == "true";
        }

        if let Ok(debounce) = std::env::var("DX_FORGE_DEBOUNCE_MS") {
            if let Ok(d) = debounce.parse() {
                self.forge.debounce_ms = d;
            }
        }
    }
}
