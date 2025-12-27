# DX Documentation

> **The Binary-First Development Platform**

Welcome to the DX documentation. DX is a revolutionary full-stack development platform built entirely in Rust that replaces the traditional JavaScript ecosystem with a binary-first architecture.

## Quick Navigation

| Section | Description |
|---------|-------------|
| [Getting Started](./getting-started/README.md) | Quick start guides and tutorials |
| [Architecture](./architecture/README.md) | System design and technical deep-dives |
| [API Reference](./api/README.md) | Complete API documentation |
| [Guides](./guides/README.md) | How-to guides and migration paths |
| [Reference](./reference/README.md) | Benchmarks, comparisons, standards |
| [Crates](./crates/) | Per-crate documentation |
| [Archive](./archive/README.md) | Historical documentation |

## Performance Summary

DX has achieved complete victory over traditional JavaScript tooling:

| Component | vs Baseline | Status |
|-----------|-------------|--------|
| JS Bundler | 3.8x faster than Bun | ✅ |
| JS Runtime | 10.59x faster than Bun | ✅ |
| Test Runner | 26x faster than Jest | ✅ |
| Package Manager | 17.2x faster than npm | ✅ |
| Serializer | 27x faster than rkyv | ✅ |

## Core Components

### DX Serializer
Three-format serialization system optimized for humans, LLMs, and machines:
- [Human Format](./api/serializer/human-format.md) - Beautiful, readable display
- [LLM Format](./api/serializer/llm-format.md) - 3x more token-efficient than TOON
- [Machine Format](./api/serializer/machine-format.md) - 27x faster than rkyv

### DX WWW
Binary web framework with zero hydration. [Architecture →](./architecture/README.md)

## The Vision

> "The Browser was built for Text. We built DX for Applications."

DX eliminates traditional bottlenecks through:
- **Zero Parse** - Binary formats, no JSON/HTML parsing
- **Zero GC** - Linear memory, no garbage collection
- **Zero Hydration** - Resumable state snapshots
- **Zero Diffing** - O(1) updates via dirty bits

## Quick Links

**New to DX?**
- [Quick Start](./getting-started/quickstart.md)
- [Architecture Overview](./architecture/README.md)

**Building with DX?**
- [API Reference](./api/README.md)
- [Coding Standards](./reference/coding-standards.md)

**Contributing?**
- [Development Guide](./getting-started/development.md)
- [Contributing Guide](./guides/contributing.md)

---

**Last Updated:** December 27, 2025
