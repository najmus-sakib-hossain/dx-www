#[macro_use]
extern crate lazy_static;
extern crate simple_error;

pub mod configuration;
pub mod error;
pub mod file_linter;
pub mod module_linter;

use module_linter::ModuleLinter;
use simple_error::{bail, SimpleError};
use walkdir::WalkDir;

/// Lint Go files in the given path
/// 
/// # Arguments
/// * `path` - Path to directory or file to lint (must contain or be in a module with go.mod)
/// * `fix` - Whether to automatically fix issues
/// 
/// # Returns
/// * `Ok(true)` - No linting errors found
/// * `Ok(false)` - Linting errors found
/// * `Err(_)` - Error during linting (e.g., no go.mod found)
pub fn lint(path: &str, fix: bool) -> Result<bool, SimpleError> {
    let mut go_mod_files = WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| {
            e.file_name()
                .to_str()
                .map(|s| {
                    !(e.path().join("..").join("go.mod").is_file()
                        || s.starts_with('.') && s != "." && s != "..")
                })
                .unwrap_or(false)
        })
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_name()
                .to_str()
                .map(|s| s == "go.mod")
                .unwrap_or(false)
        })
        .peekable();

    if go_mod_files.peek().is_none() {
        bail!("no go.mod file found in {}", path);
    }

    let mut exit = true;

    for file in go_mod_files {
        let module_linter = ModuleLinter::new(fix);

        let mut dir = file.path().to_path_buf();
        dir.pop();

        exit &= module_linter.run(dir.to_str().unwrap());
    }

    Ok(exit)
}
