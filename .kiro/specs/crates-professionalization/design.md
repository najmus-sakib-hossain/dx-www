# Design Document: Crates Professionalization

## Overview

This design document outlines the approach for professionalizing the `crates/` folder in the DX monorepo. The implementation will use a combination of automated validation scripts and manual cleanup tasks to ensure consistent documentation, file structure, naming conventions, and quality standards across all 16+ top-level crates and 28+ www subcrates.

The solution follows a "validate-then-fix" approach: first creating validation tools to identify issues, then systematically addressing them while maintaining the validation as ongoing CI checks.

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    Crates Professionalization                    │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │  Validation  │  │   Template   │  │   Cleanup    │          │
│  │    Script    │  │   Generator  │  │    Script    │          │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘          │
│         │                 │                 │                   │
│  ┌──────▼─────────────────▼─────────────────▼──────┐           │
│  │              Crate Scanner Module                │           │
│  │  - Discovers all crates and subcrates            │           │
│  │  - Parses Cargo.toml files                       │           │
│  │  - Analyzes README content                       │           │
│  └──────────────────────┬──────────────────────────┘           │
│                         │                                       │
│  ┌──────────────────────▼──────────────────────────┐           │
│  │              Report Generator                    │           │
│  │  - Generates compliance reports                  │           │
│  │  - Outputs actionable fix lists                  │           │
│  └─────────────────────────────────────────────────┘           │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

## Components and Interfaces

### 1. Crate Scanner Module

Responsible for discovering and analyzing all crates in the repository.

```rust
/// Represents a discovered crate in the monorepo
pub struct CrateInfo {
    /// Path relative to repository root
    pub path: PathBuf,
    /// Crate name from Cargo.toml
    pub name: String,
    /// Whether this is a library or binary crate
    pub crate_type: CrateType,
    /// Whether this crate has subcrates
    pub has_subcrates: bool,
    /// List of files present in the crate
    pub files: Vec<String>,
    /// Parsed Cargo.toml metadata
    pub manifest: CargoManifest,
}

pub enum CrateType {
    Library,
    Binary,
    Mixed,
}

/// Scans the crates directory and returns all crate information
pub fn scan_crates(root: &Path) -> Result<Vec<CrateInfo>>;

/// Checks if a crate is a library (no [[bin]] sections, lib target)
pub fn is_library_crate(manifest: &CargoManifest) -> bool;
```

### 2. Validation Module

Performs compliance checks against the professionalization requirements.

```rust
pub struct ValidationResult {
    pub crate_path: PathBuf,
    pub issues: Vec<ValidationIssue>,
    pub passed: bool,
}

pub struct ValidationIssue {
    pub severity: Severity,
    pub category: IssueCategory,
    pub message: String,
    pub fix_suggestion: Option<String>,
}

pub enum Severity {
    Error,   // Must fix
    Warning, // Should fix
    Info,    // Nice to have
}

pub enum IssueCategory {
    MissingFile,
    IncompleteReadme,
    NamingConvention,
    LicenseCompliance,
    DevelopmentArtifact,
    CargoMetadata,
}

/// Validates a single crate against all requirements
pub fn validate_crate(crate_info: &CrateInfo) -> ValidationResult;

/// Validates all crates and returns aggregated results
pub fn validate_all(crates: &[CrateInfo]) -> Vec<ValidationResult>;
```

### 3. README Template System

Provides templates for generating consistent README files.

```markdown
# {crate_name}

{description}

[![Crates.io](https://img.shields.io/crates/v/{crate_name}.svg)](https://crates.io/crates/{crate_name})
[![Documentation](https://docs.rs/{crate_name}/badge.svg)](https://docs.rs/{crate_name})
[![License](https://img.shields.io/badge/license-{license}-blue.svg)](LICENSE)

## Overview

{overview}

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
{crate_name} = "{version}"
```

## Usage

{usage_examples}

## License

