//! Add command

use anyhow::Result;

pub async fn run(package: &str, dev: bool, verbose: bool) -> Result<()> {
    if verbose {
        println!("📦 Adding {} to {}", package, if dev { "devDependencies" } else { "dependencies" });
    }

    // TODO: Implement full add logic
    println!("⏳ Add command - Coming soon!");
    println!("Will add {} to package.json", package);

    Ok(())
}
