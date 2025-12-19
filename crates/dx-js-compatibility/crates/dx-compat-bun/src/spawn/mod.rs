//! Bun.spawn() process spawning.
//!
//! High-performance subprocess spawning targeting 10,000+ spawns/second.

use crate::error::{BunError, BunResult};
use bytes::Bytes;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;

/// Stdio configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StdioConfig {
    /// Pipe stdio for reading/writing
    #[default]
    Pipe,
    /// Inherit from parent process
    Inherit,
    /// Ignore (null)
    Ignore,
}

impl From<StdioConfig> for Stdio {
    fn from(config: StdioConfig) -> Self {
        match config {
            StdioConfig::Pipe => Stdio::piped(),
            StdioConfig::Inherit => Stdio::inherit(),
            StdioConfig::Ignore => Stdio::null(),
        }
    }
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
    /// Clear environment before adding env vars
    pub clear_env: bool,
}

impl SpawnOptions {
    /// Create new spawn options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set working directory.
    pub fn cwd(mut self, path: impl Into<PathBuf>) -> Self {
        self.cwd = Some(path.into());
        self
    }

    /// Set environment variable.
    pub fn env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.env
            .get_or_insert_with(HashMap::new)
            .insert(key.into(), value.into());
        self
    }

    /// Set multiple environment variables.
    pub fn envs(mut self, vars: impl IntoIterator<Item = (String, String)>) -> Self {
        self.env.get_or_insert_with(HashMap::new).extend(vars);
        self
    }

    /// Set stdin configuration.
    pub fn stdin(mut self, config: StdioConfig) -> Self {
        self.stdin = config;
        self
    }

    /// Set stdout configuration.
    pub fn stdout(mut self, config: StdioConfig) -> Self {
        self.stdout = config;
        self
    }

    /// Set stderr configuration.
    pub fn stderr(mut self, config: StdioConfig) -> Self {
        self.stderr = config;
        self
    }
}

/// Subprocess handle for async process management.
pub struct Subprocess {
    /// Process ID
    pub pid: u32,
    /// Child process handle
    child: tokio::process::Child,
    /// Stdin writer (if piped)
    stdin: Option<tokio::process::ChildStdin>,
    /// Stdout reader (if piped)
    stdout: Option<tokio::process::ChildStdout>,
    /// Stderr reader (if piped)
    stderr: Option<tokio::process::ChildStderr>,
}

impl Subprocess {
    /// Wait for the process to exit and return exit code.
    pub async fn exited(&mut self) -> BunResult<ExitStatus> {
        let status = self
            .child
            .wait()
            .await
            .map_err(|e| BunError::Spawn(format!("Wait failed: {}", e)))?;

        Ok(ExitStatus {
            code: status.code(),
            success: status.success(),
            #[cfg(unix)]
            signal: std::os::unix::process::ExitStatusExt::signal(&status),
            #[cfg(not(unix))]
            signal: None,
        })
    }

    /// Kill the process.
    pub async fn kill(&mut self) -> BunResult<()> {
        self.child
            .kill()
            .await
            .map_err(|e| BunError::Spawn(format!("Kill failed: {}", e)))
    }

    /// Send a signal to the process (Unix only).
    #[cfg(unix)]
    pub fn signal(&self, signal: i32) -> BunResult<()> {
        use nix::sys::signal::{kill, Signal};
        use nix::unistd::Pid;

        let sig = Signal::try_from(signal)
            .map_err(|e| BunError::Spawn(format!("Invalid signal: {}", e)))?;
        kill(Pid::from_raw(self.pid as i32), sig)
            .map_err(|e| BunError::Spawn(format!("Signal failed: {}", e)))
    }

    /// Write to stdin.
    pub async fn write_stdin(&mut self, data: &[u8]) -> BunResult<()> {
        if let Some(stdin) = &mut self.stdin {
            stdin
                .write_all(data)
                .await
                .map_err(|e| BunError::Spawn(format!("Stdin write failed: {}", e)))?;
            stdin
                .flush()
                .await
                .map_err(|e| BunError::Spawn(format!("Stdin flush failed: {}", e)))?;
        }
        Ok(())
    }

    /// Close stdin.
    pub fn close_stdin(&mut self) {
        self.stdin.take();
    }

    /// Read all stdout.
    pub async fn read_stdout(&mut self) -> BunResult<Bytes> {
        if let Some(stdout) = &mut self.stdout {
            let mut buffer = Vec::new();
            stdout
                .read_to_end(&mut buffer)
                .await
                .map_err(|e| BunError::Spawn(format!("Stdout read failed: {}", e)))?;
            Ok(Bytes::from(buffer))
        } else {
            Ok(Bytes::new())
        }
    }

    /// Read all stderr.
    pub async fn read_stderr(&mut self) -> BunResult<Bytes> {
        if let Some(stderr) = &mut self.stderr {
            let mut buffer = Vec::new();
            stderr
                .read_to_end(&mut buffer)
                .await
                .map_err(|e| BunError::Spawn(format!("Stderr read failed: {}", e)))?;
            Ok(Bytes::from(buffer))
        } else {
            Ok(Bytes::new())
        }
    }

    /// Read stdout line by line.
    pub async fn read_stdout_lines(&mut self) -> BunResult<Vec<String>> {
        if let Some(stdout) = self.stdout.take() {
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();
            let mut result = Vec::new();
            while let Some(line) = lines
                .next_line()
                .await
                .map_err(|e| BunError::Spawn(format!("Read line failed: {}", e)))?
            {
                result.push(line);
            }
            Ok(result)
        } else {
            Ok(Vec::new())
        }
    }
}
