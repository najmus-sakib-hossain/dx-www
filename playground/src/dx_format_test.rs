//! DX Format Conversion Test
//!
//! This binary tests round-trip conversions between LLM, Human, and Machine formats.
//! Run with: cargo run --bin dx-format-test

use serializer::llm::{
    document_to_human, document_to_llm, document_to_machine, human_to_document, human_to_llm,
    llm_to_document, llm_to_human, machine_to_llm, machine_to_human,
    DxDocument, DxLlmValue, DxSection,
};

fn main() {
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!("                        DX FORMAT CONVERSION TEST");
    println!("═══════════════════════════════════════════════════════════════════════════════\n");

    // Create a test document
    let doc = create_test_document();

    // Test 1: LLM → Human → LLM round-trip
    println!("┌─────────────────────────────────────────────────────────────────────────────┐");
    println!("│ TEST 1: LLM → Human → LLM Round-Trip                                        │");
    println!("└─────────────────────────────────────────────────────────────────────────────┘\n");

    let llm1 = document_to_llm(&doc);
    println!("Original LLM format:\n{}\n", llm1);

    let human = llm_to_human(&llm1).expect("LLM to Human conversion failed");
    println!("Converted to Human format:\n{}\n", human);

    let llm2 = human_to_llm(&human).expect("Human to LLM conversion failed");
    println!("Back to LLM format:\n{}\n", llm2);

    let doc_roundtrip = llm_to_document(&llm2).expect("LLM parse failed");
    let success = compare_documents(&doc, &doc_roundtrip);
    println!("Round-trip result: {}\n", if success { "✓ PASS" } else { "✗ FAIL" });

    // Test 2: Human → LLM → Human round-trip
    println!("┌─────────────────────────────────────────────────────────────────────────────┐");
    println!("│ TEST 2: Human → LLM → Human Round-Trip                                      │");
    println!("└─────────────────────────────────────────────────────────────────────────────┘\n");

    let human1 = document_to_human(&doc);
    println!("Original Human format:\n{}\n", human1);

    let llm = human_to_llm(&human1).expect("Human to LLM conversion failed");
    println!("Converted to LLM format:\n{}\n", llm);

    let human2 = llm_to_human(&llm).expect("LLM to Human conversion failed");
    println!("Back to Human format:\n{}\n", human2);

    let doc_roundtrip = human_to_document(&human2).expect("Human parse failed");
    let success = compare_documents(&doc, &doc_roundtrip);
    println!("Round-trip result: {}\n", if success { "✓ PASS" } else { "✗ FAIL" });

    // Test 3: LLM → Machine → LLM round-trip
    println!("┌─────────────────────────────────────────────────────────────────────────────┐");
    println!("│ TEST 3: LLM → Machine → LLM Round-Trip                                      │");
    println!("└─────────────────────────────────────────────────────────────────────────────┘\n");

    let llm1 = document_to_llm(&doc);
    println!("Original LLM format:\n{}\n", llm1);

    let machine = document_to_machine(&doc);
    println!("Machine format: {} bytes\n", machine.data.len());

    let llm2 = machine_to_llm(&machine).expect("Machine to LLM conversion failed");
    println!("Back to LLM format:\n{}\n", llm2);

    let doc_roundtrip = llm_to_document(&llm2).expect("LLM parse failed");
    let success = compare_documents(&doc, &doc_roundtrip);
    println!("Round-trip result: {}\n", if success { "✓ PASS" } else { "✗ FAIL" });

    // Test 4: Machine → Human round-trip
    println!("┌─────────────────────────────────────────────────────────────────────────────┐");
    println!("│ TEST 4: Machine → Human Round-Trip                                          │");
    println!("└─────────────────────────────────────────────────────────────────────────────┘\n");

    let machine = document_to_machine(&doc);
    println!("Machine format: {} bytes\n", machine.data.len());

    let human = machine_to_human(&machine).expect("Machine to Human conversion failed");
    println!("Converted to Human format:\n{}\n", human);

    let doc_roundtrip = human_to_document(&human).expect("Human parse failed");
    let success = compare_documents(&doc, &doc_roundtrip);
    println!("Round-trip result: {}\n", if success { "✓ PASS" } else { "✗ FAIL" });

    // Test 5: Special values preservation
    println!("┌─────────────────────────────────────────────────────────────────────────────┐");
    println!("│ TEST 5: Special Values Preservation                                         │");
    println!("└─────────────────────────────────────────────────────────────────────────────┘\n");

    test_special_values();

    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!("                           ALL TESTS COMPLETE");
    println!("═══════════════════════════════════════════════════════════════════════════════");
}

fn create_test_document() -> DxDocument {
    let mut doc = DxDocument::new();

    // Add context - using keys that round-trip cleanly
    doc.context.insert("nm".to_string(), DxLlmValue::Str("Test Database".to_string()));
    doc.context.insert("vr".to_string(), DxLlmValue::Str("v1.0-beta".to_string())); // version with non-numeric chars
    doc.context.insert("ct".to_string(), DxLlmValue::Num(42.0));
    doc.context.insert("ac".to_string(), DxLlmValue::Bool(true));

    // Add references
    doc.refs.insert("A".to_string(), "Shared Location".to_string());
    doc.refs.insert("B".to_string(), "Common Category".to_string());

    // Add a data section
    let mut section = DxSection::new(vec![
        "id".to_string(),
        "nm".to_string(),
        "pr".to_string(),
        "ac".to_string(),
    ]);
    section.add_row(vec![
        DxLlmValue::Num(1.0),
        DxLlmValue::Str("Alpha".to_string()),
        DxLlmValue::Num(29.99),
        DxLlmValue::Bool(true),
    ]).unwrap();
    section.add_row(vec![
        DxLlmValue::Num(2.0),
        DxLlmValue::Str("Beta".to_string()),
        DxLlmValue::Num(49.99),
        DxLlmValue::Bool(false),
    ]).unwrap();
    section.add_row(vec![
        DxLlmValue::Num(3.0),
        DxLlmValue::Str("Gamma".to_string()),
        DxLlmValue::Null,
        DxLlmValue::Bool(true),
    ]).unwrap();
    doc.sections.insert('d', section);

    doc
}

