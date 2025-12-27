# Design Document: Documentation Reorganization

## Overview

This design document outlines the approach for reorganizing the DX project's documentation folder from a flat, cluttered structure with 80+ root-level files into a clean, professional, hierarchical structure that is easy to navigate and maintain.

## Architecture

### Current State Analysis

The current `/docs` folder contains:
- **80+ files at root level** - making navigation difficult
- **Duplicate content** - multiple files covering the same topics
- **Mixed content types** - specifications, progress reports, brainstorming notes all together
- **Inconsistent naming** - both UPPER_CASE.md and lower-case.md conventions
- **Scattered serializer docs** - HUMAN.md, LLM.md, MACHINE.md, dx-serializer.md, DX_SERIALIZER.md

### Target Structure

```
docs/
├── README.md                           # Main documentation index
├── getting-started/
│   ├── README.md                       # Getting started index
│   ├── quickstart.md                   # Quick start guide
│   ├── installation.md                 # Installation instructions
│   └── first-project.md                # First project tutorial
├── architecture/
│   ├── README.md                       # Architecture index
│   ├── overview.md                     # High-level architecture
│   ├── binary-protocol.md              # Binary protocol design
│   ├── compiler.md                     # Compiler architecture
│   └── project-structure.md            # Codebase organization
├── api/
│   ├── README.md                       # API documentation index
│   ├── serializer/
│   │   ├── README.md                   # Serializer overview
│   │   ├── human-format.md             # Human-readable format spec
│   │   ├── llm-format.md               # LLM-optimized format spec
│   │   └── machine-format.md           # Binary machine format spec
│   ├── cli.md                          # CLI reference
│   └── stack.md                        # Stack API reference
├── guides/
│   ├── README.md                       # Guides index
│   ├── migration/
│   │   ├── from-nextjs.md              # Next.js migration guide
│   │   └── from-react.md               # React migration guide
│   ├── development.md                  # Development guide
│   └── contributing.md                 # Contributing guide
├── reference/
│   ├── README.md                       # Reference index
│   ├── benchmarks/
│   │   ├── README.md                   # Benchmarks overview
│   │   ├── bundler.md                  # Bundler benchmarks
│   │   ├── runtime.md                  # Runtime benchmarks
│   │   └── serializer.md               # Serializer benchmarks
│   ├── comparisons/
│   │   ├── vs-bun.md                   # DX vs Bun comparison
│   │   └── vs-frameworks.md            # Framework comparisons
│   └── coding-standards.md             # Coding standards
├── crates/                             # Keep existing structure
│   └── [existing crate docs]
└── archive/
    ├── progress/                       # Historical progress reports
    ├── planning/                       # Planning documents
    └── legacy/                         # Outdated specifications
```

## Components and Interfaces

### File Classification System

Files will be classified into categories based on their content:

| Category | Criteria | Destination |
|----------|----------|-------------|
| **Core** | Essential specs, APIs, architecture | Appropriate subfolder |
| **Guide** | How-to content, tutorials | `guides/` |
| **Reference** | Benchmarks, comparisons | `reference/` |
| **Progress** | Dated reports, session summaries | `archive/progress/` |
| **Planning** | Roadmaps, brainstorming | `archive/planning/` |
| **Legacy** | Outdated specs, superseded docs | `archive/legacy/` |
| **Duplicate** | Redundant content | Delete (after merging) |

### File Mapping

#### Files to Keep at Root
- `README.md` - Main index (will be rewritten)

#### Files to Move to `getting-started/`
- `guides/QUICKSTART.md` → `getting-started/quickstart.md`
- `guides/DEVELOPMENT.md` → `getting-started/development.md`

#### Files to Move to `architecture/`
- `architecture/ARCHITECTURE.md` → `architecture/overview.md`
- `architecture/COMPILER.md` → `architecture/compiler.md`
- `architecture/COMPILER_INTELLIGENCE.md` → `architecture/compiler-intelligence.md`
- `architecture/PROJECT_STRUCTURE.md` → `architecture/project-structure.md`
- `BIDIRECTIONAL_SYSTEM.md` → `architecture/bidirectional-system.md`

