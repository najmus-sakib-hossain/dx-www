//! Build package for distribution

use std::path::Path;

use dx_py_package_manager::BuildFrontend;
use dx_py_core::Result;

/// Run the build command
pub fn run(output: &str, wheel_only: bool, sdist_only: bool) -> Result<()> {
    let project_dir = std::env::current_dir()
        .map_err(|e| dx_py_core::Error::Cache(format!("Failed to get current directory: {}", e)))?;

    let build_frontend = BuildFrontend::new(&project_dir)?;

    let name = build_frontend.name().ok_or_else(|| {
        dx_py_core::Error::Cache("No project name in pyproject.toml".to_string())
    })?;

    let version = build_frontend.version().unwrap_or("0.0.0");

    println!("Building {} v{}...", name, version);
    println!("  Build backend: {}", build_frontend.build_backend());
    println!("  Build requires: {:?}", build_frontend.build_requires());

    // Create output directory
    let output_dir = Path::new(output);
    if !output_dir.exists() {
        std::fs::create_dir_all(output_dir)?;
    }

    let build_wheel = !sdist_only;
    let build_sdist = !wheel_only;

    if build_wheel {
        println!("\nBuilding wheel...");
        match build_frontend.build_wheel(output_dir) {
            Ok(wheel_path) => {
                println!("  Created: {}", wheel_path.display());
            }
            Err(e) => {
                eprintln!("  Failed to build wheel: {}", e);
                return Err(e);
            }
        }
    }

    if build_sdist {
        println!("\nBuilding sdist...");
        match build_frontend.build_sdist(output_dir) {
            Ok(sdist_path) => {
                println!("  Created: {}", sdist_path.display());
            }
            Err(e) => {
                eprintln!("  Failed to build sdist: {}", e);
                return Err(e);
            }
        }
    }

    println!("\nBuild complete!");

    Ok(())
}
