/**
 * `dx new` - Project scaffolder
 * 
 * Creates a new dx-www project with:
 * - dx.toml configuration
 * - src/App.tsx entry point
 * - index.html template
 * - .gitignore
 */

use anyhow::{Context, Result};
use console::style;
use std::fs;
use std::path::{Path, PathBuf};

pub async fn execute(name: &str, path: Option<PathBuf>, template: &str) -> Result<()> {
    let target_dir = path.unwrap_or_else(|| PathBuf::from(name));

    println!("{} {}", style("Creating project:").bold(), style(name).cyan());
    println!("{} {}", style("Template:").bold(), style(template).cyan());
    println!("{} {}\n", style("Directory:").bold(), style(target_dir.display()).cyan());

    // Validate template
    let templates = vec!["counter", "dashboard", "hackernews"];
    if !templates.contains(&template) {
        anyhow::bail!("Unknown template: {}. Available: {:?}", template, templates);
    }

    // Create directory structure
    create_directory_structure(&target_dir)?;

    // Generate files based on template
    match template {
        "counter" => generate_counter_template(&target_dir, name)?,
        "dashboard" => generate_dashboard_template(&target_dir, name)?,
        "hackernews" => generate_hackernews_template(&target_dir, name)?,
        _ => unreachable!(),
    }

    // Create shared files
    create_config_file(&target_dir, name)?;
    create_html_template(&target_dir)?;
    create_gitignore(&target_dir)?;

    println!("\n{}", style("✨ Project created successfully!").green().bold());
    println!("\n{}", style("Next steps:").bold());
    println!("  {} cd {}", style("1.").cyan(), name);
    println!("  {} dx dev", style("2.").cyan());
    println!("\n{}", style("Your app will be running at http://localhost:3000").dim());

    Ok(())
}

/// Create directory structure
fn create_directory_structure(target: &Path) -> Result<()> {
    fs::create_dir_all(target.join("src"))
        .with_context(|| "Failed to create src directory")?;
    fs::create_dir_all(target.join("public"))
        .with_context(|| "Failed to create public directory")?;

    println!("  {} src/", style("✓").green());
    println!("  {} public/", style("✓").green());

    Ok(())
}

/// Generate counter template
fn generate_counter_template(target: &Path, name: &str) -> Result<()> {
    let app_tsx = r#"import { useState } from 'dx';

export default function App() {
  const [count, setCount] = useState(0);
  
  return (
    <div class="container">
      <h1>Welcome to {name}!</h1>
      <div class="counter">
        <button onClick={() => setCount(count - 1)}>-</button>
        <span class="count">{count}</span>
        <button onClick={() => setCount(count + 1)}>+</button>
      </div>
      <p class="info">
        Edit <code>src/App.tsx</code> and save to reload.
      </p>
    </div>
  );
}
"#.replace("{name}", name);

    fs::write(target.join("src/App.tsx"), app_tsx)
        .with_context(|| "Failed to write App.tsx")?;

    println!("  {} src/App.tsx", style("✓").green());

    Ok(())
}

