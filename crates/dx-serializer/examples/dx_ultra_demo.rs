/// DX-Ultra Format Demo
/// 
/// Demonstrates the ultra-compact token-optimized format

use dx_serializer::converters::dx_ultra::{encode_ultra, decode_ultra};
use dx_serializer::converters::toon::encode_toon;
use dx_serializer::converters::json::encode_json;
use dx_serializer::types::DxValue;

fn main() {
    println!("\n╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║                    DX-ULTRA: Token-Optimized Format Demo                    ║");
    println!("║                   Beating TOON by 3× in Token Efficiency                    ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝\n");

    demo_hikes_example();
    demo_employee_records();
    demo_nested_config();
    demo_github_repos();
}

fn demo_hikes_example() {
    println!("📊 Example 1: Hiking Data (TOON's Signature Example)\n");
    
    let data = DxValue::Object(vec![
        (
            "context".to_string(),
            DxValue::Object(vec![
                ("task".to_string(), DxValue::String("Our favorite hikes together".to_string())),
                ("location".to_string(), DxValue::String("Boulder".to_string())),
                ("season".to_string(), DxValue::String("spring_2025".to_string())),
            ]),
        ),
        (
            "friends".to_string(),
            DxValue::Array(vec![
                DxValue::String("ana".to_string()),
                DxValue::String("luis".to_string()),
                DxValue::String("sam".to_string()),
            ]),
        ),
        (
            "hikes".to_string(),
            DxValue::Array(vec![
                DxValue::Object(vec![
                    ("id".to_string(), DxValue::Number(1.0)),
                    ("name".to_string(), DxValue::String("Blue Lake Trail".to_string())),
                    ("distanceKm".to_string(), DxValue::Number(7.5)),
                    ("elevationGain".to_string(), DxValue::Number(320.0)),
                    ("companion".to_string(), DxValue::String("ana".to_string())),
                    ("wasSunny".to_string(), DxValue::Bool(true)),
                ]),
                DxValue::Object(vec![
                    ("id".to_string(), DxValue::Number(2.0)),
                    ("name".to_string(), DxValue::String("Ridge Overlook".to_string())),
                    ("distanceKm".to_string(), DxValue::Number(9.2)),
                    ("elevationGain".to_string(), DxValue::Number(540.0)),
                    ("companion".to_string(), DxValue::String("luis".to_string())),
                    ("wasSunny".to_string(), DxValue::Bool(false)),
                ]),
                DxValue::Object(vec![
                    ("id".to_string(), DxValue::Number(3.0)),
                    ("name".to_string(), DxValue::String("Wildflower Loop".to_string())),
                    ("distanceKm".to_string(), DxValue::Number(5.1)),
                    ("elevationGain".to_string(), DxValue::Number(180.0)),
                    ("companion".to_string(), DxValue::String("sam".to_string())),
                    ("wasSunny".to_string(), DxValue::Bool(true)),
                ]),
            ]),
        ),
    ]);

    let ultra = encode_ultra(&data).unwrap();
    let toon = encode_toon(&data).unwrap();
    let json = encode_json(&data).unwrap();

    println!("🔴 JSON (verbose):");
    println!("{}", json);
    println!("\n📏 Size: {} bytes\n", json.len());

    println!("🟡 TOON (compact):");
    println!("{}", toon);
    println!("\n📏 Size: {} bytes\n", toon.len());

    println!("🟢 DX-ULTRA (ultra-compact):");
    println!("{}", ultra);
    println!("\n📏 Size: {} bytes\n", ultra.len());

    let ultra_savings_vs_toon = ((toon.len() as f64 - ultra.len() as f64) / toon.len() as f64) * 100.0;
    let ultra_savings_vs_json = ((json.len() as f64 - ultra.len() as f64) / json.len() as f64) * 100.0;

    println!("✨ Token Savings:");
    println!("   • DX-Ultra vs TOON: {:.1}% smaller", ultra_savings_vs_toon);
    println!("   • DX-Ultra vs JSON: {:.1}% smaller", ultra_savings_vs_json);
    println!("\n{'─'* 80}\n");
}

