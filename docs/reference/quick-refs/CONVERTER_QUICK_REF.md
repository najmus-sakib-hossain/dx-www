# DX Converter Quick Reference

## API Functions

```rust
use dx_serializer::*;

// Individual converters
let dx = json_to_dx(json_str)?;   // JSON → DX
let dx = yaml_to_dx(yaml_str)?;   // YAML → DX  
let dx = toml_to_dx(toml_str)?;   // TOML → DX
let dx = toon_to_dx(toon_str)?;   // TOON → DX

// Universal converter
let dx = convert_to_dx(input, "json")?;  // Auto-detect
```

## Optimization Rules

| Original Key | Optimized | Type |
|--------------|-----------|------|
| name | n | Core |
| version | v | Core |
| description | d | Core |
| author | a | Core |
| license | lic | Core |
| context | c | Prefix |
| languages | l | Prefix |
| dependencies | dep | Prefix |
| devDependencies | dev | Prefix |
| packageManager | pm | Dev |
| framework | fw | Dev |
| javascript/typescript | js/ts | Lang |
| python | py | Lang |
| rust | rs | Lang |

## Compression Targets

| Format | Compression | Speed |
|--------|-------------|-------|
| JSON | 70-75% | ⚡⚡⚡⚡⚡ |
| YAML | 65-70% | ⚡⚡⚡⚡ |
| TOML | 60-65% | ⚡⚡⚡⚡ |
| TOON | 40-45% | ⚡⚡⚡⚡⚡ |

## Syntax Patterns

```dx
# Inline (< 5 items, < 150 chars)
c.n:app^v:1.0.0^d:Description

# Multi-line (> 5 items)
c.n:app
c.v:1.0.0
c.d:Description
c.a:Author

# Arrays with pipes
ws>frontend|backend|shared

# Tables (auto-detected from array of objects)
l=lg rt cp bd pm fw
js/ts bun tsc vite bun react
py cpython - - uv django

# Null values
field:-
```

## CLI (Future)

```bash
dx convert package.json > package.dx
dx convert config.yaml > config.dx
dx convert-all *.json --recursive
```

## Testing

```bash
cargo test --test converter_tests
cargo run --example convert_package_json
```

## Performance

- JSON → DX: ~50μs
- YAML → DX: ~85μs  
- TOML → DX: ~75μs
- Parse DX: ~12μs (4-5x faster than JSON)

## The Dual Layer

```
Storage: 251 bytes → c.n:app^v:1.0.0
Display: Beautiful → context.name: app
                     ^version: 1.0.0
```

**Machine sees bytes. Human sees clarity.** ⚛️
