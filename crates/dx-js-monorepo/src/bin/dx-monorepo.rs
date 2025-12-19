//! dx-monorepo CLI binary
//!
//! Binary-first monorepo management system.

use std::process::ExitCode;

fn main() -> ExitCode {
    let exit_code = dx_js_monorepo::cli::main();
    ExitCode::from(exit_code as u8)
}
