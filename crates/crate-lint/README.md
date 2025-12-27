# dx-crate-lint

[![Crates.io](https://img.shields.io/crates/v/dx-crate-lint.svg)](https://crates.io/crates/dx-crate-lint)
[![Documentation](https://docs.rs/dx-crate-lint/badge.svg)](https://docs.rs/dx-crate-lint)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

Validation tool for DX ecosystem crate standards and conventions.

## Overview

`dx-crate-lint` ensures all crates in the DX ecosystem follow consistent standards for:

- Cargo.toml metadata and workspace inheritance
- Package naming conventions (dx-* prefix)
- Documentation files (README, CHANGELOG, LICENSE)
- Directory structure
- Dependency management

## Features

- **Metadata Validation**: Ensures workspace inheritance and required fields
- **Naming Enforcement**: Validates dx-{name} and dx-www-{name} patterns
- **Documentation Checks**: Verifies README sections, badges, and CHANGELOG format
- **License Validation**: Confirms proper MIT OR Apache-2.0 licensing
- **Structure Validation**: Checks src/ directory and required files
- **Dependency Analysis**: Detects version conflicts and workspace consistency
- **Auto-Fix**: Generates fixes for common violations
- **Multiple Output Formats**: JSON, Markdown, and terminal output

## Installation

```bash
cargo install dx-crate-lint
```

Or add to your project:

```toml
[dev-dependencies]
dx-crate-lint = "0.1"
```

## Usage

```bash
# Lint the current workspace
dx-crate-lint lint

# Lint with specific output format
dx-crate-lint lint --format json
dx-crate-lint lint --format markdown

# Generate a detailed report
dx-crate-lint report --output report.md

# Auto-fix violations (with confirmation)
dx-crate-lint fix

# Fix without confirmation (CI mode)
dx-crate-lint fix --yes
```

## Validation Rules

### Metadata (Cargo.toml)

| Rule | Description |
|------|-------------|
| `version.workspace` | Must use workspace inheritance |
| `edition.workspace` | Must use workspace inheritance |
| `authors.workspace` | Must use workspace inheritance |
| `license.workspace` | Must use workspace inheritance |
| `repository.workspace` | Must use workspace inheritance |
| `keywords` | Must have 1-5 relevant terms |
| `categories` | Must match crates.io categories |
| `description` | Required, 1-2 sentences |

### Naming Conventions

| Location | Pattern |
|----------|---------|
| Top-level tools | `dx-{name}` |
| WWW modules | `dx-www-{name}` |
| Library names | `dx_{name}` (underscores) |

### Required Files

- `README.md` with: title, badges, overview, features, installation, usage, license
- `CHANGELOG.md` following Keep a Changelog format
- `LICENSE` or `LICENSE-MIT` + `LICENSE-APACHE`

## Library Usage

```rust
use dx_crate_lint::{CrateScanner, MetadataValidator, ValidationReport};

// Scan workspace
let scanner = CrateScanner::new(".")?;
let crates = scanner.scan()?;

// Validate metadata
let validator = MetadataValidator::new(&scanner.workspace_config());
for crate_info in &crates {
    let violations = validator.validate(crate_info);
    for v in violations {
        println!("{}: {}", v.crate_name, v.message);
    }
}
```

## CI Integration

Add to your CI pipeline:

```yaml
- name: Lint crates
  run: dx-crate-lint lint --format json > lint-report.json

- name: Check for violations
  run: |
    if [ $(jq '.summary.total_violations' lint-report.json) -gt 0 ]; then
      exit 1
    fi
```

## License

This project is dual-licensed under MIT OR Apache-2.0.
