//! Run a command in the virtual environment

use std::path::Path;
use std::process::Command;

use dx_py_core::Result;

/// Run the run command
pub fn run(command: &[String]) -> Result<()> {
    if command.is_empty() {
        return Err(dx_py_core::Error::Cache(
            "No command specified. Usage: dx-py run <command> [args...]".to_string(),
        ));
    }

    let venv_path = Path::new(".venv");
    if !venv_path.exists() {
        return Err(dx_py_core::Error::Cache(
            "No virtual environment found. Run 'dx-py init' first.".to_string(),
        ));
    }

    // Get the bin/Scripts directory
    #[cfg(unix)]
    let bin_dir = venv_path.join("bin");
    #[cfg(windows)]
    let bin_dir = venv_path.join("Scripts");

    // Build the PATH with venv bin directory first
    let path_var = std::env::var("PATH").unwrap_or_default();
    #[cfg(unix)]
    let new_path = format!("{}:{}", bin_dir.display(), path_var);
    #[cfg(windows)]
    let new_path = format!("{};{}", bin_dir.display(), path_var);

    // Check if the command exists in the venv
    let cmd_name = &command[0];
    #[cfg(unix)]
    let cmd_path = bin_dir.join(cmd_name);
    #[cfg(windows)]
    let cmd_path = bin_dir.join(format!("{}.exe", cmd_name));

    let actual_cmd = if cmd_path.exists() {
        cmd_path.to_string_lossy().to_string()
    } else {
        cmd_name.clone()
    };

    // Run the command
    let status = Command::new(&actual_cmd)
        .args(&command[1..])
        .env("PATH", &new_path)
        .env("VIRTUAL_ENV", venv_path)
        .status()
        .map_err(|e| dx_py_core::Error::Cache(format!("Failed to run command: {}", e)))?;

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }

    Ok(())
}
