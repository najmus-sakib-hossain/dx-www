# DX Serializer Human Format V2

## Overview

Human Format V2 is a clean, minimal configuration format that provides:
- **Flat TOML-like structure** (no indentation anywhere)
- **Full key name expansion** (no cryptic abbreviations)
- **Full section names** in brackets (`[forge]` instead of `[f]`)
- **Comma-separated arrays** without brackets
- **Flat data tables** with column alignment (no borders, no indentation)

## VS Code / Kiro Extension

The DX Serializer VS Code extension provides seamless editing of `.dx` files with Human Format V2:

- **Install**: `kiro --install-extension crates/vscode-dx-serializer/vscode-dx-serializer-0.1.0.vsix`
- **Features**: Syntax highlighting, real-time validation, auto-save compatible
- **Dual Format**: Edit human-readable V2 format while storing token-efficient LLM format on disk

See **[Extension README](crates/vscode-dx-serializer/README.md)** for full documentation.

## Format Specification

### Config Section

```toml
[config]
name        = MyProject
version     = 1.0.0
title       = Enhanced Developing Experience
description = Orchestrate don't just own your code
author      = essensefromexistence
workspace   = frontend/www, frontend/mobile
editors     = vscode, vim, gitpod, github-codespace, replit
```

### Data Sections

```toml
[forge]
name   repository                                      container  ci_cd
forge  https://dx.vercel.app/essensefromexistence/dx  none       none
```

## Key Features

### 1. Full Key Name Expansion

| Abbreviated | Full Name   |
|-------------|-------------|
| `nm`        | `name`      |
| `v`         | `version`   |
| `au`        | `author`    |
| `ws`        | `workspace` |
| `ed`        | `editors`   |
| `repo`      | `repository`|
| `cont`      | `container` |
| `ci`        | `ci_cd`     |

### 2. Flat Structure (No Indentation)

```toml
[config]
name    = MyProject
version = 1.0.0
```

### 3. Comma-Separated Arrays

```toml
workspace = frontend/www, frontend/mobile, backend/api
```

### 4. Full Section Names

```toml
[forge]
[users]
[data]
```

### 5. Flat Data Tables

Data sections use flat rows with column alignment (2+ spaces as delimiter):

```toml
[users]
id  name   role
1   Alice  admin
2   Bob    developer
```

## Example Output

**LLM format (on disk):**
```dx
#c:nm|dx;v|0.0.1;au|essensefromexistence
#f(nm|repo|cont|ci)
forge|https://dx.vercel.app/essensefromexistence/dx|none|none
```

**Human Format V2 (in editor):**
```toml
[config]
name    = dx
version = 0.0.1
author  = essensefromexistence

[forge]
name   repository                                      container  ci_cd
forge  https://dx.vercel.app/essensefromexistence/dx  none       none
```

## Testing

Run tests:
```bash
# TypeScript extension tests
cd crates/vscode-dx-serializer
node out/humanFormatter.test.js
node out/humanParser.test.js
node out/dxCore.test.js
```
