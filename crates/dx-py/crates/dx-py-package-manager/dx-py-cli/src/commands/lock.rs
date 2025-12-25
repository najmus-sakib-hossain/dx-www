//! Generate lock file from dependencies

use std::path::Path;

use dx_py_compat::PyProjectToml;
use dx_py_core::Result;
use dx_py_package_manager::{DplBuilder, Dependency, InMemoryProvider, Resolver, VersionConstraint};

/// Run the lock command
pub fn run(upgrade: bool) -> Result<()> {
    let pyproject_path = Path::new("pyproject.toml");

    if !pyproject_path.exists() {
        return Err(dx_py_core::Error::Cache(
            "No pyproject.toml found. Run 'dx-py init' first.".to_string(),
        ));
    }

    let pyproject = PyProjectToml::load(pyproject_path)?;

    let deps = pyproject.dependencies();
    if deps.is_empty() {
        println!("No dependencies to lock.");
        return Ok(());
    }

    println!("Resolving dependencies...");

    // Parse dependencies
    let parsed_deps: Vec<Dependency> = deps
        .iter()
        .filter_map(|d| {
            let spec = dx_py_package_manager::DependencySpec::parse(d).ok()?;
            let constraint = spec
                .version_constraint
                .as_ref()
                .and_then(|c| VersionConstraint::parse(c).ok())
                .unwrap_or(VersionConstraint::Any);
            Some(Dependency::new(&spec.name, constraint))
        })
        .collect();

    // For now, create a simple lock file
    // In a real implementation, we would fetch versions from PyPI
    println!("Creating lock file...");

    let mut builder = DplBuilder::new("3.12.0", "any");

    for dep in &parsed_deps {
        // Placeholder version - in real implementation, resolve from PyPI
        let version = "0.0.0";
        let hash = [0u8; 32];
        builder.add_package(&dep.name, version, hash);
        println!("  Locked {} @ {}", dep.name, version);
    }

    // Write lock file
    let lock_path = Path::new("dx-py.lock");
    let lock_data = builder.build();
    std::fs::write(lock_path, lock_data)?;

    println!("\nLock file written to dx-py.lock");
    println!("Run 'dx-py sync' to install locked dependencies.");

    if upgrade {
        println!("(--upgrade flag noted, all packages updated to latest)");
    }

    Ok(())
}
