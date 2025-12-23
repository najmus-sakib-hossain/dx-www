# DX Serializer Human Format Specification

> **Version:** 1.0.0 | **Purpose:** Human-readable view of DX LLM format | **Extension:** `.dx` (same file, dual view)

## Overview

DX Serializer supports **dual-mode rendering**: the same `.dx` file can be displayed as either:
- **LLM Mode:** Ultra-compact for token efficiency (3x better than TOON)
- **Human Mode:** Beautiful TOML-like formatting with tables, indentation, and full words

The file is **stored in LLM format** but **displayed in Human format** in editors.

---

## Format Comparison

### LLM Format (Storage)

```dx
#c:t|Our favorite hikes;l|^B;s|sp25
#:B|Boulder
#f:ana|luis|sam
#h(id|nm|km|el|w|s)
1|Blue Lake Trail|7.5|320|ana|+
2|Ridge Overlook|9.2|540|luis|-
3|Wildflower Loop|5.1|180|sam|+
```

### Human Format (Display)

```toml
# ═══════════════════════════════════════════════════════════════════════════════
#                                   CONFIG
# ═══════════════════════════════════════════════════════════════════════════════

[config]
    task     = "Our favorite hikes"
    location = "Boulder"
    season   = "spring 2025"

# ═══════════════════════════════════════════════════════════════════════════════
#                                 REFERENCES
# ═══════════════════════════════════════════════════════════════════════════════

[references]
    B = "Boulder"

# ═══════════════════════════════════════════════════════════════════════════════
#                                  FRIENDS
# ═══════════════════════════════════════════════════════════════════════════════

[friends]
    members = ["ana", "luis", "sam"]

# ═══════════════════════════════════════════════════════════════════════════════
#                                   HIKES
# ═══════════════════════════════════════════════════════════════════════════════

[hikes]
    # Schema: id | name | kilometers | elevation | with | sunny

    ┌────┬───────────────────┬──────┬───────────┬──────┬───────┐
    │ ID │ Name              │  Km  │ Elevation │ With │ Sunny │
    ├────┼───────────────────┼──────┼───────────┼──────┼───────┤
    │  1 │ Blue Lake Trail   │  7.5 │       320 │ ana  │  ✓    │
    │  2 │ Ridge Overlook    │  9.2 │       540 │ luis │  ✗    │
    │  3 │ Wildflower Loop   │  5.1 │       180 │ sam  │  ✓    │
    └────┴───────────────────┴──────┴───────────┴──────┴───────┘

    Total: 3 hikes | Distance: 21.8 km | Sunny days: 2/3
```

---

## Sigil Expansion Table

| LLM Sigil | Human Expansion | Description |
|-----------|-----------------|-------------|
| `#c` | `[config]` | Configuration/context section |
| `#:` | `[references]` | Reference definitions |
| `#a` | `[a]` or `[accounts]` | Custom section (auto-detected) |
| `#b` | `[b]` or `[bookings]` | Custom section (auto-detected) |
| `#d` | `[d]` or `[data]` | Custom section (auto-detected) |
| `#e` | `[e]` or `[events]` | Custom section (auto-detected) |
| `#f` | `[f]` or `[friends]` | Custom section (auto-detected) |
| `#h` | `[h]` or `[hikes]` | Custom section (auto-detected) |
| `#i` | `[i]` or `[items]` | Custom section (auto-detected) |
| `#o` | `[o]` or `[orders]` | Custom section (auto-detected) |
| `#p` | `[p]` or `[products]` | Custom section (auto-detected) |
| `#u` | `[u]` or `[users]` | Custom section (auto-detected) |

---

## Key Abbreviation Dictionary

### Standard Abbreviations (Auto-Expand)

| Abbrev | Full Name | Category |
|--------|-----------|----------|
| `id` | id | Identity |
| `nm` | name | Identity |
| `tt` | title | Identity |
| `ds` | description | Identity |
| `t` | type | Classification |
| `v` | value | Data |
| `st` | status | State |
| `s` | sunny / state | Context-dependent |
| `ac` | active | State |
| `en` | enabled | State |
| `cr` | created | Timestamps |
| `up` | updated | Timestamps |
| `dl` | deleted | Timestamps |
| `dt` | date | Timestamps |
| `tm` | time | Timestamps |
| `ts` | timestamp | Timestamps |
| `ct` | count | Metrics |
| `tl` | total | Metrics |
| `am` | amount | Metrics |
| `pr` | price | Metrics |
| `qt` | quantity | Metrics |
| `km` | kilometers | Metrics |
| `el` | elevation | Metrics |
| `w` | with / width | Context-dependent |
| `h` | height | Dimensions |
| `sz` | size | Dimensions |
| `ur` | url | Web |
| `pt` | path | Web |
| `em` | email | Contact |
| `ph` | phone | Contact |
| `ad` | address | Location |
| `cy` | city | Location |
| `co` | country | Location |
| `l` | location | Location |
| `la` | latitude | Geo |
| `lo` | longitude | Geo |
| `cl` | color | Visual |
| `im` | image | Visual |
| `pa` | parent | Relations |
| `ch` | children | Relations |
| `us` | user | Relations |
| `ow` | owner | Relations |
| `au` | author | Relations |
| `ca` | category | Classification |
| `tg` | tags | Classification |
| `sk` | sku | Commerce |
| `cu` | customer | Commerce |
| `sh` | shipping | Commerce |
| `pd` | paid | Commerce |

