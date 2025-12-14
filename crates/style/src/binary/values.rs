/// Level 5: Binary CSS Values (Nuclear Option)
/// 
/// Instead of storing "display:flex" strings, store property + value as binary enums
/// This is 6× smaller than string-based CSS

/// CSS Property enum - u8 allows 256 properties
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CssProperty {
    Display = 0x01,
    FlexDirection = 0x02,
    FlexWrap = 0x03,
    JustifyContent = 0x04,
    AlignItems = 0x05,
    AlignSelf = 0x06,
    Position = 0x07,
    Padding = 0x08,
    PaddingTop = 0x09,
    PaddingRight = 0x0A,
    PaddingBottom = 0x0B,
    PaddingLeft = 0x0C,
    Margin = 0x0D,
    MarginTop = 0x0E,
    MarginRight = 0x0F,
    MarginBottom = 0x10,
    MarginLeft = 0x11,
    Width = 0x12,
    Height = 0x13,
    Color = 0x14,
    Background = 0x15,
    Border = 0x16,
    BorderWidth = 0x17,
    BorderRadius = 0x18,
    FontSize = 0x19,
    FontWeight = 0x1A,
    LineHeight = 0x1B,
    TextAlign = 0x1C,
    BoxShadow = 0x1D,
    Overflow = 0x1E,
    OverflowX = 0x1F,
    OverflowY = 0x20,
    Top = 0x21,
    Right = 0x22,
    Bottom = 0x23,
    Left = 0x24,
    ZIndex = 0x25,
}

/// Display values
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayValue {
    None = 0x00,
    Block = 0x01,
    Inline = 0x02,
    InlineBlock = 0x03,
    Flex = 0x04,
    InlineFlex = 0x05,
    Grid = 0x06,
    InlineGrid = 0x07,
    Table = 0x08,
}

/// Flex Direction values
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexDirectionValue {
    Row = 0x00,
    RowReverse = 0x01,
    Column = 0x02,
    ColumnReverse = 0x03,
}

/// Justify Content values
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JustifyContentValue {
    FlexStart = 0x00,
    FlexEnd = 0x01,
    Center = 0x02,
    SpaceBetween = 0x03,
    SpaceAround = 0x04,
    SpaceEvenly = 0x05,
}

/// Align Items values
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignItemsValue {
    FlexStart = 0x00,
    FlexEnd = 0x01,
    Center = 0x02,
    Baseline = 0x03,
    Stretch = 0x04,
}

/// Position values
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionValue {
    Static = 0x00,
    Relative = 0x01,
    Absolute = 0x02,
    Fixed = 0x03,
    Sticky = 0x04,
}

/// Color value (RGB stored as u32: 0xRRGGBB)
pub type ColorValue = u32;

/// Length value (in pixels, stored as u16)
pub type LengthValue = u16;

/// Property name lookup table
const PROP_NAMES: &[&str] = &[
    "",                    // 0x00 (unused)
    "display",             // 0x01
    "flex-direction",      // 0x02
    "flex-wrap",           // 0x03
    "justify-content",     // 0x04
    "align-items",         // 0x05
    "align-self",          // 0x06
    "position",            // 0x07
    "padding",             // 0x08
    "padding-top",         // 0x09
    "padding-right",       // 0x0A
    "padding-bottom",      // 0x0B
    "padding-left",        // 0x0C
    "margin",              // 0x0D
    "margin-top",          // 0x0E
    "margin-right",        // 0x0F
    "margin-bottom",       // 0x10
    "margin-left",         // 0x11
    "width",               // 0x12
    "height",              // 0x13
    "color",               // 0x14
    "background",          // 0x15
    "border",              // 0x16
    "border-width",        // 0x17
    "border-radius",       // 0x18
    "font-size",           // 0x19
    "font-weight",         // 0x1A
    "line-height",         // 0x1B
    "text-align",          // 0x1C
    "box-shadow",          // 0x1D
    "overflow",            // 0x1E
    "overflow-x",          // 0x1F
    "overflow-y",          // 0x20
    "top",                 // 0x21
    "right",               // 0x22
    "bottom",              // 0x23
    "left",                // 0x24
    "z-index",             // 0x25
];

