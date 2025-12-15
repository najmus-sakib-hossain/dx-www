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
pub mod csstext;
pub mod combos;
pub mod varint;
pub mod values;
pub mod api;
pub mod analyzer;
pub mod hybrid;

pub use ids::{StyleId, STYLE_DICT, style_name_to_id, style_id_to_csstext};
pub use csstext::apply_styles_direct;
pub use combos::{ComboId, COMBO_DICT, is_common_combo, get_combo_csstext, try_apply_combo};
pub use varint::{encode_varint, decode_varint, encode_id_list, decode_id_list};
pub use values::{CssProperty, DisplayValue, AlignItemsValue, ColorValue, apply_binary_css, encode_properties};
pub use api::{EncodingMode, generate_css_optimized, encode_for_transmission, decode_and_generate};
pub use analyzer::{StyleAnalyzer, StylePattern};
pub use hybrid::{MacroId, StyleOpcode, GROUPING_THRESHOLD, should_use_macro, get_macro_csstext, 
                 encode_hybrid, decode_hybrid, encode_for_wire, decode_from_wire, analyze_for_macros};