/// Generate dashboard template
fn generate_dashboard_template(target: &Path, name: &str) -> Result<()> {
    let app_tsx = r#"import { useState } from 'dx';

interface Metric {
  label: string;
  value: number;
  unit: string;
}

export default function App() {
  const [metrics] = useState<Metric[]>([
    { label: 'Users', value: 1234, unit: '' },
    { label: 'Revenue', value: 45678, unit: '$' },
    { label: 'Load Time', value: 192, unit: 'ms' },
    { label: 'Requests', value: 98765, unit: '' },
  ]);
  
  return (
    <div class="dashboard">
      <header>
        <h1>{name}</h1>
        <p>Real-time metrics dashboard</p>
      </header>
      
      <div class="metrics-grid">
        {metrics.map(m => (
          <div class="metric-card" key={m.label}>
            <div class="metric-label">{m.label}</div>
            <div class="metric-value">
              {m.unit}{m.value.toLocaleString()}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
"#.replace("{name}", name);

    fs::write(target.join("src/App.tsx"), app_tsx)
        .with_context(|| "Failed to write App.tsx")?;

    println!("  {} src/App.tsx", style("✓").green());

    Ok(())
}

/// Generate Hacker News template
fn generate_hackernews_template(target: &Path, _name: &str) -> Result<()> {
    let app_tsx = r#"import { useState, useEffect } from 'dx';

interface Story {
  id: number;
  title: string;
  by: string;
  score: number;
  url?: string;
}

export default function App() {
  const [stories, setStories] = useState<Story[]>([]);
  const [loading, setLoading] = useState(true);
  
  useEffect(() => {
    fetch('https://hacker-news.firebaseio.com/v0/topstories.json')
      .then(r => r.json())
      .then(ids => {
        // Load first 30 stories
        const promises = ids.slice(0, 30).map((id: number) =>
          fetch(`https://hacker-news.firebaseio.com/v0/item/${id}.json`)
            .then(r => r.json())
        );
        return Promise.all(promises);
      })
      .then(stories => {
        setStories(stories);
        setLoading(false);
      });
  }, []);
  
  if (loading) {
    return <div class="loading">Loading stories...</div>;
  }
  
  return (
    <div class="hackernews">
      <header>
        <h1>🔥 Hacker News</h1>
        <p>Built with dx-www (Binary Web)</p>
      </header>
      
      <div class="stories">
        {stories.map((story, i) => (
          <div class="story" key={story.id}>
            <span class="rank">{i + 1}.</span>
            <div class="story-content">
              <a href={story.url} class="story-title">{story.title}</a>
              <div class="story-meta">
                {story.score} points by {story.by}
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
"#;

    fs::write(target.join("src/App.tsx"), app_tsx)
        .with_context(|| "Failed to write App.tsx")?;

    println!("  {} src/App.tsx", style("✓").green());

    Ok(())
}

/// Create dx.toml configuration
fn create_config_file(target: &Path, name: &str) -> Result<()> {
    let config = format!(r#"[project]
name = "{}"
version = "0.1.0"

[build]
# Compiler automatically selects Micro (338B) or Macro (7.5KB)
# based on application complexity
auto_select = true

# Manually force a runtime (optional)
# runtime = "micro"  # or "macro"

# Enable source maps for debugging
sourcemaps = true

# Target directory
output = "dist"

[server]
# Development server configuration
port = 3000
host = "localhost"

# Enable hot module replacement
hmr = true

# CORS origins (for API development)
cors_origins = ["http://localhost:3000"]

[optimize]
# WASM optimization level (0-4, or 's', 'z')
wasm_opt = "z"

# Strip debug symbols in release
strip = true

# Enable advanced optimizations
lto = true
"#, name);

    fs::write(target.join("dx.toml"), config)
        .with_context(|| "Failed to write dx.toml")?;

    println!("  {} dx.toml", style("✓").green());

    Ok(())
}

/// Create HTML template
fn create_html_template(target: &Path) -> Result<()> {
    let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Dx App</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            color: #333;
        }
        
        .container {
            background: white;
            border-radius: 16px;
            padding: 48px;
            box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
            text-align: center;
            max-width: 600px;
        }
        
        h1 {
            font-size: 36px;
            margin-bottom: 32px;
            color: #667eea;
        }
        
        .counter {
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 24px;
            margin: 32px 0;
        }
        
        button {
            width: 60px;
            height: 60px;
            border: none;
            border-radius: 12px;
            background: #667eea;
            color: white;
            font-size: 24px;
            font-weight: bold;
            cursor: pointer;
            transition: all 0.2s;
        }
        
        button:hover {
            background: #5568d3;
            transform: scale(1.05);
        }
        
        button:active {
            transform: scale(0.95);
        }
        
        .count {
            font-size: 48px;
            font-weight: bold;
            color: #667eea;
            min-width: 80px;
        }
        
        .info {
            margin-top: 32px;
            color: #666;
            font-size: 14px;
        }
        
        code {
            background: #f0f0f0;
            padding: 4px 8px;
            border-radius: 4px;
            font-family: 'Courier New', monospace;
        }
    </style>
</head>
<body>
    <div id="root"></div>
    
    <!-- dx-www Binary Web Runtime -->
    <script type="module">
        // Fetch the compiled binary from the dev server
        async function loadDxApp() {
            try {
                const response = await fetch('/stream/app');
                const binary = await response.arrayBuffer();
                
                console.log('📦 Loaded dx binary:', binary.byteLength, 'bytes');
                
                // For now, show a message that we've loaded the binary
                // In Phase 2, dx-client.wasm will parse and render this
                document.getElementById('root').innerHTML = `
                    <div style="padding: 20px; text-align: center; font-family: sans-serif;">
                        <h1>🚀 Dx Binary Loaded Successfully!</h1>
                        <p>Binary size: ${binary.byteLength} bytes</p>
                        <p style="color: #666; margin-top: 20px;">
                            ⚠️ Client runtime (dx-client.wasm) integration coming in Phase 2<br>
                            For now, the binary is compiled and ready at <code>/stream/app</code>
                        </p>
                        <div style="margin-top: 30px; padding: 20px; background: #f0f0f0; border-radius: 8px;">
                            <strong>✅ Working:</strong><br>
                            • TSX compilation<br>
                            • Binary generation (${binary.byteLength} bytes)<br>
                            • Hot reload<br>
                            • HTTP serving<br>
                            <br>
                            <strong>⏳ Coming Soon:</strong><br>
                            • Browser rendering via dx-client.wasm
                        </div>
                    </div>
                `;
            } catch (error) {
                console.error('Failed to load dx binary:', error);
                document.getElementById('root').innerHTML = `
                    <div style="padding: 20px; color: red;">
                        <h2>Error loading binary</h2>
                        <p>${error.message}</p>
                    </div>
                `;
            }
        }
        
        loadDxApp();
    </script>
</body>
</html>
"#;

    fs::write(target.join("index.html"), html)
        .with_context(|| "Failed to write index.html")?;

    println!("  {} index.html", style("✓").green());

    Ok(())
}

/// Create .gitignore
fn create_gitignore(target: &Path) -> Result<()> {
    let gitignore = r#"# Dx build artifacts
dist/
target/

# Dependencies
node_modules/

# OS files
.DS_Store
Thumbs.db

# Editor
.vscode/
.idea/
*.swp
*.swo
*~

# Debug
*.log
"#;

    fs::write(target.join(".gitignore"), gitignore)
        .with_context(|| "Failed to write .gitignore")?;

    println!("  {} .gitignore", style("✓").green());

    Ok(())
}