#### Files to Move to `api/serializer/`
- `HUMAN.md` → `api/serializer/human-format.md`
- `LLM.md` → `api/serializer/llm-format.md`
- `MACHINE.md` → `api/serializer/machine-format.md`
- `dx-serializer.md` → Merge into `api/serializer/README.md`
- `DX_SERIALIZER.md` → Merge into `api/serializer/README.md`

#### Files to Move to `api/`
- `STACK.md` → `api/stack.md`
- `CLI.md` → `api/cli.md`
- `DX_WWW.md` → `api/dx-www.md`

#### Files to Move to `guides/`
- `guides/CONTRIBUTING.md` → `guides/contributing.md`
- `DX_ZERO_MIGRATION_GUIDE.md` → `guides/migration/dx-zero.md`

#### Files to Move to `reference/`
- `benchmarks/*` → `reference/benchmarks/`
- `FRAMEWORKS.md` → `reference/comparisons/frameworks.md`
- `CODE_STANDARD.md` or `CODING_STANDARD.md` → `reference/coding-standards.md` (merge)

#### Files to Archive
**Progress Reports** → `archive/progress/`
- All files in `progress/` folder
- `STATUS_REPORT.md`
- `IMPLEMENTATION_PROGRESS.md`
- `IMPLEMENTATION_CHECKLIST.md`

**Planning Documents** → `archive/planning/`
- `NEXT_STEPS.md`
- `THOUGHTS.md`
- `DX_INFINITY_ROADMAP.md`
- `WORKSPACE_PLANNING.md`
- `GENERATOR_PLANNING.md`
- `DX_JS_MONOREPO_PLANNING.md`

**Legacy/Outdated** → `archive/legacy/`
- `playground-archive/*`
- Superseded specifications
- Old completion reports

#### Files to Delete (Duplicates)
After merging content:
- `dx-serializer.md` (merged into serializer/README.md)
- `DX_SERIALIZER.md` (merged into serializer/README.md)
- `CODE_STANDARD.md` or `CODING_STANDARD.md` (keep one, merge unique content)
- `HUMAN_SERIALIZER_EXTENSION.md` (if content exists in HUMAN.md)

## Data Models

### Documentation Metadata

Each documentation file should follow this front-matter pattern:

```markdown
---
title: Document Title
category: architecture | api | guide | reference
last_updated: 2025-12-27
status: current | deprecated | draft
---
```

### Index File Structure

Each folder's README.md should follow this pattern:

```markdown
# Section Title

Brief description of what this section contains.

## Contents

- [Document 1](./document1.md) - Brief description
- [Document 2](./document2.md) - Brief description

## Related

- [Related Section](../related/)
```

## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system-essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

### Property 1: No Orphaned Files
*For any* file in the docs folder, there SHALL exist at least one index file (README.md) that links to it.
**Validates: Requirements 3.3, 8.2**

### Property 2: No Broken Links
*For any* internal link in any documentation file, the target file SHALL exist at the specified path.
**Validates: Requirements 8.1, 8.2**

### Property 3: No Duplicate Content
*For any* topic covered in the documentation, there SHALL be exactly one authoritative file (excluding archives).
**Validates: Requirements 1.1, 1.2**

### Property 4: Archive Completeness
*For any* file moved to archive, the original content SHALL be preserved without modification.
**Validates: Requirements 2.4, 7.1**

## Error Handling

### Missing Files
- If a file referenced in the mapping doesn't exist, skip it and log a warning
- Continue with remaining files

### Merge Conflicts
- When merging duplicate files, preserve all unique content
- Add clear section headers to indicate merged sources
- Keep the more recent/complete version as the base

### Link Updates
- After moving files, scan all markdown files for broken links
- Update relative paths to reflect new locations
- Log any links that cannot be automatically fixed

## Testing Strategy

### Manual Verification
1. Verify all index files link to existing documents
2. Verify no files are orphaned (not linked from any index)
3. Verify archive contains all historical documents
4. Verify no essential documentation was deleted

### Link Validation
1. Use markdown link checker to find broken links
2. Verify all relative paths resolve correctly
3. Test navigation from root README.md to all sections

### Content Verification
1. Spot-check merged files for completeness
2. Verify serializer documentation is complete
3. Verify architecture documentation is complete