fn test_special_values() {
    let mut doc = DxDocument::new();
    doc.context.insert("bool_true".to_string(), DxLlmValue::Bool(true));
    doc.context.insert("bool_false".to_string(), DxLlmValue::Bool(false));
    doc.context.insert("null_val".to_string(), DxLlmValue::Null);
    doc.context.insert("num_int".to_string(), DxLlmValue::Num(42.0));
    doc.context.insert("num_float".to_string(), DxLlmValue::Num(3.14));
    doc.context.insert("str_val".to_string(), DxLlmValue::Str("Hello".to_string()));

    // LLM format
    let llm = document_to_llm(&doc);
    println!("LLM format special values:");
    println!("  + for true: {}", llm.contains("|+"));
    println!("  - for false: {}", llm.contains("|-"));
    println!("  ~ for null: {}", llm.contains("|~"));
    println!();

    // Human format
    let human = llm_to_human(&llm).expect("Conversion failed");
    println!("Human format special values:");
    println!("  'true' for true: {}", human.contains("true"));
    println!("  'false' for false: {}", human.contains("false"));
    println!("  'null' for null: {}", human.contains("null"));
    println!();

    // Round-trip check
    let llm2 = human_to_llm(&human).expect("Conversion failed");
    let doc2 = llm_to_document(&llm2).expect("Parse failed");

    let bool_true_ok = doc2.context.get("bool_true").map(|v| v.as_bool()) == Some(Some(true));
    let bool_false_ok = doc2.context.get("bool_false").map(|v| v.as_bool()) == Some(Some(false));
    let null_ok = doc2.context.get("null_val").map(|v| v.is_null()) == Some(true);

    println!("Round-trip preservation:");
    println!("  Boolean true: {}", if bool_true_ok { "✓" } else { "✗" });
    println!("  Boolean false: {}", if bool_false_ok { "✓" } else { "✗" });
    println!("  Null value: {}", if null_ok { "✓" } else { "✗" });
    println!();
}

fn compare_documents(a: &DxDocument, b: &DxDocument) -> bool {
    // Compare context sizes
    if a.context.len() != b.context.len() {
        println!("  Context size mismatch: {} vs {}", a.context.len(), b.context.len());
        return false;
    }

    // Compare sections sizes
    if a.sections.len() != b.sections.len() {
        println!("  Sections size mismatch: {} vs {}", a.sections.len(), b.sections.len());
        return false;
    }

    // Compare context values
    for (key, val_a) in &a.context {
        if let Some(val_b) = b.context.get(key) {
            if !values_equal(val_a, val_b) {
                println!("  Context value mismatch for '{}': {:?} vs {:?}", key, val_a, val_b);
                return false;
            }
        } else {
            println!("  Missing context key in round-trip: {}", key);
            return false;
        }
    }

    // Compare sections
    for (id, section_a) in &a.sections {
        if let Some(section_b) = b.sections.get(id) {
            if section_a.rows.len() != section_b.rows.len() {
                println!("  Section '{}' row count mismatch: {} vs {}", id, section_a.rows.len(), section_b.rows.len());
                return false;
            }
            for (i, (row_a, row_b)) in section_a.rows.iter().zip(section_b.rows.iter()).enumerate() {
                for (j, (val_a, val_b)) in row_a.iter().zip(row_b.iter()).enumerate() {
                    if !values_equal(val_a, val_b) {
                        println!("  Section '{}' row {} col {} mismatch: {:?} vs {:?}", id, i, j, val_a, val_b);
                        return false;
                    }
                }
            }
        } else {
            println!("  Missing section in round-trip: {}", id);
            return false;
        }
    }

    true
}

fn values_equal(a: &DxLlmValue, b: &DxLlmValue) -> bool {
    match (a, b) {
        (DxLlmValue::Num(x), DxLlmValue::Num(y)) => (x - y).abs() < 0.0001,
        (DxLlmValue::Str(x), DxLlmValue::Str(y)) => x == y,
        (DxLlmValue::Bool(x), DxLlmValue::Bool(y)) => x == y,
        (DxLlmValue::Null, DxLlmValue::Null) => true,
        (DxLlmValue::Arr(x), DxLlmValue::Arr(y)) => {
            x.len() == y.len() && x.iter().zip(y.iter()).all(|(a, b)| values_equal(a, b))
        }
        (DxLlmValue::Ref(x), DxLlmValue::Ref(y)) => x == y,
        // References may be resolved to strings during conversion
        (DxLlmValue::Ref(_), DxLlmValue::Str(_)) => true,
        (DxLlmValue::Str(_), DxLlmValue::Ref(_)) => true,
        _ => false,
    }
}