### Context-Aware Expansion

The human formatter uses **context detection** to expand ambiguous abbreviations:

```rust
fn expand_key(abbrev: &str, section_hint: &str) -> &'static str {
    match (abbrev, section_hint) {
        ("s", "hikes") => "sunny",
        ("s", "config") => "season",
        ("s", "orders") => "status",
        ("w", "hikes") => "with",
        ("w", "images") => "width",
        ("w", "products") => "weight",
        ("t", "config") => "task",
        ("t", "products") => "type",
        ("l", _) => "location",
        _ => abbrev,
    }
}
```

---

## Human Format Rendering Rules

### 1. Section Headers

```
# ═══════════════════════════════════════════════════════════════════════════════
#                                   SECTION NAME
# ═══════════════════════════════════════════════════════════════════════════════
```

- 80 characters wide
- Centered title in UPPERCASE
- Double-line box drawing characters (`═`)

### 2. Key-Value Pairs (Config)

```toml
[config]
    key      = "value"
    long_key = "another value"
```

- 4-space indentation
- Keys right-padded for alignment
- Values in quotes for strings

### 3. Arrays (Inline)

```toml
[friends]
    members = ["ana", "luis", "sam"]
```

### 4. Tables (Data Sections)

```
┌────┬───────────────────┬──────┬───────────┐
│ ID │ Name              │  Km  │ Elevation │
├────┼───────────────────┼──────┼───────────┤
│  1 │ Blue Lake Trail   │  7.5 │       320 │
│  2 │ Ridge Overlook    │  9.2 │       540 │
└────┴───────────────────┴──────┴───────────┘
```

- Box drawing characters for borders
- Column width auto-calculated from content
- Numbers right-aligned, strings left-aligned
- Booleans displayed as `✓` / `✗`

### 5. Boolean Display

| LLM | Human Table | Human Inline |
|-----|-------------|--------------|
| `+` | `✓` | `true` |
| `-` | `✗` | `false` |

### 6. Reference Resolution

References are **resolved inline** in human mode:

**LLM:**
```dx
#c:l|^B
#:B|Boulder
```

**Human:**
```toml
[config]
    location = "Boulder"  # ref: B
```

### 7. Summary Footer

Tables include auto-generated summaries:

```
Total: 3 hikes | Distance: 21.8 km | Sunny days: 2/3
```

---

## Complete Conversion Example

### E-Commerce Order

#### LLM Format (26 tokens)

```dx
#c:id|ORD-2025-001;cu|John Doe;sh|^B;st|proc;pd|+
#:B|Boulder, CO
#i(sk|nm|qt|pr)
WIDGET-A|Premium Widget|2|29.99
GADGET-B|Super Gadget|1|149.99
```

#### Human Format (Beautiful Display)

```toml
# ═══════════════════════════════════════════════════════════════════════════════
#                                    ORDER
# ═══════════════════════════════════════════════════════════════════════════════

[config]
    order_id = "ORD-2025-001"
    customer = "John Doe"
    shipping = "Boulder, CO"
    status   = "processing"
    paid     = true

# ═══════════════════════════════════════════════════════════════════════════════
#                                    ITEMS
# ═══════════════════════════════════════════════════════════════════════════════

[items]
    # Schema: sku | name | quantity | price

    ┌───────────┬─────────────────┬──────────┬─────────┐
    │ SKU       │ Name            │ Quantity │   Price │
    ├───────────┼─────────────────┼──────────┼─────────┤
    │ WIDGET-A  │ Premium Widget  │        2 │  $29.99 │
    │ GADGET-B  │ Super Gadget    │        1 │ $149.99 │
    └───────────┴─────────────────┴──────────┴─────────┘

    Subtotal: $209.97 | Items: 3 units | SKUs: 2
```

---

## Rust Implementation

### Core Types