{license_text}
```

### 4. Cleanup Script

Handles removal and relocation of development artifacts.

```rust
/// Files/patterns to remove from crate directories
const CLEANUP_PATTERNS: &[&str] = &[
    "PHASE*_*.md",
    "TASKLIST.md",
    "PROGRESS.md",
    ".env",
    "Cargo.lock", // Only for library crates
];

/// Directories to consolidate to root
const CONSOLIDATE_DIRS: &[&str] = &[
    ".kiro",
    ".dx", 
    ".github",
];

/// Performs cleanup on a single crate
pub fn cleanup_crate(crate_info: &CrateInfo, dry_run: bool) -> CleanupResult;
```

## Data Models

### CargoManifest Structure

```rust
pub struct CargoManifest {
    pub package: PackageInfo,
    pub dependencies: HashMap<String, Dependency>,
    pub lib: Option<LibTarget>,
    pub bin: Vec<BinTarget>,
}

pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub edition: Option<String>,
    pub authors: Option<Vec<String>>,
    pub description: Option<String>,
    pub license: Option<String>,
    pub repository: Option<String>,
    pub documentation: Option<String>,
    pub publish: Option<bool>,
}
```

### README Analysis Structure

```rust
pub struct ReadmeAnalysis {
    pub has_title: bool,
    pub has_description: bool,
    pub has_badges: bool,
    pub has_installation: bool,
    pub has_usage: bool,
    pub has_license: bool,
    pub has_subcrate_table: bool,
    pub contains_task_instructions: bool,
    pub contains_raw_prompts: bool,
    pub badge_format: Option<BadgeFormat>,
}

