# DX Serializer

The DX Serializer is a revolutionary data serialization system that supports three distinct formats optimized for different use cases:

## Formats

| Format | Purpose | Efficiency |
|--------|---------|------------|
| [Human](./human-format.md) | Human-readable editing | Beautiful TOML-like display |
| [LLM](./llm-format.md) | LLM token efficiency | 3x+ better than TOON |
| [Machine](./machine-format.md) | Binary performance | 27x faster than rkyv |

## Key Features

- **Dual-mode rendering**: Same `.dx` file displayed differently based on context
- **Token efficiency**: 85% token reduction for LLM prompts
- **Zero-copy deserialization**: Sub-nanosecond field access
- **Bidirectional conversion**: Convert between all three formats seamlessly

## Quick Example

### LLM Format (Storage)
```dx
#c:t|Our favorite hikes;l|^B;s|sp25
#:B|Boulder
#h(id|nm|km|el|w|s)
1|Blue Lake Trail|7.5|320|ana|+
```

### Human Format (Display)
```toml
[config]
    task     = "Our favorite hikes"
    location = "Boulder"
    season   = "spring 2025"

[hikes]
    ┌────┬─────────────────┬──────┬───────────┐
    │ ID │ Name            │  Km  │ Elevation │
    ├────┼─────────────────┼──────┼───────────┤
    │  1 │ Blue Lake Trail │  7.5 │       320 │
    └────┴─────────────────┴──────┴───────────┘
```

## Performance

- **Serialization**: 27x faster than rkyv
- **Deserialization**: 0.70 ns (hardware limit)
- **Size**: 32.6% smaller than rkyv
- **LLM Tokens**: 6x reduction vs JSON

## Related

- [Architecture](../../architecture/README.md)
- [Benchmarks](../../reference/benchmarks/README.md)
