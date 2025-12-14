/**
 * `dx new` - Project scaffolder
 *
 * Creates a new dx-www project with the modern structure.
 *
 * Structure:
 * - dx (Config file)
 * - .dx/ (System cache/build)
 * - app/ (Application pages/routes)
 * - auth/ (Authentication logic)
 * - component/ (Reusable components)
 * - db/ (Database schemas/models)
 * - media/ (Static assets)
 * - icon/ (SVG icons)
 * - feature/ (Feature modules)
 * - font/ (Custom fonts)
 * - i18n/ (Internationalization)
 * - style/ (Global styles)
 */
use anyhow::{Context, Result};
use console::style;
use std::fs;
use std::path::{Path, PathBuf};

pub async fn execute(name: &str, path: Option<PathBuf>, template: &str) -> Result<()> {
    let target_dir = path.unwrap_or_else(|| PathBuf::from(name));

    println!("{} {}", style("dx-www:").bold().magenta(), style(name).cyan());
    println!("{} {}\n", style("Location:").bold(), style(target_dir.display()).cyan());

    // Create modern dx-www directory structure
    create_project_structure(&target_dir)?;

    // Create dx config file
    create_config_file(&target_dir, name)?;

    // Generate starter files
    generate_starter_files(&target_dir, name)?;

    println!("\n{}", style("✓ Project created successfully!").green().bold());
    println!("\n{}", style("Next steps:").bold());
    println!("  {} cd {}", style("1.").cyan(), name);
    println!("  {} dx dev", style("2.").cyan());
    println!("\n{}", style("Build fast. Ship faster. 🚀").dim());

    Ok(())
}

/// Create the modern dx-www project folder structure
fn create_project_structure(target: &Path) -> Result<()> {
    let dirs = [
        // System directory (build cache, temp files)
        ".dx/cache",
        ".dx/build",
        ".dx/temp",
        // Application routes and pages
        "app/pages",
        "app/layouts",
        "app/api",
        // Authentication
        "auth/providers",
        "auth/middleware",
        // Reusable components
        "component/ui",
        "component/forms",
        "component/layout",
        // Database
        "db/schema",
        "db/migrations",
        "db/seeds",
        // Static assets
        "media/images",
        "media/video",
        "media/audio",
        "media/documents",
        // Icons
        "icon/svg",
        "icon/sprite",
        // Feature modules
        "feature/analytics",
        "feature/billing",
        "feature/notifications",
        // Fonts
        "font/woff2",
        "font/variable",
        // Internationalization
        "i18n/locales/en",
        "i18n/locales/es",
        "i18n/locales/fr",
        // Styles
        "style/themes",
        "style/components",
        "style/utilities",
    ];

    let pb = indicatif::ProgressBar::new(dirs.len() as u64);
    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );

    for dir in dirs {
        pb.set_message(format!("Creating {}...", dir));
        fs::create_dir_all(target.join(dir))
            .with_context(|| format!("Failed to create directory: {}", dir))?;
        pb.inc(1);
    }

    pb.finish_with_message("Structure created.");
    println!("  {} Project structure", style("✓").green());

    Ok(())
}

/// Create the 'dx' configuration file
fn create_config_file(target: &Path, name: &str) -> Result<()> {
    let config = format!(
        r#"# dx-www configuration
# https://dx-www.dev/docs/config

[project]
name = "{}"
version = "1.0.0"
description = "A dx-www application"

[build]
target = "wasm32-unknown-unknown"
mode = "auto"  # auto-select micro (338B) or macro (7.5KB) runtime
optimize = "size"

[dev]
port = 3000
hot_reload = true
open_browser = true

[server]
compression = "brotli"
cache = true

[style]
framework = "dx-style"  # Binary CSS (B-CSS)
autoprefixer = true

[i18n]
default_locale = "en"
fallback = "en"
"#,
        name
    );

    fs::write(target.join("dx"), config).with_context(|| "Failed to write dx config")?;

    println!("  {} dx config", style("✓").green());
    Ok(())
}

