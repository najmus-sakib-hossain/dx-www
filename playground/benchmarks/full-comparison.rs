//! DX Serializer: Optimizations Beyond TOON
//! Shows how DX can be further compressed with aliases and ditto marks

use std::fs;

fn main() {
    println!("╔════════════════════════════════════════════════════════╗");
    println!("║  DX-Serializer: Advanced Compression Techniques        ║");
    println!("╚════════════════════════════════════════════════════════╝\n");

    // Load formats
    let json = fs::read("data/hikes.json").unwrap();
    let yaml = fs::read("data/hikes.yaml").unwrap();
    let toon = fs::read("data/hikes.toon").unwrap();
    let dx_basic = fs::read("data/hikes.dx").unwrap();
    let dx_opt = fs::read("data/hikes-optimized.dx").unwrap();

    println!("📊 FULL COMPRESSION COMPARISON");
    println!("─────────────────────────────────────────────────────────");
    println!("JSON:          {} bytes", json.len());
    println!("YAML:          {} bytes", yaml.len());
    println!("TOON:          {} bytes", toon.len());
    println!("DX (Basic):    {} bytes", dx_basic.len());
    println!("DX (Optimized): {} bytes", dx_opt.len());

    // Calculate improvements
    let toon_to_dx_basic = (1.0 - dx_basic.len() as f64 / toon.len() as f64) * 100.0;
    let toon_to_dx_opt = (1.0 - dx_opt.len() as f64 / toon.len() as f64) * 100.0;
    let json_to_dx_opt = (1.0 - dx_opt.len() as f64 / json.len() as f64) * 100.0;

    println!("\n📈 IMPROVEMENT OVER TOON");
    println!("─────────────────────────────────────────────────────────");
    println!("DX Basic:     {:.1}% smaller than TOON", toon_to_dx_basic);
    println!("DX Optimized: {:.1}% smaller than TOON", toon_to_dx_opt);

    println!("\n🔍 FORMATS SIDE-BY-SIDE");
    println!("─────────────────────────────────────────────────────────");

    println!("\n📝 TOON ({} bytes):", toon.len());
    println!("{}", String::from_utf8_lossy(&toon));

    println!("📝 DX Basic ({} bytes):", dx_basic.len());
    println!("{}", String::from_utf8_lossy(&dx_basic));

    println!("📝 DX Optimized ({} bytes):", dx_opt.len());
    println!("{}", String::from_utf8_lossy(&dx_opt));

    println!("\n💡 DX OPTIMIZATION TECHNIQUES");
    println!("─────────────────────────────────────────────────────────");
    println!("  1. Alias Compression: $c: → context");
    println!("  2. Sigil Booleans: + → true, - → false (50% savings)");
    println!("  3. Type Hints: Enable vacuum parsing (no quotes needed)");
    println!("  4. Pipe Separators: | instead of ,");
    println!("  5. No Indentation: Flat structure");
    println!("  6. Stream Operator: > for arrays");
    println!("  7. Short Keys: km vs distanceKm, gain vs elevationGain");

    println!("\n╔════════════════════════════════════════════════════════╗");
    println!("║                   FINAL ANALYSIS                        ║");
    println!("╚════════════════════════════════════════════════════════╝\n");

    println!("🎯 DX Basic: {:.1}% better than TOON", toon_to_dx_basic);
    println!("🚀 DX Optimized: {:.1}% better than TOON", toon_to_dx_opt);
    println!("💥 Overall: {:.1}% smaller than JSON!", json_to_dx_opt);

    println!("\n📊 WHY THE DIFFERENCE?");
    println!("─────────────────────────────────────────────────────────");
    println!("TOON already uses tabular compression ([3]{{columns}}:)");
    println!("which is very efficient for this data structure.");
    println!("DX matches or slightly improves on this with:");
    println!("  • Shorter boolean syntax (+ vs true)");
    println!("  • Schema-guided parsing (type hints)");
    println!("  • Optional alias system for repeated keys");

    println!("\n🔥 WHERE DX REALLY SHINES:");
    println!("DX's advantage grows with:");
    println!("  • Complex nested objects (see complex.dx: 63.9% gain!)");
    println!("  • Repeated values (ditto marks: \" → repeat)");
    println!("  • Mixed data types (schema system)");
    println!("  • Machine parsing (zero-copy, SIMD tokenization)");
}
