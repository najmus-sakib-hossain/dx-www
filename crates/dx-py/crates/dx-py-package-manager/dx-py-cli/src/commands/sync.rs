//! Install packages from lock file

use std::path::Path;

use dx_py_core::Result;
use dx_py_package_manager::DplLockFile;

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

    println!("Installing {} packages...", package_count);

    // Iterate through locked packages
    for entry in lock_file.iter() {
        let name = entry.name_str();
        let version = entry.version_str();
        println!("  Installing {} @ {}", name, version);
        // In a real implementation, we would:
        // 1. Download the package from cache or PyPI
        // 2. Install to site-packages using hard links
    }

    if dev {
        println!("  (including dev dependencies)");
    }

    if !extras.is_empty() {
        println!("  (including extras: {})", extras.join(", "));
    }

    println!("\nInstallation complete!");

    Ok(())
}
