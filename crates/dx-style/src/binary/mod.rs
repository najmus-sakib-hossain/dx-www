pub mod analyzer;
pub mod api;
pub mod combos;
pub mod csstext;
pub mod hybrid;
/// Binary Style System
///
/// This module implements the 5-level optimization strategy from STYLE.md:
///
/// Level 1: Binary IDs → classList.add()
/// Level 2: Direct cssText Injection (Skip classList)
/// Level 3: Pre-Computed Style Combinations
/// Level 4: Varint Encoding
/// Level 5: Binary CSS Values (Nuclear Option)
pub mod ids;
pub mod values;
pub mod varint;

pub use analyzer::{StyleAnalyzer, StylePattern};
pub use api::{EncodingMode, decode_and_generate, encode_for_transmission, generate_css_optimized};
pub use combos::{COMBO_DICT, ComboId, get_combo_csstext, is_common_combo, try_apply_combo};
pub use csstext::apply_styles_direct;
pub use hybrid::{
    GROUPING_THRESHOLD, MacroId, StyleOpcode, analyze_for_macros, decode_from_wire, decode_hybrid,
    encode_for_wire, encode_hybrid, get_macro_csstext, should_use_macro,
};
pub use ids::{STYLE_DICT, StyleId, style_id_to_csstext, style_name_to_id};
pub use values::{
    AlignItemsValue, ColorValue, CssProperty, DisplayValue, apply_binary_css, encode_properties,
};
pub use varint::{decode_id_list, decode_varint, encode_id_list, encode_varint};
