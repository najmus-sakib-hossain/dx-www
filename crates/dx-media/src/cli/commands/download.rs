//! Download command implementation.

use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};

use crate::DxMedia;
use crate::cli::args::DownloadArgs;
use crate::error::{DxError, Result};

/// Execute the download command.
pub async fn execute(args: DownloadArgs, quiet: bool) -> Result<()> {
    let dx = DxMedia::new()?;

    // Parse asset ID (format: provider:id)
    let (provider_name, asset_id) = parse_asset_id(&args.asset_id)?;

    // First, search for the asset to get full details
    if !quiet {
        println!("{} {}:{}", "Looking up".cyan(), provider_name, asset_id);
    }

    // We need to search for the asset to get the download URL
    // This is a limitation - ideally we'd have a get_by_id endpoint
    let query = crate::types::SearchQuery::new(asset_id);
    let search_result = dx.search_query(&query).await?;

    // Find the matching asset
    let asset = search_result
        .assets
        .iter()
        .find(|a| a.provider == provider_name && a.id == asset_id)
        .or_else(|| {
            // If exact match not found, try finding by ID only
            search_result.assets.iter().find(|a| a.id == asset_id)
        })
        .ok_or_else(|| DxError::NoResults {
            query: args.asset_id.clone(),
        })?;

    // Show progress
    let spinner = if !quiet {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
                .template("{spinner:.cyan} {msg}")
                .unwrap(),
        );
        pb.set_message(format!("Downloading '{}'...", asset.title));
        pb.enable_steady_tick(std::time::Duration::from_millis(80));
        Some(pb)
    } else {
        None
    };

    // Download
    let path = if let Some(ref output_dir) = args.output {
        dx.download_to(asset, std::path::Path::new(output_dir))
            .await?
    } else {
        dx.download(asset).await?
    };

    // Rename if custom filename provided
    if let Some(ref filename) = args.filename {
        let new_path = path
            .parent()
            .unwrap_or(std::path::Path::new("."))
            .join(filename);
        tokio::fs::rename(&path, &new_path)
            .await
            .map_err(|e| DxError::FileIo {
                path: path.clone(),
                message: format!("Failed to rename file: {}", e),
                source: Some(e),
            })?;

        if let Some(pb) = spinner {
            pb.finish_and_clear();
        }

        if !quiet {
            println!("{} {}", "Downloaded:".green().bold(), new_path.display());
        }
    } else {
        if let Some(pb) = spinner {
            pb.finish_and_clear();
        }

        if !quiet {
            println!("{} {}", "Downloaded:".green().bold(), path.display());
        }
    }

    Ok(())
}

/// Parse asset ID in format "provider:id" or just "id".
fn parse_asset_id(asset_id: &str) -> Result<(&str, &str)> {
    if let Some((provider, id)) = asset_id.split_once(':') {
        Ok((provider, id))
    } else {
        // Default to openverse if no provider specified
        Ok(("openverse", asset_id))
    }
}