```rust
use std::collections::HashMap;

/// Human format configuration
#[derive(Debug, Clone)]
pub struct HumanFormatConfig {
    pub table_style: TableStyle,
    pub indent_size: usize,
    pub max_width: usize,
    pub show_references: bool,
    pub show_summaries: bool,
    pub expand_abbreviations: bool,
}

impl Default for HumanFormatConfig {
    fn default() -> Self {
        Self {
            table_style: TableStyle::Unicode,
            indent_size: 4,
            max_width: 80,
            show_references: true,
            show_summaries: true,
            expand_abbreviations: true,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TableStyle {
    Unicode,   // ┌─┬─┐ │ ├─┼─┤ └─┴─┘
    Ascii,     // +-+-+ | +-+-+ +-+-+
    Markdown,  // | --- | --- |
    Minimal,   // No borders
}

/// Abbreviation dictionary for key expansion
pub struct AbbrevDict {
    global: HashMap<&'static str, &'static str>,
    contextual: HashMap<(&'static str, &'static str), &'static str>,
}

impl AbbrevDict {
    pub fn new() -> Self {
        let mut global = HashMap::new();
        
        // Identity
        global.insert("nm", "name");
        global.insert("tt", "title");
        global.insert("ds", "description");
        
        // State
        global.insert("st", "status");
        global.insert("ac", "active");
        global.insert("en", "enabled");
        
        // Timestamps
        global.insert("cr", "created");
        global.insert("up", "updated");
        global.insert("dl", "deleted");
        global.insert("dt", "date");
        global.insert("tm", "time");
        global.insert("ts", "timestamp");
        
        // Metrics
        global.insert("ct", "count");
        global.insert("tl", "total");
        global.insert("am", "amount");
        global.insert("pr", "price");
        global.insert("qt", "quantity");
        global.insert("km", "kilometers");
        global.insert("el", "elevation");
        
        // Dimensions
        global.insert("sz", "size");
        
        // Web
        global.insert("ur", "url");
        global.insert("pt", "path");
        
        // Contact
        global.insert("em", "email");
        global.insert("ph", "phone");
        global.insert("ad", "address");
        
        // Location
        global.insert("cy", "city");
        global.insert("co", "country");
        global.insert("la", "latitude");
        global.insert("lo", "longitude");
        
        // Visual
        global.insert("cl", "color");
        global.insert("im", "image");
        
        // Relations
        global.insert("pa", "parent");
        global.insert("ch", "children");
        global.insert("us", "user");
        global.insert("ow", "owner");
        global.insert("au", "author");
        
        // Classification
        global.insert("ca", "category");
        global.insert("tg", "tags");
        
        // Commerce
        global.insert("sk", "sku");
        global.insert("cu", "customer");
        global.insert("sh", "shipping");
        global.insert("pd", "paid");
        
        let mut contextual = HashMap::new();
        
        // Context-specific expansions
        contextual.insert(("s", "hikes"), "sunny");
        contextual.insert(("s", "config"), "season");
        contextual.insert(("s", "orders"), "status");
        contextual.insert(("w", "hikes"), "with");
        contextual.insert(("w", "images"), "width");
        contextual.insert(("w", "products"), "weight");
        contextual.insert(("t", "config"), "task");
        contextual.insert(("t", "products"), "type");
        contextual.insert(("t", "events"), "time");
        contextual.insert(("l", "config"), "location");
        contextual.insert(("l", "products"), "label");
        contextual.insert(("v", "config"), "version");
        contextual.insert(("v", "products"), "value");
        contextual.insert(("h", "images"), "height");
        contextual.insert(("h", "hikes"), "hikes");
        
        Self { global, contextual }
    }
    
    pub fn expand(&self, abbrev: &str, context: &str) -> String {
        // Try contextual first
        if let Some(&expanded) = self.contextual.get(&(abbrev, context)) {
            return expanded.to_string();
        }
        
        // Then global
        if let Some(&expanded) = self.global.get(abbrev) {
            return expanded.to_string();
        }
        
        // Return original if no expansion found
        abbrev.to_string()
    }
}
```

### LLM to Human Converter

