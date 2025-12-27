# DX Root Config File Specification

**Root Configuration File: `dx`**

Version: 1.0  
Status: Draft  
Date: December 27, 2025

## Overview

The `dx` file (no extension) is the root configuration file for dx-check projects. It lives in the project root and specifies:
- Which rule files to load
- Project-wide settings
- Language-specific configurations
- Formatter preferences

## File Location

```
project-root/
  dx                 # Root config (this file)
  rules/
    js-rules.dxs     # JavaScript rules
    py-rules.dxs     # Python rules
    rust-rules.dxs   # Rust rules
```

## File Format

Uses dx-serializer LLM format (same as .dxs files).

## Structure

```
# DX Check Configuration
# Project: <project_name>
# Version: 1.0

@project
  name: <project_name>
  version: <version>
  languages:
    - <lang1>
    - <lang2>

@rules
  mode: <strict|recommended|custom>
  sources:
    - rules/js-rules.dxs
    - rules/py-rules.dxs
    - rules/rust-rules.dxs
  overrides:
    js/no-console: off
    py/F841: error
    rs/clippy::unwrap_used: warn

@format
  enabled: <true|false>
  line_width: <number>
  indent_width: <number>
  use_tabs: <true|false>
  quote_style: <single|double>
  semicolons: <always|as_needed>
  trailing_comma: <always|never|multi_line>

@languages
  js:
    parser: <oxc|biome>
    target: <es2020|es2022|esnext>
    jsx: <true|false>
  py:
    version: <3.8|3.9|3.10|3.11|3.12>
    type_checking: <true|false>
  rs:
    edition: <2021|2024>
    clippy_pedantic: <true|false>

@paths
  include:
    - src/**
    - tests/**
  exclude:
    - node_modules/**
    - target/**
    - dist/**
    - build/**

@cache
  enabled: <true|false>
  directory: <path>
  max_size_mb: <number>

@parallel
  threads: <number>  # 0 = auto-detect
  chunk_size: <number>

@watch
  enabled: <true|false>
  debounce_ms: <number>
  clear_screen: <true|false>
```

## Example: Full Configuration

```dx
# DX Check Configuration
# Project: my-awesome-app
# Version: 1.0

@project
  name: my-awesome-app
  version: 1.0.0
  languages:
    - js
    - ts
    - py
    - rs

@rules
  mode: recommended
  sources:
    - rules/js-rules.dxs
    - rules/py-rules.dxs
    - rules/rust-rules.dxs
  overrides:
    # Disable console in production
    js/no-console: error
    js/no-debugger: error
    # Python unused vars are errors
    py/F841: error
    # Allow unwrap in tests
    rs/clippy::unwrap_used: off

@format
  enabled: true
  line_width: 100
  indent_width: 2
  use_tabs: false
  quote_style: double
  semicolons: always
  trailing_comma: multi_line

@languages
  js:
    parser: oxc
    target: es2022
    jsx: true
  py:
    version: 3.12
    type_checking: true
  rs:
    edition: 2024
    clippy_pedantic: true

@paths
  include:
    - src/**/*.{js,ts,jsx,tsx}
    - src/**/*.py
    - src/**/*.rs
    - tests/**
  exclude:
    - node_modules/**
    - target/**
    - dist/**
    - build/**
    - **/*.min.js
    - **/__pycache__/**

@cache
  enabled: true
  directory: .dx-cache
  max_size_mb: 1024

@parallel
  threads: 0  # auto-detect
  chunk_size: 100

@watch
  enabled: true
  debounce_ms: 250
  clear_screen: true
```

## Example: Minimal Configuration

```dx
# Minimal DX Config

@project
  name: simple-project
  languages:
    - js

@rules
  mode: recommended
  sources:
    - rules/js-rules.dxs
```

## Example: Monorepo Configuration

```dx
# Monorepo DX Config

@project
  name: monorepo
  languages:
    - js
    - ts
    - py
    - rs
    - go

@rules
  mode: strict
  sources:
    - rules/js-rules.dxs
    - rules/py-rules.dxs
    - rules/rust-rules.dxs
    - rules/go-rules.dxs
  # Override per workspace
  overrides:
    # Backend is strict
    py/F841: error
    rs/clippy::unwrap_used: error
    # Frontend allows console for now
    js/no-console: warn

@paths
  include:
    - packages/*/src/**
    - apps/*/src/**
  exclude:
    - packages/*/node_modules/**
    - apps/*/dist/**
    - **/target/**

@parallel
  threads: 16  # High-perf CI
  chunk_size: 50
```

