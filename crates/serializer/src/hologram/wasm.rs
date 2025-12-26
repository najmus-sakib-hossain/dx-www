//! WASM Bindings for DX Hologram
//!
//! Provides JavaScript interop for the VS Code extension and browser usage.
//!
//! ## Usage from JavaScript
//!
//! ```javascript
//! import init, { inflate, deflate, json_to_dense } from 'dx_serializer';
//!
//! await init();
//!
//! // Inflate LLM-dense to human-pretty (for editor display)
//! const pretty = inflate('server#host:localhost#port:5432');
//! // ▼ server
//! //     host: localhost
//! //     port: 5432
//!
//! // Deflate human-pretty to LLM-dense (for disk storage)
//! const dense = deflate(pretty);
//! // server#host:localhost#port:5432
//!
//! // Convert JSON to LLM-dense
//! const json = '{"host": "localhost", "port": 5432}';
//! const dx = json_to_dense(json);
//! // host:localhost#port:5432
//! ```

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
use super::{Deflater, HologramConfig, Inflater};

/// Initialize the hologram WASM module (called by main init)
#[cfg(feature = "wasm")]
pub fn init_hologram() {
    // Hologram-specific initialization (if any)
    // Panic hook is set up by the main init_wasm() function
}

/// Inflate LLM-dense format to human-pretty format
///
/// This is called when opening a .dx file in the editor.
/// The dense format stored on disk is transformed to the beautiful
/// format shown to the user.
///
/// @param dense - The LLM-dense format string from disk
/// @returns The human-pretty format string for editor display
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn inflate(dense: &str) -> String {
    let inflater = Inflater::new(HologramConfig::default());
    inflater.inflate(dense)
}

/// Inflate with ASCII-only output (no Unicode symbols)
///
/// @param dense - The LLM-dense format string
/// @returns ASCII-only human-readable format
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn inflate_ascii(dense: &str) -> String {
    let inflater = Inflater::new(HologramConfig::ascii());
    inflater.inflate(dense)
}

/// Inflate with compact output
///
/// @param dense - The LLM-dense format string
/// @returns Compact human-readable format
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn inflate_compact(dense: &str) -> String {
    let inflater = Inflater::new(HologramConfig::compact());
    inflater.inflate(dense)
}

/// Deflate human-pretty format to LLM-dense format
///
/// This is called when saving a .dx file in the editor.
/// The beautiful format shown to the user is transformed back to
/// the token-efficient format stored on disk.
///
/// @param pretty - The human-pretty format string from editor
/// @returns The LLM-dense format string for disk storage
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn deflate(pretty: &str) -> String {
    let deflater = Deflater::new(HologramConfig::default());
    deflater.deflate(pretty)
}

/// Convert JSON to LLM-dense format
///
/// @param json - A JSON string
/// @returns The LLM-dense format string
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn json_to_dense(json: &str) -> Result<String, JsValue> {
    let deflater = Deflater::new(HologramConfig::default());
    deflater
        .json_to_dense(json)
        .map_err(|e| JsValue::from_str(&e))
}

/// Verify round-trip: checks that deflate(inflate(x)) preserves data
///
/// @param dense - The LLM-dense format string to test
/// @returns true if round-trip preserves data
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn verify_round_trip(dense: &str) -> bool {
    super::verify_round_trip(dense)
}

/// Configuration object for JavaScript
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub struct HologramConfigJs {
    inner: HologramConfig,
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl HologramConfigJs {
    /// Create default configuration
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: HologramConfig::default(),
        }
    }

    /// Create ASCII-only configuration
    #[wasm_bindgen(js_name = ascii)]
    pub fn ascii() -> Self {
        Self {
            inner: HologramConfig::ascii(),
        }
    }

    /// Create compact configuration
    #[wasm_bindgen(js_name = compact)]
    pub fn compact() -> Self {
        Self {
            inner: HologramConfig::compact(),
        }
    }

    /// Set indent size
    #[wasm_bindgen(js_name = setIndentSize)]
    pub fn set_indent_size(&mut self, size: usize) {
        self.inner.indent_size = size;
    }

    /// Set whether to use Unicode symbols
    #[wasm_bindgen(js_name = setUseUnicodeSymbols)]
    pub fn set_use_unicode_symbols(&mut self, use_unicode: bool) {
        self.inner.use_unicode_symbols = use_unicode;
    }

    /// Set whether to use box drawing for tables
    #[wasm_bindgen(js_name = setUseBoxDrawing)]
    pub fn set_use_box_drawing(&mut self, use_box: bool) {
        self.inner.use_box_drawing = use_box;
    }

    /// Set whether to preserve comments
    #[wasm_bindgen(js_name = setPreserveComments)]
    pub fn set_preserve_comments(&mut self, preserve: bool) {
        self.inner.preserve_comments = preserve;
    }

    /// Set whether to align values
    #[wasm_bindgen(js_name = setAlignValues)]
    pub fn set_align_values(&mut self, align: bool) {
        self.inner.align_values = align;
    }
}

/// Inflater for JavaScript with custom configuration
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub struct InflaterJs {
    inner: Inflater,
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl InflaterJs {
    /// Create an inflater with default configuration
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: Inflater::new(HologramConfig::default()),
        }
    }

    /// Create an inflater with custom configuration
    #[wasm_bindgen(js_name = withConfig)]
    pub fn with_config(config: &HologramConfigJs) -> Self {
        Self {
            inner: Inflater::new(config.inner.clone()),
        }
    }

    /// Inflate LLM-dense to human-pretty
    #[wasm_bindgen]
    pub fn inflate(&self, dense: &str) -> String {
        self.inner.inflate(dense)
    }
}

/// Deflater for JavaScript with custom configuration
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub struct DeflaterJs {
    inner: Deflater,
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl DeflaterJs {
    /// Create a deflater with default configuration
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: Deflater::new(HologramConfig::default()),
        }
    }

    /// Create a deflater with custom configuration
    #[wasm_bindgen(js_name = withConfig)]
    pub fn with_config(config: &HologramConfigJs) -> Self {
        Self {
            inner: Deflater::new(config.inner.clone()),
        }
    }

    /// Deflate human-pretty to LLM-dense
    #[wasm_bindgen]
    pub fn deflate(&self, pretty: &str) -> String {
        self.inner.deflate(pretty)
    }

    /// Convert JSON to LLM-dense
    #[wasm_bindgen(js_name = jsonToDense)]
    pub fn json_to_dense(&self, json: &str) -> Result<String, JsValue> {
        self.inner
            .json_to_dense(json)
            .map_err(|e| JsValue::from_str(&e))
    }
}

/// Get version information
#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "hologramVersion")]
pub fn hologram_version() -> String {
    format!(
        "dx-serializer-hologram v{} ({})",
        env!("CARGO_PKG_VERSION"),
        if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        }
    )
}

#[cfg(all(test, feature = "wasm"))]
mod tests {
    use super::*;

    #[test]
    fn test_wasm_inflate() {
        let dense = "server#host:localhost#port:5432";
        let pretty = inflate(dense);
        assert!(pretty.contains("▼ server"));
    }

    #[test]
    fn test_wasm_deflate() {
        let pretty = "▼ server\n    host: localhost\n    port: 5432";
        let dense = deflate(pretty);
        assert!(dense.contains("server#host:localhost"));
    }

    #[test]
    fn test_wasm_round_trip() {
        let original = "config#debug:1#prod:0";
        assert!(verify_round_trip(original));
    }
}
