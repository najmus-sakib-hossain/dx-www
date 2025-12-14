//! Size Comparison: JSON vs TOON vs DX
//! Proves that DX is 65%+ more efficient than TOON

use std::fs;

fn main() {
    println!("╔════════════════════════════════════════════════════════╗");
    println!("║         SIZE COMPARISON: JSON vs TOON vs DX            ║");
    println!("╚════════════════════════════════════════════════════════╝\n");

    // Test 1: Simple Object
    println!("📦 TEST 1: Simple Object");
    println!("─────────────────────────────────────────────────────────");
    
    let simple_json = fs::read("data/simple.json").unwrap();
    let simple_toon = fs::read("data/simple.toon").unwrap();
    let simple_dx = fs::read("data/simple.dx").unwrap();
    
    println!("JSON:  {} bytes", simple_json.len());
    println!("TOON:  {} bytes", simple_toon.len());
    println!("DX:    {} bytes", simple_dx.len());
    
    let json_to_toon = (1.0 - simple_toon.len() as f64 / simple_json.len() as f64) * 100.0;
    let toon_to_dx = (1.0 - simple_dx.len() as f64 / simple_toon.len() as f64) * 100.0;
    let json_to_dx = (1.0 - simple_dx.len() as f64 / simple_json.len() as f64) * 100.0;
    
    println!("\nCompression:");
    println!("  JSON → TOON: {:.1}% smaller", json_to_toon);
    println!("  TOON → DX:   {:.1}% smaller 🎯", toon_to_dx);
    println!("  JSON → DX:   {:.1}% smaller", json_to_dx);
    
    if toon_to_dx >= 65.0 {
        println!("\n✅ SUCCESS: DX is {:.1}% more efficient than TOON!", toon_to_dx);
    } else {
        println!("\n⚠️  Target: 65%+ efficiency gain (current: {:.1}%)", toon_to_dx);
    }

    // Test 2: Complex Structure
    println!("\n\n📦 TEST 2: Complex Structure (Tables + Nested Data)");
    println!("─────────────────────────────────────────────────────────");
    
    let complex_json = fs::read("data/complex.json").unwrap();
    let complex_toon = fs::read("data/complex.toon").unwrap();
    let complex_dx = fs::read("data/complex.dx").unwrap();
    
    println!("JSON:  {} bytes", complex_json.len());
    println!("TOON:  {} bytes", complex_toon.len());
    println!("DX:    {} bytes", complex_dx.len());
    
    let json_to_toon = (1.0 - complex_toon.len() as f64 / complex_json.len() as f64) * 100.0;
    let toon_to_dx = (1.0 - complex_dx.len() as f64 / complex_toon.len() as f64) * 100.0;
    let json_to_dx = (1.0 - complex_dx.len() as f64 / complex_json.len() as f64) * 100.0;
    
    println!("\nCompression:");
    println!("  JSON → TOON: {:.1}% smaller", json_to_toon);
    println!("  TOON → DX:   {:.1}% smaller 🎯", toon_to_dx);
    println!("  JSON → DX:   {:.1}% smaller", json_to_dx);
    
    if toon_to_dx >= 65.0 {
        println!("\n✅ SUCCESS: DX is {:.1}% more efficient than TOON!", toon_to_dx);
    } else {
        println!("\n⚠️  Target: 65%+ efficiency gain (current: {:.1}%)", toon_to_dx);
    }

    // Summary
    println!("\n\n╔════════════════════════════════════════════════════════╗");
    println!("║                    FINAL SUMMARY                        ║");
    println!("╚════════════════════════════════════════════════════════╝\n");
    
    let total_json = simple_json.len() + complex_json.len();
    let total_toon = simple_toon.len() + complex_toon.len();
    let total_dx = simple_dx.len() + complex_dx.len();
    
    let avg_toon_to_dx = (1.0 - total_dx as f64 / total_toon as f64) * 100.0;
    let avg_json_to_dx = (1.0 - total_dx as f64 / total_json as f64) * 100.0;
    
    println!("Total Bytes:");
    println!("  JSON:  {} bytes", total_json);
    println!("  TOON:  {} bytes", total_toon);
    println!("  DX:    {} bytes", total_dx);
    
    println!("\nOverall Efficiency:");
    println!("  DX vs TOON: {:.1}% smaller 🎯", avg_toon_to_dx);
    println!("  DX vs JSON: {:.1}% smaller", avg_json_to_dx);
    
    if avg_toon_to_dx >= 65.0 {
        println!("\n🎉 MISSION ACCOMPLISHED! DX is {:.1}% more efficient than TOON!", avg_toon_to_dx);
        println!("   (Target was 65%+ improvement)");
    } else {
        println!("\n📈 Current: {:.1}% (Target: 65%+)", avg_toon_to_dx);
    }
}
