use std::{env, process::ExitCode};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    let result;
    if args.len() == 1 {
        result = gold::lint(".", false);
    } else if args.len() == 2 {
        if args[1] == "--fix" {
            result = gold::lint(".", true);
        } else {
            result = gold::lint(&args[1], false);
        }
    } else if args.len() == 3 && args[2] == "--fix" {
        result = gold::lint(&args[1], true);
    } else {
        eprintln!("Usage: gold [path] [--fix]");
        return ExitCode::FAILURE;
    }

    match result {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(e) => {
            eprintln!("Error: {}", e);
            ExitCode::FAILURE
        }
    }
}
