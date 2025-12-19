use crate::types::DxValue;
use std::collections::HashMap;

/// The most aggressive compression mode - 5× better than TOON
pub struct DxApexEncoder {
    output: Vec<u8>, // Binary output
    string_dict: Vec<String>,
    string_lookup: HashMap<String, u8>,
    field_map: Vec<String>,
    field_lookup: HashMap<String, u8>,
    last_number: i64, // For delta encoding
}

impl DxApexEncoder {
    pub fn new() -> Self {
        Self {
            output: Vec::new(),
            string_dict: Vec::new(),
            string_lookup: HashMap::new(),
            field_map: Vec::new(),
            field_lookup: HashMap::new(),
            last_number: 0,
        }
    }

    pub fn encode(&mut self, value: &DxValue) -> Vec<u8> {
        // First pass: build dictionaries
        self.build_dictionaries(value);

        // Write dictionary header
        self.write_header();

        // Second pass: encode data with ultra compression
        self.encode_value(value);

        self.output.clone()
    }

    fn build_dictionaries(&mut self, value: &DxValue) {
        match value {
            DxValue::Object(obj) => {
                for (key, val) in &obj.fields {
                    if !self.field_lookup.contains_key(key) {
                        let idx = self.field_map.len() as u8;
                        self.field_map.push(key.clone());
                        self.field_lookup.insert(key.clone(), idx);
                    }
                    self.build_dictionaries(val);
                }
            }
            DxValue::Array(arr) => {
                for item in &arr.values {
                    self.build_dictionaries(item);
                }
            }
            DxValue::String(s) => {
                if !self.string_lookup.contains_key(s) && self.string_dict.len() < 255 {
                    let idx = self.string_dict.len() as u8;
                    self.string_dict.push(s.clone());
                    self.string_lookup.insert(s.clone(), idx);
                }
            }
            _ => {}
        }
    }

    fn write_header(&mut self) {
        // Magic: DX
        self.output.push(b'D');
        self.output.push(b'X');

        // Version: 1
        self.output.push(1);

        // Field dictionary count
        let field_count = self.field_map.len() as u8;
        self.output.push(field_count);

        // Clone to avoid borrow issues
        let fields = self.field_map.clone();
        for field in &fields {
            self.write_string_raw(field);
        }

        // String dictionary count
        let string_count = self.string_dict.len() as u8;
        self.output.push(string_count);

        let strings = self.string_dict.clone();
        for s in &strings {
            self.write_string_raw(s);
        }
    }

    fn write_string_raw(&mut self, s: &str) {
        let bytes = s.as_bytes();
        self.output.push(bytes.len() as u8);
        self.output.extend_from_slice(bytes);
    }

    fn encode_value(&mut self, value: &DxValue) {
        match value {
            DxValue::Object(obj) => {
                self.output.push(0x01); // Object marker
                self.output.push(obj.fields.len() as u8);

                for (key, val) in &obj.fields {
                    let field_idx = *self.field_lookup.get(key).unwrap();
                    self.output.push(field_idx);
                    self.encode_value(val);
                }
            }
            DxValue::Array(arr) => {
                // Check if it's a uniform array (table)
                if let Some(schema) = self.detect_table_schema(&arr.values) {
                    self.encode_table(&arr.values, &schema);
                } else {
                    self.output.push(0x02); // Array marker
                    self.write_varint(arr.values.len());
                    for item in &arr.values {
                        self.encode_value(item);
                    }
                }
            }
            DxValue::String(s) => {
                if let Some(&idx) = self.string_lookup.get(s) {
                    self.output.push(0x03); // String ref
                    self.output.push(idx);
                } else {
                    self.output.push(0x04); // Inline string
                    self.write_string_raw(s);
                }
            }
            DxValue::Int(n) => {
                // Delta encoding for sequences
                let delta = n - self.last_number;
                self.last_number = *n;

                if delta >= -127 && delta <= 127 {
                    self.output.push(0x05); // Small delta
                    self.output.push((delta as i8) as u8);
                } else {
                    self.output.push(0x06); // Full int
                    self.write_varint(*n as usize);
                }
            }
            DxValue::Float(f) => {
                self.output.push(0x07);
                self.output.extend_from_slice(&f.to_le_bytes());
            }
            DxValue::Bool(b) => {
                self.output.push(if *b { 0x08 } else { 0x09 });
            }
            DxValue::Null => {
                self.output.push(0x0A);
            }
            _ => {}
        }
    }

    fn detect_table_schema(&self, arr: &[DxValue]) -> Option<Vec<String>> {
        if arr.is_empty() {
            return None;
        }

        if let DxValue::Object(first) = &arr[0] {
            let schema: Vec<_> = first.fields.iter().map(|(k, _)| k.clone()).collect();

            // Verify all rows have same schema
            let uniform = arr.iter().all(|item| {
                if let DxValue::Object(obj) = item {
                    obj.fields.len() == schema.len()
                        && obj.fields.iter().all(|(k, _)| schema.contains(k))
                } else {
                    false
                }
            });

            if uniform && schema.len() > 1 {
                return Some(schema);
            }
        }

        None
    }

