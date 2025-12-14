//! DX Machine Format Example
//! Demonstrates parsing, encoding, and human formatting

use dx_serializer::*;
use std::fs;

fn main() -> Result<()> {
    println!("╔════════════════════════════════════════════════════════╗");
    println!("║      DX MACHINE FORMAT - EXAMPLE SHOWCASE              ║");
    println!("╚════════════════════════════════════════════════════════╝\n");

    // Example 1: Simple Object
    println!("📦 EXAMPLE 1: Simple Object");
    println!("─────────────────────────────────────────────────────────");
    
    let simple_dx = fs::read("data/simple.dx")?;
    println!("Input (DX Machine):\n{}\n", String::from_utf8_lossy(&simple_dx));
    
    let parsed = parse(&simple_dx)?;
    println!("Parsed Structure: {:#?}\n", parsed);
    
    let human = format_human(&parsed)?;
    println!("Human View:\n{}\n", human);

    // Example 2: Complex Structure
    println!("📦 EXAMPLE 2: Complex Structure with Tables");
    println!("─────────────────────────────────────────────────────────");
    
    let complex_dx = fs::read("data/complex.dx")?;
    println!("Input (DX Machine - {} bytes):\n{}\n", 
        complex_dx.len(), String::from_utf8_lossy(&complex_dx));
    
    let parsed = parse(&complex_dx)?;
    
    let human = format_human(&parsed)?;
    println!("Human View:\n{}\n", human);

    // Example 3: Advanced Features
    println!("📦 EXAMPLE 3: Advanced DX Features");
    println!("─────────────────────────────────────────────────────────");
    
    let advanced = b"$c=config
$c.db.host:localhost
$c.db.port:5432
$c.cache!
logs=ts%i event%s user%s code%i
1000 login alice 200
1001 login bob 200
1002 logout alice 200
_ _ bob _";

    println!("Input (with aliases, ditto, implicit flags):\n{}\n", 
        String::from_utf8_lossy(advanced));
    
    let parsed = parse(advanced)?;
    let human = format_human(&parsed)?;
    println!("Human View:\n{}\n", human);

    // Example 4: Round-trip Test
    println!("📦 EXAMPLE 4: Round-Trip (Parse → Encode → Parse)");
    println!("─────────────────────────────────────────────────────────");
    
    let original = b"users=id%i name%s score%f
1 Alice 95.5
2 Bob 87.3
3 Charlie 92.0";

    let parsed1 = parse(original)?;
    let encoded = encode(&parsed1)?;
    let parsed2 = parse(&encoded)?;
    
    println!("Original: {} bytes", original.len());
    println!("Encoded:  {} bytes", encoded.len());
    println!("Match:    {}", parsed1 == parsed2);
    println!("\nEncoded output:\n{}", String::from_utf8_lossy(&encoded));

    // Example 5: Compression Demo
    println!("\n📦 EXAMPLE 5: Compression Power");
    println!("─────────────────────────────────────────────────────────");
    
    let repetitive = b"events=timestamp%i status%s code%i message%s
1000 active 200 OK
1001 active 200 OK
1002 active 200 OK
1003 active 200 OK
1004 active 200 OK";

    let parsed = parse(repetitive)?;
    let encoded = encode(&parsed)?;
    
    println!("Original: {} bytes", repetitive.len());
    println!("Encoded:  {} bytes (with ditto compression)", encoded.len());
    println!("Savings:  {:.1}%\n", 
        (1.0 - encoded.len() as f64 / repetitive.len() as f64) * 100.0);
    
    println!("Compressed output:\n{}", String::from_utf8_lossy(&encoded));

    Ok(())
}
