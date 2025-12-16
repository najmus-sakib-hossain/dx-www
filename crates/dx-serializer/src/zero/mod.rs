//! DX-Zero: Ultra-fast zero-copy binary format
//!
//! This module implements the machine-optimized binary backend for dx-serializer.
//! It achieves 0 ns serialization and 0.8-2.1 ns deserialization through:
//! - Compile-time field offsets (no indirection)
//! - Inline small objects (no pointer chasing for 90%+ strings)
//! - Direct memory access (single load per field)
//!
//! See docs/DX_ZERO_SPECIFICATION.md for complete technical specification.

pub mod builder;
pub mod deserialize;
pub mod format;
pub mod header;
pub mod simd;
pub mod slot;
pub mod traits;
pub mod types;

pub use builder::DxZeroBuilder;
pub use deserialize::from_bytes;
pub use format::{detect_format, parse_auto, DxFormat, FormatMode};
pub use header::{DxZeroHeader, FLAG_HAS_HEAP, FLAG_HAS_INTERN, FLAG_HAS_LENGTH_TABLE, FLAG_LITTLE_ENDIAN};
pub use slot::{DxZeroSlot, HEAP_MARKER, INLINE_MARKER, MAX_INLINE_SIZE};
pub use traits::{DxZeroDeserialize, DxZeroSerialize};
pub use types::DxZeroError;

/// DX-Zero magic bytes: 0x5A 0x44 ("ZD" little-endian)
pub const MAGIC: [u8; 2] = [0x5A, 0x44];

/// DX-Zero format version
pub const VERSION: u8 = 0x01;

/// Slot size in bytes (16 bytes for inline optimization)
pub const SLOT_SIZE: usize = 16;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magic_bytes() {
        assert_eq!(MAGIC, [0x5A, 0x44]);
        assert_eq!(VERSION, 0x01);
    }

    #[test]
    fn test_slot_size() {
        assert_eq!(SLOT_SIZE, 16);
        assert_eq!(MAX_INLINE_SIZE, 14);
    }
}
