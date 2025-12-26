# Implementation Plan: Human Format V3 Fix

## Overview

This implementation plan fixes the Human Format V3 output inconsistency. The WASM module outputs the OLD format with decorative comments and Unicode tables, while the TypeScript fallback outputs the correct TOML-like format. The fix is surgical - update `WasmDxCore.toHuman()` to use the TypeScript formatter.

**CRITICAL**: Do NOT modify dxDocumentManager.ts, dxLensFileSystem.ts, extension.ts, or cacheManager.ts.

## Tasks

- [x] 1. Fix WasmDxCore.toHuman() to use TypeScript formatter
  - [x] 1.1 Update WasmDxCore.toHuman() method in dxCore.ts
    - Change from calling `this.serializer.toHuman()` to calling `formatDx()`
    - This ensures TOML-like output instead of old format with Unicode tables
    - Keep WASM for toDense() and validate() - they work correctly
    - _Requirements: 1.1-1.6, 2.4_

- [x] 2. Verify the fix works correctly
  - [x] 2.1 Test that opening a .dx file shows Human Format V3
    - Open the `dx` file in the workspace
    - Verify NO decorative comments (`# ════...`)
    - Verify NO Unicode box-drawing tables (`┌──────┐`)
    - Verify clean TOML-like format with `[section]` headers
    - _Requirements: 1.1-1.6, 3.3_

- [x] 3. Add format validation test
  - [x] 3.1 Create test to verify no old format markers in output
    - Test that output contains no `# ═` patterns
    - Test that output contains no Unicode box-drawing characters
    - Test that output uses `[section]` headers
    - Test that output uses ` | ` array separator
    - _Requirements: 3.3, Property 1-5_

- [x] 4. Checkpoint - Verify extension works correctly
  - Open a .dx file and verify Human Format V3 is displayed
  - Edit and save the file, verify round-trip works
  - Verify WASM is still used for validation (check console logs)
  - Ask the user if questions arise

## Notes

- This is a surgical fix - only one method in dxCore.ts is changed
- WASM is still used for toDense() and validate() to preserve security limits
- The TypeScript formatter (formatDocumentV3) already outputs the correct format
- No changes to the complex auto-save, document manager, or file system provider
- The fix ensures consistent output regardless of whether WASM loads successfully

## Expected Output After Fix

When opening the `dx` file, users should see:

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

NOT the old format with:
- `# ════════════════════════════════════════════════════════════════════════════════`
- `#                                CONFIGURATION`
- `┌──────────┐`
- `│ path     │`
- `└──────────┘`
- `Total: 1 rows`

