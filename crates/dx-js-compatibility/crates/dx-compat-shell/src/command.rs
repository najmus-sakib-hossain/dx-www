//! Shell command builder.

use crate::error::ShellResult;
use crate::output::ShellOutput;
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::process::Command;

/// Shell command builder.
pub struct ShellCommand {
    cmd: String,
    args: Vec<String>,
    env: HashMap<String, String>,
    cwd: Option<PathBuf>,
    quiet: bool,
    nothrow: bool,
}

impl ShellCommand {
    /// Create a new shell command.
    pub fn new(cmd: &str) -> Self {
        Self {
            cmd: cmd.to_string(),
            args: Vec::new(),
            env: HashMap::new(),
            cwd: None,
            quiet: false,
            nothrow: false,
        }
    }

    /// Add arguments.
    pub fn args(mut self, args: &[&str]) -> Self {
        self.args.extend(args.iter().map(|s| s.to_string()));
        self
    }

    /// Set environment variable.
    pub fn env(mut self, key: &str, value: &str) -> Self {
        self.env.insert(key.to_string(), value.to_string());
        self
    }

    /// Set working directory.
    pub fn cwd(mut self, path: impl Into<PathBuf>) -> Self {
        self.cwd = Some(path.into());
        self
    }

    /// Suppress output.
    pub fn quiet(mut self) -> Self {
        self.quiet = true;
        self
    }

    /// Don't throw on non-zero exit.
    pub fn nothrow(mut self) -> Self {
        self.nothrow = true;
        self
    }

    /// Run the command.
    pub async fn run(self) -> ShellResult<ShellOutput> {
        let mut cmd = Command::new(&self.cmd);
        cmd.args(&self.args);

        for (key, value) in &self.env {
            cmd.env(key, value);
        }

        if let Some(cwd) = &self.cwd {
            cmd.current_dir(cwd);
        }

        let output = cmd.output().await?;

        Ok(ShellOutput {
            stdout: output.stdout,
            stderr: output.stderr,
            exit_code: output.status.code().unwrap_or(-1),
        })
    }
}
