/**
 * `dx new` - Project scaffolder
 * 
 * Creates a new dx-www project with the Binary Dawn v1.0 structure.
 * 
 * Structure:
 * - dx (God file)
 * - .dx/ (System)
 * - media/ (Assets)
 * - pages/ (Routes)
 * - units/ (Components)
 * - server/ (Backend)
 * - pwa/ (Manifest)
 */

use anyhow::{Context, Result};
use console::style;
use std::fs;
use std::path::{Path, PathBuf};

pub async fn execute(name: &str, path: Option<PathBuf>, template: &str) -> Result<()> {
    let target_dir = path.unwrap_or_else(|| PathBuf::from(name));

    println!("{} {}", style("Binary Dawn v1.0:").bold().magenta(), style(name).cyan());
    println!("{} {}\n", style("Location:").bold(), style(target_dir.display()).cyan());

    // Create Binary Dawn Directory Structure
    create_binary_dawn_structure(&target_dir)?;

    // Create God File (dx)
    create_god_file(&target_dir, name)?;

    // Generate specific template content if needed (for now, default to binary-dawn starter)
    generate_binary_dawn_starter(&target_dir, name)?;

    println!("\n{}", style("🔥 Project created successfully!").green().bold());
    println!("{}", style("The structure is perfect.").dim());
    println!("\n{}", style("Next steps:").bold());
    println!("  {} cd {}", style("1.").cyan(), name);
    println!("  {} dx dev", style("2.").cyan());
    println!("\n{}", style("Your app is ready for 1 January 2026.").dim());

    Ok(())
}

/// Create the complete Binary Dawn v1.0 folder structure
fn create_binary_dawn_structure(target: &Path) -> Result<()> {
    let dirs = [
        // .dx System Directory
        ".dx/cache",
        ".dx/forge",
        ".dx/style",
        ".dx/www",
        ".dx/temp",

        // Media Reactor
        "media/images",
        "media/video",
        "media/audio",
        "media/fonts",
        "media/raw",

        // Routes
        "pages",
        "pages/(marketing)",
        "pages/(auth)",
        "pages/(app)/projects",
        "pages/(app)/settings",

        // Units (Components)
        "units/ui",
        "units/cart",
        "units/auth",
        "units/analytics",

        // Server (Binary Backend)
        "server/api",
        "server/actions",
        "server/cron",

        // PWA
        "pwa/icons",
    ];

    let pb = indicatif::ProgressBar::new(dirs.len() as u64);
    pb.set_style(indicatif::ProgressStyle::default_bar()
        .template("{spinner:.green} {msg}")
        .unwrap());

    for dir in dirs {
        pb.set_message(format!("Creating {}...", dir));
        fs::create_dir_all(target.join(dir))
            .with_context(|| format!("Failed to create directory: {}", dir))?;
        pb.inc(1);
    }
    
    pb.finish_with_message("Structure created.");
    println!("  {} Folder structure (Binary Dawn v1.0)", style("✓").green());

    Ok(())
}

/// Create the 'dx' God File
fn create_god_file(target: &Path, name: &str) -> Result<()> {
    let config = format!(r#"# dx - God File
# Binary Dawn v1.0 Config
    
name: "{}"
version: "1.0.0"
    
[build]
# dx knows what to do.
target: "wasm32-unknown-unknown"
optimize: "max"
    
[server]
port: 3000
compression: "brotli"
"#, name);

    fs::write(target.join("dx"), config)
        .with_context(|| "Failed to write dx file")?;

    println!("  {} dx (God File)", style("✓").green());
    Ok(())
}

/// Generate starter files for the structure
fn generate_binary_dawn_starter(target: &Path, name: &str) -> Result<()> {
    // 1. pages/index.dx
    let index_dx = format!(r#"import {{ Button }} from 'units/ui/button';

export default function Landing() {{
  return (
    <div class="flex flex-col items-center justify-center min-h-screen bg-black text-white">
        <h1 class="text-6xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-blue-400 to-purple-600">
            {name}
        </h1>
        <p class="mt-4 text-xl text-gray-400">Binary Dawn v1.0</p>
        <div class="mt-8">
            <Button>Get Started</Button>
        </div>
    </div>
  );
}}
"#);
    fs::write(target.join("pages/index.dx"), index_dx)?;

    // 2. files in units/ui/button.dx
    let button_dx = r#"export function Button({ children, ...props }) {
    return (
        <button 
            class="px-6 py-3 rounded-lg bg-white text-black font-bold hover:scale-105 transition-transform"
            {...props}
        >
            {children}
        </button>
    );
}
"#;
    fs::write(target.join("units/ui/button.dx"), button_dx)?;

    // 3. pages/_layout.dx
    let layout_dx = r#"export default function Layout({ children }) {
    return (
        <main>
            {children}
        </main>
    );
}
"#;
    fs::write(target.join("pages/_layout.dx"), layout_dx)?;

    // 4. pwa/manifest.dx
    let manifest_dx = format!(r##"{{
    "name": "{}",
    "short_name": "{}",
    "start_url": "/",
    "display": "standalone",
    "background_color": "#000000",
    "theme_color": "#000000",
    "icons": [
        {{ "src": "/icons/icon-192.png", "sizes": "192x192", "type": "image/png" }},
        {{ "src": "/icons/icon-512.png", "sizes": "512x512", "type": "image/png" }}
    ]
}}
"##, name, name);
    fs::write(target.join("pwa/manifest.dx"), manifest_dx)?;

    // 5. Create .gitignore
    let gitignore = r#"# dx system
.dx/temp/
.dx/cache/

# Environment
.env
.env.local

# IDE
.vscode/
.idea/
"#;
    fs::write(target.join(".gitignore"), gitignore)?;
    
    println!("  {} Starter files (index, layout, button)", style("✓").green());

    Ok(())
}
