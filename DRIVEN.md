Please learn from integrations/Speck-Kit and integrations/BMAD_METHOD and create a new rust crate at crates/ folder to create driven crate.

Here is the planning:
```markdown
# Driven Crate Design Document

## Vision Statement

**Driven** is a professional AI-assisted development orchestrator that brings structure, consistency, and intelligence to AI-powered coding workflows. It combines the template-driven approach of Speck-Kit with the methodical framework of BMAD_METHOD, reimagined in Rust with DX's binary-first philosophy for unparalleled performance and developer experience.

---

## Core Philosophy

### The Problem
- AI agents lack project context and coding standards awareness
- Developers repeat the same instructions across different AI tools
- No unified format for editor-agnostic AI rules
- Text-based rule formats are slow to parse and bloated
- Switching between Cursor, Copilot, Windsurf requires rewriting rules

### The Driven Solution
- **Universal Rule Format**: One source of truth, convert to any editor
- **Binary-First Storage**: Using DX serializer for 70%+ size reduction
- **Context Intelligence**: Deep project analysis for AI guidance
- **Professional Templates**: Battle-tested patterns for AI agents
- **Zero-Parse Loading**: Instant rule loading with memory-mapped binaries

---

## Architecture Overview

```
driven/
├── Cargo.toml
├── src/
│   ├── lib.rs                      # Public API surface
│   │
│   ├── format/                     # Binary Rule Format (.drv)
│   │   ├── mod.rs
│   │   ├── schema.rs               # Binary schema definitions
│   │   ├── encoder.rs              # Rule → Binary encoding
│   │   ├── decoder.rs              # Binary → Rule decoding
│   │   └── versioning.rs           # Format version handling
│   │
│   ├── parser/                     # Universal Rule Parser
│   │   ├── mod.rs
│   │   ├── cursor.rs               # .cursorrules parser
│   │   ├── copilot.rs              # copilot-instructions.md parser
│   │   ├── windsurf.rs             # .windsurfrules parser
│   │   ├── claude.rs               # .claude/ folder parser
│   │   ├── aider.rs                # .aider files parser
│   │   ├── cline.rs                # Cline config parser
│   │   └── unified.rs              # Unified AST representation
│   │
│   ├── emitter/                    # Target Format Generators
│   │   ├── mod.rs
│   │   ├── cursor.rs               # Emit .cursorrules
│   │   ├── copilot.rs              # Emit copilot-instructions.md
│   │   ├── windsurf.rs             # Emit .windsurfrules
│   │   ├── claude.rs               # Emit .claude/ structure
│   │   └── generic.rs              # Emit generic markdown
│   │
│   ├── templates/                  # Template System
│   │   ├── mod.rs
│   │   ├── registry.rs             # Template registry & discovery
│   │   ├── persona.rs              # AI persona templates
│   │   ├── project.rs              # Project structure templates
│   │   ├── standards.rs            # Coding standards templates
│   │   ├── workflow.rs             # Development workflow templates
│   │   ├── task.rs                 # Task-specific templates
│   │   └── composer.rs             # Template composition engine
│   │
│   ├── context/                    # AI Context Intelligence
│   │   ├── mod.rs
│   │   ├── scanner.rs              # Project structure scanner
│   │   ├── analyzer.rs             # Pattern & convention analyzer
│   │   ├── extractor.rs            # Convention extraction engine
│   │   ├── indexer.rs              # Codebase indexer (binary format)
│   │   └── provider.rs             # Context provision API
│   │
│   ├── sync/                       # Multi-Editor Synchronization
│   │   ├── mod.rs
│   │   ├── watcher.rs              # File system watcher
│   │   ├── differ.rs               # Rule difference calculator
│   │   └── propagator.rs           # Change propagation engine
│   │
│   ├── validation/                 # Rule Validation
│   │   ├── mod.rs
│   │   ├── linter.rs               # Rule linting & suggestions
│   │   ├── conflicts.rs            # Conflict detection
│   │   └── completeness.rs         # Coverage analysis
│   │
│   └── cli/                        # CLI Command Handlers
│       ├── mod.rs
│       ├── init.rs                 # Initialize driven config
│       ├── convert.rs              # Convert between formats
│       ├── sync.rs                 # Sync across editors
│       ├── template.rs             # Template management
│       ├── analyze.rs              # Analyze project context
│       └── validate.rs             # Validate rules
│
├── templates/                      # Built-in Template Library
│   ├── personas/
│   │   ├── architect.drv           # Senior architect persona
│   │   ├── reviewer.drv            # Code reviewer persona
│   │   ├── documenter.drv          # Documentation specialist
│   │   ├── security.drv            # Security auditor persona
│   │   ├── performance.drv         # Performance optimizer
│   │   └── teacher.drv             # Teaching/explaining persona
│   │
│   ├── projects/
│   │   ├── rust-workspace.drv      # Rust workspace conventions
│   │   ├── typescript-monorepo.drv # TS monorepo patterns
│   │   ├── fullstack.drv           # Full-stack app patterns
│   │   ├── cli-tool.drv            # CLI tool conventions
│   │   ├── library.drv             # Library crate patterns
│   │   └── dx-project.drv          # DX ecosystem project
│   │
│   ├── standards/
│   │   ├── rust-idioms.drv         # Rust idiomatic patterns
│   │   ├── error-handling.drv      # Error handling conventions
│   │   ├── testing.drv             # Testing standards
│   │   ├── documentation.drv       # Documentation standards
│   │   ├── git-conventions.drv     # Git commit/branch standards
│   │   └── api-design.drv          # API design principles
│   │
│   ├── workflows/
│   │   ├── tdd.drv                 # Test-driven development
│   │   ├── feature-development.drv # Feature development flow
│   │   ├── bug-fixing.drv          # Bug fixing workflow
│   │   ├── refactoring.drv         # Refactoring guidelines
│   │   ├── code-review.drv         # Code review process
│   │   └── bmad-method.drv         # BMAD methodology adapted
│   │
│   └── tasks/
│       ├── implement-feature.drv   # Feature implementation
│       ├── write-tests.drv         # Test writing guidance
│       ├── fix-bug.drv             # Bug fixing steps
│       ├── optimize.drv            # Optimization task
│       ├── migrate.drv             # Migration task
│       └── document.drv            # Documentation task
│
├── schemas/                        # Binary Schema Definitions
│   ├── driven.schema               # Main schema file
│   ├── persona.schema              # Persona schema
│   ├── project.schema              # Project schema
│   └── workflow.schema             # Workflow schema
│
├── docs/
│   ├── README.md                   # Crate overview
│   ├── BINARY_FORMAT.md            # .drv format specification
│   ├── TEMPLATES.md                # Template authoring guide
│   ├── CONVERSION.md               # Format conversion reference
│   ├── CONTEXT_ENGINE.md           # Context intelligence docs
│   ├── INTEGRATION.md              # Editor integration guide
│   └── BMAD_ADAPTATION.md          # How BMAD concepts map
│
└── tests/
    ├── format/                     # Binary format tests
    ├── parser/                     # Parser tests per format
    ├── emitter/                    # Emitter tests per target
    ├── templates/                  # Template system tests
    ├── context/                    # Context engine tests
    ├── integration/                # End-to-end tests
    └── fixtures/                   # Test fixtures (sample rules)
