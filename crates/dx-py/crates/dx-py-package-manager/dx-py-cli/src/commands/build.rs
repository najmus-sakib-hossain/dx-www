//! Build package for distribution

use std::path::Path;

use dx_py_compat::PyProjectToml;
use dx_py_core::Result;

/// Run the build command
pub fn run(output: &str, wheel_only: bool, sdist_only: bool) -> Result<()> {
    let pyproject_path = Path::new("pyproject.toml");

    if !pyproject_path.exists() {
        return Err(dx_py_core::Error::Cache(
            "No pyproject.toml found. Run 'dx-py init' first.".to_string(),
        ));
    }

    let pyproject = PyProjectToml::load(pyproject_path)?;

    let name = pyproject.name().ok_or_else(|| {
        dx_py_core::Error::Cache("No project name in pyproject.toml".to_string())
    })?;

    let version = pyproject.version().unwrap_or("0.0.0");

    println!("Building {} v{}...", name, version);

    // Create output directory
    let output_dir = Path::new(output);
    if !output_dir.exists() {
        std::fs::create_dir_all(output_dir)?;
    }

    let build_wheel = !sdist_only;
    let build_sdist = !wheel_only;

    if build_wheel {
        println!("Building wheel...");
        let wheel_name = format!(
            "{}-{}-py3-none-any.whl",
            name.replace('-', "_"),
            version
        );
        println!("  Would create: {}/{}", output, wheel_name);
    }

    if build_sdist {
        println!("Building sdist...");
        let sdist_name = format!("{}-{}.tar.gz", name, version);
        println!("  Would create: {}/{}", output, sdist_name);
    }

    println!("\n(This is a placeholder - actual build not implemented)");
    println!("\nTo build manually:");
    println!("  python -m build");

    Ok(())
}
