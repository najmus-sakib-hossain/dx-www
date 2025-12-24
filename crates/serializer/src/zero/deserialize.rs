//! DX-Zero deserialization (zero-copy)

use super::header::DxZeroHeader;
use super::types::{DxZeroError, Result};

/// Deserialize DX-Zero format from bytes (zero-copy, ~2ns)
///
/// This is a generic function that validates the header and casts
/// the buffer to the target type. Type-specific accessors handle
/// field extraction.
///
/// # Safety
///
/// This function performs an unsafe pointer cast. The caller must ensure:
/// - Buffer remains valid during struct lifetime
/// - Buffer is not modified during access
/// - Alignment is correct for target type
#[inline]
pub fn from_bytes<T>(bytes: &[u8]) -> Result<&T> {
    // Validate header
    let header = DxZeroHeader::from_bytes(bytes)?;
    header.validate()?;

    // Check minimum size
    let min_size = std::mem::size_of::<T>();
    if bytes.len() < min_size {
        return Err(DxZeroError::BufferTooSmall {
            required: min_size,
            actual: bytes.len(),
        });
    }

    // Zero-copy cast (the magic happens here)
    Ok(unsafe { &*(bytes.as_ptr() as *const T) })
}

/// Validate buffer alignment for type T
#[inline]
pub fn check_alignment<T>(bytes: &[u8]) -> Result<()> {
    let align = std::mem::align_of::<T>();
    let ptr = bytes.as_ptr() as usize;

    if ptr % align != 0 {
        return Err(DxZeroError::InvalidAlignment);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[repr(C, packed)]
    struct TestStruct {
        header: [u8; 4],
        value: u32,
    }

    #[test]
    fn test_from_bytes_valid() {
        let mut bytes = vec![0u8; 16];
        bytes[0] = 0x5A; // Magic
        bytes[1] = 0x44;
        bytes[2] = 0x01; // Version
        bytes[3] = 0b0000_0100; // Little-endian flag

        // This would normally succeed with a proper struct
        let header = DxZeroHeader::from_bytes(&bytes);
        assert!(header.is_ok());
    }

    #[test]
    fn test_from_bytes_invalid_magic() {
        let bytes = vec![0x00, 0x00, 0x01, 0x00];
        let result = DxZeroHeader::from_bytes(&bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_bytes_too_small() {
        let bytes = vec![0x5A, 0x44]; // Only 2 bytes
        let result = DxZeroHeader::from_bytes(&bytes);
        use crate::zero::header::HeaderError;
        assert!(matches!(result, Err(HeaderError::BufferTooSmall)));
    }
}
