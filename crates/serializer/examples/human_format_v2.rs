//! Human Format V2 Example
//!
//! This example demonstrates the Human Format V2 features:
//! - Flat TOML-like structure without indentation
//! - Full key name expansion
//! - Full section names in brackets
//! - Comma-separated arrays without brackets
//! - Unicode box-drawing tables
//! - Pretty printer with validation
//! - Cache generation

use serializer::llm::{
    document_to_human_v2, document_to_llm, llm_to_human_v2,
    DxDocument, DxLlmValue, DxSection, HumanFormatV2Config, HumanFormatterV2,
    PrettyPrinter,
};

fn main() {
    println!("=== Human Format V2 Example ===\n");

    // Create a sample document
    let mut doc = DxDocument::new();

    // Add context (config) values
    doc.context.insert("nm".to_string(), DxLlmValue::Str("MyProject".to_string()));
    doc.context.insert("v".to_string(), DxLlmValue::Str("1.0.0".to_string()));
    doc.context.insert("ct".to_string(), DxLlmValue::Num(42.0));
    doc.context.insert("ac".to_string(), DxLlmValue::Bool(true));
    
    // Add array value (workspace paths)
    doc.context.insert(
        "ws".to_string(),
        DxLlmValue::Arr(vec![
            DxLlmValue::Str("frontend/www".to_string()),
            DxLlmValue::Str("frontend/mobile".to_string()),
            DxLlmValue::Str("backend/api".to_string()),
        ]),
    );

    // Add a data section (forge)
    let mut forge_section = DxSection::new(vec![
        "id".to_string(),
        "nm".to_string(),
        "st".to_string(),
        "ac".to_string(),
    ]);
    forge_section.rows.push(vec![
        DxLlmValue::Num(1.0),
        DxLlmValue::Str("Component A".to_string()),
        DxLlmValue::Str("active".to_string()),
        DxLlmValue::Bool(true),
    ]);
    forge_section.rows.push(vec![
        DxLlmValue::Num(2.0),
        DxLlmValue::Str("Component B".to_string()),
        DxLlmValue::Str("pending".to_string()),
        DxLlmValue::Bool(false),
    ]);
    forge_section.rows.push(vec![
        DxLlmValue::Num(3.0),
        DxLlmValue::Str("Component C".to_string()),
        DxLlmValue::Str("inactive".to_string()),
        DxLlmValue::Null,
    ]);
    doc.sections.insert('f', forge_section);

    // Add another data section (users)
    let mut users_section = DxSection::new(vec![
        "id".to_string(),
        "nm".to_string(),
        "role".to_string(),
    ]);
    users_section.rows.push(vec![
        DxLlmValue::Num(1.0),
        DxLlmValue::Str("Alice".to_string()),
        DxLlmValue::Str("admin".to_string()),
    ]);
    users_section.rows.push(vec![
        DxLlmValue::Num(2.0),
        DxLlmValue::Str("Bob".to_string()),
        DxLlmValue::Str("developer".to_string()),
    ]);
    doc.sections.insert('u', users_section);

    // Format to Human V2
    println!("--- Human Format V2 Output ---\n");
    let human_v2 = document_to_human_v2(&doc);
    println!("{}", human_v2);

    // Format to LLM format for comparison
    println!("\n--- LLM Format (Token-Optimized) ---\n");
    let llm = document_to_llm(&doc);
    println!("{}", llm);

    // Demonstrate round-trip conversion
    println!("\n--- Round-Trip Demonstration ---\n");
    let back_to_human = llm_to_human_v2(&llm).unwrap();
    println!("LLM → Human V2 conversion successful!");
    println!("Output matches original: {}", human_v2 == back_to_human);

    // Demonstrate PrettyPrinter with validation
    println!("\n--- PrettyPrinter with Validation ---\n");
    let printer = PrettyPrinter::new();
    match printer.format(&doc) {
        Ok(output) => {
            println!("PrettyPrinter output validated successfully!");
            println!("Output length: {} characters", output.len());
        }
        Err(e) => {
            println!("PrettyPrinter validation failed: {}", e);
        }
    }

    // Demonstrate custom config
    println!("\n--- Custom Config (No Key Expansion) ---\n");
    let config = HumanFormatV2Config {
        expand_keys: false,
        max_line_width: 80,
        show_summaries: false,
        show_references: false,
    };
    let formatter = HumanFormatterV2::with_config(config);
    let compact_output = formatter.format(&doc);
    println!("{}", compact_output);

    println!("\n=== Example Complete ===");
}
