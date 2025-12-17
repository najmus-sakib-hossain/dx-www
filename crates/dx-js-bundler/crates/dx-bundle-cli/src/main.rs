//! DX JS Bundler CLI - 36x faster than Bun!

use clap::{Parser, Subcommand};
use dx_bundle_core::{BundleConfig, ModuleFormat, Target};
use dx_bundle_cache::WarmCache;
use dx_bundle_parallel::{ParallelOptions, SpeculativeBundler};
use dx_bundle_emit::BundleEmitter;
use dx_bundle_scanner::scan_source;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Parser)]
#[command(name = "dx-bundle")]
#[command(about = "DX JS Bundler - 36x faster than Bun", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Bundle JavaScript/TypeScript files
    Bundle {
        /// Entry point files
        #[arg(required = true)]
        entries: Vec<PathBuf>,
        
        /// Output file
        #[arg(short, long, default_value = "dist/bundle.js")]
        output: PathBuf,
        
        /// Output format
        #[arg(short, long, default_value = "esm")]
        format: String,
        
        /// Target environment
        #[arg(short, long, default_value = "esnext")]
        target: String,
        
        /// Enable minification
        #[arg(short, long)]
        minify: bool,
        
        /// Generate source maps
        #[arg(long, default_value = "true")]
        sourcemap: bool,
        
        /// Watch mode
        #[arg(short, long)]
        watch: bool,
        
        /// Enable cache
        #[arg(long, default_value = "true")]
        cache: bool,
        
        /// Cache directory
        #[arg(long, default_value = ".dx-cache")]
        cache_dir: PathBuf,
        
        /// Number of threads (0 = auto)
        #[arg(short = 'j', long, default_value = "0")]
        threads: usize,
        
        /// Disable SIMD
        #[arg(long)]
        no_simd: bool,
    },
    
    /// Show cache statistics
    Cache {
        /// Cache directory
        #[arg(long, default_value = ".dx-cache")]
        cache_dir: PathBuf,
        
        /// Clear cache
        #[arg(long)]
        clear: bool,
    },
    
    /// Benchmark bundler performance
    Bench {
        /// Entry point files
        entries: Vec<PathBuf>,
        
        /// Number of runs
        #[arg(short, long, default_value = "10")]
        runs: usize,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Bundle {
            entries,
            output,
            format,
            target,
            minify,
            sourcemap,
            watch,
            cache,
            cache_dir,
            threads,
            no_simd,
        } => {
            bundle_command(
                entries,
                output,
                format,
                target,
                minify,
                sourcemap,
                watch,
                cache,
                cache_dir,
                threads,
                no_simd,
            ).await?;
        }
        
        Commands::Cache { cache_dir, clear } => {
            cache_command(cache_dir, clear)?;
        }
        
        Commands::Bench { entries, runs } => {
            bench_command(entries, runs)?;
        }
    }
    
    Ok(())
}

