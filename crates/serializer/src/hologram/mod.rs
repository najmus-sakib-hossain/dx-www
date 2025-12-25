//! # DX Hologram: Quantum Superposition Serialization
//!
//! The Holographic DX Architecture provides three simultaneous representations:
//!
//! 1. **Human Format** (Editor View) - Beautiful, readable, collapsible
//! 2. **LLM Format** (Disk Storage) - Token-efficient, minimal bytes
//! 3. **Machine Format** (Runtime) - Binary, 0.70ns access
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────────┐
//! │                    THE HOLOGRAPHIC DX SYSTEM                            │
//! ├─────────────────────────────────────────────────────────────────────────┤
//! │                                                                         │
//! │                           ┌─────────────────┐                           │
//! │                           │   EDITOR TAB    │                           │
//! │                           │  ┌───────────┐  │                           │
//! │                           │  │  HOLOGRAM │  │                           │
//! │                           │  │  (Pretty) │  │                           │
//! │                           │  └───────────┘  │                           │
//! │                           └────────┬────────┘                           │
//! │                                    │                                    │
//! │               inflate()            │           deflate()                │
//! │              ┌─────────────────────┼─────────────────────┐              │
//! │              ▼                     │                     ▼              │
//! │   ┌──────────────────┐             │          ┌──────────────────┐      │
//! │   │   READ (Open)    │             │          │  WRITE (Save)    │      │
//! │   └──────────────────┘             │          └──────────────────┘      │
//! │                                    │                                    │
//! │                           ┌────────▼────────┐                           │
//! │                           │   config/dx     │                           │
//! │                           │  (LLM-Dense)    │                           │
//! │                           └────────┬────────┘                           │
//! │                                    │ dx build                           │
//! │                                    ▼                                    │
//! │                           ┌─────────────────┐                           │
//! │                           │  config/dx.dxb  │                           │
//! │                           │   (Machine)     │                           │
//! │                           └─────────────────┘                           │
//! │                                                                         │
//! └─────────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Usage
//!
//! ```rust
//! use serializer::hologram::{Inflater, Deflater, HologramConfig};
//!
//! // LLM format on disk
//! let llm_dense = r#"server#host:localhost#port:5432#ssl:1"#;
//!
//! // Inflate to human-readable format (for editor display)
//! let config = HologramConfig::default();
//! let inflater = Inflater::new(config.clone());
//! let human_pretty = inflater.inflate(llm_dense);
//! // Result:
//! // ▼ server
//! //     host: localhost
//! //     port: 5432
//! //     ssl:  ✓
//!
//! // User edits in editor, then save...
//!
//! // Deflate back to LLM format (for disk storage)
//! let deflater = Deflater::new(config);
//! let llm_dense_again = deflater.deflate(&human_pretty);
//! // Result: server#host:localhost#port:5432#ssl:1
//! ```
//!
//! ## Key Features
//!
//! - **Comment Anchoring**: `!comment!` in LLM format preserved across round-trips
//! - **Symbol Expansion**: `1`→`✓`, `0`→`✗`, `~`→`—`
//! - **Section Folding**: `▼`/`▾` collapsible sections
//! - **Table Formatting**: Unicode box-drawing for aligned tables
//! - **Reference Arrows**: `*ref` → `→ref` for visual clarity

pub mod deflater;
pub mod inflater;
pub mod types;

#[cfg(feature = "wasm")]
pub mod wasm;

pub use deflater::Deflater;
pub use inflater::Inflater;
pub use types::{CommentAnchor, HologramConfig, HologramError, HologramResult};

/// Convenience function to inflate LLM-dense format to human-pretty format
pub fn inflate(dense: &str) -> String {
    Inflater::new(HologramConfig::default()).inflate(dense)
}

/// Convenience function to deflate human-pretty format to LLM-dense format
pub fn deflate(pretty: &str) -> String {
    Deflater::new(HologramConfig::default()).deflate(pretty)
}

/// Round-trip test: ensures deflate(inflate(x)) == x (modulo whitespace)
pub fn verify_round_trip(dense: &str) -> bool {
    let config = HologramConfig::default();
    let inflater = Inflater::new(config.clone());
    let deflater = Deflater::new(config);

    let pretty = inflater.inflate(dense);
    let back = deflater.deflate(&pretty);

    // Normalize for comparison
    normalize_dense(dense) == normalize_dense(&back)
}

/// Normalize dense format for comparison (removes insignificant whitespace)
fn normalize_dense(s: &str) -> String {
    s.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_inflate_deflate() {
        let dense = "server#host:localhost#port:5432";
        let pretty = inflate(dense);

        assert!(pretty.contains("▼ server"));
        assert!(pretty.contains("host:"));
        assert!(pretty.contains("localhost"));

        let back = deflate(&pretty);
        assert!(back.contains("server#"));
        assert!(back.contains("host:localhost"));
    }

    #[test]
    fn test_boolean_symbols() {
        let dense = "config#debug:1#prod:0";
        let pretty = inflate(dense);

        assert!(pretty.contains("✓") || pretty.contains("true"));
        assert!(pretty.contains("✗") || pretty.contains("false"));
    }

    #[test]
    fn test_array_inflate() {
        let dense = "items@3>apple|banana|cherry";
        let pretty = inflate(dense);

        assert!(pretty.contains("items"));
        assert!(pretty.contains("3 items") || pretty.contains("(3"));
        assert!(pretty.contains("apple"));
        assert!(pretty.contains("banana"));
        assert!(pretty.contains("cherry"));
    }

    #[test]
    fn test_comment_preservation() {
        let dense = "!Database config!db#host:localhost";
        let pretty = inflate(dense);
        let back = deflate(&pretty);

        assert!(back.contains("!Database config!") || back.contains("Database config"));
    }

    #[test]
    fn test_null_symbol() {
        let dense = "data#value:~";
        let pretty = inflate(dense);

        assert!(pretty.contains("—") || pretty.contains("null") || pretty.contains("none"));
    }
}
