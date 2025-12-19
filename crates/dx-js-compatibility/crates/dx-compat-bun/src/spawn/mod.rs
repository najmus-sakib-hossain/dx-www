//! Bun.spawn() process spawning.

use crate::error::{BunError, BunResult};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::process::Command;

/// Stdio configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StdioConfig {
    /// Pipe stdio
    #[default]
    Pipe,
    /// Inherit from parent
    Inherit,
    /// Ignore
    Ignore,
}

/// Spawn options.
#[derive(Debug, Clone, Default)]
pub struct SpawnOptions {
    /// Working directory
    pub cwd: Option<PathBuf>,
    /// Environment variables
    pub env: Option<HashMap<String, String>>,
    /// Stdin configuration
    pub stdin: StdioConfig,
    /// Stdout configuration
    pub stdout: StdioConfig,
    /// Stderr configuration
    pub stderr: StdioConfig,
}

/// Subprocess handle.
pub struct Subprocess {
    /// Process ID
    pub pid: u32,
    child: tokio::process::Child,
}

impl Subprocess {
    /// Wait for the process to exit.
    pub async fn exited(&mut self) -> BunResult<i32> {
        let status = self.child.wait().await?;
        Ok(status.code().unwrap_or(-1))
    }

    /// Kill the process.
    pub async fn kill(&mut self, _signal: Option<i32>) -> BunResult<()> {
        self.child.kill().await?;
        Ok(())
    }
}

/// Synchronous subprocess result.
#[derive(Debug)]
pub struct SyncSubprocess {
    /// Exit code
    pub exit_code: i32,
    /// Stdout bytes
    pub stdout: Vec<u8>,
    /// Stderr bytes
    pub stderr: Vec<u8>,
}

/// Spawn async subprocess.
pub async fn spawn(cmd: &[&str], options: Option<SpawnOptions>) -> BunResult<Subprocess> {
    if cmd.is_empty() {
        return Err(BunError::Spawn("Empty command".to_string()));
    }

    let options = options.unwrap_or_default();
    let mut command = Command::new(cmd[0]);
    
    if cmd.len() > 1 {
        command.args(&cmd[1..]);
    }

    if let Some(cwd) = &options.cwd {
        command.current_dir(cwd);
    }

    if let Some(env) = &options.env {
        command.envs(env);
    }

    let child = command.spawn()?;
    let pid = child.id().unwrap_or(0);

    Ok(Subprocess { pid, child })
}

/// Spawn sync subprocess.
pub fn spawn_sync(cmd: &[&str], options: Option<SpawnOptions>) -> BunResult<SyncSubprocess> {
    if cmd.is_empty() {
        return Err(BunError::Spawn("Empty command".to_string()));
    }

    let options = options.unwrap_or_default();
    let mut command = std::process::Command::new(cmd[0]);
    
    if cmd.len() > 1 {
        command.args(&cmd[1..]);
    }

    if let Some(cwd) = &options.cwd {
        command.current_dir(cwd);
    }

    if let Some(env) = &options.env {
        command.envs(env);
    }

    let output = command.output()?;

    Ok(SyncSubprocess {
        exit_code: output.status.code().unwrap_or(-1),
        stdout: output.stdout,
        stderr: output.stderr,
    })
}
