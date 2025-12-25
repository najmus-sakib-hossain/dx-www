//! macOS kqueue async I/O implementation
//!
//! This module provides async I/O using macOS's kqueue interface.
//! kqueue is a scalable event notification interface available on
//! BSD-derived systems including macOS.

use std::io;
use std::path::Path;

use super::AsyncFileIO;

/// kqueue-based async I/O implementation for macOS
///
/// Uses kqueue for event-driven file operations. Note that kqueue
/// is primarily designed for socket and pipe I/O; file I/O on macOS
/// typically uses Grand Central Dispatch or blocking operations.
pub struct KqueueIO {
    // kqueue doesn't directly support file I/O in the same way as io_uring
    // This implementation uses blocking I/O with potential for future
    // optimization using dispatch_io or similar APIs
}

impl KqueueIO {
    /// Create a new kqueue I/O instance
    pub fn new() -> Self {
        Self {}
    }

    /// Check if kqueue is available (always true on macOS)
    pub fn is_available() -> bool {
        true
    }
}

impl Default for KqueueIO {
    fn default() -> Self {
        Self::new()
    }
}

impl AsyncFileIO for KqueueIO {
    fn read_sync(&self, path: &Path) -> io::Result<Vec<u8>> {
        std::fs::read(path)
    }

    fn write_sync(&self, path: &Path, data: &[u8]) -> io::Result<()> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() && !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }
        std::fs::write(path, data)
    }

    fn read_batch_sync(&self, paths: &[&Path]) -> io::Result<Vec<io::Result<Vec<u8>>>> {
        // TODO: Consider using dispatch_io for true async batch operations
        Ok(paths.iter().map(|p| std::fs::read(p)).collect())
    }

    fn write_batch_sync(&self, files: &[(&Path, &[u8])]) -> io::Result<Vec<io::Result<()>>> {
        // TODO: Consider using dispatch_io for true async batch operations
        Ok(files
            .iter()
            .map(|(path, data)| {
                if let Some(parent) = path.parent() {
                    if !parent.as_os_str().is_empty() && !parent.exists() {
                        std::fs::create_dir_all(parent)?;
                    }
                }
                std::fs::write(path, data)
            })
            .collect())
    }

    fn backend_name(&self) -> &'static str {
        "kqueue"
    }

    fn is_available(&self) -> bool {
        Self::is_available()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_kqueue_read_write() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("test.txt");
        let io = KqueueIO::new();

        let data = b"Hello from kqueue!";
        io.write_sync(&path, data).unwrap();

        let read_data = io.read_sync(&path).unwrap();
        assert_eq!(read_data, data);
    }

    #[test]
    fn test_kqueue_backend_name() {
        let io = KqueueIO::new();
        assert_eq!(io.backend_name(), "kqueue");
        assert!(io.is_available());
    }
}
