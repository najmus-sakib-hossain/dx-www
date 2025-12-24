//! DX configuration file parsing

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DxConfig {
    pub project: ProjectConfig,

    #[serde(default)]
    pub build: BuildConfig,

    #[serde(default)]
    pub dev: DevConfig,

    #[serde(default)]
    pub runtime: RuntimeConfig,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProjectConfig {
    pub name: String,

    #[serde(default = "default_version")]
    pub version: String,

    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildConfig {
    #[serde(default = "default_target")]
    pub target: String,

    #[serde(default = "default_true")]
    pub minify: bool,

    #[serde(default)]
    pub sourcemap: bool,

    #[serde(default = "default_out_dir")]
    pub out_dir: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DevConfig {
    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default)]
    pub open: bool,

    #[serde(default)]
    pub https: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RuntimeConfig {
    #[serde(default = "default_jsx")]
    pub jsx: String,

    #[serde(default = "default_true")]
    pub typescript: bool,
}

impl DxConfig {
    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: DxConfig = toml::from_str(&content)?;
        Ok(config)
    }

    #[allow(dead_code)]
    pub fn load_or_default() -> Self {
        let config_path = Path::new("dx.toml");
        if config_path.exists() {
            Self::load(config_path).unwrap_or_default()
        } else {
            Self::default()
        }
    }
}

// Default value functions
fn default_version() -> String {
    "0.1.0".to_string()
}
fn default_target() -> String {
    "browser".to_string()
}
fn default_out_dir() -> String {
    "dist".to_string()
}
fn default_port() -> u16 {
    3000
}
fn default_jsx() -> String {
    "dx".to_string()
}
fn default_true() -> bool {
    true
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            target: default_target(),
            minify: true,
            sourcemap: false,
            out_dir: default_out_dir(),
        }
    }
}

impl Default for DevConfig {
    fn default() -> Self {
        Self {
            port: default_port(),
            open: false,
            https: false,
        }
    }
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            jsx: default_jsx(),
            typescript: true,
        }
    }
}
