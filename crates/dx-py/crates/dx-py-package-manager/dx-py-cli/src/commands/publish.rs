//! Publish package to PyPI

use std::path::Path;

use dx_py_package_manager::{PublishClient, DEFAULT_REPOSITORY_URL, TEST_PYPI_URL};
use dx_py_core::Result;

/// Run the publish command
pub fn run(repository: Option<&str>, token: Option<&str>, files: &str) -> Result<()> {
    // Determine repository URL
    let repo_url = match repository {
        Some("testpypi") => TEST_PYPI_URL,
        Some(url) => url,
        None => DEFAULT_REPOSITORY_URL,
    };

    // Get API token
    let api_token = match token {
        Some(t) => t.to_string(),
        None => {
            // Try environment variables
            std::env::var("DX_PY_TOKEN")
                .or_else(|_| std::env::var("TWINE_PASSWORD"))
                .map_err(|_| {
                    dx_py_core::Error::Cache(
                        "No API token provided. Use --token or set DX_PY_TOKEN environment variable"
                            .to_string(),
                    )
                })?
        }
    };

    // Parse file paths
    let file_paths: Vec<&Path> = files
        .split(',')
        .map(|s| Path::new(s.trim()))
        .collect();

    // Validate files exist
    for path in &file_paths {
        if !path.exists() {
            return Err(dx_py_core::Error::Cache(format!(
                "File not found: {}",
                path.display()
            )));
        }
    }

    println!("Publishing to {}...", repo_url);
    println!("Files: {:?}", file_paths);

    // Create publish client
    let client = PublishClient::with_repository(repo_url);

    // Upload each file
    for path in &file_paths {
        println!("\nUploading {}...", path.display());
        match client.upload(path, &api_token) {
            Ok(result) => {
                println!("  ✓ Uploaded {} v{}", result.name, result.version);
                println!("    SHA256: {}", result.sha256);
            }
            Err(e) => {
                eprintln!("  ✗ Failed to upload: {}", e);
                return Err(e);
            }
        }
    }

    println!("\nPublish complete!");

    Ok(())
}
