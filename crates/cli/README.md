# dx-cli

[![Crates.io](https://img.shields.io/crates/v/dx-cli.svg)](https://crates.io/crates/dx-cli)
[![Documentation](https://docs.rs/dx-cli/badge.svg)](https://docs.rs/dx-cli)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

The unified command-line interface for the DX ecosystem.

## Overview

`dx-cli` provides a single entry point for all DX tools, including the web framework compiler, font utilities, code generation, and more. It orchestrates the various DX crates into a cohesive developer experience.

## Features

- Unified CLI for all DX tools
- Project scaffolding and initialization
- Development server with hot reload
- Build and optimization commands
- Code generation utilities

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
dx-cli = "0.1.0"
```

Or install globally:

```bash
cargo install dx-cli
```

## Usage

```bash
# Initialize a new project
dx init my-project

# Start development server
dx dev

# Build for production
dx build --release

# Run code generation
dx generate component MyComponent

# Access font utilities
dx font search "Roboto"

# Run the style compiler
dx style compile ./src/styles
```

## Subcommands

| Command | Description |
|---------|-------------|
| `init` | Initialize a new DX project |
| `dev` | Start development server |
| `build` | Build project for production |
| `generate` | Code generation utilities |
| `font` | Font search and download |
| `style` | CSS/style compilation |
| `check` | Lint and validate project |

## Configuration

Create a `dx.toml` in your project root:

```toml
[project]
name = "my-app"
version = "0.1.0"

[build]
target = "web"
optimize = true

[dev]
port = 3000
hot_reload = true
```

## License

This project is dual-licensed under MIT OR Apache-2.0.
