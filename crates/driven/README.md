# Driven

**AI-Assisted Development Orchestrator**

A binary-first system for managing AI coding assistant rules across multiple editors (Cursor, Copilot, Windsurf, Claude, Aider, Cline).

## Overview

Driven solves the fragmentation problem in AI-assisted development. Instead of maintaining separate rule files for each editor:

- `.cursor/rules/*.mdc` (Cursor)
- `.github/copilot-instructions.md` (Copilot)
- `.windsurf/rules/*.md` (Windsurf)
- `CLAUDE.md` (Claude)
- `.aider.conf.yml` (Aider)
- `.clinerules` (Cline)

Driven provides a **single source of truth** that synchronizes to all editors.

## Features

- **Binary Format** (.drv) - Zero-copy, Ed25519 signed, 73% smaller than JSON
- **Multi-Editor Sync** - One file, all editors stay in sync
- **Context Intelligence** - Automatically extracts project patterns
- **Template Library** - Built-in personas, standards, and workflows
- **Rule Validation** - Linting, conflict detection, coverage analysis
- **Watch Mode** - Auto-sync on file changes

## Installation

```bash
# From source
cargo install --path crates/driven

# Or add to your project
cargo add driven
```

## Quick Start

```bash
# Initialize driven in your project
driven init

# Interactive setup
driven init -i

# Sync rules to all editors
driven sync

# Watch mode (auto-sync)
driven sync --watch

# Analyze project for context
driven analyze

# Validate rules
driven validate
```

## Configuration

`.driven/config.toml`:

```toml
[editors]
cursor = true
copilot = true
windsurf = false
claude = false
aider = false
cline = false

[sync]
source_of_truth = ".driven/rules.drv"
watch = true
debounce_ms = 500

[templates]
project = "rust-workspace"
personas = ["architect"]
standards = ["rust-idioms"]

[context]
auto_detect = true
index_on_save = true
```

## Rule Format

### Markdown (Human-Readable)

```markdown
# AI Development Rules

## Persona

You are an expert Rust engineer...

### Traits
- Precise and detail-oriented
- Security-conscious

### Principles
- Zero-copy where possible
- No unwrap() in library code

## Standards

### Style
- Use rustfmt defaults
- Max line length: 100

### Naming
- snake_case for functions
- PascalCase for types

### Error Handling
- Use thiserror for library errors
- Use anyhow for application errors

## Context

### Focus
- src/
- crates/

### Exclude
- target/
- .git/
```

### Binary Format (.drv)

See [BINARY_FORMAT.md](./BINARY_FORMAT.md) for the complete specification.

## Templates

### Personas
- `architect` - System design focus
- `reviewer` - Code review specialist
- `documenter` - Documentation expert
- `security` - Security auditor
- `performance` - Optimization expert

### Projects
- `rust-workspace` - Rust Cargo workspace
- `typescript-monorepo` - TypeScript/Node.js
- `fullstack` - Rust + TypeScript
- `cli-tool` - Command-line application
- `library` - Reusable library

### Standards
- `rust-idioms` - Idiomatic Rust patterns
- `error-handling` - Error handling best practices
- `testing` - Testing standards
- `documentation` - Documentation standards

### Workflows
- `tdd` - Test-Driven Development
- `feature-development` - Feature implementation
- `bug-fixing` - Bug fix workflow
- `refactoring` - Safe refactoring

## CLI Commands

```bash
# Initialize
driven init              # Default setup
driven init -i           # Interactive
driven init --template rust-workspace

# Sync
driven sync              # One-time sync
driven sync --watch      # Watch mode
driven sync --dry-run    # Preview changes

# Convert
driven convert input.md output.drv    # To binary
driven convert input.drv output.md    # To markdown
driven convert input.drv --editor cursor  # To Cursor format

# Templates
driven template list     # List available
driven template search rust  # Search
driven template apply rust-idioms  # Apply

# Analyze
driven analyze           # Project analysis
driven analyze --context # Generate context rules
driven analyze --index   # Build codebase index

# Validate
driven validate          # Check rules
driven validate --strict # Fail on warnings
```

## Library Usage

```rust
use driven::{DrivenConfig, RuleSet, Editor};

// Load rules
let rules = RuleSet::load(".driven/rules.drv")?;

// Emit to specific editor
rules.emit(Editor::Cursor, ".cursor/rules/")?;

// Sync to all configured editors
let config = DrivenConfig::load(".driven/config.toml")?;
let engine = SyncEngine::new(&config);
engine.sync(".")?;

// Validate rules
let result = driven::validation::validate(&rules.as_unified())?;
if !result.is_valid() {
    eprintln!("Validation failed: {:?}", result);
}
```

## Architecture

```
driven/
├── format/      # Binary .drv format (encoder/decoder)
├── parser/      # Parse rules from any editor format
├── emitter/     # Generate editor-specific files
├── templates/   # Built-in template library
├── context/     # Project analysis and indexing
├── sync/        # Multi-editor synchronization
├── validation/  # Rule linting and conflict detection
└── cli/         # Command-line interface
```

## Performance

| Metric | Value |
|--------|-------|
| Parse time | 0ms (zero-copy) |
| Sync (5 editors) | <50ms |
| Binary size | 73% smaller than JSON |
| Index (10K files) | <2s |

## Security

- **Ed25519 Signatures** - All .drv files are signed
- **Compile-Time Validation** - Rules validated at build time
- **No Runtime Eval** - No dynamic code execution

## Contributing

See [CONTRIBUTING.md](../../CONTRIBUTING.md) for guidelines.

## License

MIT OR Apache-2.0
