//! # HTIP v1 Protocol Definition
//!
//! The exact binary layout of an HTIP stream.

use bincode::{Decode, Encode};
use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};

use crate::{MAGIC_BYTES, VERSION};

/// HTIP v1 Header (77 bytes fixed)
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct HtipHeader {
    /// Magic bytes: b"DXB1"
    pub magic: [u8; 4],

    /// Version: 1
    pub version: u8,

    /// Reserved (alignment)
    pub _reserved: [u8; 3],

    /// Ed25519 signature (64 bytes)
    pub signature: [u8; 64],

    /// Number of templates in dictionary
    pub template_count: u16,

    /// Alignment padding
    pub _padding: [u8; 2],

    /// Number of strings in string table
    pub string_count: u32,

    /// Total size of templates section (bytes)
    pub total_templates_size: u32,

    /// Total size of opcodes section (bytes)
    pub total_opcodes_size: u32,
}

impl HtipHeader {
    pub const SIZE: usize = 88; // 77 bytes + padding to 88 for alignment

    /// Create a new header
    pub fn new() -> Self {
        Self {
            magic: *MAGIC_BYTES,
            version: VERSION,
            _reserved: [0; 3],
            signature: [0; 64],
            template_count: 0,
            _padding: [0; 2],
            string_count: 0,
            total_templates_size: 0,
            total_opcodes_size: 0,
        }
    }

    /// Verify magic bytes and version
    pub fn verify(&self) -> crate::Result<()> {
        if &self.magic != MAGIC_BYTES {
            return Err(crate::DxBinaryError::InvalidMagic);
        }

        if self.version != VERSION {
            return Err(crate::DxBinaryError::UnsupportedVersion(self.version));
        }

        Ok(())
    }
}

impl Default for HtipHeader {
    fn default() -> Self {
        Self::new()
    }
}

/// Complete HTIP payload structure
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct HtipPayload {
    /// String table (deduplicated strings)
    pub strings: Vec<String>,

    /// Template definitions
    pub templates: Vec<crate::opcodes::TemplateDef>,

    /// Operations stream
    pub operations: Vec<crate::opcodes::Operation>,
}

impl HtipPayload {
    /// Create empty payload
    pub fn new() -> Self {
        Self {
            strings: Vec::new(),
            templates: Vec::new(),
            operations: Vec::new(),
        }
    }

    /// Calculate total size estimate
    pub fn estimate_size(&self) -> usize {
        let string_size: usize = self.strings.iter().map(|s| s.len() + 4).sum();
        let template_size = self.templates.len() * 128; // Rough estimate
        let op_size = self.operations.len() * 32; // Rough estimate
        string_size + template_size + op_size
    }
}

impl Default for HtipPayload {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_size() {
        assert_eq!(std::mem::size_of::<HtipHeader>(), HtipHeader::SIZE);
    }

    #[test]
    fn test_header_alignment() {
        // Ensure proper alignment for zero-copy (at least 4 bytes)
        assert!(std::mem::align_of::<HtipHeader>() >= 4);
    }

    #[test]
    fn test_header_new() {
        let header = HtipHeader::new();
        assert_eq!(&header.magic, MAGIC_BYTES);
        assert_eq!(header.version, VERSION);
        assert!(header.verify().is_ok());
    }

    #[test]
    fn test_header_pod() {
        // Verify we can cast to/from bytes
        let header = HtipHeader::new();
        let bytes: &[u8] = bytemuck::bytes_of(&header);
        assert_eq!(bytes.len(), HtipHeader::SIZE);

        let parsed: &HtipHeader = bytemuck::from_bytes(&bytes[..HtipHeader::SIZE]);
        assert_eq!(&parsed.magic, MAGIC_BYTES);
    }
}
