# DX ∞ Implementation Roadmap

This document outlines the implementation plan for all 5 DX ∞ features.

## Feature 1: Auto-Increment Columns (`%#`)

### Parser Changes
```rust
// In type hint parsing
pub enum TypeHint {
    // ... existing types ...
    AutoIncrement, // New: %#
}

// In table row parsing
fn parse_table_row(&mut self, schema: &TableSchema) -> Result<Vec<DxValue>> {
    let mut values = Vec::new();
    let mut auto_counter = 1;
    
    for col_type in &schema.column_types {
        match col_type {
            TypeHint::AutoIncrement => {
                // Don't read from input - generate value
                values.push(DxValue::Integer(auto_counter));
                auto_counter += 1;
            }
            _ => {
                // Normal parsing
                let val = self.parse_value(col_type)?;
                values.push(val);
            }
        }
    }
    
    Ok(values)
}
```

### Encoder Changes
```rust
fn encode_table_row(&mut self, row: &[DxValue], schema: &TableSchema) {
    for (val, col_type) in row.iter().zip(&schema.column_types) {
        match col_type {
            TypeHint::AutoIncrement => {
                // Skip writing this column - it's auto-generated
                continue;
            }
            _ => {
                self.write_value(val);
                self.write_byte(b' ');
            }
        }
    }
}
```

### Tests
```rust
#[test]
fn test_auto_increment() {
    let input = b"h=id%# name%s\nAlice\nBob\nCharlie";
    let parsed = parse(input).unwrap();
    
    assert_eq!(parsed["h"][0]["id"], 1);
    assert_eq!(parsed["h"][1]["id"], 2);
    assert_eq!(parsed["h"][2]["id"], 3);
}
```

**Estimated Impact:** 6 bytes saved on hikes

---

## Feature 2: Inline Dictionary (`$`)

### Parser Changes
```rust
pub struct Parser {
    // ... existing fields ...
    aliases: HashMap<String, String>, // New: alias dictionary
}

fn parse_string(&mut self) -> Result<String> {
    if self.peek() == Some(b'$') {
        self.advance(); // Skip '$'
        
        // Check for definition: $key:value
        let key = self.read_until(|c| c == b':' || c == b'|' || c == b' ');
        
        if self.peek() == Some(b':') {
            // Definition: $a:ana
            self.advance(); // Skip ':'
            let value = self.read_until(|c| c == b'|' || c == b' ' || c == b'\n');
            self.aliases.insert(key.clone(), value.clone());
            return Ok(value);
        } else {
            // Reference: $a
            return self.aliases.get(&key)
                .cloned()
                .ok_or_else(|| Error::UndefinedAlias(key));
        }
    }
    
    // Normal string parsing
    self.read_until(|c| c == b'|' || c == b' ' || c == b'\n')
}
```

### Encoder Changes
```rust
pub struct Encoder {
    // ... existing fields ...
    aliases: HashMap<String, String>, // value -> alias mapping
    alias_counter: u8,
}

fn encode_string(&mut self, s: &str) {
    // Check if this string has been seen before
    if let Some(alias) = self.aliases.get(s) {
        // Use alias reference: $a
        self.write_byte(b'$');
        self.write_str(alias);
        return;
    }
    
    // Check if string is long enough to warrant alias
    if s.len() > 4 && self.alias_counter < 62 {
        // Create new alias: $a:value
        let alias = self.generate_alias(); // a, b, c, ... aa, ab, ...
        self.write_byte(b'$');
        self.write_str(&alias);
        self.write_byte(b':');
        self.write_str(s);
        self.aliases.insert(s.to_string(), alias);
        self.alias_counter += 1;
        return;
    }
    
    // Normal string encoding
    self.write_str(s);
}

fn generate_alias(&self) -> String {
    // a, b, c, ... z, aa, ab, ac, ...
    let n = self.alias_counter;
    if n < 26 {
        ((b'a' + n) as char).to_string()
    } else {
        // Two-letter aliases
        let first = ((n / 26) - 1 + b'a') as char;
        let second = (n % 26 + b'a') as char;
        format!("{}{}", first, second)
    }
}
```

