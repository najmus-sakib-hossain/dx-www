# DX Serializer Human Format Specification

> **Version:** 3.0.0 | **Purpose:** Human-readable view of DX LLM format | **Extension:** `.dx` (same file, dual view)

## Overview

DX Serializer supports **dual-mode rendering**: the same `.dx` file can be displayed as either:
- **LLM Mode:** Ultra-compact for token efficiency (3x better than TOON)
- **Human Mode:** Clean vertical key-value format with aligned equals signs

The file is **stored in LLM format** but **displayed in Human format** in editors.

## Format Comparison

### LLM Format (Storage)

```dx
#c:nm|dx;v|0.0.1;tt|Enhanced Developing Experience
#:js|javascript/typescript|bun|tsc|vite|bun|react
#f(nm|repo|container)
forge|https://dx.vercel.app/essensefromexistence/dx|none
#k(nm|pt|engine|themes)
stack|@/stack|atomic|*dx,vercel,claude
```

### Human Format (Display)

```
name                 = dx
version              = 0.0.1
title                = "Enhanced Developing Experience"

[stack]              = Lang | Runtime | Compiler | Bundler | PM | Framework
js                   = javascript/typescript | bun | tsc | vite | bun | react

[forge]              = Name | Repository | Container
forge                = https://dx.vercel.app/essensefromexistence/dx | none

[stack]              = Name | Path | Engine | Themes
stack                = @/stack | atomic | dx | vercel | claude
```

## Human Format Features

### 1. No [config] Section Header

Config values appear at the top without any section header:

```
name                 = dx
version              = 0.0.1
title                = "Enhanced Developing Experience"
description          = "Orchestrate don't just own your code"
```

### 2. Key Padding and Alignment

All keys are padded to 20 characters (or longest key + 1) to align the `=` signs:

```
name                 = dx
version              = 0.0.1
title                = "Enhanced Developing Experience"
```

### 3. Pipe (|) Array Separator

Arrays use ` | ` (space-pipe-space) as separator instead of commas:

```
workspace            = frontend/www | frontend/mobile | backend/api
editors              = vscode | vim | cursor | windsurf
```

### 4. Section Headers with Schema

Data sections show `[section]` header with optional schema:

```
[stack]              = Lang | Runtime | Compiler
javascript           = js | bun | tsc
python               = py | cpython | -
```

### 5. Quoted Strings

Strings containing spaces are automatically quoted:

```
title                = "Enhanced Developing Experience"
description          = "Orchestrate don't just own your code"
```

### 6. Vertical Key-Value Layout

Each row in a data section is displayed as key = values:

```
[forge]
repository           = https://dx.vercel.app/essensefromexistence/dx
container            = none
```

## Key Abbreviation Dictionary

### Standard Abbreviations (Auto-Expand)

| Abbrev | Full Name   | Category |
|--------|-------------|----------|
| `nm`   | name        | Identity |
| `tt`   | title       | Identity |
| `ds`   | description | Identity |
| `id`   | id          | Identity |
| `v`    | version     | Version  |
| `au`   | author      | Author   |
| `ws`   | workspace   | Workspace |
| `ed`   | editors     | Editors  |
| `repo` | repository  | Repository |
| `cont` | container   | Container |
| `ci`   | ci_cd       | CI/CD    |
| `st`   | status      | State    |
| `ac`   | active      | State    |
| `en`   | enabled     | State    |
| `ct`   | count       | Metrics  |
| `tl`   | total       | Metrics  |
| `pr`   | price       | Metrics  |
| `em`   | email       | Contact  |
| `ph`   | phone       | Contact  |
| `ur`   | url         | Web      |
| `pt`   | path        | Web      |

### Section Name Mappings

| LLM ID | Full Name |
|--------|-----------|
| `c`    | config    |
| `f`    | forge     |
| `k`    | stack     |
| `y`    | style     |
| `u`    | ui        |
| `m`    | media     |
| `i`    | i18n      |
| `o`    | icon      |
| `t`    | font      |
| `d`    | data      |

## Complete Example

### LLM Format (on disk)

```dx
#c:nm|dx;v|0.0.1;tt|Enhanced Developing Experience;ds|Orchestrate don't just own your code;au|essensefromexistence;ws|*frontend/www,frontend/mobile;ed|*vscode,vim,cursor
#:js|javascript/typescript|bun|tsc|vite|bun|react;py|python|cpython|-|-|uv|django
#f(nm|repo|container|ci_cd)
forge|https://dx.vercel.app/essensefromexistence/dx|none|none
#k(nm|pt|engine|themes)
stack|@/stack|atomic|*dx,vercel,claude
```

### Human Format (in editor)

```
name                 = dx
version              = 0.0.1
title                = "Enhanced Developing Experience"
description          = "Orchestrate don't just own your code"
author               = essensefromexistence
workspace            = frontend/www | frontend/mobile
editors              = vscode | vim | cursor

[stack]              = Lang | Runtime | Compiler | Bundler | PM | Framework
js                   = javascript/typescript | bun | tsc | vite | bun | react
py                   = python | cpython | - | - | uv | django

[forge]              = Name | Repository | Container | CI_CD
forge                = https://dx.vercel.app/essensefromexistence/dx | none | none

[stack]              = Name | Path | Engine | Themes
stack                = @/stack | atomic | dx | vercel | claude
```

## Multi-Format Input Support

The DX Serializer extension can automatically convert these formats to DX:

### JSON Input

```json
{
  "name": "my-project",
  "version": "1.0.0",
  "dependencies": ["react", "typescript"]
}
```

Converts to:
```
name                 = my-project
version              = 1.0.0
dependencies         = react | typescript
```

### YAML Input

```yaml
name: my-project
version: 1.0.0
dependencies:
  - react
  - typescript
```

### TOML Input

```toml
[project]
name = "my-project"
version = "1.0.0"
```

### CSV Input

```csv
id,name,active
1,Alpha,true
2,Beta,false
```

Converts to:
```
[data]               = Id | Name | Active
1                    = Alpha | true
2                    = Beta | false
```

## Cache Files

When a `.dx` file is saved, the extension generates cache files:

- `.dx/cache/{filename}.human` - Human format for quick display
- `.dx/cache/{filename}.machine` - JSON format for efficient parsing

Cache files preserve subdirectory structure:
- `src/config.dx` → `.dx/cache/src/config.human` and `.dx/cache/src/config.machine`

## Related

- [LLM Format](./llm-format.md)
- [Machine Format](./machine-format.md)
- [Serializer Overview](./README.md)
