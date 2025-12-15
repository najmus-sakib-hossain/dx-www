//! dx-js CLI entry point

use dx_js_runtime::DxRuntime;
use std::env;
use std::process::ExitCode;
use std::time::Instant;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("dx-js v{} - The fastest JavaScript/TypeScript runtime", env!("CARGO_PKG_VERSION"));
        eprintln!();
        eprintln!("Usage: dx-js <file.js|file.ts>");
        eprintln!();
        eprintln!("Examples:");
        eprintln!("  dx-js script.js       Run a JavaScript file");
        eprintln!("  dx-js app.ts          Run a TypeScript file");
        eprintln!("  dx-js --version       Show version");
        return ExitCode::from(1);
    }

    let file = &args[1];

    // Check for special flags
    if file == "--version" || file == "-v" {
        println!("dx-js-runtime {}", env!("CARGO_PKG_VERSION"));
        return ExitCode::SUCCESS;
    }

    if file == "--help" || file == "-h" {
        println!("dx-js - The fastest JavaScript/TypeScript runtime");
        println!();
        println!("Targeting 10x faster than Bun through:");
        println!("  • OXC parser (fastest JS/TS parser)");
        println!("  • Cranelift JIT (native code generation)");
        println!("  • Arena-based memory (zero GC pauses)");
        println!("  • Persistent code cache (instant cold starts)");
        println!();
        println!("Usage: dx-js <file.js|file.ts>");
        println!();
        println!("Options:");
        println!("  -v, --version    Print version");
        println!("  -h, --help       Print this help");
        println!();
        println!("Environment:");
        println!("  DX_DEBUG=1       Show execution timing");
        return ExitCode::SUCCESS;
    }

    // Check if file exists
    if !std::path::Path::new(file).exists() {
        eprintln!("Error: File not found: {}", file);
        return ExitCode::from(1);
    }

    // Run the file - FAST PATH
    let start = Instant::now();
    
    match std::fs::read_to_string(file) {
        Ok(source) => {
            let output = dx_js_runtime::simple_exec::execute_js(&source);
            println!("{}", output);
            
            let elapsed = start.elapsed();
            if env::var("DX_DEBUG").is_ok() {
                eprintln!("\n─── Performance ───");
                eprintln!("  Total time: {:?}", elapsed);
            }
            
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            ExitCode::from(1)
        }
    }
}
