//! Tool Management Commands
//!
//! Commands for listing and running DX tools.

use anyhow::Result;
use clap::Subcommand;

use crate::daemon_client::DaemonClient;

/// Tool management commands
#[derive(Subcommand, Debug)]
pub enum ToolsCommands {
    /// List all registered tools
    List {
        /// Show detailed information
        #[arg(short, long)]
        verbose: bool,
    },

    /// Run a specific tool
    Run {
        /// Tool name to run
        tool: String,

        /// Additional arguments for the tool
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },

    /// Enable a tool
    Enable {
        /// Tool name to enable
        tool: String,
    },

    /// Disable a tool
    Disable {
        /// Tool name to disable
        tool: String,
    },
}

impl ToolsCommands {
    pub async fn execute(self) -> Result<()> {
        match self {
            ToolsCommands::List { verbose } => list_tools(verbose).await,
            ToolsCommands::Run { tool, args } => run_tool(&tool, args).await,
            ToolsCommands::Enable { tool } => enable_tool(&tool).await,
            ToolsCommands::Disable { tool } => disable_tool(&tool).await,
        }
    }
}

/// List all registered tools
async fn list_tools(verbose: bool) -> Result<()> {
    use console::style;

    if !DaemonClient::is_daemon_running() {
        println!(
            "{} Forge daemon is not running",
            style("⚠").yellow()
        );
        println!(
            "  Start with: {}",
            style("dx forge start").cyan()
        );
        return Ok(());
    }

    let mut client = DaemonClient::connect()?;
    let tools = client.list_tools()?;

    println!(
        "{} Registered Tools ({} total)",
        style("🔧").cyan(),
        tools.len()
    );
    println!();

    for tool in tools {
        let status_icon = match tool.status.as_str() {
            "Ready" => style("●").green(),
            "Running" => style("◐").yellow(),
            "Disabled" => style("○").dim(),
            "Error" => style("●").red(),
            _ => style("?").dim(),
        };

        let dummy_tag = if tool.is_dummy {
            style(" [dummy]").dim().to_string()
        } else {
            String::new()
        };

        println!(
            "  {} {:<20} v{:<10} {}{}",
            status_icon,
            style(&tool.name).bold(),
            tool.version,
            tool.status,
            dummy_tag
        );

        if verbose {
            println!("      ID:         {}", tool.id);
            if let Some(last_run) = &tool.last_run {
                println!("      Last run:   {}", last_run);
            }
            println!("      Run count:  {}", tool.run_count);
            println!("      Errors:     {}", tool.error_count);
            println!();
        }
    }

    Ok(())
}

/// Run a specific tool
async fn run_tool(tool_name: &str, args: Vec<String>) -> Result<()> {
    use console::style;

    if !DaemonClient::is_daemon_running() {
        println!(
            "{} Forge daemon is not running",
            style("⚠").yellow()
        );
        println!(
            "  Start with: {}",
            style("dx forge start").cyan()
        );
        return Ok(());
    }

    let mut client = DaemonClient::connect()?;

    println!(
        "{} Running tool: {}",
        style("▶").cyan(),
        style(tool_name).bold()
    );

    if !args.is_empty() {
        println!("  Args: {:?}", args);
    }

    let start = std::time::Instant::now();
    let result = client.run_tool(tool_name, args)?;
    let duration = start.elapsed();

    if result.success {
        println!(
            "{} {} completed in {:?}",
            style("✓").green(),
            tool_name,
            duration
        );
        
        if result.warm_start {
            println!("  {} Warm start (cached)", style("⚡").yellow());
        }

        if result.cache_hits > 0 {
            println!(
                "  Cache: {} hits, {} misses",
                result.cache_hits,
                result.cache_misses
            );
        }

        if let Some(output) = result.output {
            if !output.is_empty() {
                println!();
                println!("Output:");
                println!("{}", output);
            }
        }
    } else {
        println!(
            "{} {} failed",
            style("✗").red(),
            tool_name
        );
        
        if let Some(error) = result.error {
            println!("  Error: {}", style(error).red());
        }
    }

    Ok(())
}

/// Enable a tool
async fn enable_tool(tool_name: &str) -> Result<()> {
    use console::style;

    if !DaemonClient::is_daemon_running() {
        println!(
            "{} Forge daemon is not running",
            style("⚠").yellow()
        );
        return Ok(());
    }

    let mut client = DaemonClient::connect()?;
    client.enable_tool(tool_name)?;

    println!(
        "{} Tool {} enabled",
        style("✓").green(),
        style(tool_name).bold()
    );

    Ok(())
}

/// Disable a tool
async fn disable_tool(tool_name: &str) -> Result<()> {
    use console::style;

    if !DaemonClient::is_daemon_running() {
        println!(
            "{} Forge daemon is not running",
            style("⚠").yellow()
        );
        return Ok(());
    }

    let mut client = DaemonClient::connect()?;
    client.disable_tool(tool_name)?;

    println!(
        "{} Tool {} disabled",
        style("○").dim(),
        style(tool_name).bold()
    );

    Ok(())
}
