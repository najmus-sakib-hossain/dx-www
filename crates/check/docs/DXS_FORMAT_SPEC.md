# .dxs File Format Specification

**DX Serializer (DXS) Rule Definition Format**

Version: 1.0  
Status: Draft  
Date: December 27, 2025

## Overview

`.dxs` files are human-readable rule definition files that use dx-serializer's LLM format. They serve as the source of truth for lint and format rules, which are then compiled into binary `.dxm` files for runtime execution.

## File Naming Convention

```
<language>-rules.dxs
```

Examples:
- `js-rules.dxs` - JavaScript/TypeScript rules
- `py-rules.dxs` - Python rules
- `rust-rules.dxs` - Rust rules
- `go-rules.dxs` - Go rules

## File Structure

```
# Language Rules for <Language>
# Version: 1.0
# Maintained by: dx-check contributors

@meta
  language: <language_code>
  source: <source_tool>
  version: <version>
  total_rules: <count>

@rule <rule_id>
  name: <rule_name>
  prefixed_name: <lang>/<rule_name>
  category: <category>
  severity: <warn|error|off>
  fixable: <true|false>
  recommended: <true|false>
  is_formatter: <true|false>
  description: |
    <multi-line description>
  docs_url: <optional_url>
  options_schema: |
    <optional_json_schema>
  related_rules:
    - <rule1>
    - <rule2>
  examples:
    - type: correct
      code: |
        <example_code>
    - type: incorrect
      code: |
        <example_code>
```

## Field Specifications

### @meta Section

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `language` | string | ✅ | Language code (js, py, go, rs, etc.) |
| `source` | string | ✅ | Source tool (dx-check, biome, ruff, etc.) |
| `version` | string | ✅ | Rule set version (semver) |
| `total_rules` | integer | ✅ | Total number of rules in file |

### @rule Section

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | ✅ | Rule identifier (e.g., "no-console") |
| `prefixed_name` | string | ✅ | Full name with language prefix |
| `category` | enum | ✅ | correctness, suspicious, style, performance, security, complexity, a11y, imports, types, docs, deprecated, format |
| `severity` | enum | ✅ | warn, error, off |
| `fixable` | boolean | ✅ | Whether rule has auto-fix |
| `recommended` | boolean | ✅ | Whether rule is in recommended set |
| `is_formatter` | boolean | ✅ | Whether this is a format rule |
| `description` | string | ✅ | Human-readable description |
| `docs_url` | string | ❌ | Link to documentation |
| `options_schema` | string | ❌ | JSON Schema for configuration |
| `related_rules` | array | ❌ | List of related rule names |
| `examples` | array | ❌ | Code examples (correct/incorrect) |

## Example: js-rules.dxs

```dxs
# JavaScript/TypeScript Rules
# Version: 1.0
# Source: dx-check, biome, oxc

@meta
  language: js
  source: dx-check,biome,oxc
  version: 1.0.0
  total_rules: 8

@rule 1
  name: no-console
  prefixed_name: js/no-console
  category: suspicious
  severity: warn
  fixable: true
  recommended: true
  is_formatter: false
  description: |
    Disallow the use of console statements.
    Console statements are often used for debugging and should
    be removed before production deployment.
  docs_url: https://dx.dev/rules/no-console
  options_schema: |
    {
      "type": "object",
      "properties": {
        "allow": {
          "type": "array",
          "items": {"type": "string"},
          "description": "List of console methods to allow"
        }
      }
    }
  related_rules:
    - js/no-debugger
    - js/no-alert
  examples:
    - type: incorrect
      code: |
        console.log('debug');
        console.error('error');
    - type: correct
      code: |
        logger.info('production log');

@rule 2
  name: no-debugger
  prefixed_name: js/no-debugger
  category: suspicious
  severity: error
  fixable: true
  recommended: true
  is_formatter: false
  description: |
    Disallow the use of debugger statements.
    Debugger statements should never reach production code.
  docs_url: https://dx.dev/rules/no-debugger
  examples:
    - type: incorrect
      code: |
        function debug() {
          debugger;
        }
    - type: correct
      code: |
        function debug() {
          // Use proper debugging tools
        }
```

## Example: rust-rules.dxs

```dxs
# Rust Rules
# Version: 1.0
# Source: rustfmt, clippy

@meta
  language: rs
  source: rustfmt,clippy
  version: 1.0.0
  total_rules: 16

@rule 1
  name: fmt
  prefixed_name: rs/fmt
  category: format
  severity: warn
  fixable: true
  recommended: true
  is_formatter: true
  description: |
    Format Rust code using rustfmt.
    Ensures consistent code style across Rust projects.
  docs_url: https://rust-lang.github.io/rustfmt

@rule 2
  name: clippy::unwrap_used
  prefixed_name: rs/clippy::unwrap_used
  category: correctness
  severity: warn
  fixable: false
  recommended: true
  is_formatter: false
  description: |
    Disallow the use of .unwrap().
    Use proper error handling with Result<T, E> and ? operator instead.
  docs_url: https://rust-lang.github.io/rust-clippy/
  related_rules:
    - rs/clippy::expect_used
    - rs/clippy::panic
  examples:
    - type: incorrect
      code: |
        let value = option.unwrap();
    - type: correct
      code: |
        let value = option?;
```

## Parsing Rules

1. **Comments**: Lines starting with `#` are comments (ignored)
2. **Sections**: Start with `@` followed by section name
3. **Indentation**: Two spaces for nested properties
4. **Multi-line Values**: Use `|` after colon, indent content
5. **Arrays**: Each item on new line with `-` prefix
6. **Booleans**: `true`, `false` (lowercase)
7. **Enums**: Use exact enum variant names (case-sensitive)

## Validation Rules

1. All required fields must be present
2. Rule IDs must be unique within a file
3. Prefixed names must match pattern: `<language>/<name>`
4. Categories must be valid enum variants
5. Severity must be: warn, error, or off
6. JSON schema in `options_schema` must be valid JSON
7. Examples must specify `type: correct` or `type: incorrect`

## Compilation Process

```
.dxs files → Parser → Validator → DxRuleDatabase → Compiler → .dxm binary
```

1. **Parse**: Read .dxs files and parse into DxRule structs
2. **Validate**: Check all rules meet specification
3. **Merge**: Combine all language rules into single database
4. **Compile**: Serialize to binary .dxm format
5. **Verify**: Validate binary format integrity

## File Watching

The dx-serializer will watch for changes to:
- Root `dx` config file
- All `*.dxs` files in project root or `rules/` directory

On change:
1. Re-parse affected .dxs files
2. Re-validate rules
3. Re-compile to .dxm
4. Notify dx-check to reload rules (hot-reload in dev mode)

## Benefits

1. **Human-Readable**: Easy to edit and review
2. **Version Control Friendly**: Clear diffs, merge-friendly
3. **Contributor Accessible**: No binary editing required
4. **Documented**: Examples and descriptions inline
5. **Type-Safe**: Validated before compilation
6. **Fast Runtime**: Compiled to 0.70ns access binary format

## Migration Path

Existing extractor.rs code will be converted to generate .dxs files:
```rust
extract_all_rules() → generate_dxs_files()
```

Each language gets its own .dxs file, making it easy for contributors to:
- Add new rules
- Update rule descriptions
- Modify rule configurations
- Add examples

---

**Status**: Specification complete, ready for implementation.