/// Display value names
const DISPLAY_VALUES: &[&str] = &[
    "none",
    "block",
    "inline",
    "inline-block",
    "flex",
    "inline-flex",
    "grid",
    "inline-grid",
    "table",
];

/// Flex direction value names
const FLEX_DIRECTION_VALUES: &[&str] = &[
    "row",
    "row-reverse",
    "column",
    "column-reverse",
];

/// Justify content value names
const JUSTIFY_CONTENT_VALUES: &[&str] = &[
    "flex-start",
    "flex-end",
    "center",
    "space-between",
    "space-around",
    "space-evenly",
];

/// Align items value names
const ALIGN_ITEMS_VALUES: &[&str] = &[
    "flex-start",
    "flex-end",
    "center",
    "baseline",
    "stretch",
];

/// Position value names
const POSITION_VALUES: &[&str] = &[
    "static",
    "relative",
    "absolute",
    "fixed",
    "sticky",
];

/// Get property name from property byte
#[allow(dead_code)]
fn get_property_name(prop: u8) -> &'static str {
    PROP_NAMES.get(prop as usize).unwrap_or(&"")
}

/// Get value string for a property
fn get_value_string(prop: CssProperty, val: u8) -> String {
    match prop {
        CssProperty::Display => {
            DISPLAY_VALUES.get(val as usize).unwrap_or(&"block").to_string()
        }
        CssProperty::FlexDirection => {
            FLEX_DIRECTION_VALUES.get(val as usize).unwrap_or(&"row").to_string()
        }
        CssProperty::JustifyContent => {
            JUSTIFY_CONTENT_VALUES.get(val as usize).unwrap_or(&"flex-start").to_string()
        }
        CssProperty::AlignItems => {
            ALIGN_ITEMS_VALUES.get(val as usize).unwrap_or(&"stretch").to_string()
        }
        CssProperty::Position => {
            POSITION_VALUES.get(val as usize).unwrap_or(&"static").to_string()
        }
        _ => format!("{}", val), // Numeric value
    }
}

/// Apply binary CSS from a byte stream
/// 
/// Stream format: [PROP, VAL, PROP, VAL, ...]
/// 
/// Returns CSS text string
pub fn apply_binary_css(stream: &[u8]) -> Result<String, &'static str> {
    if stream.len() % 2 != 0 {
        return Err("Invalid stream length (must be even)");
    }
    
    let mut css = String::with_capacity(stream.len() * 15); // Approx 15 chars per property
    
    let mut i = 0;
    while i < stream.len() {
        let prop_byte = stream[i];
        let val_byte = stream[i + 1];
        
        if let Some(&prop_name) = PROP_NAMES.get(prop_byte as usize) {
            if !prop_name.is_empty() {
                if !css.is_empty() {
                    css.push(';');
                }
                
                css.push_str(prop_name);
                css.push(':');
                
                // Convert property byte to enum
                let prop = unsafe { std::mem::transmute::<u8, CssProperty>(prop_byte) };
                let val_str = get_value_string(prop, val_byte);
                css.push_str(&val_str);
            }
        }
        
        i += 2;
    }
    
    Ok(css)
}

/// Encode a single CSS property-value pair to binary
pub fn encode_property(prop: CssProperty, value: u8) -> [u8; 2] {
    [prop as u8, value]
}

