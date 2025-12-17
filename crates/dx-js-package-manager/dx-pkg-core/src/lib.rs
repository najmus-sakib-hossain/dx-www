//! dx-pkg-core: Core types and memory layouts for dx package manager
//!
//! This crate provides the fundamental data structures used across all
//! dx-package-manager crates. All structures use `#[repr(C, packed)]` for
//! binary compatibility and zero-copy memory mapping.

pub mod error;
pub mod hash;
pub mod headers;
pub mod version;

pub use error::{Error, Result};
pub use hash::{ContentHash, xxhash64, xxhash128};
pub use headers::{DxlHeader, DxpHeader, DxrpRequestHeader, DxrpResponseHeader};
pub use version::{Version, VersionConstraint, decode_version, encode_version};

/// Magic numbers for binary format identification
pub const DXP_MAGIC: &[u8; 4] = b"DXP\0";
pub const DXL_MAGIC: &[u8; 4] = b"DXL\0";
pub const DXRP_REQUEST_MAGIC: &[u8; 4] = b"DXRP";
pub const DXRP_RESPONSE_MAGIC: &[u8; 4] = b"DXRR";

/// Protocol version
pub const PROTOCOL_VERSION: u16 = 1;

/// Maximum supported file sizes (security limits)
pub const MAX_PACKAGE_SIZE: u64 = 1024 * 1024 * 1024; // 1GB
pub const MAX_FILE_COUNT: u32 = 100_000;
pub const MAX_PATH_LENGTH: u16 = 1024;
pub const MAX_LOCK_SIZE: u64 = 512 * 1024 * 1024; // 512MB

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magic_numbers() {
        assert_eq!(DXP_MAGIC, b"DXP\0");
        assert_eq!(DXL_MAGIC, b"DXL\0");
        assert_eq!(DXRP_REQUEST_MAGIC, b"DXRP");
        assert_eq!(DXRP_RESPONSE_MAGIC, b"DXRR");
    }
}