### Tests
```rust
#[test]
fn test_inline_alias() {
    let input = b"f>$a:ana|$l:luis\ncompanion: $a";
    let parsed = parse(input).unwrap();
    
    assert_eq!(parsed["f"][0], "ana");
    assert_eq!(parsed["companion"], "ana");
}

#[test]
fn test_alias_encoding() {
    let data = DxObject::from([
        ("names", vec!["Alice", "Alice", "Bob", "Alice"]),
    ]);
    
    let encoded = encode(&data).unwrap();
    // Should contain $a:Alice and then $a references
    assert!(encoded.contains("$a:Alice"));
}
```

**Estimated Impact:** ~15 bytes saved on hikes

---

## Feature 3: Base62 Integers (`%x`)

### Implementation
```rust
const BASE62: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn encode_base62(mut n: u64) -> String {
    if n == 0 {
        return "0".to_string();
    }
    
    let mut result = Vec::new();
    while n > 0 {
        result.push(BASE62[(n % 62) as usize] as char);
        n /= 62;
    }
    result.reverse();
    result.into_iter().collect()
}

fn decode_base62(s: &str) -> Result<u64> {
    let mut result = 0u64;
    
    for c in s.chars() {
        let digit = match c {
            '0'..='9' => (c as u8 - b'0') as u64,
            'a'..='z' => (c as u8 - b'a' + 10) as u64,
            'A'..='Z' => (c as u8 - b'A' + 36) as u64,
            _ => return Err(Error::InvalidBase62(c)),
        };
        
        result = result * 62 + digit;
    }
    
    Ok(result)
}
```

### Parser Integration
```rust
fn parse_integer(&mut self, hint: &TypeHint) -> Result<i64> {
    let s = self.read_until(|c| c == b' ' || c == b'|' || c == b'\n');
    
    match hint {
        TypeHint::Base62Integer => {
            let n = decode_base62(&s)?;
            Ok(n as i64)
        }
        _ => {
            s.parse().map_err(|_| Error::InvalidInteger)
        }
    }
}
```

### Tests
```rust
#[test]
fn test_base62_codec() {
    assert_eq!(encode_base62(0), "0");
    assert_eq!(encode_base62(61), "Z");
    assert_eq!(encode_base62(62), "10");
    assert_eq!(encode_base62(320), "5A");
    assert_eq!(encode_base62(540), "8k");
    assert_eq!(encode_base62(10000), "2Bi");
    
    assert_eq!(decode_base62("5A").unwrap(), 320);
    assert_eq!(decode_base62("8k").unwrap(), 540);
    assert_eq!(decode_base62("2Bi").unwrap(), 10000);
}

#[test]
fn test_base62_table() {
    let input = b"h=elevation%x\n320\n540\n180";
    let parsed = parse(input).unwrap();
    
    // When encoded, should use Base62
    let encoded = encode(&parsed).unwrap();
    assert!(encoded.contains("5A")); // 320
    assert!(encoded.contains("8k")); // 540
    assert!(encoded.contains("2T")); // 180
}
```

**Estimated Impact:** ~12 bytes saved on hikes

---

## Feature 4: Ghost Root (`.=`)

### Parser Changes
```rust
fn parse_root_schema(&mut self) -> Option<Vec<(String, TypeHint)>> {
    if self.peek() != Some(b'.') {
        return None;
    }
    
    self.advance(); // Skip '.'
    
    if self.peek() != Some(b'=') {
        // Not a schema, backtrack
        self.pos -= 1;
        return None;
    }
    
    self.advance(); // Skip '='
    
    // Parse: .=key1:type1 key2:type2 key3:type3
    let mut schema = Vec::new();
    
    loop {
        let key = self.read_until(|c| c == b':');
        self.advance(); // Skip ':'
        let type_hint = self.read_type_hint();
        
        schema.push((key, type_hint));
        
        if self.peek() == Some(b'\n') {
            self.advance();
            break;
        }
        
        if self.peek() == Some(b' ') {
            self.advance();
        }
    }
    
    Some(schema)
}

fn parse_root_with_schema(&mut self, schema: &[(String, TypeHint)]) -> Result<DxObject> {
    let mut obj = DxObject::new();
    
    // Next line should be values separated by |
    for (key, type_hint) in schema {
        let val = self.parse_value(type_hint)?;
        obj.insert(key.clone(), val);
        
        if self.peek() == Some(b'|') {
            self.advance();
        }
    }
    
    Ok(obj)
}
```

