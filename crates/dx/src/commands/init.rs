//! Initialize a new DX project

use anyhow::{Context, Result};
use clap::Args;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::fs;
use std::path::PathBuf;

use crate::templates;
use crate::ui::{spinner::Spinner, theme::Theme};

#[derive(Args)]
pub struct InitArgs {
    /// Project name (defaults to current directory)
    #[arg(index = 1)]
    pub name: Option<String>,

    /// Template to use
    #[arg(short, long, value_parser = ["counter", "dashboard", "hackernews", "minimal", "api"])]
    pub template: Option<String>,

    /// Skip interactive prompts
    #[arg(short = 'y', long)]
    pub yes: bool,

    /// Install dependencies after init
    #[arg(long, default_value = "true")]
    pub install: bool,

    /// Initialize git repository
    #[arg(long, default_value = "true")]
    pub git: bool,

    /// Directory to create project in
    #[arg(short, long)]
    pub dir: Option<PathBuf>,
}

pub async fn run(args: InitArgs, theme: &Theme) -> Result<()> {
    let (project_name, template) = if args.yes {
        // Non-interactive mode
        let name = args.name.unwrap_or_else(|| "dx-app".to_string());
        let template = args.template.unwrap_or_else(|| "counter".to_string());
        (name, template)
    } else {
        // Interactive mode
        interactive_init(&args)?
    };

    let project_dir = args.dir.unwrap_or_else(|| PathBuf::from(&project_name));

    // Create project
    let spinner = Spinner::dots(&format!("Creating {} project...", project_name));

    // Check if directory exists
    if project_dir.exists() {
        spinner.warn(&format!(
            "Directory '{}' already exists",
            project_dir.display()
        ));
        return Ok(());
    }

    // Create directory structure
    create_project_structure(&project_dir, &project_name, &template)?;

    spinner.success(&format!("Created project '{}'", project_name));

    // Initialize git
    if args.git {
        let git_spinner = Spinner::dots("Initializing git repository...");
        if init_git(&project_dir).is_ok() {
            git_spinner.success("Initialized git repository");
        } else {
            git_spinner.warn("Could not initialize git (git not found)");
        }
    }

    // Install dependencies
    if args.install {
        let install_spinner = Spinner::dots("Installing dependencies...");
        // Simulate install - in real implementation, call dx-js-package-manager
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        install_spinner.success("Installed dependencies");
    }

    // Print success message and next steps
    theme.print_success(&format!("Project '{}' created successfully!", project_name));

    eprintln!();
    theme.print_section("Next steps:");
    eprintln!();

    if project_dir != PathBuf::from(".") {
        crate::ui::logger::step(1, &format!("cd {}", project_dir.display()));
    }
    crate::ui::logger::step(2, "dx dev");

    eprintln!();
    theme.print_link("Docs", "https://dx.dev/docs");
    eprintln!();

    Ok(())
}

fn interactive_init(args: &InitArgs) -> Result<(String, String)> {
    let theme = ColorfulTheme::default();

    eprintln!();

    // Get project name
    let name: String = if let Some(ref name) = args.name {
        name.clone()
    } else {
        Input::with_theme(&theme)
            .with_prompt("  Project name")
            .default("dx-app".to_string())
            .interact_text()?
    };

    // Select template
    let templates = ["counter", "dashboard", "hackernews", "minimal", "api"];
    let template_descriptions = [
        "counter      - Simple counter example (recommended for beginners)",
        "dashboard    - SaaS dashboard with charts and tables",
        "hackernews   - Hacker News clone (real-world example)",
        "minimal      - Bare minimum setup",
        "api          - API server only (no frontend)",
    ];

    let template = if let Some(ref t) = args.template {
        t.clone()
    } else {
        let selection = Select::with_theme(&theme)
            .with_prompt("  Select a template")
            .items(&template_descriptions)
            .default(0)
            .interact()?;

        templates[selection].to_string()
    };

    Ok((name, template))
}

fn create_project_structure(dir: &PathBuf, name: &str, template: &str) -> Result<()> {
    fs::create_dir_all(dir)?;
    fs::create_dir_all(dir.join("src"))?;
    fs::create_dir_all(dir.join("public"))?;

    // Create dx.toml
    let config = format!(
        r#"[project]
name = "{name}"
version = "0.1.0"

[build]
target = "browser"
minify = true

[dev]
port = 3000
open = true

[runtime]
jsx = "dx"
typescript = true
"#
    );
    fs::write(dir.join("dx.toml"), config)?;

    // Create main file based on template
    let main_content = match template {
        "counter" => templates::COUNTER,
        "minimal" => templates::MINIMAL,
        _ => templates::COUNTER,
    };
    fs::write(dir.join("src/main.tsx"), main_content)?;

    // Create index.html
    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{name}</title>
</head>
<body>
    <div id="root"></div>
    <script type="module" src="/src/main.tsx"></script>
</body>
</html>
"#
    );
    fs::write(dir.join("index.html"), html)?;

    // Create .gitignore
    let gitignore = r#"# Build output
dist/
.dx/

# Dependencies
node_modules/
dx_modules/

# IDE
.idea/
*.swp
*.swo
.DS_Store

# Logs
*.log
"#;
    fs::write(dir.join(".gitignore"), gitignore)?;

    Ok(())
}

fn init_git(dir: &PathBuf) -> Result<()> {
    std::process::Command::new("git")
        .args(["init"])
        .current_dir(dir)
        .output()
        .context("Failed to initialize git")?;
    Ok(())
}