```

---

## Binary Rule Format (.drv - Driven Rule)

### Design Principles (Leveraging DX Binary Dawn)

| Feature | Traditional (JSON/YAML/MD) | Driven Binary (.drv) |
|---------|---------------------------|----------------------|
| **Size** | 100% (baseline) | ~25% (75% smaller) |
| **Parse Time** | ~5-15ms | ~0.1ms (50-150x faster) |
| **Memory** | Heap allocation | Zero-copy memory-mapped |
| **Validation** | Runtime parsing | Compile-time schema |
| **Versioning** | Manual migration | Binary schema evolution |

### Schema Structure

```
DrivenRule (.drv)
├── Header (16 bytes)
│   ├── Magic: "DRV\0" (4 bytes)
│   ├── Version: u16 (2 bytes)
│   ├── Flags: u16 (2 bytes)
│   ├── Section Count: u32 (4 bytes)
│   └── Checksum: u32 (4 bytes - Blake3 truncated)
│
├── String Table (variable)
│   ├── Count: u32
│   └── Strings: [length: u16, bytes: [u8; length]]...
│
├── Persona Section (optional)
│   ├── Name Index: u32 (into string table)
│   ├── Role Index: u32
│   ├── Traits Count: u16
│   └── Traits: [u32]... (string indices)
│
├── Standards Section (optional)
│   ├── Count: u32
│   └── Rules: [RuleEntry]...
│       ├── Category: u8 (enum)
│       ├── Priority: u8
│       ├── Description Index: u32
│       └── Pattern Index: u32
│
├── Context Section (optional)
│   ├── File Patterns: [u32]... (string indices)
│   ├── Exclude Patterns: [u32]...
│   ├── Focus Areas: [u32]...
│   └── Dependencies: [Dependency]...
│
└── Workflow Section (optional)
    ├── Steps Count: u16
    └── Steps: [WorkflowStep]...
        ├── Name Index: u32
        ├── Description Index: u32
        ├── Condition Index: u32
        └── Actions: [u32]...
