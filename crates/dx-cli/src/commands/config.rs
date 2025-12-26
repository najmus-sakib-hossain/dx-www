//! Configuration Management Commands
//!
//! Commands for viewing and managing DX configuration.

use anyhow::Result;
use clap::Subcommand;
use std::path::PathBuf;

use crate::config::DxConfig;

/// Configuration commands
#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    /// Show current effective configuration
    Show {
        /// Show raw config file content
        #[arg(short, long)]
        raw: bool,

        /// Output as JSON
        #[arg(short, long)]
        json: bool,
    },

    /// Initialize a new dx configuration file
    Init {
        /// Force overwrite existing config
        #[arg(short, long)]
        force: bool,

        /// Use minimal config template
        #[arg(short, long)]
        minimal: bool,
    },

    /// Get a specific config value
    Get {
        /// Config key (dot notation: forge.port)
        key: String,
    },

    /// Set a config value
    Set {
        /// Config key (dot notation: forge.port)
        key: String,

        /// Value to set
        value: String,
    },

    /// Show config file path
    Path,
}

impl ConfigCommands {
    pub async fn execute(self) -> Result<()> {
        match self {
            ConfigCommands::Show { raw, json } => show_config(raw, json).await,
            ConfigCommands::Init { force, minimal } => init_config(force, minimal).await,
            ConfigCommands::Get { key } => get_config(&key).await,
            ConfigCommands::Set { key, value } => set_config(&key, &value).await,
            ConfigCommands::Path => show_path().await,
        }
    }
}

/// Show current configuration
async fn show_config(raw: bool, json: bool) -> Result<()> {
    use console::style;

    let config_path = find_config_file()?;

    if raw {
        if let Some(path) = &config_path {
            let content = std::fs::read_to_string(path)?;
            println!("{}", content);
        } else {
            println!(
                "{} No dx config file found",
                style("ℹ").blue()
            );
        }
        return Ok(());
    }

    let config = DxConfig::load()?;

    if json {
        println!("{}", serde_json::to_string_pretty(&config)?);
        return Ok(());
    }

    println!(
        "{} DX Configuration",
        style("⚙").cyan()
    );
    println!();

    // Project section
    println!("{}:", style("[project]").bold());
    println!("  name:     {}", config.project.name);
    println!("  version:  {}", config.project.version);
    println!();

    // Forge section
    println!("{}:", style("[forge]").bold());
    println!("  lsp_port:     {}", config.forge.lsp_port);
    println!("  auto_start:   {}", config.forge.auto_start);
    println!("  debounce_ms:  {}", config.forge.debounce_ms);
    println!();

    // Tools section
    println!("{}:", style("[tools]").bold());
    if config.tools.enabled.is_empty() {
        println!("  enabled:  (all)");
    } else {
        println!("  enabled:  {:?}", config.tools.enabled);
    }
    if !config.tools.disabled.is_empty() {
        println!("  disabled: {:?}", config.tools.disabled);
    }
    println!();

    // Source info
    if let Some(path) = config_path {
        println!(
            "{}",
            style(format!("Source: {}", path.display())).dim()
        );
    } else {
        println!(
            "{}",
            style("Source: defaults (no dx file found)").dim()
        );
    }

    Ok(())
}

/// Initialize a new config file
async fn init_config(force: bool, minimal: bool) -> Result<()> {
    use console::style;

    let config_path = std::env::current_dir()?.join("dx");

    if config_path.exists() && !force {
        println!(
            "{} Config file already exists: {}",
            style("⚠").yellow(),
            config_path.display()
        );
        println!(
            "  Use {} to overwrite",
            style("--force").cyan()
        );
        return Ok(());
    }

    let content = if minimal {
        r#"nm|my-project
v|0.1.0
ds|A DX project
"#
    } else {
        r#"nm|my-project
v|0.1.0
tt|My DX Project
ds|A project using the DX ecosystem
au|developer
#f(repo|cont|pl|tk|it)
https://github.com/user/project|none|none|none|*cli,docs
#s(dv|st|build)
dx forge start|dx forge status|dx tools run bundler
"#
    };

    std::fs::write(&config_path, content)?;

    println!(
        "{} Created config file: {}",
        style("✓").green(),
        config_path.display()
    );

    Ok(())
}

/// Get a specific config value
async fn get_config(key: &str) -> Result<()> {
    use console::style;

    let config = DxConfig::load()?;

    let value = match key {
        "project.name" => Some(config.project.name),
        "project.version" => Some(config.project.version),
        "forge.lsp_port" => Some(config.forge.lsp_port.to_string()),
        "forge.auto_start" => Some(config.forge.auto_start.to_string()),
        "forge.debounce_ms" => Some(config.forge.debounce_ms.to_string()),
        _ => None,
    };

    match value {
        Some(v) => println!("{}", v),
        None => {
            println!(
                "{} Unknown config key: {}",
                style("✗").red(),
                key
            );
        }
    }

    Ok(())
}

/// Set a config value
async fn set_config(key: &str, value: &str) -> Result<()> {
    use console::style;

    // For now, just inform the user to edit the file manually
    // Full implementation would parse and modify the dx file
    println!(
        "{} Config modification not yet implemented",
        style("ℹ").blue()
    );
    println!(
        "  Edit the {} file directly to change: {} = {}",
        style("dx").cyan(),
        key,
        value
    );

    Ok(())
}

/// Show config file path
async fn show_path() -> Result<()> {
    use console::style;

    match find_config_file()? {
        Some(path) => {
            println!("{}", path.display());
        }
        None => {
            println!(
                "{} No dx config file found in current directory or parents",
                style("ℹ").blue()
            );
            println!(
                "  Create one with: {}",
                style("dx config init").cyan()
            );
        }
    }

    Ok(())
}

/// Find the dx config file by searching up the directory tree
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
