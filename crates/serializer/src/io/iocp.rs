//! Windows IOCP (I/O Completion Ports) async I/O implementation
//!
//! This module provides high-performance async I/O using Windows' I/O
//! Completion Ports (IOCP). IOCP is the most efficient async I/O mechanism
//! on Windows, used by high-performance servers and applications.

use std::io;
use std::path::Path;

use super::AsyncFileIO;

#[cfg(all(target_os = "windows", feature = "async-io"))]
use windows_sys::Win32::{
    Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE},
    Storage::FileSystem::{
        CreateFileW, ReadFile, WriteFile, CREATE_ALWAYS, FILE_ATTRIBUTE_NORMAL,
        FILE_FLAG_OVERLAPPED, FILE_SHARE_READ, GENERIC_READ, GENERIC_WRITE, OPEN_EXISTING,
    },
    System::IO::{CreateIoCompletionPort, GetQueuedCompletionStatus, OVERLAPPED},
};

/// IOCP-based async I/O implementation for Windows
///
/// Uses I/O Completion Ports for high-performance file operations.
/// IOCP provides excellent scalability for concurrent I/O operations.
pub struct IocpIO {
    #[cfg(all(target_os = "windows", feature = "async-io"))]
    completion_port: HANDLE,
    #[cfg(not(all(target_os = "windows", feature = "async-io")))]
    _phantom: std::marker::PhantomData<()>,
}

impl IocpIO {
    /// Create a new IOCP I/O instance
    ///
    /// # Errors
    ///
    /// Returns an error if IOCP creation fails.
    #[cfg(all(target_os = "windows", feature = "async-io"))]
    pub fn new() -> io::Result<Self> {
        unsafe {
            let completion_port = CreateIoCompletionPort(INVALID_HANDLE_VALUE, 0, 0, 0);
            if completion_port == 0 {
                return Err(io::Error::last_os_error());
            }
            Ok(Self { completion_port })
        }
    }

    #[cfg(not(all(target_os = "windows", feature = "async-io")))]
    pub fn new() -> io::Result<Self> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "IOCP support not compiled (enable 'async-io' feature on Windows)",
        ))
    }

    /// Check if IOCP is available (always true on Windows with feature enabled)
    pub fn is_available() -> bool {
        #[cfg(all(target_os = "windows", feature = "async-io"))]
        {
            true
        }
        #[cfg(not(all(target_os = "windows", feature = "async-io")))]
        {
            false
        }
    }
}

#[cfg(all(target_os = "windows", feature = "async-io"))]
impl Drop for IocpIO {
    fn drop(&mut self) {
        unsafe {
            if self.completion_port != 0 {
                CloseHandle(self.completion_port);
            }
        }
    }
}

impl AsyncFileIO for IocpIO {
    fn read_sync(&self, path: &Path) -> io::Result<Vec<u8>> {
        // For now, use blocking read
        // A full async implementation would use overlapped I/O with IOCP
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
        // TODO: Implement true batch read using IOCP
        Ok(paths.iter().map(|p| std::fs::read(p)).collect())
    }

    fn write_batch_sync(&self, files: &[(&Path, &[u8])]) -> io::Result<Vec<io::Result<()>>> {
        // TODO: Implement true batch write using IOCP
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
        "iocp"
    }

    fn is_available(&self) -> bool {
        Self::is_available()
    }
}

#[cfg(all(test, target_os = "windows", feature = "async-io"))]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_iocp_read_write() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("test.txt");
        let io = IocpIO::new().unwrap();

        let data = b"Hello from IOCP!";
        io.write_sync(&path, data).unwrap();

        let read_data = io.read_sync(&path).unwrap();
        assert_eq!(read_data, data);
    }

    #[test]
    fn test_iocp_backend_name() {
        let io = IocpIO::new().unwrap();
        assert_eq!(io.backend_name(), "iocp");
        assert!(io.is_available());
    }
}