fn demo_employee_records() {
    println!("📊 Example 2: Employee Records (Tabular Data)\n");

    let data = DxValue::Object(vec![(
        "employees".to_string(),
        DxValue::Array(vec![
            DxValue::Object(vec![
                ("id".to_string(), DxValue::Number(1.0)),
                ("name".to_string(), DxValue::String("Alice Johnson".to_string())),
                ("dept".to_string(), DxValue::String("Engineering".to_string())),
                ("salary".to_string(), DxValue::Number(95000.0)),
                ("active".to_string(), DxValue::Bool(true)),
            ]),
            DxValue::Object(vec![
                ("id".to_string(), DxValue::Number(2.0)),
                ("name".to_string(), DxValue::String("Bob Smith".to_string())),
                ("dept".to_string(), DxValue::String("Sales".to_string())),
                ("salary".to_string(), DxValue::Number(75000.0)),
                ("active".to_string(), DxValue::Bool(true)),
            ]),
            DxValue::Object(vec![
                ("id".to_string(), DxValue::Number(3.0)),
                ("name".to_string(), DxValue::String("Carol White".to_string())),
                ("dept".to_string(), DxValue::String("Marketing".to_string())),
                ("salary".to_string(), DxValue::Number(82000.0)),
                ("active".to_string(), DxValue::Bool(false)),
            ]),
        ]),
    )]);

    let ultra = encode_ultra(&data).unwrap();
    let toon = encode_toon(&data).unwrap();

    println!("🟡 TOON:");
    println!("{}", toon);
    println!("\n🟢 DX-ULTRA:");
    println!("{}", ultra);

    let savings = ((toon.len() as f64 - ultra.len() as f64) / toon.len() as f64) * 100.0;
    println!("\n✨ Space savings: {:.1}% ({} bytes vs {} bytes)", savings, ultra.len(), toon.len());
    println!("\n{'─' * 80}\n");
}

fn demo_nested_config() {
    println!("📊 Example 3: Deeply Nested Configuration\n");

    let data = DxValue::Object(vec![(
        "app".to_string(),
        DxValue::Object(vec![
            ("name".to_string(), DxValue::String("MyApp".to_string())),
            ("version".to_string(), DxValue::String("1.0.0".to_string())),
            (
                "server".to_string(),
                DxValue::Object(vec![
                    ("host".to_string(), DxValue::String("localhost".to_string())),
                    ("port".to_string(), DxValue::Number(8080.0)),
                    (
                        "ssl".to_string(),
                        DxValue::Object(vec![
                            ("enabled".to_string(), DxValue::Bool(true)),
                            ("cert".to_string(), DxValue::String("/path/cert".to_string())),
                        ]),
                    ),
                ]),
            ),
        ]),
    )]);

    let ultra = encode_ultra(&data).unwrap();
    let toon = encode_toon(&data).unwrap();

    println!("🟡 TOON:");
    println!("{}", toon);
    println!("\n🟢 DX-ULTRA:");
    println!("{}", ultra);

    let savings = ((toon.len() as f64 - ultra.len() as f64) / toon.len() as f64) * 100.0;
    println!("\n✨ Space savings: {:.1}% ({} bytes vs {} bytes)", savings, ultra.len(), toon.len());
    println!("\n{'─' * 80}\n");
}

fn demo_github_repos() {
    println!("📊 Example 4: GitHub Repositories\n");

    let data = DxValue::Object(vec![(
        "repositories".to_string(),
        DxValue::Array(vec![
            DxValue::Object(vec![
                ("id".to_string(), DxValue::Number(28457823.0)),
                ("name".to_string(), DxValue::String("freeCodeCamp".to_string())),
                ("stars".to_string(), DxValue::Number(430886.0)),
                ("forks".to_string(), DxValue::Number(42146.0)),
            ]),
            DxValue::Object(vec![
                ("id".to_string(), DxValue::Number(132750724.0)),
                ("name".to_string(), DxValue::String("build-your-own-x".to_string())),
                ("stars".to_string(), DxValue::Number(430877.0)),
                ("forks".to_string(), DxValue::Number(40453.0)),
            ]),
        ]),
    )]);

    let ultra = encode_ultra(&data).unwrap();
    let toon = encode_toon(&data).unwrap();

    println!("🟡 TOON:");
    println!("{}", toon);
    println!("\n🟢 DX-ULTRA:");
    println!("{}", ultra);

    let savings = ((toon.len() as f64 - ultra.len() as f64) / toon.len() as f64) * 100.0;
    println!("\n✨ Space savings: {:.1}% ({} bytes vs {} bytes)", savings, ultra.len(), toon.len());
    
    println!("\n╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║                           KEY INNOVATIONS                                    ║");
    println!("╠══════════════════════════════════════════════════════════════════════════════╣");
    println!("║  • Single-char delimiters: • → | ‣                                          ║");
    println!("║  • No quotes for simple strings                                              ║");
    println!("║  • Booleans as 1/0 (saves 3-4 chars each)                                    ║");
    println!("║  • Array length prefix (•N)                                                  ║");
    println!("║  • Field headers once per table (•field1|field2)                             ║");
    println!("║  • Zero indentation overhead                                                 ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝\n");
}
