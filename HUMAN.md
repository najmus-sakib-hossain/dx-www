# DX Serializer Human Format V2

## Overview

Human Format V2 is a clean, readable configuration format that provides:
- **Flat TOML-like structure** without YAML indentation
- **Full key name expansion** (no cryptic abbreviations)
- **Full section names** in brackets (`[forge]` instead of `[f]`)
- **Comma-separated arrays** without brackets
- **Unicode box-drawing tables** without indentation
- **Automatic cache generation** with path preservation

## VS Code / Kiro Extension

The DX Serializer VS Code extension provides seamless editing of `.dx` files with Human Format V2:

- **Install**: `kiro --install-extension crates/vscode-dx-serializer/vscode-dx-serializer-0.1.0.vsix`
- **Features**: Syntax highlighting, real-time validation, auto-save compatible
- **Dual Format**: Edit human-readable V2 format while storing token-efficient LLM format on disk

See **[Extension README](crates/vscode-dx-serializer/README.md)** for full documentation.

## Format Specification

### Config Section

```toml
# ════════════════════════════════════════════════════════════════════════════════
#                                  CONFIGURATION
# ════════════════════════════════════════════════════════════════════════════════

[config]
name        = "MyProject"
version     = "1.0.0"
title       = "Enhanced Developing Experience"
description = "Orchestrate don't just own your code"
author      = essensefromexistence
workspace   = frontend/www, frontend/mobile
editors     = vscode, vim, gitpod, github-codespace, replit
```

### Data Sections

```toml
# ════════════════════════════════════════════════════════════════════════════════
#                                      FORGE
# ════════════════════════════════════════════════════════════════════════════════

[forge]
┌───────┬───────────────────────────────────────────────┬───────────┬───────┐
│ name  │                     repo                      │ container │ ci_cd │
├───────┼───────────────────────────────────────────────┼───────────┼───────┤
│ forge │ https://dx.vercel.app/essensefromexistence/dx │ none      │ none  │
└───────┴───────────────────────────────────────────────┴───────────┴───────┘

Total: 1 rows
```

## Key Features

### 1. Full Key Name Expansion

| Abbreviated | Full Name   |
|-------------|-------------|
| `nm`        | `name`      |
| `v`         | `version`   |
| `au`        | `author`    |
| `ws`        | `workspace` |
| `eds`       | `editors`   |
| `repo`      | `repository`|
| `cont`      | `container` |
| `ci`        | `ci_cd`     |

### 2. Flat Structure (No Indentation)

```toml
# ✅ Human Format V2 (flat)
[config]
name    = "MyProject"
version = "1.0.0"

# ❌ Old format (indented)
[config]
    name    = "MyProject"
    version = "1.0.0"
```

### 3. Comma-Separated Arrays

```toml
# ✅ Human Format V2 (comma-separated)
workspace = frontend/www, frontend/mobile, backend/api

# ❌ Old format (bracketed)
workspace = [frontend/www, frontend/mobile, backend/api]
```

### 4. Full Section Names

```toml
# ✅ Human Format V2 (full names)
[forge]
[users]
[data]

# ❌ Old format (single letters)
[f]
[u]
[d]
```

### 5. Unicode Box-Drawing Tables

Tables use Unicode box-drawing characters for clean borders:
- `┌` `┐` `└` `┘` - Corners
- `─` `│` - Lines
- `┬` `┴` `├` `┤` `┼` - Junctions

### 6. Dynamic Column Sizing

Tables automatically adjust column widths based on content:
- Minimum column width: 5 characters
- Maximum line width: 120 characters (configurable)
- Wide content wraps to multiple lines

## API Usage

### Basic Formatting

```rust
use serializer::llm::{
    document_to_human_v2, llm_to_human_v2,
    DxDocument, DxLlmValue, DxSection,
};

// Create a document
let mut doc = DxDocument::new();
doc.context.insert("nm".to_string(), DxLlmValue::Str("MyProject".to_string()));
doc.context.insert("v".to_string(), DxLlmValue::Str("1.0.0".to_string()));

// Format to Human V2
let human_v2 = document_to_human_v2(&doc);
println!("{}", human_v2);
```