pub enum BadgeFormat {
    ShieldsIo,
    BadgenNet,
    Custom,
    Inconsistent,
}
```

## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system—essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

### Property 1: Required Files Existence

*For any* crate directory in the monorepo, the directory SHALL contain at minimum: README.md, Cargo.toml, and either a src/ directory or a LICENSE file (or reference to root LICENSE).

**Validates: Requirements 1.1, 2.1**

### Property 2: README Content Completeness

*For any* README.md file in a crate, parsing the content SHALL identify the presence of: project name (H1 heading), description paragraph, installation section, usage/examples section, and license information.

**Validates: Requirements 1.2**

### Property 3: Badge Format Consistency

*For any* set of README.md files across all crates, if badges are present, they SHALL follow the same format pattern (shields.io style with consistent ordering).

**Validates: Requirements 1.3**

### Property 4: No Development Instructions in README

*For any* README.md file, scanning for patterns like "please read", "create tasklist", "step by step", "TODO:", or AI prompt markers SHALL return no matches.

**Validates: Requirements 1.4**

### Property 5: Subcrate Documentation

*For any* crate that contains subdirectories with Cargo.toml files (subcrates), the parent crate's README SHALL contain a table or list documenting all subcrates.

**Validates: Requirements 1.5**

### Property 6: No Cargo.lock in Libraries

*For any* crate classified as a library (has lib target, no bin targets), the crate directory SHALL NOT contain a Cargo.lock file.

**Validates: Requirements 2.2**

### Property 7: No Scattered Config Directories

*For any* crate directory (excluding root), the directory SHALL NOT contain .kiro/, .dx/, or .github/ subdirectories unless documented in README.

**Validates: Requirements 2.3, 2.4**

### Property 8: Valid Directory Names

*For any* crate directory name, the name SHALL match the pattern `^[a-z][a-z0-9-]*$` (kebab-case, starting with letter).

**Validates: Requirements 2.5, 3.1**

### Property 9: Name Consistency

*For any* crate, the directory name SHALL match the package name in Cargo.toml (with underscores converted to hyphens).

**Validates: Requirements 3.3**

### Property 10: License Compliance

*For any* crate, either a LICENSE file SHALL exist in the crate directory, OR the Cargo.toml SHALL contain a license field, OR the README SHALL reference the root LICENSE.

**Validates: Requirements 4.1, 4.2**

### Property 11: No Sensitive Files

*For any* crate directory, files matching patterns `.env`, `.env.*` (excluding `.env.example`) SHALL NOT exist.

**Validates: Requirements 4.4**

### Property 12: No Progress Tracking Files

*For any* crate directory, files matching patterns `PHASE*.md`, `TASKLIST.md`, `PROGRESS.md`, `*_COMPLETE.md`, `*_STATUS.md` SHALL NOT exist in the root of the crate.

**Validates: Requirements 5.1**

### Property 13: No Empty Documentation Files

*For any* markdown file in a crate directory, the file size SHALL be greater than 0 bytes and contain meaningful content (not just whitespace).

**Validates: Requirements 5.3**

### Property 14: WWW Subcrate Completeness

*For any* subdirectory in crates/www/, the directory SHALL contain both README.md and Cargo.toml files.

**Validates: Requirements 6.4**

### Property 15: Cargo.toml Required Fields

*For any* Cargo.toml file, the [package] section SHALL contain: name, version, edition, and description fields.

**Validates: Requirements 7.1**

### Property 16: Version Format Consistency

*For any* set of Cargo.toml files, all version fields SHALL follow semantic versioning format (X.Y.Z) with consistent precision.

**Validates: Requirements 7.2**

## Error Handling

### Validation Errors

| Error Type | Handling Strategy |
|------------|-------------------|
| Missing Cargo.toml | Skip directory (not a crate) |
| Malformed Cargo.toml | Report as Error, provide parse error details |
| Missing README | Report as Error, suggest template |
| Incomplete README | Report as Warning, list missing sections |
| Naming violation | Report as Error, suggest correct name |
| Stray files | Report as Warning, suggest removal |

### Script Execution Errors

- File permission errors: Log and continue with other crates
- Parse errors: Report with line numbers and context
- I/O errors: Retry once, then report and continue

## Testing Strategy

### Unit Tests

Unit tests will verify specific validation logic:

- Cargo.toml parsing for various formats
- README section detection algorithms
- Pattern matching for cleanup targets
- Name validation regex patterns

### Property-Based Tests

Property-based tests will use `proptest` to verify universal properties:

- **Property tests run minimum 100 iterations**
- Each test tagged with: **Feature: crates-professionalization, Property N: {description}**

Test categories:
1. File existence properties (1, 6, 11, 12, 13, 14)
2. Content validation properties (2, 3, 4, 5)
3. Naming properties (8, 9)
4. Metadata properties (10, 15, 16)

### Integration Tests

- Run full validation on test fixture crates
- Verify cleanup script in dry-run mode
- Test template generation output

## Implementation Notes

### Phase 1: Validation Infrastructure
Create the scanning and validation modules first to establish a baseline of current issues.

### Phase 2: Template System
Build README templates and Cargo.toml templates for consistent generation.

### Phase 3: Cleanup Execution
Run cleanup scripts with dry-run first, then execute fixes.

### Phase 4: Manual Fixes
Address issues requiring human judgment (content writing, organization decisions).

### Phase 5: CI Integration
Add validation as a CI check to prevent regression.

## File Changes Summary

### New Files
- `scripts/validate-crates.rs` - Main validation script
- `scripts/cleanup-crates.rs` - Cleanup automation
- `templates/README.template.md` - README template
- `templates/Cargo.template.toml` - Cargo.toml template
- `crates/www/README.md` - WWW subcrates documentation

### Modified Files
- Multiple `README.md` files across crates
- Multiple `Cargo.toml` files for metadata completion
- `.gitignore` - Add patterns for development artifacts

### Deleted/Relocated Files
- `crates/check/PHASE3_*.md` → `docs/archive/`
- `crates/check/TASKLIST.md` → `docs/archive/`
- `crates/forge/.env` → Remove (use .env.example only)
- `crates/python/COMPABILITY.md` → Rename to COMPATIBILITY.md
- Various `Cargo.lock` files in library crates
