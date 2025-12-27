//! Traffic Branch Control Commands
//!
//! Commands for managing the traffic branch system (Green/Yellow/Red).

use anyhow::Result;
use clap::Subcommand;

use crate::daemon_client::DaemonClient;

/// Branch control commands
#[derive(Subcommand, Debug)]
pub enum BranchCommands {
    /// Show current branch status and pending changes
    Status,

    /// Approve pending Yellow branch changes
    Approve {
        /// Approve all pending changes
        #[arg(short, long)]
        all: bool,

        /// Specific change ID to approve
        #[arg(short, long)]
        id: Option<String>,
    },

    /// Reject pending Yellow branch changes
    Reject {
        /// Reject all pending changes
        #[arg(short, long)]
        all: bool,

        /// Specific change ID to reject
        #[arg(short, long)]
        id: Option<String>,
    },

    /// Show branch decision history
    History {
        /// Number of entries to show
        #[arg(short, long, default_value = "20")]
        limit: usize,
    },
}

impl BranchCommands {
    pub async fn execute(self) -> Result<()> {
        match self {
            BranchCommands::Status => show_status().await,
            BranchCommands::Approve { all, id } => approve_changes(all, id).await,
            BranchCommands::Reject { all, id } => reject_changes(all, id).await,
            BranchCommands::History { limit } => show_history(limit).await,
        }
    }
}

/// Show branch status
async fn show_status() -> Result<()> {
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
    let status = client.get_branch_status()?;

    // Display current branch color
    let branch_icon = match status.current_color.as_str() {
        "Green" => style("🟢").to_string(),
        "Yellow" => style("🟡").to_string(),
        "Red" => style("🔴").to_string(),
        _ => style("⚪").to_string(),
    };

    println!(
        "{} Traffic Branch: {}",
        branch_icon,
        style(&status.current_color).bold()
    );
    println!();

    // Statistics
    println!("Statistics:");
    println!(
        "  Auto-approved (Green):  {}",
        style(status.auto_approved).green()
    );
    println!(
        "  Manual approved:        {}",
        style(status.manual_approved).cyan()
    );
    println!(
        "  Rejected:               {}",
        style(status.rejected).red()
    );
    println!();

    // Pending changes
    if status.pending_changes.is_empty() {
        println!(
            "{} No pending changes",
            style("✓").green()
        );
    } else {
        println!(
            "{} Pending Changes ({}):",
            style("⏳").yellow(),
            status.pending_changes.len()
        );
        println!();

        for change in &status.pending_changes {
            let color_icon = match change.color.as_str() {
                "Green" => style("🟢").to_string(),
                "Yellow" => style("🟡").to_string(),
                "Red" => style("🔴").to_string(),
                _ => style("⚪").to_string(),
            };

            let change_type = match change.change_type.as_str() {
                "Created" => style("+").green(),
                "Modified" => style("~").yellow(),
                "Deleted" => style("-").red(),
                _ => style("?").dim(),
            };

            println!(
                "  {} {} {} {}",
                color_icon,
                change_type,
                change.path,
                style(format!("[{}]", change.id)).dim()
            );

            if let Some(tool) = &change.tool {
                println!(
                    "      Tool: {}",
                    style(tool).cyan()
                );
            }
        }

        println!();
        println!(
            "Use {} to approve or {} to reject",
            style("dx branch approve --all").cyan(),
            style("dx branch reject --all").cyan()
        );
    }

    Ok(())
}

/// Approve pending changes
async fn approve_changes(all: bool, id: Option<String>) -> Result<()> {
    use console::style;

    if !DaemonClient::is_daemon_running() {
        println!(
            "{} Forge daemon is not running",
            style("⚠").yellow()
        );
        return Ok(());
    }

    let mut client = DaemonClient::connect()?;

    if all {
        let count = client.approve_all_pending()?;
        println!(
            "{} Approved {} pending change(s)",
            style("✓").green(),
            count
        );
    } else if let Some(change_id) = id {
        client.approve_change(&change_id)?;
        println!(
            "{} Approved change: {}",
            style("✓").green(),
            change_id
        );
    } else {
        println!(
            "{} Specify {} or {} to approve changes",
            style("ℹ").blue(),
            style("--all").cyan(),
            style("--id <ID>").cyan()
        );
    }

    Ok(())
}

/// Reject pending changes
async fn reject_changes(all: bool, id: Option<String>) -> Result<()> {
    use console::style;

    if !DaemonClient::is_daemon_running() {
        println!(
            "{} Forge daemon is not running",
            style("⚠").yellow()
        );
        return Ok(());
    }

    let mut client = DaemonClient::connect()?;

    if all {
        let count = client.reject_all_pending()?;
        println!(
            "{} Rejected {} pending change(s)",
            style("✗").red(),
            count
        );
    } else if let Some(change_id) = id {
        client.reject_change(&change_id)?;
        println!(
            "{} Rejected change: {}",
            style("✗").red(),
            change_id
        );
    } else {
        println!(
            "{} Specify {} or {} to reject changes",
            style("ℹ").blue(),
            style("--all").cyan(),
            style("--id <ID>").cyan()
        );
    }

    Ok(())
}

/// Show branch history
async fn show_history(limit: usize) -> Result<()> {
    use console::style;

    if !DaemonClient::is_daemon_running() {
        println!(
            "{} Forge daemon is not running",
            style("⚠").yellow()
        );
        return Ok(());
    }

    let mut client = DaemonClient::connect()?;
    let history = client.get_branch_history(limit)?;

    println!(
        "{} Branch History (last {} entries):",
        style("📜").cyan(),
        limit
    );
    println!();

    if history.is_empty() {
        println!("  No history available");
        return Ok(());
    }

    for entry in history {
        let action_icon = match entry.action.as_str() {
            "approved" => style("✓").green(),
            "rejected" => style("✗").red(),
            "auto" => style("⚡").yellow(),
            _ => style("?").dim(),
        };

        let color_icon = match entry.color.as_str() {
            "Green" => "🟢",
            "Yellow" => "🟡",
            "Red" => "🔴",
            _ => "⚪",
        };

        println!(
            "  {} {} {} {} {}",
            style(&entry.timestamp).dim(),
            color_icon,
            action_icon,
            entry.path,
            style(format!("[{}]", entry.action)).dim()
        );
    }

    Ok(())
}
