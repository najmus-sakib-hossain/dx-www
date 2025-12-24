//! File system operations with memory-mapped I/O optimization.
//!
//! This module provides Node.js `fs` module compatibility with enhanced performance
//! through memory-mapped I/O for large files.

use crate::error::{NodeError, NodeResult};
use bytes::Bytes;
use std::path::Path;
use std::time::SystemTime;
use tokio::fs as async_fs;

/// Threshold for using memory-mapped I/O (1MB).
const MMAP_THRESHOLD: u64 = 1_048_576;

/// Read file contents, using mmap for large files (>1MB).
pub async fn read_file(path: impl AsRef<Path>) -> NodeResult<Bytes> {
    let path = path.as_ref();
    let metadata = async_fs::metadata(path).await.map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            NodeError::enoent(path.display().to_string())
        } else {
            NodeError::from(e)
        }
    })?;

    if metadata.len() > MMAP_THRESHOLD {
        read_file_mmap(path).await
    } else {
        let data = async_fs::read(path).await?;
        Ok(Bytes::from(data))
    }
}

/// Read file using memory-mapped I/O.
async fn read_file_mmap(path: impl AsRef<Path>) -> NodeResult<Bytes> {
    let path = path.as_ref().to_path_buf();
    tokio::task::spawn_blocking(move || {
        let file = std::fs::File::open(&path)?;
        let mmap = unsafe { memmap2::Mmap::map(&file)? };
        Ok(Bytes::copy_from_slice(&mmap))
    })
    .await
    .map_err(|e| NodeError::new(crate::error::ErrorCode::UNKNOWN, e.to_string()))?
}

/// Write data to file.
pub async fn write_file(path: impl AsRef<Path>, data: impl AsRef<[u8]>) -> NodeResult<()> {
    async_fs::write(path, data).await?;
    Ok(())
}

/// Read directory contents.
pub async fn read_dir(path: impl AsRef<Path>) -> NodeResult<Vec<DirEntry>> {
    let mut entries = Vec::new();
    let mut dir = async_fs::read_dir(path).await?;

    while let Some(entry) = dir.next_entry().await? {
        entries.push(DirEntry {
            name: entry.file_name().to_string_lossy().to_string(),
            path: entry.path(),
        });
    }

    Ok(entries)
}

/// Get file/directory metadata.
pub async fn stat(path: impl AsRef<Path>) -> NodeResult<Stats> {
    let metadata = async_fs::metadata(path).await?;
    Ok(Stats::from_metadata(&metadata))
}

/// Create directory.
pub async fn mkdir(path: impl AsRef<Path>, recursive: bool) -> NodeResult<()> {
    if recursive {
        async_fs::create_dir_all(path).await?;
    } else {
        async_fs::create_dir(path).await?;
    }
    Ok(())
}

/// Delete file.
pub async fn unlink(path: impl AsRef<Path>) -> NodeResult<()> {
    async_fs::remove_file(path).await?;
    Ok(())
}

/// Rename/move file.
pub async fn rename(from: impl AsRef<Path>, to: impl AsRef<Path>) -> NodeResult<()> {
    async_fs::rename(from, to).await?;
    Ok(())
}

/// Directory entry.
#[derive(Debug, Clone)]
pub struct DirEntry {
    /// Entry name
    pub name: String,
    /// Full path
    pub path: std::path::PathBuf,
}

/// File statistics matching Node.js fs.Stats.
#[derive(Debug, Clone)]
pub struct Stats {
    /// File size in bytes
    pub size: u64,
    /// Last modification time
    pub mtime: SystemTime,
    /// Last access time
    pub atime: SystemTime,
    /// Creation time
    pub ctime: SystemTime,
    /// Is a file
    pub is_file: bool,
    /// Is a directory
    pub is_directory: bool,
    /// Is a symbolic link
    pub is_symlink: bool,
    /// File mode/permissions
    pub mode: u32,
}

impl Stats {
    /// Create Stats from std::fs::Metadata.
    pub fn from_metadata(metadata: &std::fs::Metadata) -> Self {
        Self {
            size: metadata.len(),
            mtime: metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH),
            atime: metadata.accessed().unwrap_or(SystemTime::UNIX_EPOCH),
            ctime: metadata.created().unwrap_or(SystemTime::UNIX_EPOCH),
            is_file: metadata.is_file(),
            is_directory: metadata.is_dir(),
            is_symlink: metadata.is_symlink(),
            #[cfg(unix)]
            mode: std::os::unix::fs::MetadataExt::mode(metadata),
            #[cfg(not(unix))]
            mode: 0,
        }
    }
}

/// Synchronous file operations.
pub mod sync {
    use super::*;

    /// Read file synchronously.
    pub fn read_file_sync(path: impl AsRef<Path>) -> NodeResult<Bytes> {
        let data = std::fs::read(path)?;
        Ok(Bytes::from(data))
    }

    /// Write file synchronously.
    pub fn write_file_sync(path: impl AsRef<Path>, data: impl AsRef<[u8]>) -> NodeResult<()> {
        std::fs::write(path, data)?;
        Ok(())
    }

    /// Read directory synchronously.
    pub fn read_dir_sync(path: impl AsRef<Path>) -> NodeResult<Vec<DirEntry>> {
        let entries = std::fs::read_dir(path)?
            .filter_map(|e| e.ok())
            .map(|e| DirEntry {
                name: e.file_name().to_string_lossy().to_string(),
                path: e.path(),
            })
            .collect();
        Ok(entries)
    }

    /// Get file stats synchronously.
    pub fn stat_sync(path: impl AsRef<Path>) -> NodeResult<Stats> {
        let metadata = std::fs::metadata(path)?;
        Ok(Stats::from_metadata(&metadata))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_read_write_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.txt");

        write_file(&path, b"hello world").await.unwrap();
        let content = read_file(&path).await.unwrap();

        assert_eq!(&content[..], b"hello world");
    }

    #[tokio::test]
    async fn test_stat() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.txt");

        write_file(&path, b"test").await.unwrap();
        let stats = stat(&path).await.unwrap();

        assert!(stats.is_file);
        assert!(!stats.is_directory);
        assert_eq!(stats.size, 4);
    }
}
