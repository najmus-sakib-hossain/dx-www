//! Comprehensive integration tests for dx-serializer

use serializer::*;

#[test]
fn test_complete_round_trip() {
    let input = b"$c=context
$c.task:Mission Alpha
$c.location:Base Delta
team>alice|bob|charlie
tasks=id%i name%s hours%f urgent%b
1 Code Review 2.5 +
2 Deploy Server 4.0 -
3 Write Tests 3.5 +
_ Security Audit 6.0 +";

    // Parse
    let parsed = parse(input).expect("Parse failed");

    // Encode back
    let encoded = encode(&parsed).expect("Encode failed");

    // Reparse
    let reparsed = parse(&encoded).expect("Reparse failed");

    // Should be identical
    assert_eq!(parsed, reparsed);
}

#[test]
fn test_human_format() {
    let input = b"project:DX Runtime
version:0.1.0
active:+
team>alice|bob|charlie
stats=metric%s value%i
LOC 50000
Tests 2500
Coverage 95";

    let value = parse(input).expect("Parse failed");
    let human = format_human(&value).expect("Format failed");

    // Check structure
    assert!(human.contains("DX HUMAN VIEW"));
    assert!(human.contains("DX Runtime"));
    assert!(human.contains("alice"));
    assert!(human.contains("STATS TABLE"));
    assert!(human.contains("LOC"));
    assert!(human.contains("50000"));
}

#[test]
fn test_ditto_compression() {
    let input = b"data=id%i status%s priority%i
1 active 5
2 active 5
3 active 5";

    let value = parse(input).expect("Parse failed");
    let encoded = encode(&value).expect("Encode failed");
    let encoded_str = std::str::from_utf8(&encoded).unwrap();

    // Should use ditto for repeated values
    assert!(encoded_str.contains("_"));
}

#[test]
fn test_alias_generation() {
    let input = b"configuration.database.host:localhost
configuration.database.port:5432
configuration.server.host:0.0.0.0";

    let value = parse(input).expect("Parse failed");
    let encoded = encode(&value).expect("Encode failed");
    let encoded_str = std::str::from_utf8(&encoded).unwrap();

    // Long keys should generate aliases
    println!("Encoded:\n{}", encoded_str);
}

#[test]
fn test_vacuum_parsing() {
    // Strings with spaces - no quotes needed
    let input = b"users=id%i name%s score%f
1 Alice Johnson 95.5
2 Bob Smith 87.3";

    let value = parse(input).expect("Parse failed");

    if let DxValue::Object(obj) = value {
        if let Some(DxValue::Table(table)) = obj.get("users") {
            assert_eq!(table.rows[0][1], DxValue::String("Alice Johnson".to_string()));
            assert_eq!(table.rows[1][1], DxValue::String("Bob Smith".to_string()));
        } else {
            panic!("Expected table");
        }
    }
}

#[test]
fn test_prefix_inheritance() {
    let input = b"app.name:DX
^version:1.0
^author:Team";

    let value = parse(input).expect("Parse failed");

    if let DxValue::Object(obj) = value {
        assert!(obj.get("app.name").is_some());
        assert!(obj.get("version").is_some() || obj.get("app.version").is_some());
    }
}

#[test]
fn test_implicit_flags() {
    let input = b"admin!
debug!
error?";

    let value = parse(input).expect("Parse failed");

    if let DxValue::Object(obj) = value {
        assert_eq!(obj.get("admin"), Some(&DxValue::Bool(true)));
        assert_eq!(obj.get("debug"), Some(&DxValue::Bool(true)));
        assert_eq!(obj.get("error"), Some(&DxValue::Null));
    }
}

#[test]
fn test_complex_nested() {
    let input = b"project:DX
metadata.created:2025-01-01
metadata.author:Team
dependencies>rust|wasm|web-sys
benchmarks=name%s time%f improvement%f
Parse 0.05 800
Encode 0.03 950";

    let value = parse(input).expect("Parse failed");
    let human = format_human(&value).expect("Format failed");

    assert!(human.contains("project"));
    assert!(human.contains("metadata"));
    assert!(human.contains("rust"));
    assert!(human.contains("BENCHMARKS TABLE"));
}

#[test]
fn test_empty_table() {
    let input = b"users=id%i name%s";

    let value = parse(input).expect("Parse failed");

    if let DxValue::Object(obj) = value {
        if let Some(DxValue::Table(table)) = obj.get("users") {
            assert_eq!(table.row_count(), 0);
        }
    }
}

#[test]
fn test_sigil_values() {
    let input = b"data=active%b score%i status%s
+ 100 ready
- 50 pending
~ 0 error";

    let value = parse(input).expect("Parse failed");

    if let DxValue::Object(obj) = value {
        if let Some(DxValue::Table(table)) = obj.get("data") {
            assert_eq!(table.rows[0][0], DxValue::Bool(true));
            assert_eq!(table.rows[1][0], DxValue::Bool(false));
            assert_eq!(table.rows[2][0], DxValue::Null);
        }
    }
}

#[test]
fn test_stream_array_variations() {
    let input = b"strings>alpha|beta|gamma
numbers>1|2|3|4|5
mixed>test|123|+|-";

    let value = parse(input).expect("Parse failed");

    if let DxValue::Object(obj) = value {
        if let Some(DxValue::Array(arr)) = obj.get("strings") {
            assert_eq!(arr.values.len(), 3);
            assert!(arr.is_stream);
        }
        if let Some(DxValue::Array(arr)) = obj.get("numbers") {
            assert_eq!(arr.values.len(), 5);
        }
    }
}
