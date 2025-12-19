//! DXC Cache Format
//!
//! Zero-copy task output caching with XOR differential updates.

use bytemuck::{Pod, Zeroable};
use crate::{DXC_MAGIC, FORMAT_VERSION};
use crate::error::CacheError;

/// DXC Cache entry header
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct DxcHeader {
    /// Magic bytes: "DXC\0"
    pub magic: [u8; 4],
    /// Format version
    pub version: u32,
    /// Task hash that produced this cache
    pub task_hash: [u8; 32],
    /// Ed25519 signature of content
    pub signature: [u8; 64],
    /// Public key used for signing
    pub public_key: [u8; 32],
    /// Uncompressed size
    pub uncompressed_size: u64,
    /// Compressed size (0 if uncompressed)
    pub compressed_size: u64,
    /// Number of output files
    pub file_count: u32,
    /// Offset to file entries
    pub files_offset: u64,
}

impl DxcHeader {
    /// Size of header in bytes
    pub const SIZE: usize = std::mem::size_of::<Self>();

    /// Create a new header
    pub fn new(task_hash: [u8; 32]) -> Self {
        Self {
            magic: DXC_MAGIC,
            version: FORMAT_VERSION,
            task_hash,
            signature: [0; 64],
            public_key: [0; 32],
            uncompressed_size: 0,
            compressed_size: 0,
            file_count: 0,
            files_offset: Self::SIZE as u64,
        }
    }

    /// Validate magic bytes
    pub fn validate_magic(&self) -> Result<(), CacheError> {
        if self.magic != DXC_MAGIC {
            return Err(CacheError::InvalidMagic { found: self.magic });
        }
        Ok(())
    }
}

/// File entry in cache
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct CacheFileEntry {
    /// Offset to file path in string table
    pub path_offset: u32,
    /// File content offset
    pub content_offset: u64,
    /// File size
    pub size: u64,
    /// File mode/permissions
    pub mode: u32,
}

/// XOR patch for differential updates
#[derive(Debug, Clone)]
pub struct XorPatch {
    /// Base entry hash
    pub base_hash: [u8; 32],
    /// Target entry hash
    pub target_hash: [u8; 32],
    /// Sparse XOR blocks: (offset, data)
    pub blocks: Vec<XorBlock>,
}

/// Single XOR block
#[derive(Debug, Clone)]
pub struct XorBlock {
    /// Offset in the file
    pub offset: u64,
    /// XOR data
    pub data: Vec<u8>,
}

impl XorPatch {
    /// Create a new XOR patch between two byte slices
    pub fn create(base: &[u8], target: &[u8]) -> Self {
        let base_hash = blake3::hash(base);
        let target_hash = blake3::hash(target);
        
        let mut blocks = Vec::new();
        let mut current_block: Option<(u64, Vec<u8>)> = None;
        
        let max_len = base.len().max(target.len());
        
        for i in 0..max_len {
            let base_byte = base.get(i).copied().unwrap_or(0);
            let target_byte = target.get(i).copied().unwrap_or(0);
            let xor_byte = base_byte ^ target_byte;
            
            if xor_byte != 0 {
                match &mut current_block {
                    Some((_, data)) => data.push(xor_byte),
                    None => current_block = Some((i as u64, vec![xor_byte])),
                }
            } else if let Some((offset, data)) = current_block.take() {
                blocks.push(XorBlock { offset, data });
            }
        }
        
        if let Some((offset, data)) = current_block {
            blocks.push(XorBlock { offset, data });
        }
        
        Self {
            base_hash: *base_hash.as_bytes(),
            target_hash: *target_hash.as_bytes(),
            blocks,
        }
    }

    /// Apply patch to base data
    pub fn apply(&self, base: &[u8]) -> Vec<u8> {
        let mut result = base.to_vec();
        
        for block in &self.blocks {
            let start = block.offset as usize;
            let end = start + block.data.len();
            
            // Extend if necessary
            if end > result.len() {
                result.resize(end, 0);
            }
            
            for (i, &xor_byte) in block.data.iter().enumerate() {
                result[start + i] ^= xor_byte;
            }
        }
        
        result
    }

    /// Calculate patch size
    pub fn size(&self) -> usize {
        64 + self.blocks.iter().map(|b| 8 + b.data.len()).sum::<usize>()
    }

    /// Calculate efficiency (patch size / target size)
    pub fn efficiency(&self, target_size: usize) -> f64 {
        if target_size == 0 {
            return 1.0;
        }
        self.size() as f64 / target_size as f64
    }
}

/// Cache entry for storage
#[derive(Debug, Clone)]
pub struct CacheEntry {
    /// Task hash
    pub task_hash: [u8; 32],
    /// Output files
    pub files: Vec<CacheFile>,
    /// Ed25519 signature
    pub signature: Option<[u8; 64]>,
    /// Public key
    pub public_key: Option<[u8; 32]>,
}

/// Cached file
#[derive(Debug, Clone)]
pub struct CacheFile {
    /// Relative path
    pub path: String,
    /// File content
    pub content: Vec<u8>,
    /// File mode
    pub mode: u32,
}

impl CacheEntry {
    /// Create new cache entry
    pub fn new(task_hash: [u8; 32]) -> Self {
        Self {
            task_hash,
            files: Vec::new(),
            signature: None,
            public_key: None,
        }
    }

    /// Add a file to the cache entry
    pub fn add_file(&mut self, path: String, content: Vec<u8>, mode: u32) {
        self.files.push(CacheFile { path, content, mode });
    }

    /// Calculate total content size
    pub fn total_size(&self) -> usize {
        self.files.iter().map(|f| f.content.len()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dxc_header_size() {
        assert_eq!(DxcHeader::SIZE, 168);
    }

    #[test]
    fn test_xor_patch_creation() {
        let base = b"Hello, World!";
        let target = b"Hello, Rust!!";
        
        let patch = XorPatch::create(base, target);
        let result = patch.apply(base);
        
        assert_eq!(result, target);
    }

    #[test]
    fn test_xor_patch_efficiency() {
        // Similar content should have small patch
        let base = vec![0u8; 1000];
        let mut target = base.clone();
        target[500] = 1; // Change one byte
        
        let patch = XorPatch::create(&base, &target);
        
        // Patch should be much smaller than full content
        assert!(patch.size() < 100);
        assert!(patch.efficiency(target.len()) < 0.1);
    }

    #[test]
    fn test_xor_patch_different_sizes() {
        let base = b"short";
        let target = b"much longer string";
        
        let patch = XorPatch::create(base, target);
        let result = patch.apply(base);
        
        assert_eq!(result, target);
    }

    #[test]
    fn test_cache_entry() {
        let mut entry = CacheEntry::new([1; 32]);
        entry.add_file("dist/index.js".to_string(), b"console.log('hi')".to_vec(), 0o644);
        entry.add_file("dist/index.d.ts".to_string(), b"export {}".to_vec(), 0o644);
        
        assert_eq!(entry.files.len(), 2);
        assert_eq!(entry.total_size(), 26);
    }
}
