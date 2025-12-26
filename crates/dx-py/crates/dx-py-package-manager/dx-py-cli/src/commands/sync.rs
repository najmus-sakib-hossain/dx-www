//! Install packages from lock file
//!
//! This command reads the lock file and installs all packages to the
//! virtual environment, using the cache when available.

use std::path::Path;

use dx_py_core::Result;
use dx_py_package_manager::{
    AsyncPyPiClient, DplLockFile, GlobalCache, WheelInstaller,
};

/// Run the sync command
pub fn run(dev: bool, extras: &[String]) -> Result<()> {
    let lock_path = Path::new("dx-py.lock");

    if !lock_path.exists() {
        return Err(dx_py_core::Error::Cache(
            "No lock file found. Run 'dx-py lock' first.".to_string(),
        ));
    }

    let venv_path = Path::new(".venv");
    if !venv_path.exists() {
        return Err(dx_py_core::Error::Cache(
            "No virtual environment found. Run 'dx-py init' first.".to_string(),
        ));
    }

    println!("Reading lock file...");

    let lock_file = DplLockFile::open(lock_path)?;
    let package_count = lock_file.package_count();

    if package_count == 0 {
        println!("No packages to install.");
        return Ok(());
    }

    println!("Installing {} packages...", package_count);

    // Set up cache and installer
    let cache_dir = dirs::cache_dir()
        .map(|p| p.join("dx-py"))
        .unwrap_or_else(|| Path::new(".dx-py-cache").to_path_buf());
    
    let cache = GlobalCache::new(&cache_dir)?;

    // Determine site-packages path
    #[cfg(unix)]
    let site_packages = venv_path.join("lib/python3.12/site-packages");
    #[cfg(windows)]
    let site_packages = venv_path.join("Lib/site-packages");

    std::fs::create_dir_all(&site_packages)?;

    let installer = WheelInstaller::new(GlobalCache::new(&cache_dir)?, site_packages.clone());

    // Create async runtime for downloads
    let rt = tokio::runtime::Runtime::new()
        .map_err(|e| dx_py_core::Error::Cache(format!("Failed to create runtime: {}", e)))?;

    let client = AsyncPyPiClient::new();
    let platform_env = dx_py_core::wheel::PlatformEnvironment::detect();

    let mut installed_count = 0;
    let mut cached_count = 0;
    let mut download_count = 0;

    // Iterate through locked packages
    for entry in lock_file.iter() {
        let name = entry.name_str();
        let version = entry.version_str();
        let hash = entry.source_hash;

        print!("  {} @ {} ... ", name, version);

        // Check if already in cache
        if cache.contains(&hash) {
            // Install from cache
            match installer.install_from_cache(&hash) {
                Ok(_installed) => {
                    println!("✓ (cached)");
                    cached_count += 1;
                    installed_count += 1;
                }
                Err(e) => {
                    println!("✗ cache error: {}", e);
                    // Try downloading instead
                    if let Err(e) = rt.block_on(download_and_install(
                        &client, &installer, &cache, name, version, &platform_env
                    )) {
                        eprintln!("    Failed to download: {}", e);
                        continue;
                    }
                    download_count += 1;
                    installed_count += 1;
                }
            }
        } else {
            // Download from PyPI
            match rt.block_on(download_and_install(
                &client, &installer, &cache, name, version, &platform_env
            )) {
                Ok(()) => {
                    println!("✓ (downloaded)");
                    download_count += 1;
                    installed_count += 1;
                }
                Err(e) => {
                    println!("✗ {}", e);
                }
            }
        }
    }

    if dev {
        println!("\n  (including dev dependencies)");
    }

    if !extras.is_empty() {
        println!("  (including extras: {})", extras.join(", "));
    }

    println!("\nInstallation complete!");
    println!("  {} packages installed", installed_count);
    println!("  {} from cache, {} downloaded", cached_count, download_count);

    Ok(())
}

/// Download a package from PyPI and install it
async fn download_and_install(
    client: &AsyncPyPiClient,
    installer: &WheelInstaller,
    cache: &GlobalCache,
    name: &str,
    version: &str,
    platform_env: &dx_py_core::wheel::PlatformEnvironment,
) -> Result<()> {
    // Find best wheel for this platform
    let dist = client.find_distribution(name, version, platform_env).await?
        .ok_or_else(|| dx_py_core::Error::Cache(
            format!("No compatible distribution found for {}=={}", name, version)
        ))?;

    // Download the wheel
    let data = client.download(&dist.url, &dist.digests.sha256).await?;

    // Store in cache
    let hash = {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(&data);
        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        hash
    };
    cache.store(&hash, &data)?;

    // Install the wheel
    installer.install_wheel(&data)?;

    Ok(())
}
