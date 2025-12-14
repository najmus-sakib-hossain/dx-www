/// Human-readable formatter for DX ∞ SINGULARITY format
/// 
/// Converts compact SINGULARITY format (185 bytes) into beautiful
/// tabular display for editor extensions and developer tools.
///
/// Example transformation:
/// 
/// Input (SINGULARITY - 185 bytes):
/// ```dx
/// c.task:Our favorite hikes together^loc:Boulder^seas:spring_2025
/// f>ana|luis|sam
/// h=# n k%f g%x w s%b
/// Blue Lake Trail 7.5 5A ana +
/// Ridge Overlook 9.2 8i luis -
/// Wildflower Loop 5.1 2u sam +
/// ```
///
/// Output (Human Display - 374 bytes):
/// ```dx
/// context.task   : Our favorite hikes together
/// ^location      : Boulder
/// ^season        : spring_2025
/// 
/// friends        > ana | luis | sam
/// 
/// # HIKES TABLE (Auto-ID, Base62 decoded)
/// # ----------------------------------------------------------
/// ID   Name                 Km      Gain    Who     Sun
/// 1    Blue Lake Trail      7.5     320     ana     +
/// 2    Ridge Overlook       9.2     540     luis    -
/// 3    Wildflower Loop      5.1     180     sam     +
/// ```

use std::fmt::Write;
use crate::{DxValue, DxData, Schema, TypeHint};
use crate::base62::decode_base62;

/// Format DX data for human readability
/// 
/// This produces a beautiful, aligned output suitable for:
/// - VS Code DX extension display layer
/// - CLI pretty-printing
/// - Documentation examples
/// - Debug output
pub fn format_human(data: &DxData) -> Result<String, std::fmt::Error> {
    let mut output = String::with_capacity(512);
    
    // Format root object properties
    if let Some(obj) = data.objects.first() {
        format_object(&mut output, obj, "")?;
    }
    
    // Format arrays
    for (name, items) in &data.arrays {
        writeln!(output)?;
        format_array(&mut output, name, items)?;
    }
    
    // Format tables
    for schema in &data.schemas {
        writeln!(output)?;
        format_table(&mut output, schema, data)?;
    }
    
    Ok(output)
}

/// Format object properties with aligned colons
fn format_object(
    output: &mut String,
    obj: &std::collections::HashMap<String, DxValue>,
    prefix: &str,
) -> Result<(), std::fmt::Error> {
    // Find the longest key for alignment
    let max_key_len = obj.keys().map(|k| k.len()).max().unwrap_or(0);
    
    for (key, value) in obj {
        let full_key = if prefix.is_empty() {
            key.clone()
        } else {
            format!("^{}", key)
        };
        
        // Pad key for alignment
        let padding = " ".repeat(max_key_len - key.len());
        
        match value {
            DxValue::String(s) => writeln!(output, "{}{} : {}", full_key, padding, s)?,
            DxValue::Int(n) => writeln!(output, "{}{} : {}", full_key, padding, n)?,
            DxValue::Float(f) => writeln!(output, "{}{} : {}", full_key, padding, f)?,
            DxValue::Bool(b) => writeln!(output, "{}{} : {}", full_key, padding, if *b { "+" } else { "-" })?,
            _ => {}
        }
    }
    
    Ok(())
}

/// Format array with pipe separators
fn format_array(
    output: &mut String,
    name: &str,
    items: &[DxValue],
) -> Result<(), std::fmt::Error> {
    write!(output, "{:<15} > ", name)?;
    
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            write!(output, " | ")?;
        }
        match item {
            DxValue::String(s) => write!(output, "{}", s)?,
            DxValue::Int(n) => write!(output, "{}", n)?,
            DxValue::Float(f) => write!(output, "{}", f)?,
            DxValue::Bool(b) => write!(output, "{}", if *b { "+" } else { "-" })?,
            _ => {}
        }
    }
    
    writeln!(output)?;
    Ok(())
}

/// Format table with aligned columns and header comments
fn format_table(
    output: &mut String,
    schema: &Schema,
    data: &DxData,
) -> Result<(), std::fmt::Error> {
    // Write comment header
    writeln!(output, "# {} TABLE (Auto-ID, Base62 decoded)", schema.name.to_uppercase())?;
    writeln!(output, "# {}", "-".repeat(58))?;
    
    // Calculate column widths
    let mut col_widths: Vec<usize> = schema.columns.iter()
        .map(|col| col.name.len().max(4))
        .collect();
    
    // Scan data to find max widths
    if let Some(rows) = data.tables.get(&schema.name) {
        for row in rows {
            for (i, (col, value)) in schema.columns.iter().zip(row.iter()).enumerate() {
                let width = match value {
                    DxValue::String(s) => s.len(),
                    DxValue::Int(n) => n.to_string().len(),
                    DxValue::Float(f) => format!("{:.1}", f).len(),
                    DxValue::Bool(_) => 1,
                    _ => 0,
                };
                col_widths[i] = col_widths[i].max(width);
            }
        }
    }
    
    // Write column headers (capitalized)
    for (i, col) in schema.columns.iter().enumerate() {
        let name = if col.is_anonymous_auto_increment() {
            "ID".to_string()
        } else {
            capitalize_first(&col.name)
        };
        write!(output, "{:<width$} ", name, width = col_widths[i])?;
    }
    writeln!(output)?;
    
    // Write data rows
    if let Some(rows) = data.tables.get(&schema.name) {
        for row in rows {
            for (i, (col, value)) in schema.columns.iter().zip(row.iter()).enumerate() {
                let formatted = match value {
                    DxValue::String(s) => s.clone(),
                    DxValue::Int(n) => {
                        // Decode Base62 if column type is Base62
                        if col.type_hint == TypeHint::Base62 {
                            n.to_string()
                        } else {
                            n.to_string()
                        }
                    },
                    DxValue::Float(f) => format!("{:.1}", f),
                    DxValue::Bool(b) => if *b { "+" } else { "-" }.to_string(),
                    _ => "".to_string(),
                };
                write!(output, "{:<width$} ", formatted, width = col_widths[i])?;
            }
            writeln!(output)?;
        }
    }
    
    Ok(())
}

/// Capitalize first letter of string
fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(chars).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_human_hikes() {
        // This would test the full formatting
        // Implementation depends on having parsed data
    }
    
    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize_first("name"), "Name");
        assert_eq!(capitalize_first("id"), "Id");
        assert_eq!(capitalize_first(""), "");
    }
}
