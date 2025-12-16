/// Converters from other formats to DX SINGULARITY format
///
/// Supports: JSON, YAML, TOON, TOML → DX ULTRA
///
/// All converters apply ultra-optimization automatically:
/// - Abbreviated keys (name → n, version → v)
/// - Minimal prefixes (context → c, media → m)
/// - Inline chaining with ^
/// - Compact arrays with |
/// - 2-letter language codes
pub mod json;
pub mod toml;
pub mod toon;
pub mod yaml;
pub mod dx_ultra;

pub use json::json_to_dx;
pub use toml::toml_to_dx;
pub use toon::toon_to_dx;
pub use yaml::yaml_to_dx;

use crate::optimizer;
use std::io::Write;

/// Common converter trait
pub trait ToDx {
    fn to_dx(&self) -> Result<String, String>;
}

/// Convert any supported format to DX ULTRA
pub fn convert_to_dx(input: &str, format: &str) -> Result<String, String> {
    match format.to_lowercase().as_str() {
        "json" => json_to_dx(input),
        "yaml" | "yml" => yaml_to_dx(input),
        "toon" => toon_to_dx(input),
        "toml" => toml_to_dx(input),
        _ => Err(format!("Unsupported format: {}", format)),
    }
}
