//! NaN-boxed value representation for efficient primitives

/// NaN-boxed value (64-bit)
///
/// Uses NaN-boxing technique where:
/// - Regular f64 values are stored directly
/// - Other values use the NaN space (when exponent bits are all 1s)
///
/// Layout:
/// - f64 number: regular IEEE 754 double
/// - Pointer: 0x7FF8_xxxx_xxxx_xxxx (quiet NaN with payload)
/// - Integer: 0x7FF9_xxxx_xxxx_xxxx
/// - Boolean: 0x7FFA_0000_0000_000x (x = 0 or 1)
/// - Null: 0x7FFB_0000_0000_0000
/// - Undefined: 0x7FFC_0000_0000_0000
#[derive(Clone, Copy)]
pub struct TaggedValue(u64);

// Tag bits (stored in bits 48-50)
const TAG_POINTER: u64 = 0x7FF8_0000_0000_0000;
const TAG_INTEGER: u64 = 0x7FF9_0000_0000_0000;
const TAG_BOOLEAN: u64 = 0x7FFA_0000_0000_0000;
const TAG_NULL: u64 = 0x7FFB_0000_0000_0000;
const TAG_UNDEFINED: u64 = 0x7FFC_0000_0000_0000;

// Mask for extracting payload
const PAYLOAD_MASK: u64 = 0x0000_FFFF_FFFF_FFFF;

impl TaggedValue {
    /// Create from f64
    #[inline]
    pub fn from_f64(n: f64) -> Self {
        Self(n.to_bits())
    }

    /// Create from i32
    #[inline]
    pub fn from_i32(n: i32) -> Self {
        Self(TAG_INTEGER | (n as u32 as u64))
    }

    /// Create from bool
    #[inline]
    pub fn from_bool(b: bool) -> Self {
        Self(TAG_BOOLEAN | (b as u64))
    }

    /// Create null
    #[inline]
    pub fn null() -> Self {
        Self(TAG_NULL)
    }

    /// Create undefined
    #[inline]
    pub fn undefined() -> Self {
        Self(TAG_UNDEFINED)
    }

    /// Create from pointer
    #[inline]
    pub fn from_ptr(ptr: *const u8) -> Self {
        Self(TAG_POINTER | (ptr as u64 & PAYLOAD_MASK))
    }

    /// Check if this is a number
    #[inline]
    pub fn is_number(&self) -> bool {
        let bits = self.0;
        (bits & 0x7FF0_0000_0000_0000) != 0x7FF0_0000_0000_0000
            || (bits & 0x000F_0000_0000_0000) == 0
    }

    /// Get as f64
    #[inline]
    pub fn as_f64(&self) -> Option<f64> {
        if self.is_number() {
            Some(f64::from_bits(self.0))
        } else {
            None
        }
    }

    /// Check if null
    #[inline]
    pub fn is_null(&self) -> bool {
        self.0 == TAG_NULL
    }

    /// Check if undefined
    #[inline]
    pub fn is_undefined(&self) -> bool {
        self.0 == TAG_UNDEFINED
    }

    /// Check if boolean
    #[inline]
    pub fn is_boolean(&self) -> bool {
        (self.0 & 0xFFFF_0000_0000_0000) == TAG_BOOLEAN
    }

    /// Get as bool
    #[inline]
    pub fn as_bool(&self) -> Option<bool> {
        if self.is_boolean() {
            Some((self.0 & 1) != 0)
        } else {
            None
        }
    }

    /// Get raw bits
    #[inline]
    pub fn bits(&self) -> u64 {
        self.0
    }
}

impl std::fmt::Debug for TaggedValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(n) = self.as_f64() {
            write!(f, "TaggedValue::Number({})", n)
        } else if self.is_null() {
            write!(f, "TaggedValue::Null")
        } else if self.is_undefined() {
            write!(f, "TaggedValue::Undefined")
        } else if let Some(b) = self.as_bool() {
            write!(f, "TaggedValue::Boolean({})", b)
        } else {
            write!(f, "TaggedValue::Pointer(0x{:x})", self.0 & PAYLOAD_MASK)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f64() {
        let v = TaggedValue::from_f64(3.14);
        assert!(v.is_number());
        assert_eq!(v.as_f64(), Some(3.14));
    }

    #[test]
    fn test_bool() {
        let t = TaggedValue::from_bool(true);
        let f = TaggedValue::from_bool(false);
        assert!(t.is_boolean());
        assert!(f.is_boolean());
        assert_eq!(t.as_bool(), Some(true));
        assert_eq!(f.as_bool(), Some(false));
    }

    #[test]
    fn test_null_undefined() {
        let null = TaggedValue::null();
        let undef = TaggedValue::undefined();
        assert!(null.is_null());
        assert!(undef.is_undefined());
    }
}
