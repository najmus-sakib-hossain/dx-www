//! Remove command

use anyhow::Result;

pub async fn run(package: &str, verbose: bool) -> Result<()> {
    if verbose {
        println!("🗑️  Removing {}", package);
    }

    // TODO: Implement full remove logic
    println!("⏳ Remove command - Coming soon!");
    println!("Will remove {} from package.json", package);

    Ok(())
}
