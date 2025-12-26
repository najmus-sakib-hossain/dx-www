//! DX-Py CLI - Command-line interface for the DX-Py runtime

mod bench;

use clap::{Parser, Subcommand};
use dx_py_interpreter::VirtualMachine;
use dx_py_core::pylist::PyValue;
use std::io::{self, Write, BufRead};
use std::path::PathBuf;

/// DX-Py: A revolutionary Python runtime
#[derive(Parser)]
#[command(name = "dx-py")]
#[command(author = "DX-Py Team")]
#[command(version = "0.1.0")]
#[command(about = "A high-performance Python runtime", long_about = None)]
struct Cli {
    /// Python file to execute
    #[arg(value_name = "FILE")]
    file: Option<PathBuf>,
    
    /// Execute a command string
    #[arg(short = 'c', long)]
    command: Option<String>,
    
    /// Run in interactive mode (REPL)
    #[arg(short, long)]
    interactive: bool,
    
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
    
    #[command(subcommand)]
    subcommand: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile a Python file to DPB bytecode
    Compile {
        /// Input Python file
        input: PathBuf,
        /// Output DPB file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Disassemble a DPB file
    Disasm {
        /// DPB file to disassemble
        file: PathBuf,
    },
    /// Show runtime information
    Info,
    /// Run benchmarks
    Bench {
        /// Benchmark to run
        #[arg(default_value = "all")]
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();
    
    if let Err(e) = run(cli) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    // Handle subcommands first
    if let Some(cmd) = cli.subcommand {
        return handle_subcommand(cmd, cli.verbose);
    }
    
    // Execute command string
    if let Some(command) = cli.command {
        return execute_command(&command, cli.verbose);
    }
    
    // Execute file
    if let Some(file) = cli.file {
        return execute_file(&file, cli.verbose);
    }
    
    // Default to REPL if no file or command
    if cli.interactive || (cli.file.is_none() && cli.command.is_none()) {
        return run_repl(cli.verbose);
    }
    
    Ok(())
}

fn handle_subcommand(cmd: Commands, verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        Commands::Compile { input, output } => {
            let output = output.unwrap_or_else(|| {
                let mut out = input.clone();
                out.set_extension("dpb");
                out
            });
            
            if verbose {
                println!("Compiling {} -> {}", input.display(), output.display());
            }
            
            // TODO: Implement actual compilation
            println!("Compilation not yet implemented");
            Ok(())
        }
        Commands::Disasm { file } => {
            if verbose {
                println!("Disassembling {}", file.display());
            }
            
            // TODO: Implement disassembly
            println!("Disassembly not yet implemented");
            Ok(())
        }
        Commands::Info => {
            print_info();
            Ok(())
        }
        Commands::Bench { name } => {
            if verbose {
                println!("Running benchmark: {}", name);
            }
            
            match name.as_str() {
                "all" => {
                    let results = bench::run_all_benchmarks();
                    bench::validate_targets(&results);
                }
                "startup" => {
                    let result = bench::bench_startup();
                    println!("{}: {:?}", result.name, result.mean_time);
                }
                "eval" => {
                    let result = bench::bench_eval_int();
                    println!("{}: {:?}", result.name, result.mean_time);
                }
                "list" => {
                    let result = bench::bench_list_ops();
                    println!("{}: {:?}", result.name, result.mean_time);
                }
                "dict" => {
                    let result = bench::bench_dict_ops();
                    println!("{}: {:?}", result.name, result.mean_time);
                }
                "string" => {
                    let result = bench::bench_string_ops();
                    println!("{}: {:?}", result.name, result.mean_time);
                }
                _ => {
                    println!("Unknown benchmark: {}", name);
                    println!("Available: all, startup, eval, list, dict, string");
                }
            }
            Ok(())
        }
    }
}

fn execute_command(command: &str, verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
    if verbose {
        println!("Executing: {}", command);
    }
    
    let vm = VirtualMachine::new();
    
    match vm.eval_expr(command) {
        Ok(result) => {
            if !matches!(result, PyValue::None) {
                println!("{}", format_value(&result));
            }
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
    
    Ok(())
}

fn execute_file(file: &PathBuf, verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
    if verbose {
        println!("Executing file: {}", file.display());
    }
    
    // Check file extension
    let ext = file.extension().and_then(|e| e.to_str()).unwrap_or("");
    
    match ext {
        "py" => {
            // TODO: Compile and execute Python source
            println!("Python source execution not yet implemented");
            println!("Use 'dx-py compile' to compile to DPB first");
        }
        "dpb" => {
            // TODO: Load and execute DPB bytecode
            println!("DPB execution not yet implemented");
        }
        "dpm" => {
            // TODO: Load and execute DPM module
            println!("DPM module execution not yet implemented");
        }
        _ => {
            eprintln!("Unknown file type: {}", ext);
        }
    }
    
    Ok(())
}

fn run_repl(verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!("DX-Py {} (Interactive Mode)", env!("CARGO_PKG_VERSION"));
    println!("Type 'exit()' or Ctrl+D to quit, 'help()' for help");
    println!();
    
    let vm = VirtualMachine::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    
    loop {
        print!(">>> ");
        stdout.flush()?;
        
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) => {
                // EOF
                println!();
                break;
            }
            Ok(_) => {
                let line = line.trim();
                
                if line.is_empty() {
                    continue;
                }
                
                // Handle special commands
                match line {
                    "exit()" | "quit()" => break,
                    "help()" => {
                        print_help();
                        continue;
                    }
                    _ => {}
                }
                
                if verbose {
                    println!("[eval] {}", line);
                }
                
                match vm.eval_expr(line) {
                    Ok(result) => {
                        if !matches!(result, PyValue::None) {
                            println!("{}", format_value(&result));
                        }
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }
    
    Ok(())
}

fn print_info() {
    println!("DX-Py Runtime Information");
    println!("========================");
    println!();
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!("Platform: {} {}", std::env::consts::OS, std::env::consts::ARCH);
    println!();
    println!("Features:");
    println!("  - Binary Python Bytecode (DPB)");
    println!("  - SIMD-Accelerated String Operations");
    println!("  - Lock-Free Parallel Garbage Collector");
    println!("  - Tiered JIT with Cranelift Backend");
    println!("  - Speculative Type Prediction");
    println!("  - Memory Teleportation FFI");
    println!("  - Binary Module Format (DPM)");
    println!("  - Thread-Per-Core Parallel Executor");
    println!("  - Stack Allocation Fast Path");
    println!("  - Binary Protocol IPC (HBTP)");
    println!("  - Reactive Bytecode Cache");
    println!("  - SIMD-Accelerated Collections");
    println!("  - Compiler-Inlined Decorators");
    println!("  - Persistent Compilation Cache (PCC)");
    println!("  - Cross-Process Shared Objects");
    println!();
    println!("CPU Features:");
    
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            println!("  - AVX2: enabled");
        } else {
            println!("  - AVX2: not available");
        }
        if is_x86_feature_detected!("sse4.2") {
            println!("  - SSE4.2: enabled");
        }
    }
    
    println!();
    println!("Threads: {}", std::thread::available_parallelism()
        .map(|p| p.get())
        .unwrap_or(1));
}

fn print_help() {
    println!("DX-Py Interactive Help");
    println!("=====================");
    println!();
    println!("Available built-in functions:");
    println!("  print(...)  - Print values to stdout");
    println!("  len(x)      - Return length of x");
    println!("  type(x)     - Return type name of x");
    println!("  int(x)      - Convert x to integer");
    println!("  float(x)    - Convert x to float");
    println!("  str(x)      - Convert x to string");
    println!("  bool(x)     - Convert x to boolean");
    println!("  abs(x)      - Return absolute value of x");
    println!("  min(...)    - Return minimum value");
    println!("  max(...)    - Return maximum value");
    println!("  sum(x)      - Return sum of iterable x");
    println!("  range(...)  - Return range as list");
    println!();
    println!("Special commands:");
    println!("  exit()      - Exit the REPL");
    println!("  quit()      - Exit the REPL");
    println!("  help()      - Show this help");
}

fn format_value(value: &PyValue) -> String {
    match value {
        PyValue::None => "None".to_string(),
        PyValue::Bool(b) => if *b { "True" } else { "False" }.to_string(),
        PyValue::Int(i) => i.to_string(),
        PyValue::Float(f) => format!("{}", f),
        PyValue::Str(s) => format!("'{}'", s),
        PyValue::List(l) => {
            let items: Vec<String> = l.to_vec().iter().map(format_value).collect();
            format!("[{}]", items.join(", "))
        }
        PyValue::Tuple(t) => {
            let items: Vec<String> = t.to_vec().iter().map(format_value).collect();
            if items.len() == 1 {
                format!("({},)", items[0])
            } else {
                format!("({})", items.join(", "))
            }
        }
        PyValue::Dict(d) => {
            let items: Vec<String> = d.items()
                .iter()
                .map(|(k, v)| format!("{:?}: {}", k, format_value(v)))
                .collect();
            format!("{{{}}}", items.join(", "))
        }
    }
}
