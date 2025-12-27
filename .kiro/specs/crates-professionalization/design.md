# Design Document: Crates Professionalization

## Overview

This design document outlines the approach for professionalizing the `crates/` folder in the DX monorepo. The implementation will use a combination of automated validation scripts and manual documentation improvements to ensure all crates meet professional open-source standards.

The solution consists of:
1. A **validation tool** (Rust CLI) that audits crate structure and reports issues
2. **Template files** for consistent documentation
3. **Manual fixes** for content-specific issues (README rewrites, file relocations)

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                  Crates Professionalization                  │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐  │
│  │  Validator   │    │  Templates   │    │   Manual     │  │
│  │    Tool      │    │   & Docs     │    │   Fixes      │  │
│  └──────┬───────┘    └──────┬───────┘    └──────┬───────┘  │
│         │                   │                   │           │
│         ▼                   ▼                   ▼           │
│  ┌─────────────────────────────────────────────────────┐   │
│  │              Crates Directory Structure              │   │
│  │                                                      │   │
│  │  crates/                                             │   │
│  │  ├── {crate}/                                        │   │
│  │  │   ├── README.md      (standardized)              │   │
│  │  │   ├── Cargo.toml     (complete metadata)         │   │
│  │  │   ├── src/           (source code)               │   │
│  │  │   └── tests/         (optional)                  │   │
│  │  └── www/                                            │   │
│  │      ├── README.md      (index of subcrates)        │   │
│  │      └── {subcrate}/    (28 subcrates)              │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

## Components and Interfaces

### 1. Crate Validator Tool

A Rust CLI tool that scans the crates directory and reports compliance issues.

```rust
// crates/dx/src/commands/lint_crates.rs

pub struct CrateAuditResult {
    pub crate_path: PathBuf,
    pub issues: Vec<AuditIssue>,
    pub warnings: Vec<AuditWarning>,
}

pub enum AuditIssue {
    MissingReadme,
    MissingCargoToml,
    MissingSrcDir,
    MissingLicense,
    CargoLockInLibrary,
    MalformedDirectoryName(String),
    EmptyDocFile(PathBuf),
    ForbiddenFile(PathBuf),
    IncompleteCargoToml(Vec<String>), // missing fields
}

pub enum AuditWarning {
    NoDescription,
    NoBadges,
    LocalConfigFolder(String),
    DevelopmentArtifact(PathBuf),
}

pub trait CrateValidator {
    fn validate(&self, crate_path: &Path) -> CrateAuditResult;
    fn validate_all(&self, crates_dir: &Path) -> Vec<CrateAuditResult>;
}
```

### 2. README Template

Standard template for crate documentation:

```markdown
# {crate-name}

{brief-description}

[![Crates.io](https://img.shields.io/crates/v/{crate-name}.svg)](https://crates.io/crates/{crate-name})
[![Documentation](https://docs.rs/{crate-name}/badge.svg)](https://docs.rs/{crate-name})
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

## Overview

{detailed-description}

## Installation

```toml
[dependencies]
{crate-name} = "0.1"
```

## Usage

```rust
// Basic usage example
```

## Features

- Feature 1
- Feature 2

## API Reference

See [documentation](https://docs.rs/{crate-name}) for full API reference.

## License

MIT OR Apache-2.0 (see repository root LICENSE)
```

### 3. Cargo.toml Template

Required fields for all Cargo.toml files:

```toml
[package]
name = "{crate-name}"
version = "0.1.0"
edition = "2021"
authors = ["DX Team"]
description = "{brief-description}"
license = "MIT OR Apache-2.0"
repository = "https://github.com/anthropics/dx"
documentation = "https://docs.rs/{crate-name}"
readme = "README.md"

# For unpublished crates:
# publish = false
```

### 4. Directory Structure Standards

```
crates/
├── cli/                    # dx-cli - Main CLI binary
├── driven/                 # dx-driven - AI orchestrator
├── dx/                     # dx - Core CLI
├── font/                   # dx-font - Font optimization
├── forge/                  # dx-forge - Package manager
├── generator/              # dx-generator - Code generation
├── i18n/                   # dx-i18n - Internationalization
├── icon/                   # dx-icon - Icon system
├── javascript/             # JavaScript tooling
│   ├── README.md           # Index of JS subcrates
│   ├── bundler/
│   ├── compatibility/
│   ├── monorepo/
│   ├── package-manager/
│   ├── runtime/
│   └── test-runner/
├── media/                  # dx-media - Asset management
├── python/                 # Python tooling
│   ├── README.md
│   └── crates/
├── serializer/             # dx-serializer - Data format
├── style/                  # dx-style - CSS compiler
├── workspace/              # dx-workspace - IDE config
├── www/                    # Web framework
│   ├── README.md           # Index of 28 subcrates
│   ├── a11y/
│   ├── auth/
│   ├── ... (26 more)
│   └── sync/
└── check/                  # dx-check - Linting
```

