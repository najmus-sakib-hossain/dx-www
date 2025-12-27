# Contributing to dx-crate-lint

Thank you for your interest in contributing to dx-crate-lint!

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/dx.git`
3. Create a branch: `git checkout -b feature/your-feature`
4. Make your changes
5. Run tests: `cargo test`
6. Commit your changes: `git commit -am 'Add some feature'`
7. Push to the branch: `git push origin feature/your-feature`
8. Submit a pull request

## Development Setup

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build the project
cargo build

# Run tests
cargo test

# Run clippy
cargo clippy
```

## Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Ensure all tests pass (`cargo test`)
- Ensure no clippy warnings (`cargo clippy`)
- Add documentation for public APIs
- Write tests for new functionality

## Reporting Issues

- Use the GitHub issue tracker
- Include steps to reproduce
- Include expected vs actual behavior
- Include Rust version and OS

## License

By contributing, you agree that your contributions will be licensed under the
MIT OR Apache-2.0 license.
