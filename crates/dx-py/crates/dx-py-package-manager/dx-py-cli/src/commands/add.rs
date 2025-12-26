//! Add dependencies to the project

use std::path::Path;

use dx_py_compat::PyProjectToml;
use dx_py_core::Result;

/// Run the add command
pub fn run(packages: &[String], dev: bool, optional: Option<&str>) -> Result<()> {
    let pyproject_path = Path::new("pyproject.toml");

    if !pyproject_path.exists() {
        return Err(dx_py_core::Error::Cache(
            "No pyproject.toml found. Run 'dx-py init' first.".to_string(),
        ));
    }

    let mut pyproject = PyProjectToml::load(pyproject_path)?;

    let project = pyproject.project.as_mut().ok_or_else(|| {
        dx_py_core::Error::Cache("No [project] section in pyproject.toml".to_string())
    })?;

    if dev || optional.is_some() {
        // Add to optional-dependencies
        let group = optional.unwrap_or("dev");
        let optional_deps = project.optional_dependencies.get_or_insert_with(Default::default);
        let group_deps = optional_deps.entry(group.to_string()).or_default();

        for package in packages {
            if !group_deps.contains(package) {
                group_deps.push(package.clone());
                println!("Added {} to [project.optional-dependencies.{}]", package, group);
            } else {
                println!("{} already in [project.optional-dependencies.{}]", package, group);
            }
        }
    } else {
        // Add to dependencies
        let deps = project.dependencies.get_or_insert_with(Vec::new);

        for package in packages {
            // Check if package already exists (by name, ignoring version)
            let pkg_name = package.split(['>', '<', '=', '!', '~'])
                .next()
                .unwrap_or(package);

            let existing_idx = deps.iter().position(|d| {
                d.split(['>', '<', '=', '!', '~'])
                    .next()
                    .map(|n| n == pkg_name)
                    .unwrap_or(false)
            });

            if let Some(idx) = existing_idx {
                deps[idx] = package.clone();
                println!("Updated {} in [project.dependencies]", package);
            } else {
                deps.push(package.clone());
                println!("Added {} to [project.dependencies]", package);
            }
        }
    }

    pyproject.save(pyproject_path)?;

    println!("\nRun 'dx-py install' to install the new dependencies.");

    Ok(())
}
