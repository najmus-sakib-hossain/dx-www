# Binary Style System - Quick Reference

## Installation

The binary style system is built into `dx-style`. No additional setup required.

```toml
[dependencies]
style = { path = "../style" }
```

## Basic Usage

### Auto Mode (Recommended)

```rust
use style::binary::*;

let css = generate_css_optimized(
    &["flex", "items-center", "p-4"],
    EncodingMode::Auto
);
```

### Manual Mode Selection

```rust
// Level 1: Binary IDs
let css = generate_css_optimized(&classes, EncodingMode::BinaryIds);

// Level 2: Direct cssText
let css = generate_css_optimized(&classes, EncodingMode::DirectCssText);

// Level 3: Combos (with fallback)
let css = generate_css_optimized(&classes, EncodingMode::Combos);

// Level 4: Varint encoding
let css = generate_css_optimized(&classes, EncodingMode::VarintIds);

// Level 5: Binary values
let css = generate_css_optimized(&classes, EncodingMode::BinaryValues);
```

## Network Transmission

### Send

```rust
let binary = encode_for_transmission(&["flex", "items-center", "p-4"]);
// Send `binary` over WebSocket, HTTP, etc.
```

### Receive

```rust
let css = decode_and_generate(&binary);
// Apply to DOM
```

## Analyzing Your Codebase

Find common style patterns in your project:

```bash
cd your-project
cargo run --bin analyze_styles -- src/
```

This generates `detected_combos.rs` with your most common patterns.

## Performance Tips

1. **Use Auto Mode** - It automatically picks the fastest path
2. **Pre-compute Combos** - Run analyzer on your codebase
3. **Use Varint for Network** - Smallest payload for transmission
4. **Batch Operations** - Process multiple elements together

## API Reference

### Core Functions

```rust
// Convert class name to ID
let id = style_name_to_id("flex"); // Some(4)

// Convert ID to CSS text
let css = style_id_to_csstext(4); // Some("display:flex")

// Apply styles directly
let css = apply_styles_direct(&[4, 26, 35]);

// Check for combo
let combo_id = is_common_combo(&[4, 26, 35]); // Some(0)

// Get combo CSS
let css = get_combo_csstext(0); // Some("display:flex;...")

// Encode/decode varint
let encoded = encode_id_list(&[4, 26, 35]);
let decoded = decode_id_list(&encoded);
```

### Advanced: Binary Values

```rust
use style::binary::*;

// Define properties
let props = vec![
    (CssProperty::Display, DisplayValue::Flex as u8),
    (CssProperty::AlignItems, AlignItemsValue::Center as u8),
];

// Encode to binary stream
let stream = encode_properties(&props);

// Decode to CSS
let css = apply_binary_css(&stream).unwrap();
```

## Examples

### Simple Button

```rust
let button_classes = vec!["px-4", "py-2", "bg-blue-500", "text-white", "rounded"];
let css = generate_css_optimized(&button_classes, EncodingMode::Auto);
```

### Card Component

```rust
let card_classes = vec![
    "p-6",
    "bg-white",
    "rounded-lg",
    "shadow-lg",
    "border",
];
let css = generate_css_optimized(&card_classes, EncodingMode::Auto);
```

### Flexbox Layout

```rust
let layout_classes = vec!["flex", "flex-col", "items-center", "justify-center"];
let css = generate_css_optimized(&layout_classes, EncodingMode::Auto);
```

## Benchmarking

Run the benchmark suite:

```bash
cargo bench --bench binary_styles_benchmark
```

View results in `target/criterion/`.

## Testing

```bash
# Run all tests
cargo test

# Run binary module tests only
cargo test --lib binary

# Run with output
cargo test -- --nocapture
```

## Common Patterns

### Check if combo exists

```rust
if let Some(combo_css) = try_apply_combo(&ids) {
    // Use pre-computed combo
    use_css(combo_css);
} else {
    // Fallback to individual styles
    use_css(&apply_styles_direct(&ids));
}
```

### Measure payload size

```rust
use style::binary::varint::calculate_compression;

let stats = calculate_compression(&ids);
println!("Original: {} bytes", stats.original_size);
println!("Compressed: {} bytes", stats.compressed_size);
println!("Savings: {:.1}%", stats.savings());
```

### Benchmark modes

```rust
use style::binary::api::benchmark_modes;

let stats = benchmark_modes(&["flex", "items-center", "p-4"]);
for stat in stats {
    println!("{:?}: {}µs, {} bytes", stat.mode, stat.generation_time_us, stat.output_size);
}
```

## Troubleshooting

### Class not found

```rust
if let Some(id) = style_name_to_id("my-class") {
    // Found
} else {
    // Class not in dictionary - add it to ids.rs
}
```

### Combo not detected

Common combos are predefined in `combos.rs`. Run the analyzer to find your patterns:

```bash
cargo run --bin analyze_styles -- src/
```

## What's Next?

- Add more utility classes to `ids.rs`
- Define custom combos in `combos.rs`
- Run analyzer to discover patterns
- Integrate with dx-www runtime

## Support

- Documentation: [BINARY_STYLE_SYSTEM.md](BINARY_STYLE_SYSTEM.md)
- Examples: `cargo run --example binary_demo`
- Benchmarks: `cargo bench --bench binary_styles_benchmark`