```

---

## Template Categories (Inspired by BMAD + Speck-Kit)

### 1. Personas (AI Agent Roles)
Define how the AI should behave, its expertise, and communication style:
- **Architect**: System design, architecture decisions, scalability
- **Reviewer**: Code quality, best practices, security review
- **Implementer**: Feature development, following specifications
- **Documenter**: Documentation, comments, API descriptions
- **Optimizer**: Performance tuning, memory optimization
- **Teacher**: Explanations, learning, mentoring

### 2. Projects (Structure & Conventions)
Define project-specific patterns and structure:
- Directory organization expectations
- File naming conventions
- Module/crate organization patterns
- Configuration file locations
- Build and deployment patterns

### 3. Standards (Coding Conventions)
Define code quality and style expectations:
- Naming conventions (variables, functions, types)
- Error handling patterns
- Testing requirements
- Documentation requirements
- Commit message formats
- Import organization

### 4. Workflows (Development Processes)
Define step-by-step processes:
- Feature development lifecycle
- Bug fixing procedure
- Code review checklist
- Refactoring approach
- Testing strategy
- Documentation workflow

### 5. Tasks (Specific Operations)
Define context for specific task types:
- Implementing a new feature
- Writing comprehensive tests
- Fixing a reported bug
- Optimizing performance
- Migrating code/data
- Creating documentation

---

## Rule Conversion Matrix

### Supported Formats

| Source/Target | Cursor | Copilot | Windsurf | Claude | Aider | Generic MD |
|---------------|--------|---------|----------|--------|-------|------------|
| **Cursor**    | ✓      | ✓       | ✓        | ✓      | ✓     | ✓          |
| **Copilot**   | ✓      | ✓       | ✓        | ✓      | ✓     | ✓          |
| **Windsurf**  | ✓      | ✓       | ✓        | ✓      | ✓     | ✓          |
| **Claude**    | ✓      | ✓       | ✓        | ✓      | ✓     | ✓          |
| **Aider**     | ✓      | ✓       | ✓        | ✓      | ✓     | ✓          |
| **Driven**    | ✓      | ✓       | ✓        | ✓      | ✓     | ✓          |

### File Locations Per Editor

| Editor | Rule File Location |
|--------|-------------------|
| **Cursor** | `.cursorrules`, `.cursor/rules/` |
| **VS Code Copilot** | `.github/copilot-instructions.md` |
| **Windsurf** | `.windsurfrules` |
| **Claude (Anthropic)** | `.claude/`, `CLAUDE.md` |
| **Aider** | `.aider/`, `aider.conf.yml` |
| **Driven (Universal)** | `.driven/`, `driven.toml`, `*.drv` |

---

## Context Intelligence Engine

### Project Analysis Capabilities

1. **Structure Scanning**
   - Directory tree analysis
   - File type distribution
   - Module dependency graph
   - Configuration file detection

2. **Pattern Extraction**
   - Naming convention detection (camelCase, snake_case, etc.)
   - Import organization patterns
   - Error handling patterns
   - Testing patterns

3. **Convention Inference**
   - Function signature patterns
   - Documentation style detection
   - Code organization heuristics
   - Architecture pattern recognition

4. **Binary Index Generation**
   - Zero-copy project index
   - SIMD-accelerated scanning
   - Incremental updates
   - Memory-mapped storage

---

## CLI Integration with DX

### Commands

```bash
# Initialize driven configuration
dx driven init [--template <name>] [--editor <cursor|copilot|all>]

# Convert rules between formats
dx driven convert <source> --to <target> [--output <path>]

# Sync rules across all configured editors
dx driven sync [--watch]

# List/add/remove templates
dx driven template list
dx driven template add <name|path>
dx driven template use <name>

# Analyze project and generate context
dx driven analyze [--output <path>]

# Validate rules for consistency
dx driven validate [path]

# Compose multiple templates into one rule
dx driven compose <templates...> --output <path>
```

### Configuration (driven.toml)

```toml
[driven]
version = "1.0"
default_editor = "cursor"

