//! TOON vs DX Comparison - "Hikes" Example
//! This is the official TOON benchmark from their documentation

use std::fs;

fn main() {
    println!("╔════════════════════════════════════════════════════════╗");
    println!("║     TOON vs DX: Official 'Hikes' Benchmark             ║");
    println!("╚════════════════════════════════════════════════════════╝");

    // Load all formats
    let json = fs::read("data/hikes.json").unwrap();
    let yaml = fs::read("data/hikes.yaml").unwrap();
    let toon = fs::read("data/hikes.toon").unwrap();
    let dx = fs::read("data/hikes.dx").unwrap();

    println!("📊 SIZE COMPARISON");
    println!("─────────────────────────────────────────────────────────");
    println!("JSON:  {} bytes", json.len());
    println!("YAML:  {} bytes", yaml.len());
    println!("TOON:  {} bytes", toon.len());
    println!("DX:    {} bytes", dx.len());

    // Calculate improvements
    let json_to_yaml = (1.0 - yaml.len() as f64 / json.len() as f64) * 100.0;
    let yaml_to_toon = (1.0 - toon.len() as f64 / yaml.len() as f64) * 100.0;
    let toon_to_dx = (1.0 - dx.len() as f64 / toon.len() as f64) * 100.0;
    let json_to_dx = (1.0 - dx.len() as f64 / json.len() as f64) * 100.0;

    println!("\n📈 COMPRESSION CHAIN");
    println!("─────────────────────────────────────────────────────────");
    println!("JSON → YAML: {:.1}% smaller", json_to_yaml);
    println!("YAML → TOON: {:.1}% smaller", yaml_to_toon);
    println!("TOON → DX:   {:.1}% smaller 🎯", toon_to_dx);
    println!("JSON → DX:   {:.1}% smaller (total)", json_to_dx);

    println!("\n🔍 DX ADVANTAGES OVER TOON");
    println!("─────────────────────────────────────────────────────────");

    // Show the actual files
    println!("\nTOON Format ({} bytes):", toon.len());
    println!("{}", String::from_utf8_lossy(&toon));

    println!("\nDX Format ({} bytes):", dx.len());
    println!("{}", String::from_utf8_lossy(&dx));

    println!("\n💡 KEY DIFFERENCES:");
    println!(
        "  1. No indentation in DX (saves {} bytes)",
        count_spaces(&toon) - count_spaces(&dx)
    );
    println!("  2. Shorter booleans: + vs true, - vs false");
    println!("  3. Stream operator: > instead of [3]:");
    println!("  4. Shortened keys: km, gain, who, sun vs full names");
    println!("  5. Type hints (%i %s %f %b) enable zero-copy parsing");
    println!("  6. Pipe separator | instead of comma");

    println!("\n╔════════════════════════════════════════════════════════╗");
    println!("║                    FINAL VERDICT                        ║");
    println!("╚════════════════════════════════════════════════════════╝\n");

    if toon_to_dx >= 65.0 {
        println!("✅ SUCCESS! DX is {:.1}% more efficient than TOON!", toon_to_dx);
        println!("   (Target was 65%+ improvement)");
    } else {
        println!("📊 DX is {:.1}% more efficient than TOON", toon_to_dx);
        println!("   (Target: 65%+, Achieved: {:.1}%)", toon_to_dx);

        if toon_to_dx >= 60.0 {
            println!("\n🎯 Very close to target! ({:.1}% of 65% goal)", toon_to_dx / 65.0 * 100.0);
        }
    }

    println!("\n🚀 DX vs JSON: {:.1}% smaller!", json_to_dx);
    println!("   (That's a {}x reduction in size!)", json.len() / dx.len());
}

fn count_spaces(data: &[u8]) -> usize {
    data.iter().filter(|&&b| b == b' ').count()
}