```rust
use crate::{DxDocument, DxSection, DxValue, AbbrevDict, HumanFormatConfig, TableStyle};

pub struct HumanFormatter {
    config: HumanFormatConfig,
    abbrev: AbbrevDict,
}

impl HumanFormatter {
    pub fn new(config: HumanFormatConfig) -> Self {
        Self {
            config,
            abbrev: AbbrevDict::new(),
        }
    }
    
    pub fn format(&self, doc: &DxDocument) -> String {
        let mut output = String::new();
        
        // Format config section
        if !doc.context.is_empty() {
            output.push_str(&self.format_section_header("CONFIG"));
            output.push_str(&self.format_config(&doc.context, &doc.refs));
            output.push('\n');
        }
        
        // Format references (if show_references is enabled)
        if self.config.show_references && !doc.refs.is_empty() {
            output.push_str(&self.format_section_header("REFERENCES"));
            output.push_str(&self.format_references(&doc.refs));
            output.push('\n');
        }
        
        // Format data sections
        for (id, section) in &doc.sections {
            let section_name = self.get_section_name(*id, section);
            output.push_str(&self.format_section_header(&section_name.to_uppercase()));
            output.push_str(&self.format_data_section(*id, section, &doc.refs));
            output.push('\n');
        }
        
        output
    }
    
    fn format_section_header(&self, title: &str) -> String {
        let width = self.config.max_width;
        let line = "═".repeat(width - 2);
        let padding = (width - 2 - title.len()) / 2;
        let title_line = format!(
            "#{}{}{}",
            " ".repeat(padding),
            title,
            " ".repeat(width - 2 - padding - title.len())
        );
        
        format!(
            "# {}\n#{}#\n# {}\n\n",
            line,
            title_line,
            line
        )
    }
    
    fn format_config(&self, context: &HashMap<String, DxValue>, refs: &HashMap<String, String>) -> String {
        let mut output = String::new();
        let indent = " ".repeat(self.config.indent_size);
        
        output.push_str("[config]\n");
        
        // Calculate max key length for alignment
        let max_key_len = context.keys()
            .map(|k| self.abbrev.expand(k, "config").len())
            .max()
            .unwrap_or(0);
        
        for (key, value) in context {
            let expanded_key = self.abbrev.expand(key, "config");
            let padding = max_key_len - expanded_key.len();
            let formatted_value = self.format_value(value, refs);
            
            output.push_str(&format!(
                "{}{}{} = {}\n",
                indent,
                expanded_key,
                " ".repeat(padding),
                formatted_value
            ));
        }
        
        output
    }
    
    fn format_references(&self, refs: &HashMap<String, String>) -> String {
        let mut output = String::new();
        let indent = " ".repeat(self.config.indent_size);
        
        output.push_str("[references]\n");
        
        let max_key_len = refs.keys().map(|k| k.len()).max().unwrap_or(0);
        
        for (key, value) in refs {
            let padding = max_key_len - key.len();
            output.push_str(&format!(
                "{}{}{} = \"{}\"\n",
                indent,
                key,
                " ".repeat(padding),
                value
            ));
        }
        
        output
    }
    
    fn format_data_section(&self, id: char, section: &DxSection, refs: &HashMap<String, String>) -> String {
        let mut output = String::new();
        let section_name = self.get_section_name(id, section);
        let indent = " ".repeat(self.config.indent_size);
        
        output.push_str(&format!("[{}]\n", section_name));
        
        // Schema comment
        let expanded_schema: Vec<String> = section.schema.iter()
            .map(|s| self.abbrev.expand(s, &section_name))
            .collect();
        output.push_str(&format!("{}# Schema: {}\n\n", indent, expanded_schema.join(" | ")));
        
        // Build table
        output.push_str(&self.build_table(section, &section_name, refs));
        
        // Summary
        if self.config.show_summaries && !section.rows.is_empty() {
            output.push_str(&format!("\n{}{}\n", indent, self.generate_summary(section, &section_name)));
        }
        
        output
    }
    
    fn build_table(&self, section: &DxSection, context: &str, refs: &HashMap<String, String>) -> String {
        let indent = " ".repeat(self.config.indent_size);
        
        // Expand headers
        let headers: Vec<String> = section.schema.iter()
            .map(|s| self.abbrev.expand(s, context))
            .map(|s| capitalize(&s))
            .collect();
        
        // Calculate column widths
        let mut col_widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();
        
        for row in &section.rows {
            for (i, val) in row.iter().enumerate() {
                if i < col_widths.len() {
                    let formatted = self.format_cell_value(val, refs);
                    col_widths[i] = col_widths[i].max(formatted.len());
                }
            }
        }
        
        // Add padding
        for w in &mut col_widths {
            *w += 2;
        }
        
        let mut output = String::new();
        
        match self.config.table_style {
            TableStyle::Unicode => {
                // Top border
                output.push_str(&indent);
                output.push('┌');
                for (i, &w) in col_widths.iter().enumerate() {
                    output.push_str(&"─".repeat(w));
                    if i < col_widths.len() - 1 {
                        output.push('┬');
                    }
                }
                output.push_str("┐\n");
                
                // Header row
                output.push_str(&indent);
                output.push('│');
                for (i, header) in headers.iter().enumerate() {
                    let w = col_widths[i];
                    output.push_str(&format!("{:^width$}", header, width = w));
                    output.push('│');
                }
                output.push('\n');
                
                // Header separator
                output.push_str(&indent);
                output.push('├');
                for (i, &w) in col_widths.iter().enumerate() {
                    output.push_str(&"─".repeat(w));
                    if i < col_widths.len() - 1 {
                        output.push('┼');
                    }
                }
                output.push_str("┤\n");
                
                // Data rows
                for row in &section.rows {
                    output.push_str(&indent);
                    output.push('│');
                    for (i, val) in row.iter().enumerate() {
                        if i < col_widths.len() {
                            let w = col_widths[i];
                            let formatted = self.format_cell_value(val, refs);
                            
                            // Right-align numbers, center booleans, left-align strings
                            let cell = match val {
                                DxValue::Num(_) => format!("{:>width$}", formatted, width = w),
                                DxValue::Bool(_) => format!("{:^width$}", formatted, width = w),
                                _ => format!(" {:<width$}", formatted, width = w - 1),
                            };
                            output.push_str(&cell);
                            output.push('│');
                        }
                    }
                    output.push('\n');
                }
                
                // Bottom border
                output.push_str(&indent);
                output.push('└');
                for (i, &w) in col_widths.iter().enumerate() {
                    output.push_str(&"─".repeat(w));
                    if i < col_widths.len() - 1 {
                        output.push('┴');
                    }
                }
                output.push_str("┘\n");
            }
            
            TableStyle::Markdown => {
                // Header
                output.push_str(&indent);
                output.push('|');
                for (i, header) in headers.iter().enumerate() {
                    output.push_str(&format!(" {} |", header));
                }
                output.push('\n');
                
                // Separator
                output.push_str(&indent);
                output.push('|');
                for &w in &col_widths {
                    output.push_str(&format!(" {} |", "-".repeat(w - 2)));
                }
                output.push('\n');
                
                // Rows
                for row in &section.rows {
                    output.push_str(&indent);
                    output.push('|');
                    for (i, val) in row.iter().enumerate() {
                        if i < col_widths.len() {
                            let formatted = self.format_cell_value(val, refs);
                            output.push_str(&format!(" {} |", formatted));
                        }
                    }
                    output.push('\n');
                }
            }
            
            TableStyle::Ascii => {
                // Similar to Unicode but with ASCII characters
                // +-+-+ instead of ┌─┬─┐
                output.push_str(&self.build_ascii_table(section, &headers, &col_widths, refs));
            }
            
            TableStyle::Minimal => {
                // Just aligned columns with spaces
                output.push_str(&self.build_minimal_table(section, &headers, &col_widths, refs));
            }
        }
        
        output
    }
    
    fn build_ascii_table(&self, section: &DxSection, headers: &[String], col_widths: &[usize], refs: &HashMap<String, String>) -> String {
        let indent = " ".repeat(self.config.indent_size);
        let mut output = String::new();
        
        // Top border
        output.push_str(&indent);
        output.push('+');
        for &w in col_widths {
            output.push_str(&"-".repeat(w));
            output.push('+');
        }
        output.push('\n');
        
        // Header
        output.push_str(&indent);
        output.push('|');
        for (i, header) in headers.iter().enumerate() {
            output.push_str(&format!("{:^width$}", header, width = col_widths[i]));
            output.push('|');
        }
        output.push('\n');
        
        // Separator
        output.push_str(&indent);
        output.push('+');
        for &w in col_widths {
            output.push_str(&"-".repeat(w));
            output.push('+');
        }
        output.push('\n');
        
        // Rows
        for row in &section.rows {
            output.push_str(&indent);
            output.push('|');
            for (i, val) in row.iter().enumerate() {
                if i < col_widths.len() {
                    let formatted = self.format_cell_value(val, refs);
                    output.push_str(&format!("{:^width$}", formatted, width = col_widths[i]));
                    output.push('|');
                }
            }
            output.push('\n');
        }
        
        // Bottom border
        output.push_str(&indent);
        output.push('+');
        for &w in col_widths {
            output.push_str(&"-".repeat(w));
            output.push('+');
        }
        output.push('\n');
        
        output
    }
    
    fn build_minimal_table(&self, section: &DxSection, headers: &[String], col_widths: &[usize], refs: &HashMap<String, String>) -> String {
        let indent = " ".repeat(self.config.indent_size);
        let mut output = String::new();
        
        // Header
        output.push_str(&indent);
        for (i, header) in headers.iter().enumerate() {
            output.push_str(&format!("{:width$}  ", header, width = col_widths[i]));
        }
        output.push('\n');
        
        // Separator
        output.push_str(&indent);
        for &w in col_widths {
            output.push_str(&"-".repeat(w));
            output.push_str("  ");
        }
        output.push('\n');
        
        // Rows
        for row in &section.rows {
            output.push_str(&indent);
            for (i, val) in row.iter().enumerate() {
                if i < col_widths.len() {
                    let formatted = self.format_cell_value(val, refs);
                    output.push_str(&format!("{:width$}  ", formatted, width = col_widths[i]));
                }
            }
            output.push('\n');
        }
        
        output
    }
    
    fn format_value(&self, value: &DxValue, refs: &HashMap<String, String>) -> String {
        match value {
            DxValue::Str(s) => format!("\"{}\"", s),
            DxValue::Num(n) => {
                if n.fract() == 0.0 {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            }
            DxValue::Bool(b) => if *b { "true".to_string() } else { "false".to_string() },
            DxValue::Null => "null".to_string(),
            DxValue::Arr(items) => {
                let formatted: Vec<String> = items.iter()
                    .map(|v| self.format_value(v, refs))
                    .collect();
                format!("[{}]", formatted.join(", "))
            }
            DxValue::Ref(key) => {
                if let Some(resolved) = refs.get(key) {
                    format!("\"{}\"", resolved)
                } else {
                    format!("^{}", key)
                }
            }
        }
    }
    
    fn format_cell_value(&self, value: &DxValue, refs: &HashMap<String, String>) -> String {
        match value {
            DxValue::Str(s) => s.clone(),
            DxValue::Num(n) => {
                if n.fract() == 0.0 {
                    format!("{}", *n as i64)
                } else {
                    format!("{:.2}", n)
                }
            }
            DxValue::Bool(b) => if *b { "✓".to_string() } else { "✗".to_string() },
            DxValue::Null => "—".to_string(),
            DxValue::Arr(items) => {
                let formatted: Vec<String> = items.iter()
                    .map(|v| self.format_cell_value(v, refs))
                    .collect();
                formatted.join(", ")
            }
            DxValue::Ref(key) => {
                if let Some(resolved) = refs.get(key) {
                    resolved.clone()
                } else {
                    format!("^{}", key)
                }
            }
        }
    }
    
    fn get_section_name(&self, id: char, section: &DxSection) -> String {
        // Common section name mappings
        match id {
            'a' => "accounts",
            'b' => "bookings",
            'c' => "config",
            'd' => "data",
            'e' => "events",
            'f' => "friends",
            'h' => "hikes",
            'i' => "items",
            'j' => "jobs",
            'l' => "locations",
            'm' => "messages",
            'n' => "notes",
            'o' => "orders",
            'p' => "products",
            'q' => "queries",
            'r' => "records",
            's' => "settings",
            't' => "tasks",
            'u' => "users",
            'v' => "values",
            'w' => "widgets",
            _ => return id.to_string(),
        }.to_string()
    }
    
    fn generate_summary(&self, section: &DxSection, context: &str) -> String {
        let row_count = section.rows.len();
        let mut parts = vec![format!("Total: {} {}", row_count, context)];
        
        // Try to find numeric columns for sums
        for (i, key) in section.schema.iter().enumerate() {
            let expanded = self.abbrev.expand(key, context);
            
            // Sum numeric columns
            if matches!(expanded.as_str(), "kilometers" | "price" | "amount" | "quantity" | "count" | "elevation") {
                let sum: f64 = section.rows.iter()
                    .filter_map(|row| row.get(i))
                    .filter_map(|v| match v {
                        DxValue::Num(n) => Some(*n),
                        _ => None,
                    })
                    .sum();
                
                let unit = match expanded.as_str() {
                    "kilometers" => "km",
                    "price" | "amount" => "$",
                    "elevation" => "m",
                    _ => "",
                };
                
                if unit == "$" {
                    parts.push(format!("{}: {}{:.2}", capitalize(&expanded), unit, sum));
                } else {
                    parts.push(format!("{}: {:.1} {}", capitalize(&expanded), sum, unit));
                }
            }
            
            // Count booleans
            if matches!(expanded.as_str(), "sunny" | "active" | "enabled" | "paid") {
                let true_count = section.rows.iter()
                    .filter_map(|row| row.get(i))
                    .filter(|v| matches!(v, DxValue::Bool(true)))
                    .count();
                
                parts.push(format!("{}: {}/{}", capitalize(&expanded), true_count, row_count));
            }
        }
        
        parts.join(" | ")
    }
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
```