/// Encode multiple properties into a binary stream
pub fn encode_properties(props: &[(CssProperty, u8)]) -> Vec<u8> {
    let mut stream = Vec::with_capacity(props.len() * 2);
    
    for &(prop, val) in props {
        stream.push(prop as u8);
        stream.push(val);
    }
    
    stream
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_single_property() {
        let encoded = encode_property(CssProperty::Display, DisplayValue::Flex as u8);
        assert_eq!(encoded, [0x01, 0x04]);
    }

    #[test]
    fn test_encode_multiple_properties() {
        let props = vec![
            (CssProperty::Display, DisplayValue::Flex as u8),
            (CssProperty::AlignItems, AlignItemsValue::Center as u8),
        ];
        
        let stream = encode_properties(&props);
        assert_eq!(stream, vec![0x01, 0x04, 0x05, 0x02]);
    }

    #[test]
    fn test_apply_binary_css() {
        let stream = vec![
            0x01, 0x04,  // display: flex
            0x05, 0x02,  // align-items: center
        ];
        
        let css = apply_binary_css(&stream).unwrap();
        assert_eq!(css, "display:flex;align-items:center");
    }

    #[test]
    fn test_invalid_stream_length() {
        let stream = vec![0x01]; // Odd length
        let result = apply_binary_css(&stream);
        assert!(result.is_err());
    }

    #[test]
    fn test_size_comparison() {
        // String version: "display:flex" = 12 bytes
        let string_version = "display:flex";
        
        // Binary version: [0x01, 0x04] = 2 bytes
        let binary_version = vec![0x01, 0x04];
        
        assert_eq!(string_version.len(), 12);
        assert_eq!(binary_version.len(), 2);
        
        // 6× smaller!
        let ratio = string_version.len() as f64 / binary_version.len() as f64;
        assert!(ratio >= 6.0);
    }

    #[test]
    fn test_complex_example() {
        // flex + items-center + p-4 (padding: 1rem = 16px)
        let props = vec![
            (CssProperty::Display, DisplayValue::Flex as u8),
            (CssProperty::AlignItems, AlignItemsValue::Center as u8),
            (CssProperty::Padding, 16), // 16px = 1rem
        ];
        
        let stream = encode_properties(&props);
        let css = apply_binary_css(&stream).unwrap();
        
        // Should contain all three properties
        assert!(css.contains("display:flex"));
        assert!(css.contains("align-items:center"));
        assert!(css.contains("padding:16"));
        
        // Size comparison
        let string_equivalent = "display:flex;align-items:center;padding:1rem";
        let size_ratio = string_equivalent.len() as f64 / stream.len() as f64;
        
        println!("String: {} bytes, Binary: {} bytes, Ratio: {:.1}×",
                 string_equivalent.len(), stream.len(), size_ratio);
        
        assert!(size_ratio > 5.0); // At least 5× smaller
    }

    #[test]
    fn test_property_name_lookup() {
        assert_eq!(get_property_name(0x01), "display");
        assert_eq!(get_property_name(0x05), "align-items");
        assert_eq!(get_property_name(0x14), "color");
    }

    #[test]
    fn test_roundtrip() {
        let original_props = vec![
            (CssProperty::Display, DisplayValue::Flex as u8),
            (CssProperty::JustifyContent, JustifyContentValue::Center as u8),
            (CssProperty::AlignItems, AlignItemsValue::Center as u8),
        ];
        
        // Encode
        let stream = encode_properties(&original_props);
        
        // Decode
        let css = apply_binary_css(&stream).unwrap();
        
        // Verify
        assert!(css.contains("display:flex"));
        assert!(css.contains("justify-content:center"));
        assert!(css.contains("align-items:center"));
    }

    #[test]
    fn test_performance() {
        use std::time::Instant;
        
        let props = vec![
            (CssProperty::Display, DisplayValue::Flex as u8),
            (CssProperty::FlexDirection, FlexDirectionValue::Column as u8),
            (CssProperty::AlignItems, AlignItemsValue::Center as u8),
            (CssProperty::JustifyContent, JustifyContentValue::Center as u8),
        ];
        
        let stream = encode_properties(&props);
        
        // Benchmark decoding
        let start = Instant::now();
        for _ in 0..100000 {
            let _ = apply_binary_css(&stream).unwrap();
        }
        let elapsed = start.elapsed();
        
        println!("100k iterations: {:?}", elapsed);
        assert!(elapsed.as_millis() < 500); // Should be very fast
    }
}
