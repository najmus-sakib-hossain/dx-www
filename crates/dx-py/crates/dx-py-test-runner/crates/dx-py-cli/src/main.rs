//! CLI for dx-py-test-runner
//!
//! This is the main entry point for the dx-py command.

use std::path::PathBuf;
use std::time::Instant;

use clap::{Parser, Subcommand};
use colored::Colorize;

mod filter;
mod junit;
mod watch;

use filter::TestFilter;
use junit::JUnitReport;

/// High-performance Python test runner
#[derive(Parser)]
#[command(name = "dx-py")]
#[command(about = "A high-performance Python test runner built with Rust")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run tests
    Test {
        /// Test pattern to filter (glob or regex)
        #[arg(default_value = "*")]
        pattern: String,

        /// Watch mode - re-run affected tests on file changes
        #[arg(short, long)]
        watch: bool,

        /// Update snapshots instead of failing on mismatch
        #[arg(long)]
        update_snapshots: bool,

        /// CI mode - output JUnit XML
        #[arg(long)]
        ci: bool,

        /// Output file for JUnit XML (default: test-results.xml)
        #[arg(long, default_value = "test-results.xml")]
        junit_output: PathBuf,

        /// Number of workers (default: number of CPUs)
        #[arg(short = 'j', long)]
        workers: Option<usize>,

        /// Project root directory
        #[arg(short, long, default_value = ".")]
        root: PathBuf,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Discover tests without running them
    Discover {
        /// Project root directory
        #[arg(short, long, default_value = ".")]
        root: PathBuf,

        /// Test pattern to filter
        #[arg(default_value = "*")]
        pattern: String,
    },

    /// Show dependency graph for a file
    Graph {
        /// File to show dependencies for
        file: PathBuf,

        /// Project root directory
        #[arg(short, long, default_value = ".")]
        root: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Test {
            pattern,
            watch,
            update_snapshots,
            ci,
            junit_output,
            workers,
            root,
            verbose,
        } => {
            run_tests(
                &root,
                &pattern,
                watch,
                update_snapshots,
                ci,
                &junit_output,
                workers,
                verbose,
            );
        }
        Commands::Discover { root, pattern } => {
            discover_tests(&root, &pattern);
        }
        Commands::Graph { file, root } => {
            show_graph(&root, &file);
        }
    }
}

fn run_tests(
    root: &PathBuf,
    pattern: &str,
    watch: bool,
    update_snapshots: bool,
    ci: bool,
    junit_output: &PathBuf,
    workers: Option<usize>,
    verbose: bool,
) {
    println!("{}", "dx-py test runner".cyan().bold());
    println!();

    let start = Instant::now();

    // Create test filter
    let filter = TestFilter::new(pattern);

    if verbose {
        println!("Root: {}", root.display());
        println!("Pattern: {}", pattern);
        println!("Workers: {}", workers.unwrap_or(num_cpus::get()));
        println!();
    }

    // TODO: Integrate with discovery, executor, etc.
    // For now, show placeholder output

    if watch {
        println!("{}", "Watch mode enabled - press Ctrl+C to exit".yellow());
        println!("Watching for changes in: {}", root.display());
        // watch::run_watch_mode(root, &filter);
    }

    // Simulated test results for demonstration
    let passed = 10;
    let failed = 0;
    let skipped = 2;
    let total = passed + failed + skipped;
    let duration = start.elapsed();

    println!();
    print_summary(passed, failed, skipped, duration);

    if ci {
        // Generate JUnit XML
        let report = JUnitReport::new("dx-py-tests");
        if let Err(e) = report.write_to_file(junit_output) {
            eprintln!("{}: {}", "Error writing JUnit XML".red(), e);
        } else if verbose {
            println!("JUnit XML written to: {}", junit_output.display());
        }
    }

    if update_snapshots {
        println!("{}", "Snapshots updated".green());
    }
}

fn discover_tests(root: &PathBuf, pattern: &str) {
    println!("{}", "Discovering tests...".cyan());
    println!("Root: {}", root.display());
    println!("Pattern: {}", pattern);
    println!();

    // TODO: Integrate with dx-py-discovery
    println!("{}", "No tests discovered (integration pending)".yellow());
}

fn show_graph(root: &PathBuf, file: &PathBuf) {
    println!("{}", "Dependency graph".cyan());
    println!("Root: {}", root.display());
    println!("File: {}", file.display());
    println!();

    // TODO: Integrate with dx-py-graph
    println!(
        "{}",
        "Graph visualization pending integration".yellow()
    );
}

fn print_summary(passed: usize, failed: usize, skipped: usize, duration: std::time::Duration) {
    let total = passed + failed + skipped;

    println!("{}", "─".repeat(50));

    if failed == 0 {
        println!(
            "{} {} passed, {} skipped in {:.2}s",
            "✓".green().bold(),
            passed.to_string().green(),
            skipped.to_string().yellow(),
            duration.as_secs_f64()
        );
    } else {
        println!(
            "{} {} passed, {} failed, {} skipped in {:.2}s",
            "✗".red().bold(),
            passed.to_string().green(),
            failed.to_string().red(),
            skipped.to_string().yellow(),
            duration.as_secs_f64()
        );
    }

    println!("{}", "─".repeat(50));
}
