# Biome CLI - TOML Integration Architecture

## Overview Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                      Biome CLI                               │
│                    (biome_cli)                               │
└─────────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────┐
│                   Command Router                             │
│         (format / lint / check commands)                     │
└─────────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────┐
│                   File Processor                             │
│              (process_file::process_file)                    │
└─────────────────────────────────────────────────────────────┘
                           │
                ┌──────────┴──────────┐
                ▼                     ▼
    ┌──────────────────┐    ┌──────────────────┐
    │   Extension      │    │   Extension      │
    │   Check          │    │   Check          │
    │   .toml?         │    │   Other?         │
    └──────────────────┘    └──────────────────┘
           │ YES                    │ NO
           ▼                        ▼
    ┌──────────────────┐    ┌──────────────────┐
    │  Taplo Handler   │    │  Biome Handler   │
    │  (toml.rs)       │    │  (workspace_file)│
    └──────────────────┘    └──────────────────┘
           │                        │
           ▼                        ▼
    ┌──────────────────┐    ┌──────────────────┐
    │  Taplo Parser    │    │  Biome Parser    │
    │  & Formatter     │    │  & Formatter     │
    └──────────────────┘    └──────────────────┘
           │                        │
           └────────┬───────────────┘
                    ▼
    ┌─────────────────────────────────┐
    │      Diagnostic Reporter        │
    │    (Unified Output System)      │
    └─────────────────────────────────┘
```

## File Flow

### TOML File Processing

```
Input: sample.toml
      │
      ▼
┌─────────────────┐
│ format.rs       │  → Check extension → .toml? → YES
│ lint_and_assist │                               │
│ check.rs        │                               ▼
└─────────────────┘                    ┌──────────────────┐
                                       │ toml::format_toml│
                                       │ toml::lint_toml  │
                                       │ toml::check_toml │
                                       └──────────────────┘
                                                │
                                                ▼
                                    ┌──────────────────────┐
                                    │  taplo::parser       │
                                    │  taplo::formatter    │
                                    │  taplo::validator    │
                                    └──────────────────────┘
                                                │
                                                ▼
                                       ┌────────────────┐
                                       │ Formatted TOML │
                                       │ or Diagnostics │
                                       └────────────────┘
```

### Other File Processing

```
Input: sample.js / .ts / .json / .css / etc.
      │
      ▼
┌─────────────────┐
│ format.rs       │  → Check extension → NOT .toml
│ lint_and_assist │                               │
│ check.rs        │                               ▼
└─────────────────┘                    ┌──────────────────┐
                                       │ WorkspaceFile    │
                                       │ (Biome handlers) │
                                       └──────────────────┘
                                                │
                                                ▼
                                    ┌──────────────────────┐
                                    │  Biome Parsers       │
                                    │  (JS/TS/JSON/CSS...) │
                                    └──────────────────────┘
                                                │
                                                ▼
                                       ┌────────────────┐
                                       │ Formatted Code │
                                       │ or Diagnostics │
                                       └────────────────┘
```

## Module Structure

```
biome_cli/src/execute/
├── process_file.rs
│   ├── mod check;
│   ├── mod format;
│   ├── mod lint_and_assist;
│   ├── mod toml;              ← NEW!
│   └── pub mod workspace_file;
│
├── process_file/
│   ├── check.rs               ← Modified (routes .toml)
│   ├── format.rs              ← Modified (routes .toml)
│   ├── lint_and_assist.rs     ← Modified (routes .toml)
│   ├── toml.rs                ← NEW! (handles TOML)
│   └── workspace_file.rs
│
└── ...
```

## Dependency Graph

```
┌──────────────────┐
│   Biome CLI      │
│  (biome_cli)     │
└────────┬─────────┘
         │
         ├─ depends on ─→ ┌──────────────┐
         │                │    taplo     │
         │                │  (parser +   │
         │                │  formatter)  │
         │                └──────────────┘
         │
         └─ depends on ─→ ┌──────────────┐
                          │ taplo-common │
                          │  (utilities) │
                          └──────────────┘

Workspace Path: ../taplo/crates/taplo
                ../taplo/crates/taplo-common
```

## Command Flow Examples

### Example 1: Format TOML

```
$ biome format --write playground/sample.toml
      │
      ▼
[CLI Parser] → [FormatCommand]
      │
      ▼
[Execution::Format]
      │
      ▼
[process_file()] → Detect .toml extension
      │
      ▼
[toml::format_toml()]
      │
      ├─→ [taplo::parser::parse()]
      ├─→ [taplo::formatter::format_with_path_scopes()]
      └─→ [Write to file]
      │
      ▼
✓ Formatted 1 file
```

### Example 2: Lint TOML

```
$ biome lint playground/sample.toml
      │
      ▼
[CLI Parser] → [LintCommand]
      │
      ▼
[Execution::Lint]
      │
      ▼
[process_file()] → Detect .toml extension
      │
      ▼
[toml::lint_toml()]
      │
      ├─→ [taplo::parser::parse()] → Check syntax
      ├─→ [DOM::validate()] → Check semantics
      └─→ [Report diagnostics]
      │
      ▼
✓ No issues found (or)
✗ 2 errors, 1 warning
```

### Example 3: Check All Files

```
$ biome check playground/
      │
      ▼
[CLI Parser] → [CheckCommand]
      │
      ▼
[Execution::Check]
      │
      ▼
[Traverse Directory]
      │
      ├─→ sample.js → [Biome Handler]
      ├─→ sample.ts → [Biome Handler]
      ├─→ sample.json → [Biome Handler]
      ├─→ sample.toml → [Taplo Handler] ← NEW!
      └─→ ...
      │
      ▼
Summary Report
```

## Integration Points

### 1. Extension Detection

```rust
if path.extension().map_or(false, |ext| ext == "toml") {
    return super::toml::format_toml(ctx, path);
}
```

### 2. TOML Processing

```rust
// Parse
let parse = parser::parse(&content);

// Format
let formatted = formatter::format_with_path_scopes(
    dom, 
    format_opts, 
    &error_ranges, 
    &[]
)?;

// Validate
dom.validate()?;
```

### 3. Diagnostic Reporting

```rust
ctx.push_message(Message::Diagnostics {
    file_path: path_str,
    content: input,
    diagnostics: errors,
    skipped_diagnostics: 0,
});
```

## Supported Languages

```
Biome CLI now supports:

┌─────────────────────┐
│ JavaScript (.js)    │ ← Biome Native
│ TypeScript (.ts)    │ ← Biome Native
│ JSON (.json)        │ ← Biome Native
│ CSS (.css)          │ ← Biome Native
│ GraphQL (.graphql)  │ ← Biome Native
│ HTML (.html)        │ ← Biome Native
│ TOML (.toml)        │ ← NEW! Taplo Integration
└─────────────────────┘
```

This architecture ensures a clean separation between Biome's native handlers and the Taplo integration, while maintaining a unified user experience.