### Human to LLM Converter

```rust
/// Convert human-readable format back to LLM-optimized format
pub struct LlmCompressor {
    abbrev: HashMap<&'static str, &'static str>,
}

impl LlmCompressor {
    pub fn new() -> Self {
        let mut abbrev = HashMap::new();
        
        // Reverse mappings (full -> abbreviated)
        abbrev.insert("name", "nm");
        abbrev.insert("title", "tt");
        abbrev.insert("description", "ds");
        abbrev.insert("status", "st");
        abbrev.insert("active", "ac");
        abbrev.insert("enabled", "en");
        abbrev.insert("created", "cr");
        abbrev.insert("updated", "up");
        abbrev.insert("deleted", "dl");
        abbrev.insert("date", "dt");
        abbrev.insert("time", "tm");
        abbrev.insert("timestamp", "ts");
        abbrev.insert("count", "ct");
        abbrev.insert("total", "tl");
        abbrev.insert("amount", "am");
        abbrev.insert("price", "pr");
        abbrev.insert("quantity", "qt");
        abbrev.insert("kilometers", "km");
        abbrev.insert("elevation", "el");
        abbrev.insert("size", "sz");
        abbrev.insert("url", "ur");
        abbrev.insert("path", "pt");
        abbrev.insert("email", "em");
        abbrev.insert("phone", "ph");
        abbrev.insert("address", "ad");
        abbrev.insert("city", "cy");
        abbrev.insert("country", "co");
        abbrev.insert("latitude", "la");
        abbrev.insert("longitude", "lo");
        abbrev.insert("color", "cl");
        abbrev.insert("image", "im");
        abbrev.insert("parent", "pa");
        abbrev.insert("children", "ch");
        abbrev.insert("user", "us");
        abbrev.insert("owner", "ow");
        abbrev.insert("author", "au");
        abbrev.insert("category", "ca");
        abbrev.insert("tags", "tg");
        abbrev.insert("sku", "sk");
        abbrev.insert("customer", "cu");
        abbrev.insert("shipping", "sh");
        abbrev.insert("paid", "pd");
        abbrev.insert("task", "t");
        abbrev.insert("location", "l");
        abbrev.insert("season", "s");
        abbrev.insert("sunny", "s");
        abbrev.insert("with", "w");
        abbrev.insert("width", "w");
        abbrev.insert("height", "h");
        abbrev.insert("value", "v");
        abbrev.insert("type", "t");
        abbrev.insert("version", "v");
        
        Self { abbrev }
    }
    
    pub fn compress(&self, doc: &DxDocument) -> String {
        let mut output = String::new();
        
        // Detect repeated strings for reference creation
        let refs = self.find_repeated_strings(doc);
        
        // Output config line
        if !doc.context.is_empty() {
            output.push_str("#c:");
            let pairs: Vec<String> = doc.context.iter()
                .map(|(k, v)| {
                    let abbrev_key = self.abbreviate(k);
                    let abbrev_val = self.compress_value(v, &refs);
                    format!("{}|{}", abbrev_key, abbrev_val)
                })
                .collect();
            output.push_str(&pairs.join(";"));
            output.push('\n');
        }
        
        // Output references
        for (key, value) in &refs {
            output.push_str(&format!("#:{}|{}\n", key, value));
        }
        
        // Output data sections
        for (id, section) in &doc.sections {
            // Schema line
            let abbrev_schema: Vec<String> = section.schema.iter()
                .map(|s| self.abbreviate(s))
                .collect();
            output.push_str(&format!("#{}({})\n", id, abbrev_schema.join("|")));
            
            // Data rows
            for row in &section.rows {
                let cells: Vec<String> = row.iter()
                    .map(|v| self.compress_value(v, &refs))
                    .collect();
                output.push_str(&cells.join("|"));
                output.push('\n');
            }
        }
        
        output
    }
    
    fn abbreviate(&self, key: &str) -> String {
        let lower = key.to_lowercase();
        if let Some(&abbrev) = self.abbrev.get(lower.as_str()) {
            abbrev.to_string()
        } else if key.len() <= 2 {
            key.to_lowercase()
        } else {
            key[..2].to_lowercase()
        }
    }
    
    fn compress_value(&self, value: &DxValue, refs: &HashMap<String, String>) -> String {
        match value {
            DxValue::Str(s) => {
                // Check if this string should be a reference
                for (key, val) in refs {
                    if s == val {
                        return format!("^{}", key);
                    }
                }
                s.clone()
            }
            DxValue::Num(n) => {
                if n.fract() == 0.0 {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            }
            DxValue::Bool(b) => if *b { "+".to_string() } else { "-".to_string() },
            DxValue::Null => "~".to_string(),
            DxValue::Arr(items) => {
                let compressed: Vec<String> = items.iter()
                    .map(|v| self.compress_value(v, refs))
                    .collect();
                format!("*{}", compressed.join(","))
            }
            DxValue::Ref(key) => format!("^{}", key),
        }
    }
    
    fn find_repeated_strings(&self, doc: &DxDocument) -> HashMap<String, String> {
        let mut counts: HashMap<String, usize> = HashMap::new();
        
        // Count string occurrences
        self.count_strings_in_context(&doc.context, &mut counts);
        for section in doc.sections.values() {
            for row in &section.rows {
                for val in row {
                    if let DxValue::Str(s) = val {
                        if s.len() > 5 { // Only reference strings longer than 5 chars
                            *counts.entry(s.clone()).or_insert(0) += 1;
                        }
                    }
                }
            }
        }
        
        // Create references for strings that appear 2+ times and are long enough
        let mut refs = HashMap::new();
        let mut ref_id = b'A';
        
        for (string, count) in counts {
            if count >= 2 && string.len() > 5 {
                refs.insert(
                    (ref_id as char).to_string(),
                    string,
                );
                ref_id += 1;
                if ref_id > b'Z' { break; }
            }
        }
        
        refs
    }
    
    fn count_strings_in_context(&self, context: &HashMap<String, DxValue>, counts: &mut HashMap<String, usize>) {
        for val in context.values() {
            if let DxValue::Str(s) = val {
                if s.len() > 5 {
                    *counts.entry(s.clone()).or_insert(0) += 1;
                }
            }
        }
    }
}
```