### Encoder Changes
```rust
fn encode_root_object(&mut self, obj: &DxObject) {
    // Emit schema first
    self.write_str(".=");
    
    for (i, (key, val)) in obj.iter().enumerate() {
        self.write_str(key);
        self.write_byte(b':');
        self.write_type_hint(val);
        
        if i < obj.len() - 1 {
            self.write_byte(b' ');
        }
    }
    
    self.write_byte(b'\n');
    
    // Emit values without keys
    for (i, (_, val)) in obj.iter().enumerate() {
        self.write_value_raw(val);
        
        if i < obj.len() - 1 {
            self.write_byte(b'|');
        }
    }
}
```

### Tests
```rust
#[test]
fn test_ghost_root() {
    let input = b".=task:s loc:s season:s\nOur favorite hikes|Boulder|spring_2025";
    let parsed = parse(input).unwrap();
    
    assert_eq!(parsed["task"], "Our favorite hikes");
    assert_eq!(parsed["loc"], "Boulder");
    assert_eq!(parsed["season"], "spring_2025");
}
```

**Estimated Impact:** ~30 bytes saved on hikes

---

## Feature 5: Delta Compression (`Δ`)

### Implementation
```rust
fn parse_delta(&mut self, prev_value: &DxValue) -> Result<DxValue> {
    if self.peek() != Some(b'>') {
        return Err(Error::ExpectedDelta);
    }
    
    self.advance(); // Skip '>'
    
    // Check for explicit delta: Δ5
    if self.peek() == Some(b'Δ') || self.is_digit(self.peek()) {
        let delta = self.read_integer()?;
        
        match prev_value {
            DxValue::Integer(n) => Ok(DxValue::Integer(n + delta)),
            _ => Err(Error::DeltaOnNonInteger),
        }
    } else {
        // Implicit +1
        match prev_value {
            DxValue::Integer(n) => Ok(DxValue::Integer(n + 1)),
            _ => Err(Error::DeltaOnNonInteger),
        }
    }
}
```

### Tests
```rust
#[test]
fn test_delta_compression() {
    let input = b"years: 2024|>|>|>";
    let parsed = parse(input).unwrap();
    
    assert_eq!(parsed["years"][0], 2024);
    assert_eq!(parsed["years"][1], 2025);
    assert_eq!(parsed["years"][2], 2026);
    assert_eq!(parsed["years"][3], 2027);
}
```

**Estimated Impact:** Significant for time series

---

## Implementation Priority

1. **Base62 Integers** (Highest ROI, clear spec)
   - Standalone feature
   - Clear test cases
   - ~12 bytes savings

2. **Auto-Increment** (Clean implementation)
   - Minimal parser changes
   - ~6 bytes savings
   - Useful for all tabular data

3. **Ghost Root** (Medium complexity)
   - Requires schema parsing
   - ~30 bytes savings on nested data
   - High impact on complex benchmark

4. **Inline Aliases** (Complex state management)
   - Requires dictionary tracking
   - ~15 bytes savings
   - Most complex feature

5. **Delta Compression** (Optional, niche)
   - Time series optimization
   - Less general applicability
   - Implement last

---

## Testing Strategy

1. Unit tests for each codec (Base62, Delta)
2. Integration tests for parser + encoder
3. Round-trip tests (encode → parse → encode)
4. Benchmark comparison tests
5. Edge case tests (empty data, single values, etc.)

---

## Expected Final Results

With **full implementation**:

| Benchmark | Current | Target | Improvement |
|-----------|---------|--------|-------------|
| Hikes | 203B | 185B | -37.5% vs TOON |
| Complex | 135B | 120B | -88.9% vs TOON |
| Simple | 28B | 25B | -66.7% vs TOON |

**Average:** -77% vs TOON 🚀

---

*Implementation timeline: 2-3 days for all features*
