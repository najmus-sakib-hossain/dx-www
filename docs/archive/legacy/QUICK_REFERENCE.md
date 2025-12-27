# DX Serializer Quick Reference

## Installation

Add to `Cargo.toml`:
```toml
[dependencies]
dx-serializer = { path = "../crates/dx-serializer" }
```

## Basic Usage

```rust
use dx_serializer::{parse, encode, format_human};

// Parse DX format
let data = parse(b"name:Alice\nage:30\nactive:+")?;

// Encode to DX
let bytes = encode(&data)?;

// Format for humans (LSP)
let human = format_human(&data)?;
```

## DX Syntax Cheatsheet

### Basic Key-Value
```dx
name:Alice
age:30
score:95.5
```

### Booleans
```dx
active:+      # true
disabled:-    # false
deleted:~     # null
admin!        # implicit true
error?        # implicit null
```

### Arrays (Stream)
```dx
tags>rust|wasm|performance
team>alice|bob|charlie
```

### Tables (Schema-Defined)
```dx
users=id%i name%s score%f active%b
1 Alice 95.5 +
2 Bob 87.3 -
3 Charlie 92.0 +
```

### Type Hints
- `%i` - Integer
- `%s` - String (vacuum parsing, no quotes needed)
- `%f` - Float
- `%b` - Boolean

### Vertical Compression (Ditto)
```dx
logs=time%i event%s user%s
1000 login alice
1001 login bob
1002 logout alice
_ _ bob          # Copies time and event from above
```

### Aliases
```dx
$c=config
$c.db.host:localhost
$c.db.port:5432
```

### Prefix Inheritance
```dx
app.name:My App
^version:1.0     # Becomes app.version
^author:Team     # Becomes app.author
```

## API Reference

### Parse Functions
```rust
// Parse from bytes
parse(bytes: &[u8]) -> Result<DxValue>

// Parse from string
parse_str(input: &str) -> Result<DxValue>

// Stream parsing
parse_stream<R: Read>(reader: R) -> Result<DxValue>
```

### Encode Functions
```rust
// Encode to bytes
encode(value: &DxValue) -> Result<Vec<u8>>

// Encode to writer
encode_to_writer<W: Write>(value: &DxValue, writer: &mut W) -> Result<()>
```

### Format Functions
```rust
// Default human format
format_human(value: &DxValue) -> Result<String>

// Custom config
let config = FormatterConfig {
    column_padding: 4,
    use_unicode: true,
    add_dividers: true,
    use_colors: false,
};
format_human_with_config(value, config) -> Result<String>
```

## Performance Tips

1. **Use Type Hints:** Enable zero-copy parsing
2. **Use Ditto:** Compress repetitive data
3. **Use Aliases:** Shorten long key names
4. **Use Stream Arrays:** More compact than vertical arrays
5. **Use Sigils:** `+`/`-`/`~` instead of words

## Common Patterns

### Configuration Files
```dx
app.name:My Application
app.debug!
database.host:localhost
database.port:5432
cache.enabled:+
features>auth|payments|analytics
```

### Log Data
```dx
logs=ts%i level%s msg%s code%i
1000 info Started 200
1001 info Request 200
1002 error Failed 500
_ warn Retry 503
```

### API Responses
```dx
status:+
code:200
data.user.id:12345
data.user.name:Alice
data.items>item1|item2|item3
```

## LSP Integration Example

```rust
// In your LSP server
fn format_dx_file(content: &str) -> Result<String> {
    let parsed = parse_str(content)?;
    format_human(&parsed)
}

// Display formatted view in IDE while keeping original DX on disk
```

## Benchmarks

Run from `playground/` directory:

```bash
# Size comparison
cargo run --bin size-comparison --release

# Speed comparison
cargo run --bin speed-comparison --release
```

## Results Summary

- **63.9% smaller** than TOON (complex data)
- **60.9% overall** efficiency gain
- **3-4x faster** parsing than traditional formats
- **~1.9µs** parse time per operation
- **~50µs** human format time (LSP-ready)

## Links

- [Full Specification](../integrations/dx.md)
- [Benchmark Results](results/BENCHMARK_RESULTS.md)
- [Implementation Summary](IMPLEMENTATION_SUMMARY.md)
- [Examples](examples/)
