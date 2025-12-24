//! Memory buffer utilities for zero-copy media processing.
//!
//! Provides efficient memory-mapped and owned buffers for handling
//! large media files without excessive memory copies.

use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::Arc;

use super::CoreResult;

/// A media buffer that can be either owned or memory-mapped.
#[derive(Debug)]
pub enum MediaBuffer {
    /// Owned byte vector (for small files or in-memory processing).
    Owned(Vec<u8>),
    /// Memory-mapped file (for large files, zero-copy).
    Mapped(MappedBuffer),
}

impl MediaBuffer {
    /// Create a new buffer by reading a file.
    /// Uses memory mapping for files larger than the threshold.
    pub fn from_file(path: impl AsRef<Path>, mmap_threshold: u64) -> CoreResult<Self> {
        let path = path.as_ref();
        let metadata = std::fs::metadata(path)?;
        let file_size = metadata.len();

        if file_size > mmap_threshold {
            Ok(Self::Mapped(MappedBuffer::new(path)?))
        } else {
            let mut file = File::open(path)?;
            let mut buffer = Vec::with_capacity(file_size as usize);
            file.read_to_end(&mut buffer)?;
            Ok(Self::Owned(buffer))
        }
    }

    /// Create an owned buffer from bytes.
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self::Owned(bytes)
    }

    /// Get the buffer contents as a byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Self::Owned(vec) => vec,
            Self::Mapped(mapped) => mapped.as_bytes(),
        }
    }

    /// Get the length of the buffer.
    pub fn len(&self) -> usize {
        match self {
            Self::Owned(vec) => vec.len(),
            Self::Mapped(mapped) => mapped.len(),
        }
    }

    /// Check if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Write the buffer contents to a file.
    pub fn write_to_file(&self, path: impl AsRef<Path>) -> CoreResult<()> {
        let mut file = File::create(path)?;
        file.write_all(self.as_bytes())?;
        Ok(())
    }
}

/// Memory-mapped file buffer for zero-copy access.
#[derive(Debug)]
pub struct MappedBuffer {
    /// The memory-mapped region.
    // Note: Using a simple Vec fallback since memmap2 is optional.
    // When memmap2 feature is enabled, this would use actual mmap.
    data: Arc<Vec<u8>>,
    /// Original file path (for reference).
    #[allow(dead_code)]
    path: std::path::PathBuf,
}

impl MappedBuffer {
    /// Create a new memory-mapped buffer from a file.
    pub fn new(path: impl AsRef<Path>) -> CoreResult<Self> {
        let path = path.as_ref();
        
        // Read the file into memory
        // TODO: Use actual memory mapping with memmap2 crate when feature is enabled
        let mut file = File::open(path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;

        Ok(Self {
            data: Arc::new(data),
            path: path.to_path_buf(),
        })
    }

    /// Get the mapped data as a byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    /// Get the length of the mapped data.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_buffer_from_bytes() {
        let data = vec![1, 2, 3, 4, 5];
        let buffer = MediaBuffer::from_bytes(data.clone());
        assert_eq!(buffer.as_bytes(), &data);
        assert_eq!(buffer.len(), 5);
    }

    #[test]
    fn test_buffer_write_to_file() {
        let data = vec![1, 2, 3, 4, 5];
        let buffer = MediaBuffer::from_bytes(data.clone());
        
        let temp = NamedTempFile::new().unwrap();
        buffer.write_to_file(temp.path()).unwrap();
        
        let read_back = std::fs::read(temp.path()).unwrap();
        assert_eq!(read_back, data);
    }
}