---

## Editor Integration

### VS Code Extension Features

```json
{
  "dx.displayMode": "human",
  "dx.tableStyle": "unicode",
  "dx.showReferences": true,
  "dx.showSummaries": true,
  "dx.expandAbbreviations": true,
  "dx.autoFormat": true,
  "dx.theme": {
    "headerColor": "#4CAF50",
    "tableColor": "#2196F3",
    "refColor": "#FF9800",
    "boolTrue": "#4CAF50",
    "boolFalse": "#F44336"
  }
}
```

### Editor Behavior

| Action | LLM Format | Human Format |
|--------|------------|--------------|
| **File Save** | ✅ Stored | ❌ Not stored |
| **Editor Display** | ❌ Hidden | ✅ Shown |
| **Copy/Paste** | ✅ LLM format | Human ↔ LLM auto-convert |
| **Search** | Both formats | Both formats |
| **Diff/Version Control** | LLM format | N/A |

### Toggle Command

```
Ctrl+Shift+D - Toggle DX Display Mode (Human ↔ LLM)
```

---

## Complete Example: User Management

### LLM Format (Stored)

```dx
#c:app|UserAdmin;v|2.1;env|prod
#:C|Acme Corporation
#:A|Administrator
#u(id|nm|em|role|ac|cr)
1|Alice Chen|alice@^C.com|^A|+|2025-01-15
2|Bob Smith|bob@^C.com|editor|+|2025-01-16
3|Carol Jones|carol@^C.com|viewer|-|2025-01-17
```

