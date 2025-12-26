# Requirements Document

## Introduction

This document specifies requirements for fixing the Human Format V3 output in the VS Code DX Serializer extension. Currently, the WASM module outputs the OLD format with comments (`# ════...`) and Unicode box-drawing tables (`┌──────┐`), while the TypeScript fallback outputs the correct TOML-like Human Format V3. This inconsistency needs to be fixed so both implementations produce the same clean TOML-like output.

**PROBLEM**: When WASM is loaded, users see the old YAML-like format with decorative comments and Unicode tables. When TypeScript fallback is used, users see the correct TOML-like format.

**SOLUTION**: Configure the WASM module to output TOML-like format without box-drawing tables and decorative comments, matching the TypeScript implementation.

## Glossary

- **Human_Format_V3**: The clean TOML-like format with `key = value` pairs and `[section]` headers, NO comments, NO Unicode tables
- **Old_Human_Format**: The deprecated format with `# ════...` comment headers and Unicode box-drawing tables (`┌──────┐`)
- **WASM_Module**: The WebAssembly binary compiled from the Rust serializer crate
- **DxSerializer**: The main WASM class that provides `toHuman()` and `toDense()` methods
- **HologramConfigJs**: The WASM configuration class with options like `setUseBoxDrawing()` and `setUseUnicodeSymbols()`

## Requirements

### Requirement 1: WASM Output Must Match Human Format V3

**User Story:** As a developer, I want the WASM module to output the same TOML-like format as the TypeScript fallback, so that the editor display is consistent.

#### Acceptance Criteria

1. WHEN the WASM `toHuman()` method is called, THE output SHALL NOT contain decorative comment headers (`# ════...`)
2. WHEN the WASM `toHuman()` method is called, THE output SHALL NOT contain Unicode box-drawing characters (`┌`, `┐`, `└`, `┘`, `─`, `│`)
3. WHEN the WASM `toHuman()` method is called, THE output SHALL use `[section]` headers for sections
4. WHEN the WASM `toHuman()` method is called, THE output SHALL use `key = value` format with aligned equals signs
5. WHEN the WASM `toHuman()` method is called, THE output SHALL use ` | ` as array separator (not commas)
6. THE WASM output SHALL be semantically equivalent to the TypeScript `formatDocumentV3()` output

### Requirement 2: Configure WASM Serializer Correctly

**User Story:** As a developer, I want the dxCore.ts to configure the WASM serializer with the correct options, so that it outputs Human Format V3.

#### Acceptance Criteria

1. THE `WasmDxCore` class SHALL configure the serializer to disable box-drawing tables
2. THE `WasmDxCore` class SHALL configure the serializer to disable decorative comments
3. THE `WasmDxCore` class SHALL configure the serializer to use TOML-like output format
4. IF the WASM module doesn't support configuration, THE `WasmDxCore` SHALL use the TypeScript formatter as a workaround

### Requirement 3: Verify Format Consistency

**User Story:** As a developer, I want to verify that WASM and TypeScript produce the same output format.

#### Acceptance Criteria

1. THE extension SHALL log which core is being used (WASM or TypeScript)
2. FOR the same LLM input, WASM and TypeScript SHALL produce visually identical Human Format V3 output
3. THE output SHALL NOT contain any of these old format markers:
   - `# ════` (decorative comment headers)
   - `┌──────` (table top border)
   - `└──────` (table bottom border)
   - `│` (table cell borders)
   - `Total: X rows` (row count footer)

### Requirement 4: Human Format V3 Specification

**User Story:** As a developer, I want a clear specification of what Human Format V3 looks like.

#### Acceptance Criteria

1. Config values SHALL appear at the top WITHOUT `[config]` section header
2. Keys SHALL be padded to 20 characters for alignment
3. Arrays SHALL use ` | ` separator (space-pipe-space)
4. Strings with spaces SHALL be quoted with double quotes
5. Section headers SHALL use `[section_name]` format (e.g., `[forge]`, `[stack]`, `[i18n.locales]`)
6. References in `[stack]` section SHALL have aligned columns with ` | ` separators
7. NO decorative comments or Unicode box-drawing characters SHALL appear in output

**Example Human Format V3 Output:**
```
author              = essensefromexistence
description         = "Orchestrate don't just own your code"
editors             = neovim | zed | vscode | cursor | antigravity | replit | firebase-studio
name                = dx
title               = "Enhanced Developing Experience"
version             = 0.0.1
workspace           = @/www | @/backend

[stack]
js                  = javascript/typescript | bun    | tsc     | vite  | bun   | react
python              = py                    | python | python  | uv    | pip   | django
rust                = rs                    | rust   | native  | rustc | cargo | actix-web

[driven]
path                = @/driven

[i18n.locales]
path                = @/locales
default             = en-US
dev                 = en-US
prod                = all

[i18n.ttses]
path                = @/media/sounds
default             = en-US
dev                 = en-US
prod                = bn-BD

[icon]
path                = @/icons
pack                = Lucide
variant             = Hugeicons

[python.dependencies]
django              = latest
numpy               = latest

[style]
path                = @/style
themes              = dx

[font]
path                = @/fonts
default             = Inter

[ui]
path                = @/components/ui
components          = button
```

