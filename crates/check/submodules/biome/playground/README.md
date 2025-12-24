# Biome Playground

This directory contains sample files for testing Biome CLI with all supported languages, including the new TOML support via Taplo integration.

## Files

- `sample.js` - JavaScript file with various ES6+ features
- `sample.ts` - TypeScript file with types, interfaces, and generics
- `sample.json` - JSON file (package.json example)
- `sample.css` - CSS file with modern features, variables, and media queries
- `sample.graphql` - GraphQL schema with types, queries, and mutations
- `sample.html` - HTML5 file with semantic markup
- `sample.toml` - TOML file with comprehensive examples (formatted by Taplo)

## Usage

### Format all files in the playground

```bash
biome format playground/
```

### Lint all files in the playground

```bash
biome lint playground/
```

### Check all files (format + lint)

```bash
biome check playground/
```

### Format specific file types

```bash
# Format only TOML files
biome format playground/*.toml

# Format only JavaScript files
biome format playground/*.js

# Format only CSS files
biome format playground/*.css
```

## TOML Support

The `sample.toml` file demonstrates the new TOML formatting and linting capabilities powered by Taplo. Biome CLI now automatically detects `.toml` files and processes them using the Taplo formatter and linter.

### TOML Features Tested

- Basic key-value pairs
- Tables and nested tables
- Arrays and table arrays
- Inline tables
- Multi-line strings
- Various data types (strings, integers, floats, booleans, dates)
- Comments

### Testing TOML Formatting

```bash
# Check if TOML file needs formatting
biome format --check playground/sample.toml

# Format TOML file
biome format --write playground/sample.toml
```

### Testing TOML Linting

```bash
# Lint TOML file for syntax and semantic errors
biome lint playground/sample.toml

# Check TOML file (lint + format)
biome check playground/sample.toml
```

## Integration Details

Biome CLI integrates Taplo for TOML processing through:

1. **Workspace Dependencies**: Taplo crates are added as workspace dependencies in `Cargo.toml`
2. **Format Module**: `src/execute/process_file/format.rs` detects `.toml` extensions and routes to Taplo
3. **Lint Module**: `src/execute/process_file/lint_and_assist.rs` detects `.toml` extensions and routes to Taplo
4. **Check Module**: `src/execute/process_file/check.rs` combines both format and lint for TOML files

## Expected Behavior

- `.toml` files should be formatted according to Taplo's default formatting rules
- Syntax errors in TOML files should be reported by Taplo's parser
- Semantic errors (duplicate keys, invalid table structures) should be caught
- The formatting is consistent and idempotent (formatting twice produces the same result)