### Human Format (Displayed)

```toml
# ═══════════════════════════════════════════════════════════════════════════════
#                                    CONFIG
# ═══════════════════════════════════════════════════════════════════════════════

[config]
    app         = "UserAdmin"
    version     = "2.1"
    environment = "prod"

# ═══════════════════════════════════════════════════════════════════════════════
#                                  REFERENCES
# ═══════════════════════════════════════════════════════════════════════════════

[references]
    C = "Acme Corporation"
    A = "Administrator"

# ═══════════════════════════════════════════════════════════════════════════════
#                                    USERS
# ═══════════════════════════════════════════════════════════════════════════════

[users]
    # Schema: id | name | email | role | active | created

    ┌────┬─────────────┬─────────────────────────────┬───────────────┬────────┬────────────┐
    │ ID │ Name        │ Email                       │ Role          │ Active │ Created    │
    ├────┼─────────────┼─────────────────────────────┼───────────────┼────────┼────────────┤
    │  1 │ Alice Chen  │ alice@Acme Corporation.com  │ Administrator │   ✓    │ 2025-01-15 │
    │  2 │ Bob Smith   │ bob@Acme Corporation.com    │ editor        │   ✓    │ 2025-01-16 │
    │  3 │ Carol Jones │ carol@Acme Corporation.com  │ viewer        │   ✗    │ 2025-01-17 │
    └────┴─────────────┴─────────────────────────────┴───────────────┴────────┴────────────┘

    Total: 3 users | Active: 2/3
```

---

## Summary

| Feature | LLM Format | Human Format |
|---------|------------|--------------|
| **Purpose** | Token efficiency | Readability |
| **Storage** | ✅ Primary | ❌ Derived |
| **Sigils** | `#c`, `#:`, `#x` | `[config]`, `[references]`, `[section]` |
| **Keys** | Abbreviated (`nm`, `st`) | Expanded (`name`, `status`) |
| **Booleans** | `+` / `-` | `✓` / `✗` or `true` / `false` |
| **Tables** | Pipe-delimited | Box-drawn with headers |
| **References** | `^B` | Resolved inline |
| **Summaries** | None | Auto-generated |
| **Indentation** | None | 4-space |

**Result:** Same data, dual views. LLMs get 3x efficiency, humans get beautiful tables!