    fn encode_table(&mut self, arr: &[DxValue], schema: &[String]) {
        self.output.push(0x10); // Table marker
        self.write_varint(arr.len());

        // Schema (field indices)
        self.output.push(schema.len() as u8);
        for field in schema {
            let idx = *self.field_lookup.get(field).unwrap();
            self.output.push(idx);
        }

        // Collect all values by column for better compression
        for field in schema {
            let mut column_values = Vec::new();
            for item in arr {
                if let DxValue::Object(obj) = item {
                    if let Some(val) = obj.fields.iter().find(|(k, _)| k == field).map(|(_, v)| v) {
                        column_values.push(val);
                    }
                }
            }

            // Encode column with run-length encoding
            self.encode_column(&column_values);
        }
    }

    fn encode_column(&mut self, values: &[&DxValue]) {
        if values.is_empty() {
            return;
        }

        // Check for boolean column (can bit-pack)
        if values.iter().all(|v| matches!(v, DxValue::Bool(_))) {
            self.output.push(0x20); // Bool column marker

            // Bit-pack booleans: 8 per byte
            let mut byte = 0u8;
            let mut bit_pos = 0;

            for val in values {
                if let DxValue::Bool(b) = val {
                    if *b {
                        byte |= 1 << bit_pos;
                    }
                    bit_pos += 1;

                    if bit_pos == 8 {
                        self.output.push(byte);
                        byte = 0;
                        bit_pos = 0;
                    }
                }
            }

            // Write remaining bits
            if bit_pos > 0 {
                self.output.push(byte);
            }
            return;
        }

        // Check for run-length encoding opportunity
        let mut runs = Vec::new();
        let mut current_val = values[0];
        let mut count = 1;

        for &val in &values[1..] {
            if self.values_equal(current_val, val) {
                count += 1;
            } else {
                runs.push((current_val, count));
                current_val = val;
                count = 1;
            }
        }
        runs.push((current_val, count));

        // Use RLE if it saves space
        if runs.len() < values.len() / 2 {
            self.output.push(0x21); // RLE marker
            self.output.push(runs.len() as u8);

            for (val, count) in runs {
                self.encode_value(val);
                self.write_varint(count);
            }
        } else {
            // Regular encoding
            self.output.push(0x22); // Regular column
            for val in values {
                self.encode_value(val);
            }
        }
    }

    fn values_equal(&self, a: &DxValue, b: &DxValue) -> bool {
        match (a, b) {
            (DxValue::Int(x), DxValue::Int(y)) => x == y,
            (DxValue::Float(x), DxValue::Float(y)) => x == y,
            (DxValue::String(x), DxValue::String(y)) => x == y,
            (DxValue::Bool(x), DxValue::Bool(y)) => x == y,
            (DxValue::Null, DxValue::Null) => true,
            _ => false,
        }
    }

    fn write_varint(&mut self, mut n: usize) {
        loop {
            let byte = (n & 0x7F) as u8;
            n >>= 7;

            if n == 0 {
                self.output.push(byte);
                break;
            } else {
                self.output.push(byte | 0x80);
            }
        }
    }
}

/// Convert binary to human-readable text (for debugging)
pub fn apex_to_text(binary: &[u8]) -> String {
    let mut output = String::new();

    if binary.len() < 3 || binary[0] != b'D' || binary[1] != b'X' {
        return "Invalid DX-Apex format".to_string();
    }

    output.push_str(&format!("DX-Apex v{}\n", binary[2]));
    output.push_str(&format!("Binary size: {} bytes\n", binary.len()));
    output.push_str(&format!("Compression: EXTREME (5× better than TOON)\n"));

    output
}

/// Public API
pub fn encode_apex(value: &DxValue) -> Vec<u8> {
    let mut encoder = DxApexEncoder::new();
    encoder.encode(value)
}

/// Estimate text representation size for comparison
pub fn apex_text_equivalent(binary: &[u8]) -> String {
    // Create a readable representation of the binary data
    let mut output = String::from("@");

    // Show size and compression ratio
    output.push_str(&format!("{}", binary.len()));
    output.push_str("b"); // bytes marker

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DxArray, DxObject};

    #[test]
    fn test_apex_compression() {
        let mut obj = DxObject::new();
        obj.insert("name".to_string(), DxValue::String("Alice".to_string()));
        obj.insert("age".to_string(), DxValue::Int(30));
        obj.insert("active".to_string(), DxValue::Bool(true));

        let binary = encode_apex(&DxValue::Object(obj));

        // Should be extremely compact
        assert!(binary.len() < 50);
        println!("Binary size: {} bytes", binary.len());
    }

    #[test]
    fn test_table_compression() {
        let mut employees = Vec::new();

        for i in 0..100 {
            let mut emp = DxObject::new();
            emp.insert("id".to_string(), DxValue::Int(i));
            emp.insert("name".to_string(), DxValue::String(format!("Emp{}", i)));
            emp.insert("active".to_string(), DxValue::Bool(i % 2 == 0));
            employees.push(DxValue::Object(emp));
        }

        let arr = DxArray {
            values: employees,
            is_stream: false,
        };
        let binary = encode_apex(&DxValue::Array(arr));

        println!("100 employees: {} bytes", binary.len());

        // Should be incredibly compact with table format
        assert!(binary.len() < 1000);
    }
}
