//! # dx-compat-shell
//!
//! Shell scripting compatibility layer.

#![warn(missing_docs)]

mod command;
mod error;
mod output;

pub use command::ShellCommand;
pub use error::{ShellError, ShellResult};
pub use output::ShellOutput;

/// Execute a shell command.
pub async fn shell(cmd: &str, args: &[&str]) -> ShellResult<ShellOutput> {
    ShellCommand::new(cmd).args(args).run().await
}
