# Contributing to dx-www Runtime

Thank you for your interest in contributing to dx-www! This is a systems-level project with strict performance requirements.

## Code Standards

### The Three Sacred Rules

1. **No String Rule**: Use `u32` indices and `&[u8]` slices internally
2. **Zero-Copy Memory**: Use `bytemuck` for byte slice mapping
3. **Data-Oriented Design**: Prefer SoA over AoS, use object pooling

### Rust Edition

We use **Rust Edition 2024**. Ensure your toolchain is up to date:

```bash
rustup update stable
rustup default stable
```

### Code Style

- Use `rustfmt` with default settings
- Use `clippy` with all lints enabled
- Document all `unsafe` blocks with safety invariants
- Prioritize performance over readability (when necessary)

## Architecture Guidelines

### Memory Safety

All memory operations must be documented:

```rust
/// # Safety
/// The caller must ensure that `ptr` points to valid memory
/// of at least `len` bytes and is properly aligned.
pub unsafe fn read_slice(ptr: *const u8, len: usize) -> &[u8] {
    std::slice::from_raw_parts(ptr, len)
}
```

### Performance

Every PR must include benchmarks if it touches hot paths:

```bash
cargo bench --workspace
```

### Testing

All crates must have unit tests:

```bash
# Run tests
cargo test --workspace

# Run with coverage
cargo tarpaulin --workspace
```

## Commit Guidelines

Use conventional commits:

```
feat(dx-dom): add batch clone optimization
fix(dx-core): memory leak in state region
docs(readme): update installation instructions
perf(dx-morph): reduce dirty bit check overhead
```

## PR Process

1. Fork the repository
2. Create a feature branch (`feat/amazing-feature`)
3. Make your changes
4. Add tests
5. Run `cargo fmt && cargo clippy`
6. Submit PR with benchmarks

## Areas Needing Help

- [ ] Binding Map compiler
- [ ] SSR implementation
- [ ] Dev tools (inspector)
- [ ] Performance benchmarks vs React
- [ ] Documentation examples

## Questions?

Open a GitHub Discussion. We're happy to help!

---

Remember: **Binary Everywhere. Zero Parse. Zero GC. Zero Hydration.**