## Data Models

### Validation Report Schema

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationReport {
    pub timestamp: DateTime<Utc>,
    pub total_crates: usize,
    pub compliant_crates: usize,
    pub issues_by_severity: HashMap<Severity, usize>,
    pub crate_results: Vec<CrateAuditResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Error,   // Must fix before merge
    Warning, // Should fix
    Info,    // Suggestion
}
```

### Required Files Checklist

| File | Required | Notes |
|------|----------|-------|
| README.md | Yes | Must follow template |
| Cargo.toml | Yes | Must have all required fields |
| src/ | Yes | Source directory |
| LICENSE | No | Can reference root LICENSE |
| CHANGELOG.md | No | Recommended for published crates |
| tests/ | No | Recommended |
| benches/ | No | Optional |
| examples/ | No | Recommended |

## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system—essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

### Property 1: Required Files Exist

*For any* crate directory in `crates/`, the directory SHALL contain at minimum: README.md, Cargo.toml, and src/ directory.

**Validates: Requirements 1.1, 2.1, 6.4**

### Property 2: README Contains Required Sections

*For any* README.md file in a crate, the file SHALL contain sections for: project name (H1), description, installation, and usage.

**Validates: Requirements 1.2**

### Property 3: No Forbidden Files in Library Crates

*For any* library crate (no `[[bin]]` section in Cargo.toml), the crate directory SHALL NOT contain: Cargo.lock, .env files, or empty .md files.

**Validates: Requirements 2.2, 4.4, 5.3**

### Property 4: Directory Names Are Valid

*For any* crate directory name, the name SHALL match the kebab-case pattern `^[a-z][a-z0-9]*(-[a-z0-9]+)*$` and SHALL NOT contain concatenated words without separators.

**Validates: Requirements 2.5, 3.1**

### Property 5: Cargo.toml Is Complete

*For any* Cargo.toml file, the file SHALL contain all required fields: name, version, edition, authors, description, and license.

**Validates: Requirements 4.2, 7.1, 7.2, 7.3**

### Property 6: License Information Present

*For any* crate, either a LICENSE file SHALL exist in the crate directory, OR the Cargo.toml SHALL contain a `license` field referencing a valid SPDX identifier.

**Validates: Requirements 4.1, 4.2**

### Property 7: No Development Artifacts in Crate Roots

*For any* crate directory, the directory SHALL NOT contain files matching patterns: `*PROGRESS*`, `*COMPLETE*`, `*TASKLIST*`, `*STATUS*` (case-insensitive).

**Validates: Requirements 5.1**

### Property 8: Parent Crates Document Subcrates

*For any* crate directory containing subdirectories with Cargo.toml files (subcrates), the parent crate's README.md SHALL contain a table or list documenting all subcrates.

**Validates: Requirements 1.5, 6.2**

## Error Handling

### Validation Errors

| Error Type | Handling |
|------------|----------|
| Missing required file | Report as Error severity, block CI |
| Incomplete Cargo.toml | Report as Error severity, list missing fields |
| Forbidden file present | Report as Warning, suggest removal |
| Malformed directory name | Report as Error, suggest fix |
| Empty documentation | Report as Warning, suggest removal or content |

### Recovery Strategies

1. **Auto-fix mode**: For simple issues (missing license field, Cargo.lock removal), offer `--fix` flag
2. **Template generation**: For missing READMEs, generate from template with placeholders
3. **Interactive mode**: For complex issues, prompt user for input

## Testing Strategy

### Unit Tests

- Test individual validation rules in isolation
- Test Cargo.toml parsing for required fields
- Test README section detection
- Test directory name validation regex

### Property-Based Tests

Using `proptest` crate with minimum 100 iterations per property:

1. **Property 1 test**: Generate random crate structures, verify required files detection
2. **Property 4 test**: Generate random directory names, verify kebab-case validation
3. **Property 5 test**: Generate random Cargo.toml content, verify field completeness check

### Integration Tests

- Run validator against actual crates/ directory
- Verify report generation
- Test fix mode on test fixtures

### Test Configuration

```rust
// Property test configuration
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    
    #[test]
    fn test_kebab_case_validation(name in "[a-z][a-z0-9-]*") {
        // Feature: crates-professionalization, Property 4: Directory Names Are Valid
        // Validates: Requirements 2.5, 3.1
        let result = validate_directory_name(&name);
        prop_assert!(result.is_ok() || name.contains("--") || name.ends_with("-"));
    }
}
```
