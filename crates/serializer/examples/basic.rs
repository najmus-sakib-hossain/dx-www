//! Basic DX Serializer Usage
//!
//! This example demonstrates fundamental parsing and encoding operations.

use serializer::{encode, format_human, parse, DxValue};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== DX Serializer: Basic Usage ===\n");

    // Example 1: Simple key-value pairs
    println!("1. Simple Key-Value Pairs");
    let input = b"name:Alice^age:30^active:+";
    let data = parse(input)?;

    if let DxValue::Object(obj) = &data {
        println!("   Name: {}", obj.get("name").unwrap().as_str().unwrap());
        println!("   Age: {}", obj.get("age").unwrap().as_int().unwrap());
        println!("   Active: {}", obj.get("active").unwrap().as_bool().unwrap());
    }

    // Example 2: Nested objects with dot notation
    println!("\n2. Nested Objects");
    let input = b"user.name:Bob^user.email:bob@example.com^user.role:admin";
    let data = parse(input)?;
    println!("   Parsed: {:?}", data);

    // Example 3: Arrays with stream operator
    println!("\n3. Arrays");
    let input = b"colors>red|blue|green|yellow";
    let data = parse(input)?;
    if let DxValue::Object(obj) = &data {
        if let Some(DxValue::Array(arr)) = obj.get("colors") {
            print!("   Colors: ");
            for elem in &arr.values {
                print!("{} ", elem.as_str().unwrap());
            }
            println!();
        }
    }

    // Example 4: Encoding data
    println!("\n4. Encoding");
    let input = b"project:DX Runtime^version:0.1.0^status:+";
    let data = parse(input)?;
    let encoded = encode(&data)?;
    println!("   Original: {}", String::from_utf8_lossy(input));
    println!("   Encoded:  {}", String::from_utf8_lossy(&encoded));

    // Example 5: Human formatting (LSP)
    println!("\n5. Human Formatting");
    let human = format_human(&data)?;
    println!("{}", human);

    Ok(())
}
