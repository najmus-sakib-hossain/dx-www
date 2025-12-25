//! Python version management commands

use std::path::Path;

use dx_py_core::Result;
use dx_py_project_manager::PythonManager;

/// Install a Python version
pub fn install(version: &str) -> Result<()> {
    println!("Installing Python {}...", version);

    let manager = PythonManager::new();
    let install_path = manager.version_path(version);

    if manager.is_installed(version) {
        println!("Python {} is already installed at {}", version, install_path.display());
        return Ok(());
    }

    // In a real implementation, we would download from python-build-standalone
    println!("Downloading Python {} from python-build-standalone...", version);
    println!("(This is a placeholder - actual download not implemented)");

    println!("\nTo install Python manually:");
    println!("  1. Download from https://github.com/indygreg/python-build-standalone/releases");
    println!("  2. Extract to {}", install_path.display());

    Ok(())
}

/// List installed Python versions
pub fn list() -> Result<()> {
    let mut manager = PythonManager::new();
    let installations = manager.discover();

    if installations.is_empty() {
        println!("No Python installations found.");
        println!("\nTo install Python:");
        println!("  dx-py python install 3.12.0");
        return Ok(());
    }

    println!("Installed Python versions:\n");

    for install in installations {
        let marker = if install.is_managed {
            " (managed by dx-py)"
        } else if install.is_system {
            " (system)"
        } else {
            ""
        };

        println!("  {} @ {}{}", install.version, install.path.display(), marker);
    }

    Ok(())
}

/// Pin Python version for the current project
pub fn pin(version: &str) -> Result<()> {
    let project_dir = Path::new(".");

    let manager = PythonManager::new();
    manager.pin(project_dir, version)?;

    println!("Pinned Python version to {} in .python-version", version);

    Ok(())
}

/// Show which Python would be used
pub fn which() -> Result<()> {
    let project_dir = Path::new(".");

    let mut manager = PythonManager::new();

    // Check for pinned version
    if let Some(pinned) = manager.read_pin(project_dir)? {
        println!("Pinned version: {}", pinned);

        manager.discover();
        if let Some(install) = manager.find(&pinned) {
            println!("Python path: {}", install.path.display());
        } else {
            println!("Warning: Pinned version {} not found", pinned);
            println!("Run 'dx-py python install {}' to install it", pinned);
        }
    } else {
        // Use first available
        let installations = manager.discover();
        if let Some(install) = installations.first() {
            println!("Using: {} @ {}", install.version, install.path.display());
            println!("(No .python-version file found, using first available)");
        } else {
            println!("No Python installation found.");
            println!("Run 'dx-py python install <version>' to install Python.");
        }
    }

    Ok(())
}
