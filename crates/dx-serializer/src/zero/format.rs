//! Format detection and dual-mode support
//!
//! Automatically detects DX-Zero vs DX-Infinity format based on magic bytes.

use crate::types::DxValue;
use crate::zero::types::{DxZeroError, Result as ZeroResult};
use crate::zero::MAGIC as DX_ZERO_MAGIC;

/// Binary format type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DxFormat {
    /// DX-Zero: Ultra-fast binary (0x5A 0x44)
    Zero,
    /// DX-Infinity: Human-optimized text (0x44 0x58)
    Infinity,
    /// Unknown format
    Unknown,
}

/// Detect format from magic bytes
#[inline]
pub fn detect_format(bytes: &[u8]) -> DxFormat {
    if bytes.len() < 2 {
        return DxFormat::Unknown;
    }

    match &bytes[0..2] {
        [0x5A, 0x44] => DxFormat::Zero,     // "ZD" little-endian
        [0x44, 0x58] => DxFormat::Infinity, // "DX" (hypothetical)
        _ => DxFormat::Unknown,
    }
}

/// Parse DX format (auto-detect)
///
/// This function automatically detects whether the input is DX-Zero binary
/// or DX-Infinity text format and parses accordingly.
pub fn parse_auto(bytes: &[u8]) -> Result<DxValue, String> {
    match detect_format(bytes) {
        DxFormat::Zero => {
            // Parse as DX-Zero binary
            Err("DX-Zero to DxValue conversion not yet implemented (use direct struct access)"
                .to_string())
        }
        DxFormat::Infinity | DxFormat::Unknown => {
            // Parse as DX-Infinity text (fallback)
            crate::parse(bytes).map_err(|e| format!("Parse error: {:?}", e))
        }
    }
}

/// Configuration for format selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatMode {
    /// Always use DX-Zero binary
    Zero,
    /// Always use DX-Infinity text
    Infinity,
    /// Auto-detect based on input
    Auto,
}

impl Default for FormatMode {
    fn default() -> Self {
        Self::Auto
    }
}

impl FormatMode {
    /// Parse from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "zero" | "binary" => Some(Self::Zero),
            "infinity" | "text" => Some(Self::Infinity),
            "auto" => Some(Self::Auto),
            _ => None,
        }
    }

    /// Get format name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Zero => "zero",
            Self::Infinity => "infinity",
            Self::Auto => "auto",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_zero_format() {
        let bytes = [0x5A, 0x44, 0x01, 0x04]; // DX-Zero header
        assert_eq!(detect_format(&bytes), DxFormat::Zero);
    }

    #[test]
    fn test_detect_infinity_format() {
        let bytes = [0x44, 0x58, b'_', b'I']; // Hypothetical DX-Infinity
        assert_eq!(detect_format(&bytes), DxFormat::Infinity);
    }

    #[test]
    fn test_detect_unknown() {
        let bytes = [0x00, 0x00, 0x00, 0x00];
        assert_eq!(detect_format(&bytes), DxFormat::Unknown);
    }

    #[test]
    fn test_detect_too_small() {
        let bytes = [0x5A];
        assert_eq!(detect_format(&bytes), DxFormat::Unknown);
    }

    #[test]
    fn test_format_mode_from_str() {
        assert_eq!(FormatMode::from_str("zero"), Some(FormatMode::Zero));
        assert_eq!(FormatMode::from_str("infinity"), Some(FormatMode::Infinity));
        assert_eq!(FormatMode::from_str("auto"), Some(FormatMode::Auto));
        assert_eq!(FormatMode::from_str("binary"), Some(FormatMode::Zero));
        assert_eq!(FormatMode::from_str("text"), Some(FormatMode::Infinity));
        assert_eq!(FormatMode::from_str("invalid"), None);
    }

    #[test]
    fn test_format_mode_name() {
        assert_eq!(FormatMode::Zero.name(), "zero");
        assert_eq!(FormatMode::Infinity.name(), "infinity");
        assert_eq!(FormatMode::Auto.name(), "auto");
    }
}
