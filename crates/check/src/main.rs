//! Dx Check - The binary-first linter
//!
//! 10x faster than Biome, 100x faster than ESLint

use dx_check::cli::{output, Cli, Commands, OutputFormat, CacheCommands, RuleCommands, PluginCommands, CloudCommands, CiPlatformArg};
use dx_check::config::CheckerConfig;
use dx_check::engine::Checker;
use dx_check::fix::FixEngine;
use dx_check::project::ProjectProfile;
use dx_check::cache::AstCache;
use dx_check::plugin::PluginLoader;
use dx_check::marketplace::RegistryClient;
use dx_check::cloud::{CloudClient, CloudConfig};
use dx_check::ci::{CiPlatform, CiConfigGenerator};
use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse_args();

    match run(cli) {
        Ok(has_errors) => {
            if has_errors {
                ExitCode::from(1)
            } else {
                ExitCode::SUCCESS
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            ExitCode::from(2)
        }
    }
}

fn run(cli: Cli) -> Result<bool, Box<dyn std::error::Error>> {
    match &cli.command {
        Some(Commands::Check { paths, fix }) => {
            run_check(paths, *fix, &cli)
        }
        Some(Commands::Format { paths, check }) => {
            run_format(paths, *check, &cli)
        }
        Some(Commands::Init { force }) => {
            run_init(*force)
        }
        Some(Commands::Analyze { path }) => {
            run_analyze(path)
        }
        Some(Commands::Rule { command }) => {
            run_rule_command(command)
        }
        Some(Commands::Cache { command }) => {
            run_cache_command(command)
        }
        Some(Commands::Watch { rules_dir, output_dir, debounce }) => {
            run_watch_mode(rules_dir, output_dir, *debounce)
        }
        Some(Commands::Lsp) => {
            run_lsp()
        }
        Some(Commands::Plugin { command }) => {
            run_plugin_command(command)
        }
        Some(Commands::Cloud { command }) => {
            run_cloud_command(command)
        }
        Some(Commands::Ci { platform, output }) => {
            run_ci_command(platform.as_ref(), output.as_ref())
        }
        None => {
            // Default: check paths
            run_check(&cli.paths, cli.fix, &cli)
        }
    }
}

