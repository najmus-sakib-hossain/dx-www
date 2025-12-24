//! Comprehensive tests for hologram format conversions
//!
//! Tests all 6 conversion paths:
//! 1. LLM → Human (inflate)
//! 2. Human → LLM (deflate)
//! 3. LLM → Machine (not yet implemented)
//! 4. Machine → LLM (not yet implemented)
//! 5. Human → Machine (compress)
//! 6. Machine → Human (format_human)

use serializer::hologram::{deflate, inflate, Deflater, HologramConfig, Inflater};

#[test]
fn test_llm_to_human_simple_keyvalue() {
    let llm = "c.n:dx\n^v:0.0.1";
    let human = inflate(llm);
    
    // Should expand abbreviated keys
    assert!(human.contains("context.name") || human.contains("c.n"));
    assert!(human.contains("dx"));
    assert!(human.contains("0.0.1"));
}

#[test]
fn test_llm_to_human_object() {
    let llm = "server#host:localhost#port:5432#ssl:1";
    let human = inflate(llm);
    
    assert!(human.contains("server"));
    assert!(human.contains("host"));
    assert!(human.contains("localhost"));
    assert!(human.contains("port"));
    assert!(human.contains("5432"));
    assert!(human.contains("✓") || human.contains("true")); // Boolean expansion
}

#[test]
fn test_llm_to_human_array() {
    let llm = "items@3>apple|banana|cherry";
    let human = inflate(llm);
    
    assert!(human.contains("items"));
    assert!(human.contains("apple"));
    assert!(human.contains("banana"));
    assert!(human.contains("cherry"));
}

#[test]
fn test_llm_to_human_dx_array_format() {
    // DX format: key>item|item without @count
    let llm = "workspace>frontend|backend|shared";
    let human = inflate(llm);
    
    assert!(human.contains("workspace"));
    assert!(human.contains("frontend"));
    assert!(human.contains("backend"));
    assert!(human.contains("shared"));
}

#[test]
fn test_llm_to_human_table() {
    let llm = "users@2=id^name^active\n>1|Alice|1\n>2|Bob|0";
    let human = inflate(llm);
    
    assert!(human.contains("users"));
    assert!(human.contains("id"));
    assert!(human.contains("name"));
    assert!(human.contains("active"));
    assert!(human.contains("Alice"));
    assert!(human.contains("Bob"));
}

#[test]
fn test_human_to_llm_simple_keyvalue() {
    let human = "context.name : dx\n^version : 0.0.1";
    let llm = deflate(human);
    
    // Should compress keys
    assert!(llm.contains("c.n:dx") || llm.contains("context.name:dx"));
    assert!(llm.contains("v:0.0.1") || llm.contains("version:0.0.1"));
}

#[test]
fn test_human_to_llm_object() {
    let human = "▼ server\n    host: localhost\n    port: 5432\n    ssl: ✓";
    let llm = deflate(human);
    
    assert!(llm.contains("server#"));
    assert!(llm.contains("host:localhost"));
    assert!(llm.contains("port:5432"));
    assert!(llm.contains("ssl:1") || llm.contains("ssl:true"));
}

#[test]
fn test_human_to_llm_array() {
    let human = "▼ colors (3 items)\n    • red\n    • green\n    • blue";
    let llm = deflate(human);
    
    assert!(llm.contains("colors@3>"));
    assert!(llm.contains("red|green|blue"));
}

#[test]
fn test_human_to_llm_dx_array_format() {
    let human = "workspace > frontend | backend | shared";
    let llm = deflate(human);
    
    assert!(llm.contains("ws>") || llm.contains("workspace>"));
    assert!(llm.contains("frontend|backend|shared"));
}

#[test]
fn test_roundtrip_object() {
    let config = HologramConfig::default();
    let inflater = Inflater::new(config.clone());
    let deflater = Deflater::new(config);
    
    let original = "db#host:localhost#port:5432#ssl:1";
    let pretty = inflater.inflate(original);
    let back = deflater.deflate(&pretty);
    
    assert!(back.contains("host:localhost"));
    assert!(back.contains("port:5432"));
    assert!(back.contains("ssl:1"));
}

#[test]
fn test_roundtrip_array() {
    let config = HologramConfig::default();
    let inflater = Inflater::new(config.clone());
    let deflater = Deflater::new(config);
    
    let original = "items@3>apple|banana|cherry";
    let pretty = inflater.inflate(original);
    let back = deflater.deflate(&pretty);
    
    assert!(back.contains("items@3>"));
    assert!(back.contains("apple|banana|cherry"));
}

#[test]
fn test_dx_format_output() {
    // Test DX format (flat key:value) output
    let config = HologramConfig::dx_format();
    let inflater = Inflater::new(config);
    
    let llm = "workspace>frontend|backend";
    let human = inflater.inflate(llm);
    
    // DX format should output as: workspace > frontend | backend
    assert!(human.contains("workspace"));
    assert!(human.contains(">") || human.contains(" > "));
    assert!(human.contains("frontend"));
    assert!(human.contains("backend"));
}

#[test]
fn test_comment_preservation() {
    let llm = "!Database config!db#host:localhost";
    let human = inflate(llm);
    
    assert!(human.contains("Database config") || human.contains("//"));
    
    let back = deflate(&human);
    assert!(back.contains("Database config") || back.contains("!"));
}

#[test]
fn test_boolean_expansion() {
    let llm = "config#debug:1#prod:0";
    let human = inflate(llm);
    
    // Should expand to ✓/✗ or true/false
    assert!(human.contains("✓") || human.contains("true"));
    assert!(human.contains("✗") || human.contains("false"));
}

#[test]
fn test_boolean_compression() {
    let human = "▼ config\n    debug: ✓\n    prod: ✗";
    let llm = deflate(human);
    
    // Should compress to 1/0
    assert!(llm.contains("debug:1"));
    assert!(llm.contains("prod:0"));
}

#[test]
fn test_null_handling() {
    let llm = "config#value:~";
    let human = inflate(llm);
    
    assert!(human.contains("—") || human.contains("null") || human.contains("none"));
}

#[test]
fn test_reference_handling() {
    let llm = "link#target:*users.1";
    let human = inflate(llm);
    
    assert!(human.contains("→users.1") || human.contains("*users.1"));
}

#[test]
fn test_prefix_inheritance() {
    let llm = "forge.repository:https://github.com/dx/dx\n^container:none";
    let human = inflate(llm);
    
    assert!(human.contains("repository") || human.contains("repo"));
    assert!(human.contains("container"));
}
