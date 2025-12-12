/**
 * Project configuration (dx.toml)
 */
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub project: ProjectInfo,

    #[serde(default)]
    pub build: BuildConfig,

    #[serde(default)]
    pub server: ServerConfig,

    #[serde(default)]
    pub optimize: OptimizeConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    #[serde(default = "default_true")]
    pub auto_select: bool,

    pub runtime: Option<String>,

    #[serde(default = "default_true")]
    pub sourcemaps: bool,

    #[serde(default = "default_output")]
    pub output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default = "default_host")]
    pub host: String,

    #[serde(default = "default_true")]
    pub hmr: bool,

    #[serde(default)]
    pub cors_origins: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizeConfig {
    #[serde(default = "default_wasm_opt")]
    pub wasm_opt: String,

    #[serde(default = "default_true")]
    pub strip: bool,

    #[serde(default = "default_true")]
    pub lto: bool,
}

// Default values
fn default_true() -> bool {
    true
}
fn default_output() -> String {
    "dist".to_string()
}
fn default_port() -> u16 {
    3000
}
fn default_host() -> String {
    "localhost".to_string()
}
fn default_wasm_opt() -> String {
    "z".to_string()
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            auto_select: true,
            runtime: None,
            sourcemaps: true,
            output: "dist".to_string(),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: 3000,
            host: "localhost".to_string(),
            hmr: true,
            cors_origins: vec![],
        }
    }
}

impl Default for OptimizeConfig {
    fn default() -> Self {
        Self {
            wasm_opt: "z".to_string(),
            strip: true,
            lto: true,
        }
    }
}

impl ProjectConfig {
    /// Load configuration from dx (God File) or dx.toml (legacy)
    pub fn load<P: AsRef<Path>>(dir: P) -> Result<Self> {
        let dx_path = dir.as_ref().join("dx");
        let toml_path = dir.as_ref().join("dx.toml");

        // Try dx (God File) first
        if dx_path.exists() {
            let content = fs::read_to_string(&dx_path)
                .with_context(|| format!("Failed to read {}", dx_path.display()))?;

            // Parse God File format (TOML-like but simpler)
            return Self::parse_god_file(&content);
        }

        // Fallback to dx.toml for legacy projects
        if toml_path.exists() {
            let content = fs::read_to_string(&toml_path)
                .with_context(|| format!("Failed to read {}", toml_path.display()))?;

            let config: ProjectConfig =
                toml::from_str(&content).with_context(|| "Failed to parse dx.toml")?;

            return Ok(config);
        }

        // Neither found
        anyhow::bail!("No dx config found. Run 'dx new' to create a project.")
    }

    /// Parse the dx God File format
    fn parse_god_file(content: &str) -> Result<Self> {
        // God File is TOML-compatible, just with different naming
        // Convert name: -> [project]\nname =
        let normalized =
            content.replace("name:", "[project]\nname =").replace("version:", "version =");

        // Try to parse as TOML
        if let Ok(config) = toml::from_str::<ProjectConfig>(&normalized) {
            return Ok(config);
        }

        // Manual parsing for simple God File format
        let mut name = String::new();
        let mut version = String::from("1.0.0");

        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("name:") {
                name = line.trim_start_matches("name:").trim().trim_matches('"').to_string();
            }
            if line.starts_with("version:") {
                version = line.trim_start_matches("version:").trim().trim_matches('"').to_string();
            }
        }

        if name.is_empty() {
            anyhow::bail!("Invalid dx config: missing 'name'");
        }

        Ok(ProjectConfig {
            project: ProjectInfo { name, version },
            build: BuildConfig::default(),
            server: ServerConfig::default(),
            optimize: OptimizeConfig::default(),
        })
    }

    /// Save configuration to dx.toml
    pub fn save<P: AsRef<Path>>(&self, dir: P) -> Result<()> {
        let path = dir.as_ref().join("dx.toml");

        let content = toml::to_string_pretty(self).with_context(|| "Failed to serialize config")?;

        fs::write(&path, content).with_context(|| format!("Failed to write {}", path.display()))?;

        Ok(())
    }

    // Convenience accessors
    pub fn name(&self) -> &str {
        &self.project.name
    }

    pub fn version(&self) -> &str {
        &self.project.version
    }

    pub fn runtime(&self) -> &str {
        if self.build.auto_select {
            "auto"
        } else {
            self.build.runtime.as_deref().unwrap_or("micro")
        }
    }
}
