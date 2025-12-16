//! dx-pkg-format: DXP binary package format implementation
//!
//! Provides zero-copy memory-mapped access to DXP packages with O(1) file lookups.

use dx_pkg_core::{DxpHeader, Error, Result, xxhash64, xxhash128, DXP_MAGIC};
use memmap2::Mmap;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

mod compression;
mod index;

use compression::{compress_lz4, compress_zstd, decompress};
use index::FileIndex;

/// DXP Package (memory-mapped)
pub struct DxpPackage {
    mmap: Mmap,
    header: DxpHeader,
    index: FileIndex,
}

impl DxpPackage {
    /// Open a DXP package file (zero-copy memory mapping)
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        let mmap = unsafe { Mmap::map(&file)? };

        // Verify magic number
        if &mmap[0..4] != DXP_MAGIC {
            return Err(Error::InvalidMagic {
                expected: *DXP_MAGIC,
                found: [mmap[0], mmap[1], mmap[2], mmap[3]],
            });
        }

        // Read header (zero-copy cast)
        let header = *bytemuck::from_bytes::<DxpHeader>(&mmap[0..128]);

        // Verify content hash
        let content_end = header.total_size as usize;
        if content_end > mmap.len() {
            return Err(Error::CorruptedData);
        }

        let computed_hash = xxhash128(&mmap[128..content_end]);
        if computed_hash != header.content_hash {
            return Err(Error::CorruptedData);
        }

        // Load file index
        let index = FileIndex::from_mmap(&mmap, &header)?;

        Ok(Self { mmap, header, index })
    }

    /// Get file content by path (zero-copy or decompressed)
    pub fn get_file(&self, path: &str) -> Result<Vec<u8>> {
        let path_hash = xxhash64(path.as_bytes());
        let entry = self.index.find(path_hash)?;

        let start = entry.offset as usize;
        let end = start + entry.compressed_size as usize;
        let data = &self.mmap[start..end];

        if entry.compressed_size > 0 && entry.compressed_size != entry.size {
            // Decompress
            decompress(data, entry.size as usize, entry.flags)
        } else {
            // Uncompressed, return copy
            Ok(data.to_vec())
        }
    }

    /// List all files in package
    pub fn list_files(&self) -> Vec<String> {
        self.index.list()
    }

    /// Get package metadata
    pub fn header(&self) -> &DxpHeader {
        &self.header
    }

    /// Get file count
    pub fn file_count(&self) -> u32 {
        self.header.file_count
    }
}

/// Builder for creating DXP packages
pub struct DxpBuilder {
    name: String,
    version: String,
    files: HashMap<String, Vec<u8>>,
}

impl DxpBuilder {
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            files: HashMap::new(),
        }
    }

    pub fn add_file(&mut self, path: impl Into<String>, content: Vec<u8>) {
        self.files.insert(path.into(), content);
    }

    pub fn build<P: AsRef<Path>>(self, output: P) -> Result<()> {
        // TODO: Implement DXP package creation
        // This is a complex task that will be completed in the next iteration
        todo!("DXP package creation not yet implemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder() {
        let mut builder = DxpBuilder::new("test-package", "1.0.0");
        builder.add_file("index.js", b"console.log('hello');".to_vec());
        assert_eq!(builder.files.len(), 1);
    }
}
