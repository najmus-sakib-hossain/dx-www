//! DX Human Format Example
//! Demonstrates the beautiful formatting for LSP/IDE display

use dx_serializer::*;
use std::fs;

fn main() -> Result<()> {
    println!("╔════════════════════════════════════════════════════════╗");
    println!("║       DX HUMAN FORMAT - LSP DISPLAY SHOWCASE           ║");
    println!("╚════════════════════════════════════════════════════════╝\n");

    // Read complex data
    let complex_dx = fs::read("data/complex.dx")?;
    let parsed = parse(&complex_dx)?;

    // Standard Human Format
    println!("📊 STANDARD HUMAN FORMAT (Default)");
    println!("══════════════════════════════════════════════════════════\n");
    
    let human = format_human(&parsed)?;
    println!("{}", human);

    // Custom Formatted Version
    println!("\n📊 CUSTOM HUMAN FORMAT (No Unicode)");
    println!("══════════════════════════════════════════════════════════\n");
    
    let config = FormatterConfig {
        column_padding: 4,
        use_unicode: false,
        add_dividers: true,
        use_colors: false,
    };
    
    let custom = format_human_with_config(&parsed, config)?;
    println!("{}", custom);

    // Compact Format
    println!("\n📊 COMPACT FORMAT (No Dividers)");
    println!("══════════════════════════════════════════════════════════\n");
    
    let compact_config = FormatterConfig {
        column_padding: 2,
        use_unicode: true,
        add_dividers: false,
        use_colors: false,
    };
    
    let compact = format_human_with_config(&parsed, compact_config)?;
    println!("{}", compact);

    // Real-world Example: Configuration File
    println!("\n📊 REAL-WORLD: Application Configuration");
    println!("══════════════════════════════════════════════════════════\n");
    
    let config_dx = b"app.name:My Application
app.version:2.0.1
app.debug!
database.host:localhost
database.port:5432
database.pool_size:10
cache.enabled:+
cache.ttl:3600
features>auth|analytics|notifications|payments
environments=name%s url%s active%b
dev http://localhost:3000 +
staging https://staging.example.com +
production https://example.com -";

    let config_parsed = parse(config_dx)?;
    let config_human = format_human(&config_parsed)?;
    println!("{}", config_human);

    Ok(())
}