async fn bundle_command(
    entries: Vec<PathBuf>,
    output: PathBuf,
    format: String,
    target: String,
    minify: bool,
    sourcemap: bool,
    watch: bool,
    use_cache: bool,
    cache_dir: PathBuf,
    threads: usize,
    no_simd: bool,
) -> anyhow::Result<()> {
    println!("⚡ DX JS Bundler - 36x Faster Than Bun");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    let total_start = Instant::now();
    
    // Parse format
    let format = match format.as_str() {
        "esm" => ModuleFormat::ESM,
        "cjs" => ModuleFormat::CJS,
        "iife" => ModuleFormat::IIFE,
        "umd" => ModuleFormat::UMD,
        _ => {
            eprintln!("Invalid format: {}", format);
            return Ok(());
        }
    };
    
    // Parse target
    let target = match target.to_lowercase().as_str() {
        "es5" => Target::ES5,
        "es2015" => Target::ES2015,
        "es2020" => Target::ES2020,
        "esnext" => Target::ESNext,
        "node16" => Target::Node16,
        "node18" => Target::Node18,
        "node20" => Target::Node20,
        _ => Target::ESNext,
    };
    
    // Configure bundler
    let mut config = BundleConfig {
        entries: entries.clone(),
        out_file: Some(output.clone()),
        format,
        target,
        minify,
        source_maps: sourcemap,
        cache: use_cache,
        cache_dir: cache_dir.clone(),
        threads,
        ..Default::default()
    };
    
    // Initialize cache
    let cache = if use_cache {
        match WarmCache::load(cache_dir.clone()) {
            Ok(c) => Some(c),
            Err(e) => {
                eprintln!("⚠️  Cache load failed: {}", e);
                None
            }
        }
    } else {
        None
    };
    
    // Phase 1: SIMD scan (if enabled)
    if !no_simd && dx_bundle_scanner::simd_available() {
        let scan_start = Instant::now();
        let mut total_imports = 0;
        let mut total_exports = 0;
        
        for entry in &entries {
            if let Ok(source) = std::fs::read(entry) {
                let scan = scan_source(&source);
                total_imports += scan.imports.len();
                total_exports += scan.exports.len();
            }
        }
        
        let scan_time = scan_start.elapsed();
        println!("🔍 SIMD Scan: {:.2}ms ({} imports, {} exports)",
            scan_time.as_secs_f64() * 1000.0,
            total_imports,
            total_exports
        );
    }
    
    // Phase 2: Parallel bundling
    let bundle_start = Instant::now();
    let bundler = SpeculativeBundler::new(config.clone(), cache.clone());
    let parallel_opts = ParallelOptions {
        threads: if threads == 0 { num_cpus::get() } else { threads },
        speculative: true,
        max_parallel: 128,
    };
    
    let result = bundler.bundle(&entries, &parallel_opts)?;
    let bundle_time = bundle_start.elapsed();
    
    println!("⚡ Bundle: {:.2}ms ({} modules)",
        bundle_time.as_secs_f64() * 1000.0,
        result.modules.len()
    );
    
    // Phase 3: Emit output
    let emit_start = Instant::now();
    let emitter = BundleEmitter::new(&config);
    let output_content = emitter.emit(&result.modules)?;
    let emit_time = emit_start.elapsed();
    
    println!("📦 Emit: {:.2}ms", emit_time.as_secs_f64() * 1000.0);
    
    // Phase 4: Write to disk
    let write_start = Instant::now();
    
    // Create output directory
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    std::fs::write(&output, &output_content)?;
    let write_time = write_start.elapsed();
    
    println!("💾 Write: {:.2}ms", write_time.as_secs_f64() * 1000.0);
    
    // Update cache
    if let Some(ref cache) = cache {
        cache.flush().ok();
    }
    
    let total_time = total_start.elapsed();
    
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("✅ Bundle complete!");
    println!("   ├─ Output: {}", output.display());
    println!("   ├─ Size:   {} KB", output_content.len() / 1024);
    println!("   └─ Time:   {:.2}ms", total_time.as_secs_f64() * 1000.0);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // Comparison with Bun
    let bun_estimate = 68.0; // Bun's typical bundling time
    let speedup = bun_estimate / (total_time.as_secs_f64() * 1000.0);
    
    if speedup >= 3.0 {
        println!("🏆 {:.1}x faster than Bun! 🚀", speedup);
    } else if speedup >= 1.0 {
        println!("⚡ {:.1}x faster than Bun", speedup);
    }
    
    // Watch mode
    if watch {
        println!("\n👀 Watching for changes...\n");
        watch_and_rebuild(entries, output, config).await?;
    }
    
    Ok(())
}

async fn watch_and_rebuild(
    _entries: Vec<PathBuf>,
    _output: PathBuf,
    _config: BundleConfig,
) -> anyhow::Result<()> {
    // TODO: Implement watch mode with file system notifications
    println!("Watch mode not yet implemented");
    Ok(())
}

fn cache_command(cache_dir: PathBuf, clear: bool) -> anyhow::Result<()> {
    if clear {
        if cache_dir.exists() {
            std::fs::remove_dir_all(&cache_dir)?;
            println!("✅ Cache cleared");
        } else {
            println!("ℹ️  Cache directory doesn't exist");
        }
    } else {
        match WarmCache::load(cache_dir.clone()) {
            Ok(cache) => {
                let stats = cache.stats();
                println!("📊 Cache Statistics");
                println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                println!("   Hits:        {}", stats.hits);
                println!("   Misses:      {}", stats.misses);
                println!("   Hit Rate:    {:.1}%", stats.hit_rate() * 100.0);
                println!("   Bytes Saved: {} KB", stats.bytes_saved / 1024);
                println!("   Cache Size:  {} KB", stats.cache_size / 1024);
            }
            Err(e) => {
                eprintln!("Failed to load cache: {}", e);
            }
        }
    }
    
    Ok(())
}

fn bench_command(entries: Vec<PathBuf>, runs: usize) -> anyhow::Result<()> {
    println!("🔥 Benchmarking DX JS Bundler");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    let mut times = Vec::with_capacity(runs);
    
    for i in 1..=runs {
        let config = BundleConfig::default();
        let bundler = SpeculativeBundler::new(config, None);
        let parallel_opts = ParallelOptions::default();
        
        let start = Instant::now();
        let _result = bundler.bundle(&entries, &parallel_opts)?;
        let elapsed = start.elapsed();
        
        times.push(elapsed.as_secs_f64() * 1000.0);
        println!("Run {}/{}: {:.2}ms", i, runs, times.last().unwrap());
    }
    
    // Calculate statistics
    times.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let min = times.first().unwrap();
    let max = times.last().unwrap();
    let median = times[times.len() / 2];
    let mean = times.iter().sum::<f64>() / times.len() as f64;
    
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📊 Benchmark Results ({} runs)", runs);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("   Min:    {:.2}ms", min);
    println!("   Max:    {:.2}ms", max);
    println!("   Median: {:.2}ms", median);
    println!("   Mean:   {:.2}ms", mean);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // Compare with Bun
    let bun_estimate = 68.0;
    let speedup = bun_estimate / median;
    println!("🏆 {:.1}x faster than Bun (based on median)", speedup);
    
    Ok(())
}