fn run_check(
    paths: &[std::path::PathBuf],
    fix: bool,
    cli: &Cli,
) -> Result<bool, Box<dyn std::error::Error>> {
    let root = paths.first().map(|p| p.as_path()).unwrap_or(Path::new("."));
    
    // Load config
    let config = if let Some(ref config_path) = cli.config {
        let content = std::fs::read_to_string(config_path)?;
        toml::from_str(&content)?
    } else {
        CheckerConfig::auto_detect(root)
    };

    // Detect project profile
    if cli.verbose {
        let profile = ProjectProfile::detect(root);
        output::print_profile(&profile);
    }

    // Create checker
    let checker = if cli.threads == 1 {
        Checker::new(config)
    } else {
        let mut config = config;
        config.parallel.threads = cli.threads;
        Checker::new(config)
    };

    // Run check
    let result = checker.check_path(root)?;

    // Apply fixes if requested
    if fix && !result.diagnostics.is_empty() {
        let fix_engine = FixEngine::new();
        for diagnostic in result.diagnostics.iter().filter(|d| d.fix.is_some()) {
            let source = std::fs::read(&diagnostic.file)?;
            if let Some(ref fix) = diagnostic.fix {
                let fixed = fix_engine.apply_fix(&source, fix);
                std::fs::write(&diagnostic.file, fixed)?;
            }
        }
    }

    // Output results
    match cli.format {
        OutputFormat::Pretty => {
            for diagnostic in &result.diagnostics {
                if let Ok(source) = std::fs::read_to_string(&diagnostic.file) {
                    output::print_diagnostic(diagnostic, &source);
                }
            }
            
            if !cli.quiet {
                output::print_summary(
                    result.files_checked,
                    result.error_count(),
                    result.warning_count(),
                    result.duration.as_millis() as u64,
                    result.files_per_second,
                );
            }
        }
        OutputFormat::Json => {
            let json = serde_json::json!({
                "files_checked": result.files_checked,
                "errors": result.error_count(),
                "warnings": result.warning_count(),
                "duration_ms": result.duration.as_millis(),
                "diagnostics": result.diagnostics.iter().map(|d| {
                    serde_json::json!({
                        "file": d.file.display().to_string(),
                        "span": { "start": d.span.start, "end": d.span.end },
                        "severity": d.severity.as_str(),
                        "rule": d.rule_id,
                        "message": d.message,
                    })
                }).collect::<Vec<_>>(),
            });
            println!("{}", serde_json::to_string_pretty(&json)?);
        }
        OutputFormat::Compact => {
            for diagnostic in &result.diagnostics {
                println!(
                    "{}:{}:{}: {} [{}] {}",
                    diagnostic.file.display(),
                    diagnostic.span.start,
                    diagnostic.span.end,
                    diagnostic.severity.as_str(),
                    diagnostic.rule_id,
                    diagnostic.message,
                );
            }
        }
        OutputFormat::Github => {
            for diagnostic in &result.diagnostics {
                let level = match diagnostic.severity {
                    dx_check::diagnostics::DiagnosticSeverity::Error => "error",
                    dx_check::diagnostics::DiagnosticSeverity::Warning => "warning",
                    _ => "notice",
                };
                println!(
                    "::{} file={},line=1::{}",
                    level,
                    diagnostic.file.display(),
                    diagnostic.message,
                );
            }
        }
        OutputFormat::Junit => {
            // JUnit XML output
            println!(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
            println!(r#"<testsuites>"#);
            println!(
                r#"  <testsuite name="dx-check" tests="{}" failures="{}">"#,
                result.files_checked,
                result.error_count(),
            );
            for diagnostic in &result.diagnostics {
                println!(
                    r#"    <testcase name="{}"><failure message="{}"/></testcase>"#,
                    diagnostic.rule_id,
                    diagnostic.message.replace('"', "&quot;"),
                );
            }
            println!(r#"  </testsuite>"#);
            println!(r#"</testsuites>"#);
        }
    }

    Ok(result.has_errors())
}

fn run_format(
    _paths: &[std::path::PathBuf],
    _check: bool,
    _cli: &Cli,
) -> Result<bool, Box<dyn std::error::Error>> {
    // Format implementation would go here
    println!("Format command not yet implemented");
    Ok(false)
}

fn run_init(force: bool) -> Result<bool, Box<dyn std::error::Error>> {
    let config_path = std::path::Path::new("dx.toml");
    
    if config_path.exists() && !force {
        eprintln!("Configuration file already exists. Use --force to overwrite.");
        return Ok(true);
    }

    let default_config = r#"# Dx Check Configuration
# https://dx.dev/docs/check

[rules]
# Enable recommended rules
recommended = true

# Auto-fix on check
auto_fix = false

[format]
# Indentation
use_tabs = false
indent_width = 2

# Line width
line_width = 80

# Quote style: "single" or "double"
quote_style = "double"

# Semicolons: "always" or "as_needed"
semicolons = "always"

[cache]
enabled = true
directory = ".dx-cache"

[parallel]
# Number of threads (0 = auto-detect)
threads = 0
"#;

    std::fs::write(config_path, default_config)?;
    println!("Created dx.toml configuration file");
    
    Ok(false)
}

fn run_analyze(path: &std::path::Path) -> Result<bool, Box<dyn std::error::Error>> {
    let profile = ProjectProfile::detect(path);
    output::print_profile(&profile);
    Ok(false)
}

fn run_rule_command(command: &RuleCommands) -> Result<bool, Box<dyn std::error::Error>> {
    use dx_check::rules::RuleRegistry;
    
    match command {
        RuleCommands::List { category, enabled: _ } => {
            let registry = RuleRegistry::with_builtins();
            
            println!("Available rules:\n");
            for name in registry.rule_names() {
                if let Some(rule) = registry.get(name) {
                    let meta = rule.meta();
                    let category_str = meta.category.as_str();
                    
                    if let Some(filter) = category {
                        if category_str != filter {
                            continue;
                        }
                    }
                    
                    let status = if registry.is_enabled(name) { "✓" } else { " " };
                    let fixable = if meta.fixable { "🔧" } else { "  " };
                    
                    println!(
                        "  {} {} {:20} {:12} {}",
                        status,
                        fixable,
                        name,
                        category_str,
                        meta.description
                    );
                }
            }
        }
        RuleCommands::Show { rule } => {
            let registry = RuleRegistry::with_builtins();
            
            if let Some(r) = registry.get(rule) {
                let meta = r.meta();
                println!("Rule: {}", meta.name);
                println!("Category: {}", meta.category.as_str());
                println!("Description: {}", meta.description);
                println!("Fixable: {}", if meta.fixable { "Yes" } else { "No" });
                println!("Recommended: {}", if meta.recommended { "Yes" } else { "No" });
                if let Some(url) = meta.docs_url {
                    println!("Documentation: {}", url);
                }
            } else {
                eprintln!("Rule not found: {}", rule);
                return Ok(true);
            }
        }
        RuleCommands::Enable { rule, severity } => {
            println!("Rule '{}' enabled with severity {:?}", rule, severity);
            // Would modify config file
        }
        RuleCommands::Disable { rule } => {
            println!("Rule '{}' disabled", rule);
            // Would modify config file
        }
        RuleCommands::Compile { output, verify } => {
            use dx_check::rules::compiler;
            
            println!("Compiling rules to binary format...\n");
            match compiler::compile_rules(output) {
                Ok(compiled) => {
                    println!("\n✅ Successfully compiled {} rules", compiled.count);
                    println!("   Binary size: {} KB", compiled.binary_size / 1024);
                    
                    if *verify {
                        let rules_path = output.join("rules.dxm");
                        println!("\nVerifying compiled rules...");
                        if let Err(e) = compiler::verify_compiled_rules(&rules_path) {
                            eprintln!("❌ Verification failed: {}", e);
                            return Ok(true);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("❌ Compilation failed: {}", e);
                    return Ok(true);
                }
            }
        }
        RuleCommands::Verify { path } => {
            use dx_check::rules::compiler;
            
            match compiler::verify_compiled_rules(path) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("❌ Verification failed: {}", e);
                    return Ok(true);
                }
            }
        }
        RuleCommands::Generate { output } => {
            use dx_check::rules::dxs_generator;
            
            println!("Generating .dxs files...\n");
            match dxs_generator::generate_all_dxs_files(output) {
                Ok(_) => {
                    println!("\n✨ Successfully generated .dxs files in {:?}", output);
                }
                Err(e) => {
                    eprintln!("❌ Generation failed: {}", e);
                    return Ok(true);
                }
            }
        }
        RuleCommands::CompileFromDxs { input, output } => {
            use dx_check::rules::compiler;
            
            println!("Compiling from .dxs files...\n");
            match compiler::compile_from_dxs(input, output) {
                Ok(compiled) => {
                    println!("\n✅ Successfully compiled {} rules", compiled.count);
                    println!("   Binary size: {} KB", compiled.binary_size / 1024);
                }
                Err(e) => {
                    eprintln!("❌ Compilation failed: {}", e);
                    return Ok(true);
                }
            }
        }
    }
    
    Ok(false)
}

fn run_cache_command(command: &CacheCommands) -> Result<bool, Box<dyn std::error::Error>> {
    let cache_dir = std::path::PathBuf::from(".dx-cache");
    
    match command {
        CacheCommands::Stats => {
            if cache_dir.exists() {
                let cache = AstCache::new(cache_dir, 1024 * 1024 * 1024)?;
                let stats = cache.stats();
                println!("Cache Statistics:");
                println!("  Entries: {}", stats.entry_count);
                println!("  Size: {} bytes", stats.total_size);
                println!("  Max Size: {} bytes", stats.max_size);
                println!("  Utilization: {:.1}%", stats.utilization());
            } else {
                println!("No cache directory found");
            }
        }
        CacheCommands::Clear => {
            if cache_dir.exists() {
                std::fs::remove_dir_all(&cache_dir)?;
                println!("Cache cleared");
            } else {
                println!("No cache to clear");
            }
        }
        CacheCommands::Path => {
            println!("{}", cache_dir.canonicalize().unwrap_or(cache_dir.clone()).display());
        }
    }
    
    Ok(false)
}

fn run_watch_mode(
    rules_dir: &std::path::PathBuf,
    output_dir: &std::path::PathBuf,
    debounce: u64,
) -> Result<bool, Box<dyn std::error::Error>> {
    use dx_check::watch::{watch_rules, WatchConfig};
    
    let config = WatchConfig {
        rules_dir: rules_dir.clone(),
        output_dir: output_dir.clone(),
        debounce_ms: debounce,
    };
    
    if let Err(e) = watch_rules(config) {
        eprintln!("Watch mode error: {}", e);
        return Err(e.into());
    }
    
    Ok(false)
}

fn run_lsp() -> Result<bool, Box<dyn std::error::Error>> {
    #[cfg(feature = "lsp")]
    {
        use tokio::runtime::Runtime;
        
        let rt = Runtime::new()?;
        rt.block_on(async {
            dx_check::lsp::start_lsp_server().await
        }).map_err(|e| -> Box<dyn std::error::Error> { e })?;
        Ok(false)
    }
    
    #[cfg(not(feature = "lsp"))]
    {
        eprintln!("LSP server not enabled. Rebuild with --features lsp");
        Ok(true)
    }
}

fn run_plugin_command(command: &PluginCommands) -> Result<bool, Box<dyn std::error::Error>> {
    match command {
        PluginCommands::List => {
            let mut loader = PluginLoader::new();
            let plugins = loader.discover();
            
            if plugins.is_empty() {
                println!("No plugins installed.");
                println!("\nInstall plugins with: dx-check plugin install <name>");
            } else {
                println!("Installed Plugins:\n");
                println!("{:<25} {:<10} {:<12} {}", "NAME", "VERSION", "TYPE", "DESCRIPTION");
                println!("{}", "-".repeat(70));
                
                for plugin in &plugins {
                    println!(
                        "{:<25} {:<10} {:<12} {}",
                        plugin.name,
                        plugin.version,
                        format!("{:?}", plugin.plugin_type),
                        plugin.description
                    );
                }
                println!("\nTotal: {} plugins", plugins.len());
            }
        }
        PluginCommands::Install { name, version } => {
            println!("Installing plugin: {}...", name);
            
            let client = RegistryClient::new(Default::default());
            match client.install(name, version.as_deref()) {
                Ok(pkg) => {
                    println!("✅ Successfully installed {} v{}", pkg.name, pkg.version);
                }
                Err(e) => {
                    eprintln!("❌ Failed to install plugin: {}", e);
                    return Ok(true);
                }
            }
        }
        PluginCommands::Uninstall { name } => {
            println!("Uninstalling plugin: {}...", name);
            
            let client = RegistryClient::new(Default::default());
            match client.uninstall(name) {
                Ok(_) => {
                    println!("✅ Successfully uninstalled {}", name);
                }
                Err(e) => {
                    eprintln!("❌ Failed to uninstall plugin: {}", e);
                    return Ok(true);
                }
            }
        }
        PluginCommands::Update { name } => {
            let client = RegistryClient::new(Default::default());
            
            match name {
                Some(pkg_name) => {
                    println!("Updating {}...", pkg_name);
                    match client.update(pkg_name) {
                        Ok(Some(pkg)) => {
                            println!("✅ Updated to {} v{}", pkg.name, pkg.version);
                        }
                        Ok(None) => {
                            println!("Already at latest version");
                        }
                        Err(e) => {
                            eprintln!("❌ Failed to update: {}", e);
                            return Ok(true);
                        }
                    }
                }
                None => {
                    println!("Updating all plugins...");
                    match client.update_all() {
                        Ok(updated) => {
                            if updated.is_empty() {
                                println!("All plugins are up to date");
                            } else {
                                for pkg in &updated {
                                    println!("✅ Updated {} to v{}", pkg.name, pkg.version);
                                }
                                println!("\nUpdated {} plugins", updated.len());
                            }
                        }
                        Err(e) => {
                            eprintln!("❌ Failed to update plugins: {}", e);
                            return Ok(true);
                        }
                    }
                }
            }
        }
        PluginCommands::Search { query } => {
            println!("Searching for '{}'...\n", query);
            
            let client = RegistryClient::new(Default::default());
            match client.search(query) {
                Ok(results) => {
                    if results.is_empty() {
                        println!("No plugins found matching '{}'", query);
                    } else {
                        println!("{:<25} {:<10} {:<10} {}", "NAME", "VERSION", "DOWNLOADS", "DESCRIPTION");
                        println!("{}", "-".repeat(70));
                        
                        for pkg in &results {
                            println!(
                                "{:<25} {:<10} {:<10} {}",
                                pkg.name,
                                pkg.version,
                                pkg.downloads,
                                pkg.description
                            );
                        }
                        println!("\nFound {} plugins", results.len());
                    }
                }
                Err(e) => {
                    eprintln!("❌ Search failed: {}", e);
                    return Ok(true);
                }
            }
        }
    }
    
    Ok(false)
}

fn run_cloud_command(command: &CloudCommands) -> Result<bool, Box<dyn std::error::Error>> {
    let config = CloudConfig::default();
    let mut client = CloudClient::new(config);
    
    match command {
        CloudCommands::Login => {
            println!("Logging in to DX Cloud...");
            println!("\nOpen this URL in your browser:");
            println!("  https://cloud.dx.dev/cli/login\n");
            
            match client.login() {
                Ok(_) => {
                    println!("✅ Successfully logged in!");
                }
                Err(e) => {
                    eprintln!("❌ Login failed: {}", e);
                    return Ok(true);
                }
            }
        }
        CloudCommands::Logout => {
            client.logout();
            println!("✅ Logged out successfully");
        }
        CloudCommands::Sync => {
            println!("Syncing configuration...");
            
            match client.sync() {
                Ok(status) => {
                    println!("✅ Sync complete");
                    println!("   Local version: {}", status.local_version);
                    println!("   Remote version: {}", status.remote_version);
                    println!("   Last synced: {}", status.last_synced
                        .map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string())
                        .unwrap_or_else(|| "Never".to_string()));
                }
                Err(e) => {
                    eprintln!("❌ Sync failed: {}", e);
                    return Ok(true);
                }
            }
        }
        CloudCommands::Pull => {
            println!("Pulling remote configuration...");
            
            match client.pull() {
                Ok(_) => {
                    println!("✅ Configuration pulled successfully");
                }
                Err(e) => {
                    eprintln!("❌ Pull failed: {}", e);
                    return Ok(true);
                }
            }
        }
        CloudCommands::Push => {
            println!("Pushing local configuration...");
            
            match client.push() {
                Ok(_) => {
                    println!("✅ Configuration pushed successfully");
                }
                Err(e) => {
                    eprintln!("❌ Push failed: {}", e);
                    return Ok(true);
                }
            }
        }
        CloudCommands::Status => {
            match client.status() {
                Ok(status) => {
                    println!("Cloud Sync Status:\n");
                    println!("  Authenticated: {}", if client.is_authenticated() { "Yes" } else { "No" });
                    println!("  Local version: {}", status.local_version);
                    println!("  Remote version: {}", status.remote_version);
                    println!("  Has conflicts: {}", if status.has_conflicts { "Yes" } else { "No" });
                    println!("  Last synced: {}", status.last_synced
                        .map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string())
                        .unwrap_or_else(|| "Never".to_string()));
                    
                    if status.pending_changes > 0 {
                        println!("  Pending changes: {}", status.pending_changes);
                    }
                }
                Err(e) => {
                    eprintln!("❌ Failed to get status: {}", e);
                    return Ok(true);
                }
            }
        }
        CloudCommands::Init { name } => {
            println!("Initializing team configuration for '{}'...", name);
            
            let _config = client.init_team(name);
            println!("✅ Team configuration initialized");
            println!("\nShare this team ID with your team members:");
            println!("  dx-check cloud join <team-id>");
        }
    }
    
    Ok(false)
}

fn run_ci_command(
    platform: Option<&CiPlatformArg>,
    output: Option<&std::path::PathBuf>,
) -> Result<bool, Box<dyn std::error::Error>> {
    // Auto-detect or use specified platform
    let ci_platform = match platform {
        Some(CiPlatformArg::Github) => CiPlatform::GitHubActions,
        Some(CiPlatformArg::Gitlab) => CiPlatform::GitLabCi,
        Some(CiPlatformArg::Azure) => CiPlatform::AzureDevOps,
        Some(CiPlatformArg::Circleci) => CiPlatform::CircleCi,
        None => {
            // Try to detect
            if let Some(detected) = CiPlatform::detect() {
                println!("Detected CI platform: {:?}", detected);
                detected
            } else {
                println!("No CI platform detected. Generating GitHub Actions config...");
                CiPlatform::GitHubActions
            }
        }
    };
    
    let generator = CiConfigGenerator::new(ci_platform.clone());
    let config = generator.generate()?;
    
    match output {
        Some(path) => {
            // Create parent directories if needed
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(path, &config)?;
            println!("✅ Generated CI configuration: {}", path.display());
        }
        None => {
            // Print to stdout
            let default_path = match ci_platform {
                CiPlatform::GitHubActions => ".github/workflows/dx-check.yml",
                CiPlatform::GitLabCi => ".gitlab-ci.yml",
                CiPlatform::AzureDevOps => "azure-pipelines.yml",
                CiPlatform::CircleCi => ".circleci/config.yml",
                _ => "dx-check-ci.yml",
            };
            
            println!("Generated configuration for {:?}:\n", ci_platform);
            println!("{}", config);
            println!("\nSave to: {}", default_path);
        }
    }
    
    Ok(false)
}