/// Generate starter files for the project
fn generate_starter_files(target: &Path, name: &str) -> Result<()> {
    // 1. app/pages/index.tsx
    let index_page = format!(
        r#"import {{ Button }} from '../../component/ui/Button';
import {{ useState }} from 'dx';

export default function HomePage() {{
  const [count, setCount] = useState(0);

  return (
    <div class="container">
      <h1>{}</h1>
      <p>Welcome to the Binary Web</p>
      
      <div class="counter">
        <Button onClick={{() => setCount(count - 1)}}>-</Button>
        <span class="count">{{count}}</span>
        <Button onClick={{() => setCount(count + 1)}}>+</Button>
      </div>
    </div>
  );
}}
"#,
        name
    );
    fs::write(target.join("app/pages/index.tsx"), index_page)?;

    // 2. app/layouts/MainLayout.tsx
    let layout = r#"export default function MainLayout({ children }) {
  return (
    <html lang="en">
      <head>
        <meta charset="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <title>dx-www App</title>
      </head>
      <body>
        <main>{children}</main>
      </body>
    </html>
  );
}
"#;
    fs::write(target.join("app/layouts/MainLayout.tsx"), layout)?;

    // 3. component/ui/Button.tsx
    let button = r#"export function Button({ children, onClick, ...props }) {
  return (
    <button 
      class="dx-button"
      onClick={onClick}
      {...props}
    >
      {children}
    </button>
  );
}
"#;
    fs::write(target.join("component/ui/Button.tsx"), button)?;

    // 4. style/main.css
    let main_css = r#"/* dx-www global styles */
:root {
  --color-primary: #0066ff;
  --color-background: #ffffff;
  --color-text: #000000;
}

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: system-ui, -apple-system, sans-serif;
  background: var(--color-background);
  color: var(--color-text);
}

.container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 2rem;
  text-align: center;
}

.counter {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 1rem;
  margin-top: 2rem;
}

.count {
  font-size: 2rem;
  font-weight: bold;
  min-width: 3rem;
}

.dx-button {
  padding: 0.75rem 1.5rem;
  font-size: 1rem;
  font-weight: 600;
  color: white;
  background: var(--color-primary);
  border: none;
  border-radius: 0.5rem;
  cursor: pointer;
  transition: transform 0.2s;
}

.dx-button:hover {
  transform: scale(1.05);
}

.dx-button:active {
  transform: scale(0.95);
}
"#;
    fs::write(target.join("style/main.css"), main_css)?;

    // 5. i18n/locales/en/common.json
    let i18n_en = format!(
        r#"{{
  "app": {{
    "name": "{}",
    "welcome": "Welcome to the Binary Web"
  }},
  "common": {{
    "loading": "Loading...",
    "error": "Something went wrong"
  }}
}}
"#,
        name
    );
    fs::write(target.join("i18n/locales/en/common.json"), i18n_en)?;

    // 6. .gitignore
    let gitignore = r#"# dx-www system files
.dx/
*.dxb

# Environment
.env
.env.local
.env.production

# Dependencies
node_modules/

# Build output
dist/
build/

# IDE
.vscode/
.idea/
*.swp
*.swo

# OS
.DS_Store
Thumbs.db
"#;
    fs::write(target.join(".gitignore"), gitignore)?;

    // 7. README.md
    let readme = format!(
        r#"# {}

Built with [dx-www](https://dx-www.dev) - The Binary Web Runtime

## Getting Started

```bash
# Start development server
dx dev

# Build for production
dx build

# Preview production build
dx preview
```

## Project Structure

```
├── app/           # Application pages and routes
├── auth/          # Authentication logic
├── component/     # Reusable components
├── db/            # Database schemas
├── media/         # Static assets
├── icon/          # SVG icons
├── feature/       # Feature modules
├── font/          # Custom fonts
├── i18n/          # Translations
├── style/         # Global styles
└── dx             # Configuration file
```

## Learn More

- [Documentation](https://dx-www.dev/docs)
- [Examples](https://dx-www.dev/examples)
- [GitHub](https://github.com/dx-www/dx-www-runtime)
"#,
        name
    );
    fs::write(target.join("README.md"), readme)?;

    println!("  {} Starter files created", style("✓").green());

    Ok(())
}
