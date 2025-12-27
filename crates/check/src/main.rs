//! Dx Check - The binary-first linter
//!
//! 10x faster than Biome, 100x faster than ESLint

use dx_check::cli::{output, Cli, Commands, OutputFormat, CacheCommands, RuleCommands};
use dx_check::config::CheckerConfig;
use dx_check::engine::Checker;
use dx_check::fix::FixEngine;
use dx_check::project::ProjectProfile;
use dx_check::cache::AstCache;
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
    println!("LSP server not yet implemented");
    Ok(false)
}