[editors]
cursor = true
copilot = true
windsurf = false
claude = true

[templates]
personas = ["architect", "reviewer"]
project = "rust-workspace"
standards = ["rust-idioms", "testing"]
workflow = "feature-development"

[sync]
watch = true
auto_convert = true
source_of_truth = ".driven/rules.drv"

[context]
include = ["src/**", "crates/**"]
exclude = ["target/**", "node_modules/**"]
index_path = ".driven/index.drv"
```

---

## DX Binary Dawn Integration Points

### 1. dx-serializer Integration
- Use DX ∞ format principles for .drv encoding
- 70%+ size reduction vs JSON/YAML rules
- Sub-millisecond parsing with zero-copy access

### 2. dx-js-runtime Integration
- Execute JavaScript-based rule transformations
- Support for complex template logic
- 10x faster than Node.js execution

### 3. dx-style Pattern Adoption
- Integer IDs for rule categories (like B-CSS StyleIds)
- Pre-computed lookup tables for common patterns
- Binary combo patterns for template composition

### 4. SIMD Optimization
- AVX2 pattern matching for codebase scanning
- Parallel file analysis
- Vectorized string matching for convention detection

### 5. Zero-Copy Architecture
- Memory-mapped rule files
- No heap allocation in hot paths
- SharedArrayBuffer for cross-module state (future worker support)

---

## Performance Targets

| Operation | Traditional Tools | Driven Target | Improvement |
|-----------|------------------|---------------|-------------|
| **Rule Parsing** | 5-15ms | <0.2ms | 25-75x faster |
| **Format Conversion** | 50-100ms | <5ms | 10-20x faster |
| **Project Analysis** | 2-5s (1000 files) | <200ms | 10-25x faster |
| **Template Loading** | 10-20ms | <0.1ms | 100-200x faster |
| **Sync Detection** | 100ms+ | <1ms | 100x faster |
| **Rule Size** | 100% (baseline) | ~25% | 75% smaller |

---

## Key Differentiators from Speck-Kit & BMAD_METHOD

| Aspect | Speck-Kit (Python) | BMAD_METHOD (JS) | **Driven (Rust)** |
|--------|-------------------|------------------|-------------------|
| **Performance** | Interpreted | Interpreted | **Compiled + SIMD** |
| **Format** | Text (YAML/MD) | Text (MD/JSON) | **Binary (.drv)** |
| **Size** | Large templates | Medium | **Minimal (75% smaller)** |
| **Parsing** | Runtime | Runtime | **Zero-parse** |
| **Editor Support** | Limited | Limited | **Universal conversion** |
| **Type Safety** | Dynamic | Dynamic | **Compile-time schemas** |
| **Integration** | Standalone | Standalone | **DX ecosystem native** |
| **Context Engine** | Basic | Basic | **Deep analysis + binary index** |
| **Sync** | Manual | Manual | **Auto-sync with watch** |

---

## Implementation Phases

### Phase 1: Foundation (Week 1)
- Binary format schema and encoder/decoder
- Basic parser for Cursor rules
- Basic emitter for Cursor rules
- Template registry and loading

### Phase 2: Multi-Editor Support (Week 2)
- All parsers (Copilot, Windsurf, Claude, Aider)
- All emitters (Copilot, Windsurf, Claude, Aider)
- Conversion pipeline
- CLI convert command

### Phase 3: Template System (Week 3)
- Built-in template library
- Template composition engine
- CLI template commands
- BMAD workflow adaptation

### Phase 4: Context Intelligence (Week 4)
- Project structure scanner
- Convention extractor
- Binary index generation
- CLI analyze command

### Phase 5: Sync & Polish (Week 5)
- File watcher for auto-sync
- Change propagation engine
- Validation and linting
- Documentation and testing

---

## Success Metrics

1. **Performance**: 50x faster rule loading than text-based alternatives
2. **Size**: 75% smaller rule files with binary format
3. **Coverage**: 100% feature parity with Cursor rules + extensions
4. **Adoption**: Seamless migration from existing .cursorrules
5. **DX Integration**: Native integration with dx-cli ecosystem
6. **Template Quality**: 20+ production-ready templates

---

This design positions **Driven** as the definitive solution for AI-assisted development orchestration, leveraging DX's binary-first philosophy to deliver unmatched performance while providing the professional templates and workflows developers need for consistent, efficient AI collaboration.
```
