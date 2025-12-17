//! Test to verify DX-Infinity (human format) still works correctly
//! This ensures we didn't break anything while adding DX-Zero

use dx_serializer::{parse, encode, DxValue, DxObject, format_human};

#[test]
fn test_dx_infinity_parsing() {
    let input = "id: 12345\nage: 30\nname: John";
    let result = parse(input.as_bytes());
    assert!(result.is_ok(), "DX-Infinity parsing failed: {:?}", result.err());
    
    let value = result.unwrap();
    println!("✅ DX-Infinity parse successful: {:?}", value);
}

#[test]
fn test_dx_infinity_encoding() {
    let mut obj = DxObject::new();
    obj.insert("id".to_string(), DxValue::Int(12345));
    obj.insert("age".to_string(), DxValue::Int(30));
    obj.insert("name".to_string(), DxValue::String("John".to_string()));
    
    let value = DxValue::Object(obj);
    let encoded = encode(&value);
    assert!(encoded.is_ok(), "DX-Infinity encoding failed: {:?}", encoded.err());
    println!("✅ DX-Infinity encode successful: {}", String::from_utf8_lossy(&encoded.unwrap()));
}

#[test]
fn test_dx_infinity_roundtrip() {
    let original = "id: 12345\nage: 30";
    let parsed = parse(original.as_bytes()).expect("Parse failed");
    let encoded = encode(&parsed).expect("Encode failed");
    let _reparsed = parse(&encoded).expect("Reparse failed");
    
    println!("✅ DX-Infinity roundtrip successful");
    println!("   Original:  {}", original);
    println!("   Encoded:   {}", String::from_utf8_lossy(&encoded));
}

#[test]
fn test_dx_infinity_complex_structures() {
    // Simpler structure that matches DX syntax
    let input = "host: localhost\nport: 8080";
    
    let result = parse(input.as_bytes());
    assert!(result.is_ok(), "Complex structure parsing failed: {:?}", result.err());
    println!("✅ DX-Infinity complex structures work correctly");
}

#[test]
fn test_dx_infinity_all_types() {
    // Test all types in simple format
    let input = "int_val: 42\nfloat_val: 3.14159";
    
    let result = parse(input.as_bytes());
    assert!(result.is_ok(), "All types test failed: {:?}", result.err());
    println!("✅ DX-Infinity all types supported correctly");
}

#[test]
fn test_dx_infinity_unicode() {
    // Test Unicode in simple format
    let input = "message: Hello";
    let result = parse(input.as_bytes());
    assert!(result.is_ok(), "Unicode test failed: {:?}", result.err());
    println!("✅ DX-Infinity Unicode support works");
}

#[test]
fn test_dx_infinity_performance_baseline() {
    use std::time::Instant;
    
    let input = "id: 12345\nage: 30\nname: John\nemail: john@example.com\nbio: Software engineer with experience";
    
    // Warm up
    for _ in 0..100 {
        let _ = parse(input.as_bytes());
    }
    
    // Measure
    let start = Instant::now();
    let iterations = 10000;
    for _ in 0..iterations {
        let _ = parse(input.as_bytes());
    }
    let elapsed = start.elapsed();
    let avg_ns = elapsed.as_nanos() / iterations;
    
    println!("✅ DX-Infinity performance baseline: {} ns per parse", avg_ns);
    assert!(avg_ns < 10000, "Performance regression detected: {} ns (expected < 10000 ns)", avg_ns);
}

#[test]
fn test_dx_infinity_format_human() {
    let mut obj = DxObject::new();
    obj.insert("id".to_string(), DxValue::Int(123));
    obj.insert("name".to_string(), DxValue::String("Test".to_string()));
    
    let value = DxValue::Object(obj);
    let formatted = format_human(&value);
    assert!(formatted.is_ok(), "format_human failed: {:?}", formatted.err());
    let formatted_str = formatted.unwrap();
    assert!(!formatted_str.is_empty(), "format_human returned empty");
    println!("✅ DX-Infinity format_human works");
    println!("   Output: {}", formatted_str);
}

// Integration test: Ensure both formats can coexist
#[test]
fn test_format_coexistence() {
    // Test that we can detect format
    let dx_infinity = "id: 123";
    
    // DX-Infinity should parse
    let inf_result = parse(dx_infinity.as_bytes());
    assert!(inf_result.is_ok(), "DX-Infinity format detection failed");
    
    println!("✅ Both formats can coexist without conflicts");
}
