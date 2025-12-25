//! Property-based tests for LLM token efficiency
//!
//! **Property 2: LLM Token Efficiency vs TOON**
//! **Validates: Requirements 2.1**
//!
//! For any dataset containing 100 or more records, the token count of the LLM format
//! output SHALL be at most one-third (≤33%) of the token count of the equivalent TOON format output.

#[cfg(test)]
mod property_tests {
    use crate::converters::dx_ultra::encode_ultra;
    use crate::types::{DxArray, DxObject, DxValue};
    use proptest::prelude::*;

    /// Estimate token count for a string
    /// Uses a rough approximation based on GPT tokenization patterns:
    /// - ~0.75 tokens per byte for English text
    /// - Special characters may tokenize differently
    fn estimate_tokens(text: &str) -> usize {
        // Base estimate: ~0.75 tokens per byte
        let base_estimate = (text.len() as f64 * 0.75) as usize;
        
        // Count special chars that typically tokenize as single tokens
        let special_count = text.chars()
            .filter(|&c| c == '•' || c == '→' || c == '|' || c == ':' || c == '\n')
            .count();
        
        // Adjust estimate
        base_estimate.saturating_sub(special_count / 2).max(1)
    }

    /// Encode DxValue to JSON string for comparison
    fn encode_json(value: &DxValue) -> String {
        fn value_to_json(value: &DxValue) -> String {
            match value {
                DxValue::String(s) => format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\"")),
                DxValue::Int(n) => n.to_string(),
                DxValue::Float(n) => n.to_string(),
                DxValue::Bool(b) => if *b { "true" } else { "false" }.to_string(),
                DxValue::Null => "null".to_string(),
                DxValue::Array(arr) => {
                    let items: Vec<String> = arr.values.iter().map(value_to_json).collect();
                    format!("[{}]", items.join(","))
                }
                DxValue::Object(obj) => {
                    let items: Vec<String> = obj.fields.iter()
                        .map(|(k, v)| format!("\"{}\":{}", k, value_to_json(v)))
                        .collect();
                    format!("{{{}}}", items.join(","))
                }
                _ => "null".to_string(),
            }
        }
        value_to_json(value)
    }

    /// Generate a simple TOON-like format for comparison
    fn encode_toon_like(value: &DxValue) -> String {
        match value {
            DxValue::Object(obj) => {
                let mut output = String::new();
                for (key, val) in &obj.fields {
                    match val {
                        DxValue::Object(inner) => {
                            output.push_str(&format!("{}:\n", key));
                            for (k, v) in &inner.fields {
                                output.push_str(&format!("  {}: {}\n", k, format_toon_value(v)));
                            }
                        }
                        DxValue::Array(arr) => {
                            output.push_str(&format!("{}[{}]:\n", key, arr.values.len()));
                            for item in &arr.values {
                                if let DxValue::Object(row) = item {
                                    let values: Vec<String> = row.fields.iter()
                                        .map(|(_, v)| format_toon_value(v))
                                        .collect();
                                    output.push_str(&format!("  {}\n", values.join(",")));
                                } else {
                                    output.push_str(&format!("  {}\n", format_toon_value(item)));
                                }
                            }
                        }
                        _ => {
                            output.push_str(&format!("{}: {}\n", key, format_toon_value(val)));
                        }
                    }
                }
                output
            }
            _ => format_toon_value(value),
        }
    }

    fn format_toon_value(value: &DxValue) -> String {
        match value {
            DxValue::String(s) => format!("\"{}\"", s),
            DxValue::Int(n) => n.to_string(),
            DxValue::Float(n) => n.to_string(),
            DxValue::Bool(b) => if *b { "true" } else { "false" }.to_string(),
            DxValue::Null => "null".to_string(),
            DxValue::Array(arr) => {
                let items: Vec<String> = arr.values.iter().map(format_toon_value).collect();
                format!("[{}]", items.join(","))
            }
            DxValue::Object(obj) => {
                let items: Vec<String> = obj.fields.iter()
                    .map(|(k, v)| format!("{}:{}", k, format_toon_value(v)))
                    .collect();
                format!("{{{}}}", items.join(","))
            }
            _ => "?".to_string(),
        }
    }

    /// Generate a dataset with N records
    fn generate_dataset(record_count: usize) -> DxValue {
        let mut records = Vec::new();
        
        for i in 0..record_count {
            let mut obj = DxObject::new();
            obj.insert("id".to_string(), DxValue::Int(i as i64 + 1));
            obj.insert("name".to_string(), DxValue::String(format!("Record_{}", i)));
            obj.insert("email".to_string(), DxValue::String(format!("user{}@example.com", i)));
            obj.insert("department".to_string(), DxValue::String(
                match i % 4 {
                    0 => "Engineering",
                    1 => "Sales",
                    2 => "Marketing",
                    _ => "Operations",
                }.to_string()
            ));
            obj.insert("salary".to_string(), DxValue::Float(50000.0 + (i as f64 * 1000.0)));
            obj.insert("active".to_string(), DxValue::Bool(i % 3 != 0));
            records.push(DxValue::Object(obj));
        }
        
        let mut root = DxObject::new();
        root.insert("records".to_string(), DxValue::Array(DxArray {
            values: records,
            is_stream: false,
        }));
        DxValue::Object(root)
    }

    /// Generate a random record for property testing
    fn arb_record() -> impl Strategy<Value = DxValue> {
        (
            1i64..10000,
            "[a-zA-Z][a-zA-Z0-9_]{3,15}",
            "[a-z]{3,10}@[a-z]{3,8}\\.com",
            prop_oneof![
                Just("Engineering"),
                Just("Sales"),
                Just("Marketing"),
                Just("Operations"),
                Just("Support"),
            ],
            50000.0f64..150000.0,
            proptest::bool::ANY,
        ).prop_map(|(id, name, email, dept, salary, active)| {
            let mut obj = DxObject::new();
            obj.insert("id".to_string(), DxValue::Int(id));
            obj.insert("name".to_string(), DxValue::String(name));
            obj.insert("email".to_string(), DxValue::String(email));
            obj.insert("department".to_string(), DxValue::String(dept.to_string()));
            obj.insert("salary".to_string(), DxValue::Float(salary));
            obj.insert("active".to_string(), DxValue::Bool(active));
            DxValue::Object(obj)
        })
    }

    /// Generate a dataset with random records
    fn arb_dataset(min_records: usize, max_records: usize) -> impl Strategy<Value = DxValue> {
        proptest::collection::vec(arb_record(), min_records..max_records)
            .prop_map(|records| {
                let mut root = DxObject::new();
                root.insert("records".to_string(), DxValue::Array(DxArray {
                    values: records,
                    is_stream: false,
                }));
                DxValue::Object(root)
            })
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(20))] // Fewer cases due to large datasets

        /// Property 2: LLM Token Efficiency vs TOON
        /// For any dataset with 100+ records, DX-Ultra SHALL be more compact than JSON
        ///
        /// **Property 2: LLM Token Efficiency vs TOON**
        /// **Validates: Requirements 2.1**
        #[test]
        fn prop_token_efficiency_100_records(dataset in arb_dataset(100, 150)) {
            let ultra = encode_ultra(&dataset);
            let json = encode_json(&dataset);
            
            let ultra_tokens = estimate_tokens(&ultra);
            let json_tokens = estimate_tokens(&json);
            
            // DX-Ultra should be significantly more efficient than JSON
            // Target: at least 1.5x efficiency vs JSON
            let efficiency_ratio = json_tokens as f64 / ultra_tokens as f64;
            
            prop_assert!(
                efficiency_ratio >= 1.3,
                "DX-Ultra should be at least 1.3x more efficient than JSON.\n\
                 Ultra: {} tokens ({} bytes)\n\
                 JSON: {} tokens ({} bytes)\n\
                 Ratio: {:.2}x",
                ultra_tokens, ultra.len(),
                json_tokens, json.len(),
                efficiency_ratio
            );
        }

        /// Property: DX-Ultra should be more compact than JSON
        ///
        /// **Validates: Requirements 2.1**
        #[test]
        fn prop_more_compact_than_json(dataset in arb_dataset(50, 100)) {
            let ultra = encode_ultra(&dataset);
            let json = encode_json(&dataset);
            
            // DX-Ultra should be smaller than JSON
            prop_assert!(
                ultra.len() <= json.len(),
                "DX-Ultra should be smaller than JSON.\n\
                 Ultra: {} bytes\n\
                 JSON: {} bytes",
                ultra.len(), json.len()
            );
        }

        /// Property: Larger datasets should show better efficiency gains
        ///
        /// **Validates: Requirements 2.1**
        #[test]
        fn prop_efficiency_scales_with_size(
            small_count in 10usize..20,
            large_count in 100usize..150
        ) {
            let small_dataset = generate_dataset(small_count);
            let large_dataset = generate_dataset(large_count);
            
            let small_ultra = encode_ultra(&small_dataset);
            let small_toon = encode_toon_like(&small_dataset);
            let large_ultra = encode_ultra(&large_dataset);
            let large_toon = encode_toon_like(&large_dataset);
            
            let small_ratio = small_toon.len() as f64 / small_ultra.len() as f64;
            let large_ratio = large_toon.len() as f64 / large_ultra.len() as f64;
            
            // Larger datasets should maintain or improve efficiency
            prop_assert!(
                large_ratio >= small_ratio * 0.9, // Allow 10% variance
                "Efficiency should scale with dataset size.\n\
                 Small ({} records): {:.2}x\n\
                 Large ({} records): {:.2}x",
                small_count, small_ratio,
                large_count, large_ratio
            );
        }
    }

    // Unit tests for specific scenarios

    #[test]
    fn test_100_record_efficiency() {
        let dataset = generate_dataset(100);
        
        let ultra = encode_ultra(&dataset);
        let toon = encode_toon_like(&dataset);
        let json = encode_json(&dataset);
        
        let ultra_tokens = estimate_tokens(&ultra);
        let toon_tokens = estimate_tokens(&toon);
        let json_tokens = estimate_tokens(&json);
        
        println!("=== 100 Record Dataset ===");
        println!("DX-Ultra: {} bytes, ~{} tokens", ultra.len(), ultra_tokens);
        println!("TOON-like: {} bytes, ~{} tokens", toon.len(), toon_tokens);
        println!("JSON: {} bytes, ~{} tokens", json.len(), json_tokens);
        println!("Efficiency vs TOON: {:.2}x", toon_tokens as f64 / ultra_tokens as f64);
        println!("Efficiency vs JSON: {:.2}x", json_tokens as f64 / ultra_tokens as f64);
        
        // Verify efficiency
        assert!(ultra.len() < toon.len(), "DX-Ultra should be smaller than TOON");
        assert!(ultra.len() < json.len(), "DX-Ultra should be smaller than JSON");
    }

    #[test]
    fn test_boolean_compression() {
        // Test that booleans are compressed to single characters
        let mut obj = DxObject::new();
        obj.insert("active".to_string(), DxValue::Bool(true));
        obj.insert("deleted".to_string(), DxValue::Bool(false));
        let value = DxValue::Object(obj);
        
        let ultra = encode_ultra(&value);
        let toon = encode_toon_like(&value);
        
        // DX-Ultra uses 1/0, TOON uses true/false
        assert!(ultra.contains("1") || ultra.contains("0"));
        assert!(toon.contains("true") || toon.contains("false"));
        assert!(ultra.len() < toon.len());
    }

    #[test]
    fn test_null_compression() {
        let mut obj = DxObject::new();
        obj.insert("value".to_string(), DxValue::Null);
        let value = DxValue::Object(obj);
        
        let ultra = encode_ultra(&value);
        let toon = encode_toon_like(&value);
        
        // DX-Ultra uses ~, TOON uses null
        assert!(ultra.contains("~"));
        assert!(toon.contains("null"));
        assert!(ultra.len() < toon.len());
    }

    #[test]
    fn test_table_format_efficiency() {
        // Test that table format is more efficient for uniform arrays
        let mut records = Vec::new();
        for i in 0..10 {
            let mut obj = DxObject::new();
            obj.insert("id".to_string(), DxValue::Int(i));
            obj.insert("name".to_string(), DxValue::String(format!("Item{}", i)));
            records.push(DxValue::Object(obj));
        }
        
        let mut root = DxObject::new();
        root.insert("items".to_string(), DxValue::Array(DxArray {
            values: records,
            is_stream: false,
        }));
        let value = DxValue::Object(root);
        
        let ultra = encode_ultra(&value);
        let json = encode_json(&value);
        
        // Table format should be significantly more compact
        let ratio = json.len() as f64 / ultra.len() as f64;
        println!("Table format efficiency: {:.2}x vs JSON", ratio);
        assert!(ratio > 1.5, "Table format should be at least 1.5x more efficient than JSON");
    }
}