### Pretty Printer with Validation

```rust
use serializer::llm::{PrettyPrinter, DxDocument};

let printer = PrettyPrinter::new();
let doc = DxDocument::new();

// Format with validation (ensures output is parseable)
match printer.format(&doc) {
    Ok(output) => println!("{}", output),
    Err(e) => eprintln!("Validation failed: {}", e),
}
```

### Round-Trip Conversion

```rust
use serializer::llm::{llm_to_human_v2, human_to_llm_v2};

// LLM → Human V2
let llm = "#c:nm|Test;ct|42";
let human_v2 = llm_to_human_v2(llm).unwrap();

// Human V2 → LLM
let back_to_llm = human_to_llm_v2(&human_v2).unwrap();
```

### Cache Generation

```rust
use serializer::llm::{CacheGenerator, CacheConfig};
use std::path::PathBuf;

let config = CacheConfig::new(PathBuf::from(".dx/cache"))
    .with_llm(true)
    .with_machine(true);

let generator = CacheGenerator::new(config);

// Generate cache files (preserves subfolder structure)
let result = generator.generate(
    &PathBuf::from("config/app.dx"),
    &doc,
).unwrap();

// Result:
// - .dx/cache/config/app.llm
// - .dx/cache/config/app.machine
```

## Conversion Functions

| Function | Description |
|----------|-------------|
| `llm_to_human_v2()` | Convert LLM format to Human V2 |
| `human_to_llm_v2()` | Convert Human V2 to LLM format |
| `document_to_human_v2()` | Format DxDocument to Human V2 |
| `human_v2_to_document()` | Parse Human V2 to DxDocument |
| `human_v2_to_machine()` | Convert Human V2 to Machine format |
| `machine_to_human_v2()` | Convert Machine format to Human V2 |

## Configuration Options

```rust
use serializer::llm::HumanFormatV2Config;

let config = HumanFormatV2Config {
    max_line_width: 120,    // Maximum line width before wrapping
    expand_keys: true,      // Expand abbreviated keys to full names
    show_summaries: true,   // Show "Total: N rows" footer
    show_references: true,  // Show reference comments
};
```

## Testing

All 143 LLM tests pass, including:
- 12 PrettyPrinter tests (4 property tests)
- Round-trip consistency tests
- Unicode box-drawing validation
- Dynamic column sizing tests

Run tests:
```bash
cargo test --package serializer --lib llm::
```

## Files

| File | Description |
|------|-------------|
| `human_formatter_v2.rs` | V2 formatter with flat structure |
| `pretty_printer.rs` | Validated formatter with round-trip checking |
| `table_wrapper.rs` | Wide table wrapping support |
| `cache_generator.rs` | Automatic cache file generation |
| `convert.rs` | Format conversion functions |

## Example Output

```toml
# ════════════════════════════════════════════════════════════════════════════════
#                                  CONFIGURATION
# ════════════════════════════════════════════════════════════════════════════════

[config]
name        = "MyProject"
version     = "1.0.0"
author      = essensefromexistence
workspace   = frontend/www, frontend/mobile

# ════════════════════════════════════════════════════════════════════════════════
#                                      FORGE
# ════════════════════════════════════════════════════════════════════════════════

[forge]
┌───────┬───────────────────────────────────────────────┬───────────┬───────┐
│ name  │                     repo                      │ container │ ci_cd │
├───────┼───────────────────────────────────────────────┼───────────┼───────┤
│ forge │ https://dx.vercel.app/essensefromexistence/dx │ none      │ none  │
└───────┴───────────────────────────────────────────────┴───────────┴───────┘

Total: 1 rows

# ════════════════════════════════════════════════════════════════════════════════
#                                      USERS
# ════════════════════════════════════════════════════════════════════════════════

[users]
┌─────┬───────┬───────────┐
│ id  │ name  │   role    │
├─────┼───────┼───────────┤
│   1 │ Alice │ admin     │
│   2 │ Bob   │ developer │
└─────┴───────┴───────────┘

Total: 2 rows
```













































Good, but from dx serializer human verion please remove the comment and row details and just give a black line and you know what remove that table wrapper box and just indent it correctly so it look simple and beautiful!!!
