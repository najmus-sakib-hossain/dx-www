//! DX Ω: The TOON Killer
//! Implements advanced compression: inline prefixing (^), header minification

use std::fs;

fn main() {
    println!("╔════════════════════════════════════════════════════════╗");
    println!("║             DX Ω: THE TOON KILLER                      ║");
    println!("╚════════════════════════════════════════════════════════╝\n");

    // === HIKES BENCHMARK ===
    let json_hikes = fs::read("data/hikes.json").unwrap();
    let toon_hikes = fs::read("data/hikes.toon").unwrap();
    let dx_basic_hikes = fs::read("data/hikes.dx").unwrap();
    let dx_omega_hikes = fs::read("data/hikes-omega.dx").unwrap();

    println!("📊 TEST 1: HIKES (TABULAR DATA)");
    println!("─────────────────────────────────────────────────────────");
    println!("JSON:         {} bytes", json_hikes.len());
    println!("TOON:         {} bytes", toon_hikes.len());
    println!("DX Basic:     {} bytes", dx_basic_hikes.len());
    println!("DX Ω:         {} bytes", dx_omega_hikes.len());

    let hikes_vs_toon = (1.0 - dx_omega_hikes.len() as f64 / toon_hikes.len() as f64) * 100.0;
    let hikes_vs_json = (1.0 - dx_omega_hikes.len() as f64 / json_hikes.len() as f64) * 100.0;
    
    println!("\n📈 IMPROVEMENT:");
    println!("DX Ω vs TOON: {:.1}% smaller", hikes_vs_toon);
    println!("DX Ω vs JSON: {:.1}% smaller", hikes_vs_json);

    // Raw data estimation
    let raw_data = 130; // Estimated raw string content
    let toon_overhead = toon_hikes.len() - raw_data;
    let dx_overhead = dx_omega_hikes.len() - raw_data;
    let overhead_reduction = (1.0 - dx_overhead as f64 / toon_overhead as f64) * 100.0;

    println!("\n🔬 STRUCTURAL OVERHEAD ANALYSIS:");
    println!("TOON Overhead: {} bytes", toon_overhead);
    println!("DX Ω Overhead: {} bytes", dx_overhead);
    println!("Overhead Reduction: {:.1}%", overhead_reduction);

    // === COMPLEX BENCHMARK ===
    println!("\n\n📊 TEST 2: COMPLEX (NESTED DATA)");
    println!("─────────────────────────────────────────────────────────");
    
    let json_complex = fs::read("data/complex.json").unwrap();
    let toon_complex = fs::read("data/complex.toon").unwrap();
    let dx_basic_complex = fs::read("data/complex.dx").unwrap();
    let dx_omega_complex = fs::read("data/complex-omega.dx").unwrap();

    println!("JSON:         {} bytes", json_complex.len());
    println!("TOON:         {} bytes", toon_complex.len());
    println!("DX Basic:     {} bytes", dx_basic_complex.len());
    println!("DX Ω:         {} bytes", dx_omega_complex.len());

    let complex_vs_toon = (1.0 - dx_omega_complex.len() as f64 / toon_complex.len() as f64) * 100.0;
    let complex_vs_json = (1.0 - dx_omega_complex.len() as f64 / json_complex.len() as f64) * 100.0;

    println!("\n📈 IMPROVEMENT:");
    println!("DX Ω vs TOON: {:.1}% smaller", complex_vs_toon);
    println!("DX Ω vs JSON: {:.1}% smaller", complex_vs_json);

    // === SIMPLE BENCHMARK ===
    println!("\n\n📊 TEST 3: SIMPLE (FLAT DATA)");
    println!("─────────────────────────────────────────────────────────");
    
    let json_simple = fs::read("data/simple.json").unwrap();
    let toon_simple = fs::read("data/simple.toon").unwrap();
    let dx_basic_simple = fs::read("data/simple.dx").unwrap();
    let dx_omega_simple = fs::read("data/simple-omega.dx").unwrap();

    println!("JSON:         {} bytes", json_simple.len());
    println!("TOON:         {} bytes", toon_simple.len());
    println!("DX Basic:     {} bytes", dx_basic_simple.len());
    println!("DX Ω:         {} bytes", dx_omega_simple.len());

    let simple_vs_toon = (1.0 - dx_omega_simple.len() as f64 / toon_simple.len() as f64) * 100.0;
    let simple_vs_json = (1.0 - dx_omega_simple.len() as f64 / json_simple.len() as f64) * 100.0;

    println!("\n📈 IMPROVEMENT:");
    println!("DX Ω vs TOON: {:.1}% smaller", simple_vs_toon);
    println!("DX Ω vs JSON: {:.1}% smaller", simple_vs_json);

    // === FORMATS SIDE BY SIDE ===
    println!("\n\n🔍 FORMAT COMPARISON: HIKES");
    println!("─────────────────────────────────────────────────────────");
    
    println!("\n📝 TOON ({} bytes):", toon_hikes.len());
    print!("{}", String::from_utf8_lossy(&toon_hikes));
    
    println!("\n📝 DX Ω ({} bytes):", dx_omega_hikes.len());
    print!("{}", String::from_utf8_lossy(&dx_omega_hikes));

    println!("\n\n💡 DX Ω INNOVATIONS:");
    println!("─────────────────────────────────────────────────────────");
    println!("  1. Inline Prefixing: ^ instead of newlines (saves {}B)", 
        count_newlines(&toon_hikes) - count_newlines(&dx_omega_hikes));
    println!("  2. Header Minification: 'c' vs 'context', 'loc' vs 'location'");
    println!("  3. Single-char operators: i vs %i, + vs true");
    println!("  4. Stream operator: > for arrays");
    println!("  5. Vacuum parsing: No quotes needed");
    println!("  6. Type-guided schema: Enables zero-copy");

    // === FINAL VERDICT ===
    println!("\n\n╔════════════════════════════════════════════════════════╗");
    println!("║                   FINAL VERDICT                         ║");
    println!("╚════════════════════════════════════════════════════════╝\n");

    let avg_improvement = (hikes_vs_toon + complex_vs_toon + simple_vs_toon) / 3.0;

    println!("🎯 TABULAR DATA (Hikes):   {:.1}% better than TOON", hikes_vs_toon);
    println!("🚀 COMPLEX DATA:           {:.1}% better than TOON", complex_vs_toon);
    println!("📊 SIMPLE DATA:            {:.1}% better than TOON", simple_vs_toon);
    println!("─────────────────────────────────────────────────────────");
    println!("⚡ AVERAGE:                {:.1}% better than TOON\n", avg_improvement);

    // Check targets
    let target_regular = 30.0;
    let target_complex = 65.0;

    if hikes_vs_toon >= target_regular {
        println!("✅ TARGET MET: Regular data is {:.1}% better (target: {}%+)", 
            hikes_vs_toon, target_regular);
    } else {
        println!("⚠️  Target: {}%+, Achieved: {:.1}%", target_regular, hikes_vs_toon);
    }

    if complex_vs_toon >= target_complex {
        println!("✅ TARGET MET: Complex data is {:.1}% better (target: {}%+)", 
            complex_vs_toon, target_complex);
    } else {
        println!("⚠️  Target: {}%+, Achieved: {:.1}%", target_complex, complex_vs_toon);
    }

    println!("\n🏆 DX Ω ACHIEVEMENT:");
    println!("   Structural overhead reduced by {:.1}%", overhead_reduction);
    println!("   DX is essentially pure data with minimal format weight.");
}

fn count_newlines(data: &[u8]) -> usize {
    data.iter().filter(|&&b| b == b'\n').count()
}
