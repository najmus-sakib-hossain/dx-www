//! Format conversion functions
//!
//! Provides conversion between LLM, Human, and Machine formats.
//! All conversions go through the common DxDocument representation.

use crate::llm::human_formatter::{HumanFormatConfig, HumanFormatter};
use crate::llm::human_formatter_v2::{HumanFormatV2Config, HumanFormatterV2};
use crate::llm::human_parser::{HumanParseError, HumanParser};
use crate::llm::parser::{LlmParser, ParseError};
use crate::llm::serializer::LlmSerializer;
use crate::llm::types::DxDocument;
use thiserror::Error;

/// Conversion errors
#[derive(Debug, Error)]
pub enum ConvertError {
    #[error("LLM parse error: {0}")]
    LlmParse(#[from] ParseError),

    #[error("Human parse error: {0}")]
    HumanParse(#[from] HumanParseError),

    #[error("Machine format error: {msg}")]
    MachineFormat { msg: String },
}

/// Convert LLM format string to Human format string
///
/// # Example
/// ```
/// use serializer::llm::convert::llm_to_human;
///
/// let llm = "#c:nm|Test;ct|42\n#d(id|vl)\n1|Alpha";
/// let human = llm_to_human(llm).unwrap();
/// assert!(human.contains("name"));
/// assert!(human.contains("Test"));
/// ```
pub fn llm_to_human(llm_input: &str) -> Result<String, ConvertError> {
    let doc = LlmParser::parse(llm_input)?;
    let formatter = HumanFormatter::new();
    Ok(formatter.format(&doc))
}

/// Convert LLM format string to Human format string with custom config
pub fn llm_to_human_with_config(
    llm_input: &str,
    config: HumanFormatConfig,
) -> Result<String, ConvertError> {
    let doc = LlmParser::parse(llm_input)?;
    let formatter = HumanFormatter::with_config(config);
    Ok(formatter.format(&doc))
}


/// Convert Human format string to LLM format string
///
/// # Example
/// ```
/// use serializer::llm::convert::human_to_llm;
///
/// let human = r#"
/// [config]
///     name = "Test"
///     count = 42
/// "#;
/// let llm = human_to_llm(human).unwrap();
/// assert!(llm.contains("#c:"));
/// ```
pub fn human_to_llm(human_input: &str) -> Result<String, ConvertError> {
    let parser = HumanParser::new();
    let doc = parser.parse(human_input)?;
    let serializer = LlmSerializer::new();
    Ok(serializer.serialize(&doc))
}

/// Convert LLM format string to DxDocument
pub fn llm_to_document(llm_input: &str) -> Result<DxDocument, ConvertError> {
    Ok(LlmParser::parse(llm_input)?)
}

/// Convert Human format string to DxDocument
pub fn human_to_document(human_input: &str) -> Result<DxDocument, ConvertError> {
    let parser = HumanParser::new();
    Ok(parser.parse(human_input)?)
}

/// Convert DxDocument to LLM format string
pub fn document_to_llm(doc: &DxDocument) -> String {
    let serializer = LlmSerializer::new();
    serializer.serialize(doc)
}

/// Convert DxDocument to Human format string
pub fn document_to_human(doc: &DxDocument) -> String {
    let formatter = HumanFormatter::new();
    formatter.format(doc)
}

/// Convert DxDocument to Human format string with custom config
pub fn document_to_human_with_config(doc: &DxDocument, config: HumanFormatConfig) -> String {
    let formatter = HumanFormatter::with_config(config);
    formatter.format(doc)
}

// ============================================================================
// Human Format V2 Conversion Functions
// ============================================================================

/// Convert Human format V2 string to LLM format string
///
/// Human V2 format features:
/// - Flat TOML-like structure without indentation
/// - Full key names (version, workspace, etc.)
/// - Full section names ([forge] instead of [f])
/// - Comma-separated arrays without brackets
///
/// # Example
/// ```
/// use serializer::llm::convert::human_to_llm_v2;
///
/// let human_v2 = r#"
/// [config]
/// name = "Test"
/// workspace = frontend/www, frontend/mobile
/// "#;
/// let llm = human_to_llm_v2(human_v2).unwrap();
/// assert!(llm.contains("#c:"));
/// ```
pub fn human_to_llm_v2(human_input: &str) -> Result<String, ConvertError> {
    // The parser already handles V2 format (full key names, full section names)
    let parser = HumanParser::new();
    let doc = parser.parse(human_input)?;
    let serializer = LlmSerializer::new();
    Ok(serializer.serialize(&doc))
}

/// Convert LLM format string to Human format V2 string
///
/// # Example
/// ```
/// use serializer::llm::convert::llm_to_human_v2;
///
/// let llm = "#c:nm|Test;ct|42\n#d(id|vl)\n1|Alpha";
/// let human_v2 = llm_to_human_v2(llm).unwrap();
/// assert!(human_v2.contains("name")); // Expanded key
/// assert!(human_v2.contains("[data]")); // Full section name
/// ```
pub fn llm_to_human_v2(llm_input: &str) -> Result<String, ConvertError> {
    let doc = LlmParser::parse(llm_input)?;
    let formatter = HumanFormatterV2::new();
    Ok(formatter.format(&doc))
}

/// Convert LLM format string to Human format V2 string with custom config
pub fn llm_to_human_v2_with_config(
    llm_input: &str,
    config: HumanFormatV2Config,
) -> Result<String, ConvertError> {
    let doc = LlmParser::parse(llm_input)?;
    let formatter = HumanFormatterV2::with_config(config);
    Ok(formatter.format(&doc))
}

/// Convert DxDocument to Human format V2 string
pub fn document_to_human_v2(doc: &DxDocument) -> String {
    let formatter = HumanFormatterV2::new();
    formatter.format(doc)
}

/// Convert DxDocument to Human format V2 string with custom config
pub fn document_to_human_v2_with_config(doc: &DxDocument, config: HumanFormatV2Config) -> String {
    let formatter = HumanFormatterV2::with_config(config);
    formatter.format(doc)
}

/// Convert Human format V2 string to DxDocument
pub fn human_v2_to_document(human_input: &str) -> Result<DxDocument, ConvertError> {
    // Parser handles both V1 and V2 formats
    let parser = HumanParser::new();
    Ok(parser.parse(human_input)?)
}

/// Convert Human format V2 to Machine format (binary)
pub fn human_v2_to_machine(human_input: &str) -> Result<MachineFormat, ConvertError> {
    let parser = HumanParser::new();
    let doc = parser.parse(human_input)?;
    Ok(document_to_machine(&doc))
}

/// Convert Machine format to Human format V2 string
pub fn machine_to_human_v2(machine: &MachineFormat) -> Result<String, ConvertError> {
    let doc = machine_to_document(machine)?;
    Ok(document_to_human_v2(&doc))
}

/// Machine format representation (binary)
/// This is a placeholder for the actual binary format implementation
/// which would integrate with the existing zero-copy serializer.
#[derive(Debug, Clone)]
pub struct MachineFormat {
    /// Raw binary data
    pub data: Vec<u8>,
}

/// Convert LLM format to Machine format (binary)
///
/// This creates a simple binary representation of the document.
/// For production use, this would integrate with the zero-copy serializer.
pub fn llm_to_machine(llm_input: &str) -> Result<MachineFormat, ConvertError> {
    let doc = LlmParser::parse(llm_input)?;
    Ok(document_to_machine(&doc))
}

/// Convert Human format to Machine format (binary)
pub fn human_to_machine(human_input: &str) -> Result<MachineFormat, ConvertError> {
    let parser = HumanParser::new();
    let doc = parser.parse(human_input)?;
    Ok(document_to_machine(&doc))
}

/// Convert DxDocument to Machine format (binary)
///
/// This creates a simple binary representation using bincode-style encoding.
/// For production use, this would integrate with the zero-copy serializer.
pub fn document_to_machine(doc: &DxDocument) -> MachineFormat {
    let mut data = Vec::new();
    
    // Magic number for format identification
    data.extend_from_slice(b"DXMF"); // DX Machine Format
    
    // Version
    data.push(1);
    
    // Context section
    let context_count = doc.context.len() as u32;
    data.extend_from_slice(&context_count.to_le_bytes());
    for (key, value) in &doc.context {
        write_string(&mut data, key);
        write_value(&mut data, value);
    }
    
    // References section
    let refs_count = doc.refs.len() as u32;
    data.extend_from_slice(&refs_count.to_le_bytes());
    for (key, value) in &doc.refs {
        write_string(&mut data, key);
        write_string(&mut data, value);
    }
    
    // Sections
    let sections_count = doc.sections.len() as u32;
    data.extend_from_slice(&sections_count.to_le_bytes());
    for (id, section) in &doc.sections {
        data.push(*id as u8);
        
        // Schema
        let schema_count = section.schema.len() as u32;
        data.extend_from_slice(&schema_count.to_le_bytes());
        for col in &section.schema {
            write_string(&mut data, col);
        }
        
        // Rows
        let rows_count = section.rows.len() as u32;
        data.extend_from_slice(&rows_count.to_le_bytes());
        for row in &section.rows {
            for value in row {
                write_value(&mut data, value);
            }
        }
    }
    
    MachineFormat { data }
}


/// Convert Machine format to DxDocument
pub fn machine_to_document(machine: &MachineFormat) -> Result<DxDocument, ConvertError> {
    use crate::llm::types::DxSection;
    
    let data = &machine.data;
    let mut pos = 0;
    
    // Check magic number
    if data.len() < 5 || &data[0..4] != b"DXMF" {
        return Err(ConvertError::MachineFormat {
            msg: "Invalid magic number".to_string(),
        });
    }
    pos += 4;
    
    // Check version
    let version = data[pos];
    if version != 1 {
        return Err(ConvertError::MachineFormat {
            msg: format!("Unsupported version: {}", version),
        });
    }
    pos += 1;
    
    let mut doc = DxDocument::new();
    
    // Read context
    let context_count = read_u32(data, &mut pos)?;
    for _ in 0..context_count {
        let key = read_string(data, &mut pos)?;
        let value = read_value(data, &mut pos)?;
        doc.context.insert(key, value);
    }
    
    // Read references
    let refs_count = read_u32(data, &mut pos)?;
    for _ in 0..refs_count {
        let key = read_string(data, &mut pos)?;
        let value = read_string(data, &mut pos)?;
        doc.refs.insert(key, value);
    }
    
    // Read sections
    let sections_count = read_u32(data, &mut pos)?;
    for _ in 0..sections_count {
        if pos >= data.len() {
            return Err(ConvertError::MachineFormat {
                msg: "Unexpected end of data".to_string(),
            });
        }
        let id = data[pos] as char;
        pos += 1;
        
        // Read schema
        let schema_count = read_u32(data, &mut pos)?;
        let mut schema = Vec::new();
        for _ in 0..schema_count {
            schema.push(read_string(data, &mut pos)?);
        }
        
        let mut section = DxSection::new(schema.clone());
        
        // Read rows
        let rows_count = read_u32(data, &mut pos)?;
        for _ in 0..rows_count {
            let mut row = Vec::new();
            for _ in 0..schema.len() {
                row.push(read_value(data, &mut pos)?);
            }
            section.rows.push(row);
        }
        
        doc.sections.insert(id, section);
    }
    
    Ok(doc)
}

/// Convert Machine format to LLM format string
pub fn machine_to_llm(machine: &MachineFormat) -> Result<String, ConvertError> {
    let doc = machine_to_document(machine)?;
    Ok(document_to_llm(&doc))
}

/// Convert Machine format to Human format string
pub fn machine_to_human(machine: &MachineFormat) -> Result<String, ConvertError> {
    let doc = machine_to_document(machine)?;
    Ok(document_to_human(&doc))
}

// Helper functions for binary encoding/decoding

fn write_string(data: &mut Vec<u8>, s: &str) {
    let bytes = s.as_bytes();
    let len = bytes.len() as u32;
    data.extend_from_slice(&len.to_le_bytes());
    data.extend_from_slice(bytes);
}

fn write_value(data: &mut Vec<u8>, value: &crate::llm::types::DxLlmValue) {
    use crate::llm::types::DxLlmValue;
    
    match value {
        DxLlmValue::Str(s) => {
            data.push(0); // Type tag
            write_string(data, s);
        }
        DxLlmValue::Num(n) => {
            data.push(1);
            data.extend_from_slice(&n.to_le_bytes());
        }
        DxLlmValue::Bool(b) => {
            data.push(2);
            data.push(if *b { 1 } else { 0 });
        }
        DxLlmValue::Null => {
            data.push(3);
        }
        DxLlmValue::Arr(items) => {
            data.push(4);
            let len = items.len() as u32;
            data.extend_from_slice(&len.to_le_bytes());
            for item in items {
                write_value(data, item);
            }
        }
        DxLlmValue::Ref(key) => {
            data.push(5);
            write_string(data, key);
        }
    }
}

fn read_u32(data: &[u8], pos: &mut usize) -> Result<u32, ConvertError> {
    if *pos + 4 > data.len() {
        return Err(ConvertError::MachineFormat {
            msg: "Unexpected end of data reading u32".to_string(),
        });
    }
    let bytes: [u8; 4] = data[*pos..*pos + 4].try_into().unwrap();
    *pos += 4;
    Ok(u32::from_le_bytes(bytes))
}

fn read_string(data: &[u8], pos: &mut usize) -> Result<String, ConvertError> {
    let len = read_u32(data, pos)? as usize;
    if *pos + len > data.len() {
        return Err(ConvertError::MachineFormat {
            msg: "Unexpected end of data reading string".to_string(),
        });
    }
    let s = String::from_utf8(data[*pos..*pos + len].to_vec()).map_err(|_| {
        ConvertError::MachineFormat {
            msg: "Invalid UTF-8 string".to_string(),
        }
    })?;
    *pos += len;
    Ok(s)
}


fn read_value(data: &[u8], pos: &mut usize) -> Result<crate::llm::types::DxLlmValue, ConvertError> {
    use crate::llm::types::DxLlmValue;
    
    if *pos >= data.len() {
        return Err(ConvertError::MachineFormat {
            msg: "Unexpected end of data reading value".to_string(),
        });
    }
    
    let type_tag = data[*pos];
    *pos += 1;
    
    match type_tag {
        0 => {
            // String
            let s = read_string(data, pos)?;
            Ok(DxLlmValue::Str(s))
        }
        1 => {
            // Number
            if *pos + 8 > data.len() {
                return Err(ConvertError::MachineFormat {
                    msg: "Unexpected end of data reading number".to_string(),
                });
            }
            let bytes: [u8; 8] = data[*pos..*pos + 8].try_into().unwrap();
            *pos += 8;
            Ok(DxLlmValue::Num(f64::from_le_bytes(bytes)))
        }
        2 => {
            // Bool
            if *pos >= data.len() {
                return Err(ConvertError::MachineFormat {
                    msg: "Unexpected end of data reading bool".to_string(),
                });
            }
            let b = data[*pos] != 0;
            *pos += 1;
            Ok(DxLlmValue::Bool(b))
        }
        3 => {
            // Null
            Ok(DxLlmValue::Null)
        }
        4 => {
            // Array
            let len = read_u32(data, pos)? as usize;
            let mut items = Vec::with_capacity(len);
            for _ in 0..len {
                items.push(read_value(data, pos)?);
            }
            Ok(DxLlmValue::Arr(items))
        }
        5 => {
            // Ref
            let key = read_string(data, pos)?;
            Ok(DxLlmValue::Ref(key))
        }
        _ => Err(ConvertError::MachineFormat {
            msg: format!("Unknown type tag: {}", type_tag),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::types::{DxLlmValue, DxSection};

    #[test]
    fn test_llm_to_human() {
        let llm = "#c:nm|Test;ct|42";
        let human = llm_to_human(llm).unwrap();
        
        assert!(human.contains("name")); // nm expanded to name
        assert!(human.contains("count")); // ct expanded to count
        assert!(human.contains("Test"));
        assert!(human.contains("42"));
    }

    #[test]
    fn test_human_to_llm() {
        let human = r#"
[config]
    name = "Test"
    count = 42
"#;
        let llm = human_to_llm(human).unwrap();
        
        assert!(llm.contains("#c:"));
        assert!(llm.contains("nm|Test")); // name compressed to nm
        assert!(llm.contains("ct|42")); // count compressed to ct
    }

    #[test]
    fn test_llm_human_round_trip() {
        let original_llm = "#c:nm|Test;ct|42\n#d(id|vl)\n1|Alpha\n2|Beta";
        
        // LLM -> Human -> LLM
        let human = llm_to_human(original_llm).unwrap();
        let back_to_llm = human_to_llm(&human).unwrap();
        
        // Parse both and compare
        let original_doc = llm_to_document(original_llm).unwrap();
        let round_trip_doc = llm_to_document(&back_to_llm).unwrap();
        
        assert_eq!(original_doc.context.len(), round_trip_doc.context.len());
        assert_eq!(original_doc.sections.len(), round_trip_doc.sections.len());
    }

    #[test]
    fn test_machine_format_round_trip() {
        let mut doc = DxDocument::new();
        doc.context.insert("nm".to_string(), DxLlmValue::Str("Test".to_string()));
        doc.context.insert("ct".to_string(), DxLlmValue::Num(42.0));
        doc.context.insert("ac".to_string(), DxLlmValue::Bool(true));
        doc.context.insert("empty".to_string(), DxLlmValue::Null);
        
        let mut section = DxSection::new(vec!["id".to_string(), "vl".to_string()]);
        section.rows.push(vec![DxLlmValue::Num(1.0), DxLlmValue::Str("Alpha".to_string())]);
        section.rows.push(vec![DxLlmValue::Num(2.0), DxLlmValue::Str("Beta".to_string())]);
        doc.sections.insert('d', section);
        
        // Document -> Machine -> Document
        let machine = document_to_machine(&doc);
        let round_trip_doc = machine_to_document(&machine).unwrap();
        
        assert_eq!(doc.context.len(), round_trip_doc.context.len());
        assert_eq!(doc.sections.len(), round_trip_doc.sections.len());
        
        // Check values
        assert_eq!(
            round_trip_doc.context.get("nm").unwrap().as_str(),
            Some("Test")
        );
        assert_eq!(
            round_trip_doc.context.get("ct").unwrap().as_num(),
            Some(42.0)
        );
        assert_eq!(
            round_trip_doc.context.get("ac").unwrap().as_bool(),
            Some(true)
        );
        assert!(round_trip_doc.context.get("empty").unwrap().is_null());
    }

    #[test]
    fn test_llm_to_machine_to_llm() {
        let original_llm = "#c:nm|Test;ct|42\n#d(id|vl)\n1|Alpha";
        
        // LLM -> Machine -> LLM
        let machine = llm_to_machine(original_llm).unwrap();
        let back_to_llm = machine_to_llm(&machine).unwrap();
        
        // Parse both and compare
        let original_doc = llm_to_document(original_llm).unwrap();
        let round_trip_doc = llm_to_document(&back_to_llm).unwrap();
        
        assert_eq!(original_doc.context.len(), round_trip_doc.context.len());
        assert_eq!(original_doc.sections.len(), round_trip_doc.sections.len());
    }

    #[test]
    fn test_human_to_machine_to_human() {
        let original_human = r#"
[config]
    name = "Test"
    count = 42

[data]
    ┌─────┬───────┐
    │ id  │ value │
    ├─────┼───────┤
    │ 1   │ Alpha │
    └─────┴───────┘
"#;
        
        // Human -> Machine -> Human
        let machine = human_to_machine(original_human).unwrap();
        let back_to_human = machine_to_human(&machine).unwrap();
        
        // Parse both and compare
        let original_doc = human_to_document(original_human).unwrap();
        let round_trip_doc = human_to_document(&back_to_human).unwrap();
        
        assert_eq!(original_doc.context.len(), round_trip_doc.context.len());
        assert_eq!(original_doc.sections.len(), round_trip_doc.sections.len());
    }

    #[test]
    fn test_special_values_conversion() {
        let llm = "#c:flag|+;empty|~;off|-";
        
        // LLM -> Human
        let human = llm_to_human(llm).unwrap();
        assert!(human.contains("true")); // + becomes true in config
        assert!(human.contains("null")); // ~ becomes null
        assert!(human.contains("false")); // - becomes false
        
        // Human -> LLM
        let back_to_llm = human_to_llm(&human).unwrap();
        assert!(back_to_llm.contains("|+")); // true becomes +
        assert!(back_to_llm.contains("|~")); // null becomes ~
        assert!(back_to_llm.contains("|-")); // false becomes -
    }

    // ========================================================================
    // Human Format V2 Tests
    // ========================================================================

    #[test]
    fn test_llm_to_human_v2() {
        let llm = "#c:nm|Test;ct|42";
        let human_v2 = llm_to_human_v2(llm).unwrap();
        
        // V2 should have expanded keys
        assert!(human_v2.contains("name")); // nm expanded to name
        assert!(human_v2.contains("count")); // ct expanded to count
        assert!(human_v2.contains("Test"));
        assert!(human_v2.contains("42"));
        
        // V2 should have [config] section
        assert!(human_v2.contains("[config]"));
    }

    #[test]
    fn test_human_to_llm_v2() {
        let human_v2 = r#"
[config]
name = "Test"
count = 42
"#;
        let llm = human_to_llm_v2(human_v2).unwrap();
        
        assert!(llm.contains("#c:"));
        // Keys should be compressed in LLM format
        assert!(llm.contains("nm|Test") || llm.contains("name|Test")); 
        assert!(llm.contains("ct|42") || llm.contains("count|42"));
    }

    #[test]
    fn test_llm_human_v2_round_trip() {
        let original_llm = "#c:nm|Test;ct|42\n#d(id|vl)\n1|Alpha\n2|Beta";
        
        // LLM -> Human V2 -> LLM
        let human_v2 = llm_to_human_v2(original_llm).unwrap();
        let back_to_llm = human_to_llm_v2(&human_v2).unwrap();
        
        // Parse both and compare
        let original_doc = llm_to_document(original_llm).unwrap();
        let round_trip_doc = llm_to_document(&back_to_llm).unwrap();
        
        assert_eq!(original_doc.context.len(), round_trip_doc.context.len());
        assert_eq!(original_doc.sections.len(), round_trip_doc.sections.len());
    }

    #[test]
    fn test_human_v2_full_section_names() {
        let llm = "#f(id|nm)\n1|Test";
        let human_v2 = llm_to_human_v2(llm).unwrap();
        
        // V2 should use full section name [forge] not [f]
        assert!(human_v2.contains("[forge]"));
    }

    #[test]
    fn test_human_v2_array_format() {
        let mut doc = DxDocument::new();
        doc.context.insert(
            "ws".to_string(),
            DxLlmValue::Arr(vec![
                DxLlmValue::Str("frontend/www".to_string()),
                DxLlmValue::Str("frontend/mobile".to_string()),
            ]),
        );
        
        let human_v2 = document_to_human_v2(&doc);
        
        // V2 arrays should be comma-separated without brackets
        assert!(human_v2.contains("frontend/www, frontend/mobile"));
        // Should NOT have brackets around array
        assert!(!human_v2.contains("[frontend/www"));
    }

    #[test]
    fn test_human_v2_no_indentation() {
        let llm = "#c:nm|Test;ct|42";
        let human_v2 = llm_to_human_v2(llm).unwrap();
        
        // V2 should have no indentation for key-value pairs
        for line in human_v2.lines() {
            if line.contains(" = ") && !line.starts_with('#') && !line.starts_with('[') {
                assert!(!line.starts_with(' '), "Line should not be indented: {}", line);
            }
        }
    }

    #[test]
    fn test_human_v2_to_machine_round_trip() {
        let human_v2 = r#"
[config]
name = "Test"
count = 42

[data]
┌─────┬───────┐
│ id  │ value │
├─────┼───────┤
│ 1   │ Alpha │
└─────┴───────┘
"#;
        
        // Human V2 -> Machine -> Human V2
        let machine = human_v2_to_machine(human_v2).unwrap();
        let back_to_human_v2 = machine_to_human_v2(&machine).unwrap();
        
        // Parse both and compare
        let original_doc = human_v2_to_document(human_v2).unwrap();
        let round_trip_doc = human_v2_to_document(&back_to_human_v2).unwrap();
        
        assert_eq!(original_doc.context.len(), round_trip_doc.context.len());
        assert_eq!(original_doc.sections.len(), round_trip_doc.sections.len());
    }

    #[test]
    fn test_document_to_human_v2_with_config() {
        let mut doc = DxDocument::new();
        doc.context.insert("nm".to_string(), DxLlmValue::Str("Test".to_string()));
        
        // Test with expand_keys = false
        let config = HumanFormatV2Config {
            expand_keys: false,
            ..Default::default()
        };
        let human_v2 = document_to_human_v2_with_config(&doc, config);
        
        // With expand_keys = false, should keep abbreviated key
        assert!(human_v2.contains("nm"));
    }
}
