//! Global tool management (pipx replacement)

use std::path::PathBuf;

use dx_py_core::Result;

/// Get the tools directory
fn tools_dir() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("dx-py")
        .join("tools")
}

/// Install a tool globally
pub fn install(name: &str, python: Option<&str>) -> Result<()> {
    let tools_path = tools_dir();
    let tool_path = tools_path.join(name);

    if tool_path.exists() {
        println!("Tool '{}' is already installed.", name);
        println!("Run 'dx-py tool uninstall {}' to remove it first.", name);
        return Ok(());
    }

    println!("Installing tool '{}'...", name);

    if let Some(py) = python {
        println!("Using Python {}", py);
    }

    // In a real implementation, we would:
    // 1. Create an isolated virtual environment
    // 2. Install the package
    // 3. Create wrapper scripts in a bin directory

    println!("(This is a placeholder - actual installation not implemented)");
    println!("\nTo install tools manually:");
    println!("  pip install --user {}", name);

    Ok(())
}

/// Run a tool ephemerally
pub fn run(name: &str, args: &[String]) -> Result<()> {
    println!("Running tool '{}' ephemerally...", name);

    if !args.is_empty() {
        println!("Arguments: {}", args.join(" "));
    }

    // In a real implementation, we would:
    // 1. Create a temporary virtual environment
    // 2. Install the package
    // 3. Run the command
    // 4. Clean up

    println!("(This is a placeholder - actual execution not implemented)");
    println!("\nTo run tools manually:");
    println!("  pipx run {} {}", name, args.join(" "));

    Ok(())
}

/// List installed tools
pub fn list() -> Result<()> {
    let tools_path = tools_dir();

    if !tools_path.exists() {
        println!("No tools installed.");
        println!("\nTo install a tool:");
        println!("  dx-py tool install <name>");
        return Ok(());
    }

    let entries: Vec<_> = std::fs::read_dir(&tools_path)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .collect();

    if entries.is_empty() {
        println!("No tools installed.");
        return Ok(());
    }

    println!("Installed tools:\n");

    for entry in entries {
        let name = entry.file_name();
        println!("  {}", name.to_string_lossy());
    }

    Ok(())
}

/// Uninstall a tool
pub fn uninstall(name: &str) -> Result<()> {
    let tools_path = tools_dir();
    let tool_path = tools_path.join(name);

    if !tool_path.exists() {
        println!("Tool '{}' is not installed.", name);
        return Ok(());
    }

    println!("Uninstalling tool '{}'...", name);

    std::fs::remove_dir_all(&tool_path)?;

    println!("Tool '{}' uninstalled.", name);

    Ok(())
}
