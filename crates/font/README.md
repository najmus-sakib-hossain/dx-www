# font

[![Crates.io](https://img.shields.io/crates/v/font.svg)](https://crates.io/crates/font)
[![Documentation](https://docs.rs/font/badge.svg)](https://docs.rs/font)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A comprehensive font search and download tool with access to 50k+ commercial-free fonts from multiple providers.

## Overview

The `font` crate provides a unified interface for searching and downloading fonts from various free font providers. It supports parallel downloads, caching, and multiple output formats.

## Features

- Search across 50k+ commercial-free fonts
- Multi-provider support (Google Fonts, Font Squirrel, etc.)
- Parallel downloads with progress indicators
- Font metadata extraction and preview
- CDN URL generation for web usage
- Async/await support with Tokio

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
font = "0.0.1"
```

Or use cargo add:

```bash
cargo add font
```

## Usage

### CLI

```bash
# Search for fonts
dx-font search "Roboto"

# Download a font
dx-font download "Roboto" --output ./fonts

# Get font details
dx-font info "Open Sans"
```

### Library

```rust
use font::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Search for fonts
    let results = font::search("Roboto").await?;
    
    for font in results {
        println!("{}: {} variants", font.name, font.variants.len());
    }
    
    // Download a font
    font::download("Roboto", "./fonts").await?;
    
    Ok(())
}
```

## Supported Providers

- Google Fonts
- Font Squirrel
- DaFont (free fonts only)
- FontSpace

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
