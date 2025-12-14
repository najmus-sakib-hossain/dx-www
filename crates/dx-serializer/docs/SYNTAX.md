# DX Serializer Syntax Guide

Complete reference for the DX Ω format specification.

---

## Table of Contents

- [Basic Syntax](#basic-syntax)
- [Type Hints](#type-hints)
- [Operators](#operators)
- [Tables](#tables)
- [Advanced Features](#advanced-features)
- [Examples](#examples)

---

## Basic Syntax

### Key-Value Pairs

```dx
name:Alice
age:30
active:+
```

**Rules:**
- Use `:` for assignment
- No spaces around `:`
- No quotes needed for strings (vacuum parsing)

### Nested Objects

**Dot Notation:**
```dx
user.name:Alice
user.age:30
user.email:alice@example.com
```

**Inline Prefixing (^):**
```dx
user.name:Alice^age:30^email:alice@example.com
```

**Result:** Both produce `{ user: { name: "Alice", age: 30, email: "..." } }`

---

## Type Hints

Type hints enable zero-copy vacuum parsing by telling the parser what comes next.

### Integer (%i)

```dx
count%i:42
port%i:8080
year%i:2025
```

**Range:** `-9223372036854775808` to `9223372036854775807` (i64)

### Float (%f)

```dx
price%f:19.99
pi%f:3.14159
temp%f:-40.5
```

**Precision:** f64 (IEEE 754 double)

### String (%s)

```dx
name%s:Alice Johnson
path%s:/usr/local/bin
url%s:https://example.com
```

**Parsing:** Vacuum mode — reads until next type boundary (space, newline, `^`)

### Boolean (%b)

```dx
active%b:+
verified%b:-
enabled%b:+
```

**Values:** `+` (true), `-` (false)

---

## Operators

### Assignment (:)

```dx
key:value
```

Basic key-value assignment.

### Inline Prefix (^)

```dx
a:1^b:2^c:3
```

Combines multiple assignments on one line. Saves newline bytes.

**Equivalent to:**
```dx
a:1
b:2
c:3
```

### Stream (>)

```dx
colors>red|blue|green
```

Creates an array/list.

**Equivalent JSON:** `{ "colors": ["red", "blue", "green"] }`

### Table (=)

```dx
users=id%i name%s age%i active%b
1 Alice 30 +
2 Bob 25 -
3 Charlie 35 +
```

Defines schema then data rows.

### Separator (|)

```dx
items>a|b|c|d
```

Separates array elements (alternative to commas).

### Nested (.)

```dx
config.db.host:localhost
config.db.port:5432
```

Creates nested object paths.

### Alias ($)

```dx
$u:user
$u.name:Alice
$u.age:30
```

Define shorthand once, use many times.

**Expands to:** `{ user: { name: "Alice", age: 30 } }`

### Ditto (")

```dx
tasks=id%i project%s status%s
1 dx-www active
2 " testing
3 " complete
```

Repeats previous value (vertical compression).

**Expands to:**
```
1 dx-www active
2 dx-www testing
3 dx-www complete
```

---

## Tables

Tables are DX's killer feature — ultra-compact tabular data.

### Basic Table

```dx
users=id%i name%s email%s
1 alice alice@example.com
2 bob bob@example.com
```

### With Type Hints

```dx
products=id%i name%s price%f inStock%b
1 Laptop 999.99 +
2 Mouse 29.99 +
3 Keyboard 79.99 -
```

### Shortened Headers

```dx
h=i n%s k%f g%i w%s s%b
1 Blue Lake Trail 7.5 320 ana +
2 Ridge Overlook 9.2 540 luis -
```

**Full expansion:**
```json
{
  "hikes": [
    { "id": 1, "name": "Blue Lake Trail", "km": 7.5, "gain": 320, "who": "ana", "sun": true },
    { "id": 2, "name": "Ridge Overlook", "km": 9.2, "gain": 540, "who": "luis", "sun": false }
  ]
}
```

---

## Advanced Features

### Prefix Inheritance

```dx
api.endpoint:https://api.example.com
api.key:secret123
api.timeout:5000
```

All share `api.` prefix.

### Mixed Inline + Block

```dx
config.name:MyApp^version:1.0.0
config.db.host:localhost
config.db.port:5432
features>auth|cache|logs
```

Combine inline prefixing with regular syntax.

### Comments

```dx
# This is a comment
name:Alice  # Inline comment
```

Use `#` for comments (ignored by parser).

### Multi-line Strings

Use triple quotes for multi-line:

```dx
description:"""
This is a multi-line
string value.
"""
```

---

## Examples

### Example 1: Configuration File

```dx
app.name:DX Runtime^version:0.1.0^env:production
db.host:localhost^port:5432^name:dxdb
features>auth|cache|logging|metrics
limits.requests:1000^timeout:30
```

**JSON Equivalent (699 bytes):**
```json
{
  "app": {
    "name": "DX Runtime",
    "version": "0.1.0",
    "env": "production"
  },
  "db": {
    "host": "localhost",
    "port": 5432,
    "name": "dxdb"
  },
  "features": ["auth", "cache", "logging", "metrics"],
  "limits": {
    "requests": 1000,
    "timeout": 30
  }
}
```

**DX Ω (203 bytes):** 71% smaller

### Example 2: User Data

```dx
users=id%i name%s email%s role%s active%b
1 Alice alice@dx.dev admin +
2 Bob bob@dx.dev user +
3 Charlie charlie@dx.dev user -
```

**CSV Equivalent:**
```csv
id,name,email,role,active
1,Alice,alice@dx.dev,admin,true
2,Bob,bob@dx.dev,user,true
3,Charlie,charlie@dx.dev,user,false
```

**Advantage:** Type hints + shorter booleans (+ vs true)

### Example 3: Complex Nested

```dx
project:DX Runtime^version:0.1.0^status+
owner.name:Alice^email:alice@dx.dev^role:admin
team>bob|charlie|diana
milestones=id%i name%s date%s complete%b
1 Alpha 2025-01-01 +
2 Beta 2025-02-01 -
3 Release 2025-03-01 -
metrics.users:1500^requests:50000^uptime:99.9
```

**JSON:** 1152 bytes  
**TOON:** 1082 bytes  
**DX Ω:** 168 bytes (84.5% smaller than TOON!)

---

## Best Practices

### 1. Use Type Hints

**Bad:**
```dx
count:42
price:19.99
```

**Good:**
```dx
count%i:42
price%f:19.99
```

Type hints enable zero-copy parsing (4-5x faster).

### 2. Use Inline Prefixing for Small Objects

**Bad:**
```dx
config.a:1
config.b:2
config.c:3
```

**Good:**
```dx
config.a:1^b:2^c:3
```

Saves 2 newlines = 2 bytes.

### 3. Shorten Table Headers

**Bad:**
```dx
hikes=id%i hikeName%s distanceKm%f elevationGain%i
```

**Good:**
```dx
h=i n%s k%f g%i
```

Saves 35 bytes (71% reduction).

### 4. Use Ditto for Repeated Values

**Bad:**
```dx
tasks=id%i project%s
1 dx-www
2 dx-www
3 dx-www
```

**Good:**
```dx
tasks=id%i project%s
1 dx-www
2 "
3 "
```

Saves 12 bytes per repetition.

### 5. Use Sigil Booleans

**Bad:**
```dx
active:true
verified:false
```

**Good:**
```dx
active:+
verified:-
```

Saves 3-4 bytes per boolean (75-80% reduction).

---

## Parser Rules

1. **Whitespace:** Spaces separate table columns, newlines separate entries
2. **Quoting:** Not needed (vacuum parsing reads until type boundary)
3. **Escaping:** Use `\` for special chars: `\^`, `\|`, `\:`
4. **Type Inference:** Without hints, parser guesses (slower, less reliable)
5. **Comments:** `#` starts comment, continues to end of line

---

## Comparison Table

| Feature | JSON | YAML | TOON | **DX Ω** |
|---------|------|------|------|----------|
| **Quotes Required** | ✓ | ✗ | ✗ | ✗ |
| **Type Hints** | ✗ | ✗ | ✗ | ✓ |
| **Inline Objects** | ✗ | ✗ | ✗ | ✓ (^) |
| **Sigil Booleans** | ✗ | ✗ | ✗ | ✓ (+/-) |
| **Tabular Format** | ✗ | ✗ | ✓ | ✓ |
| **Ditto Compression** | ✗ | ✗ | ✗ | ✓ (") |
| **Zero-Copy Parse** | ✗ | ✗ | ✗ | ✓ |

---

## Grammar (EBNF)

```ebnf
document     = entry* ;
entry        = key_value | table | comment ;
key_value    = key ":" value ("^" key ":" value)* ;
key          = identifier ("." identifier)* ;
value        = string | number | boolean | stream ;
stream       = ">" element ("|" element)* ;
table        = key "=" column+ "\n" row+ ;
column       = identifier type_hint? ;
type_hint    = "%" ("i" | "f" | "s" | "b") ;
row          = value+ "\n" ;
boolean      = "+" | "-" | "~" ;
comment      = "#" .*? "\n" ;
```

---

**See Also:**
- [API Reference](API.md)
- [Performance Guide](PERFORMANCE.md)
- [Migration Guide](MIGRATION.md)
