//! Global tool management (pipx replacement)

use dx_py_project_manager::ToolManager;
use dx_py_core::Result;

/// Install a tool globally
pub fn install(name: &str, _python: Option<&str>) -> Result<()> {
    let tool_manager = ToolManager::new()?;

    if tool_manager.is_installed(name) {
        println!("Tool '{}' is already installed.", name);
        println!("Run 'dx-py tool uninstall {}' to remove it first.", name);
        return Ok(());
    }

    println!("Installing tool '{}'...", name);

    match tool_manager.install(name) {
        Ok(installed) => {
            println!("✓ Tool '{}' installed successfully!", name);
            println!("  Location: {}", installed.tool_dir.display());
            if !installed.scripts.is_empty() {
                println!("  Scripts:");
                for script in &installed.scripts {
                    println!("    - {}", script.display());
                }
            }
            println!("\nMake sure {} is in your PATH", tool_manager.bin_dir().display());
        }
        Err(e) => {
            eprintln!("✗ Failed to install '{}': {}", name, e);
            return Err(e);
        }
    }

    Ok(())
}

/// Run a tool ephemerally
pub fn run(name: &str, args: &[String]) -> Result<()> {
    println!("Running tool '{}' ephemerally...", name);

    let tool_manager = ToolManager::new()?;

    match tool_manager.run_ephemeral(name, args) {
        Ok(exit_code) => {
            if exit_code != 0 {
                std::process::exit(exit_code);
            }
        }
        Err(e) => {
            eprintln!("✗ Failed to run '{}': {}", name, e);
            return Err(e);
        }
    }

    Ok(())
}

/// List installed tools
pub fn list() -> Result<()> {
    let tool_manager = ToolManager::new()?;
    let tools = tool_manager.list()?;

    if tools.is_empty() {
        println!("No tools installed.");
        println!("\nTo install a tool:");
        println!("  dx-py tool install <name>");
        return Ok(());
    }

    println!("Installed tools:\n");

    for name in tools {
        println!("  {}", name);
    }

    println!("\nTools directory: {}", tool_manager.tools_dir().display());
    println!("Bin directory: {}", tool_manager.bin_dir().display());

    Ok(())
}

/// Uninstall a tool
pub fn uninstall(name: &str) -> Result<()> {
    let tool_manager = ToolManager::new()?;

    if !tool_manager.is_installed(name) {
        println!("Tool '{}' is not installed.", name);
        return Ok(());
    }

    println!("Uninstalling tool '{}'...", name);

    match tool_manager.uninstall(name) {
        Ok(()) => {
            println!("✓ Tool '{}' uninstalled.", name);
        }
        Err(e) => {
            eprintln!("✗ Failed to uninstall '{}': {}", name, e);
            return Err(e);
        }
    }

    Ok(())
}

/// Upgrade a tool to the latest version
pub fn upgrade(name: &str) -> Result<()> {
    let tool_manager = ToolManager::new()?;

    if !tool_manager.is_installed(name) {
        println!("Tool '{}' is not installed.", name);
        return Ok(());
    }

    println!("Upgrading tool '{}'...", name);

    match tool_manager.upgrade(name) {
        Ok(()) => {
            println!("✓ Tool '{}' upgraded.", name);
        }
        Err(e) => {
            eprintln!("✗ Failed to upgrade '{}': {}", name, e);
            return Err(e);
        }
    }

    Ok(())
}