## Field Specifications

### @project

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `name` | string | ✅ | - | Project name |
| `version` | string | ❌ | "1.0.0" | Project version |
| `languages` | array | ✅ | - | List of language codes |

### @rules

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `mode` | enum | ❌ | "recommended" | strict, recommended, or custom |
| `sources` | array | ✅ | - | Paths to .dxs files |
| `overrides` | map | ❌ | {} | Rule-specific severity overrides |

**Modes**:
- `strict`: Enable all rules at error level
- `recommended`: Enable recommended rules at default severity
- `custom`: Only rules explicitly enabled

### @format

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `enabled` | boolean | ❌ | true | Enable formatter |
| `line_width` | integer | ❌ | 80 | Maximum line width |
| `indent_width` | integer | ❌ | 2 | Spaces per indent |
| `use_tabs` | boolean | ❌ | false | Use tabs vs spaces |
| `quote_style` | enum | ❌ | "double" | single or double |
| `semicolons` | enum | ❌ | "always" | always or as_needed |
| `trailing_comma` | enum | ❌ | "multi_line" | always, never, multi_line |

### @languages.<lang>

Language-specific settings (varies by language).

### @paths

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `include` | array | ❌ | ["**/*"] | Glob patterns to include |
| `exclude` | array | ❌ | [standard exclusions] | Glob patterns to exclude |

### @cache

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `enabled` | boolean | ❌ | true | Enable AST cache |
| `directory` | string | ❌ | ".dx-cache" | Cache directory path |
| `max_size_mb` | integer | ❌ | 1024 | Max cache size in MB |

### @parallel

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `threads` | integer | ❌ | 0 | Number of threads (0=auto) |
| `chunk_size` | integer | ❌ | 100 | Files per work chunk |

### @watch

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `enabled` | boolean | ❌ | false | Enable watch mode |
| `debounce_ms` | integer | ❌ | 250 | Debounce delay in ms |
| `clear_screen` | boolean | ❌ | true | Clear screen on change |

## Parsing and Loading

1. **Discovery**: Search for `dx` file in project root
2. **Parse**: Parse using dx-serializer LLM format parser
3. **Validate**: Check all required fields present
4. **Resolve**: Resolve all `.dxs` file paths (relative to `dx` file)
5. **Load Rules**: Load and parse all referenced `.dxs` files
6. **Merge**: Merge configurations with CLI overrides
7. **Apply**: Apply to dx-check engine

## Configuration Priority

```
CLI args > dx file > .dxs files > defaults
```

Example:
```bash
# dx file sets js/no-console: warn
# But CLI overrides to error
dx-check --rule js/no-console=error
```

## File Watching

Watch for changes to:
1. Root `dx` config file
2. All referenced `.dxs` files

On change:
1. Re-parse configuration
2. Reload rule database
3. Re-run checks (if watch mode enabled)

## Integration with dx-serializer

The `dx` file and `.dxs` files both use dx-serializer's LLM format:

```rust
use serializer::llm::{LlmParser, DxDocument};

// Parse dx config file
let config_doc = LlmParser::parse(&fs::read_to_string("dx")?)?;

// Parse rule files
let js_rules = LlmParser::parse(&fs::read_to_string("rules/js-rules.dxs")?)?;
```

## Migration from dx.toml

Existing `dx.toml` files can be converted:

```bash
dx-check config migrate dx.toml
```

This generates:
- `dx` - Root config
- `rules/*.dxs` - Rule files (one per language)

## Benefits

1. **Single Format**: dx-serializer LLM format everywhere
2. **Version Control**: Clear diffs, merge-friendly
3. **Modular**: Rules separated by language
4. **Hot-Reload**: Changes detected automatically
5. **Type-Safe**: Validated before use
6. **Extensible**: Easy to add new configuration options

---

**Status**: Specification complete, ready for implementation.
