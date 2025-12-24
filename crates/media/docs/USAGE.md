# DX Media Usage Guide

> **The Universal Digital Asset Engine** - Search and download royalty-free media from 50+ providers with a single command.

---

## Table of Contents

- [Quick Start](#quick-start)
- [Installation](#installation)
- [Configuration](#configuration)
- [CLI Commands](#cli-commands)
  - [Search](#search-command)
  - [Download](#download-command)
  - [Scrape](#scrape-command)
  - [Providers](#providers-command)
  - [Config](#config-command)
- [Library Usage](#library-usage)
- [Provider Setup](#provider-setup)
- [Output Formats](#output-formats)
- [Examples](#examples)
- [Troubleshooting](#troubleshooting)

---

## Quick Start

```bash
# 1. Install DX Media
cargo install --path .

# 2. Set up API keys (at least one provider)
export UNSPLASH_ACCESS_KEY="your-key-here"
# or
export PEXELS_API_KEY="your-key-here"
# or
export PIXABAY_API_KEY="your-key-here"

# 3. Search for images
dx search "mountain sunset" --count 10

# 4. Download results
dx search "mountain sunset" --download --output ./photos
```

---

## Installation

### From Source

```bash
git clone https://github.com/anthropics/dx-media.git
cd dx-media
cargo build --release

# Binary will be at: target/release/dx
# Optionally install globally:
cargo install --path .
```

### Verify Installation

```bash
dx --version
# dx-media 0.1.0

dx --help
```

---

## Configuration

DX Media reads configuration from environment variables. You can set these in your shell or use a `.env` file in your project directory.

### Environment Variables

| Variable | Description | Required |
|----------|-------------|----------|
| `UNSPLASH_ACCESS_KEY` | Unsplash API access key | For Unsplash |
| `PEXELS_API_KEY` | Pexels API key | For Pexels |
| `PIXABAY_API_KEY` | Pixabay API key | For Pixabay |
| `DX_DOWNLOAD_DIR` | Default download directory | No (default: `./downloads`) |
| `DX_CONCURRENT_DOWNLOADS` | Max parallel downloads | No (default: `5`) |

### Example `.env` File

```env
# Provider API Keys
UNSPLASH_ACCESS_KEY=your_unsplash_key_here
PEXELS_API_KEY=your_pexels_key_here
PIXABAY_API_KEY=your_pixabay_key_here

# Download Settings
DX_DOWNLOAD_DIR=./media
DX_CONCURRENT_DOWNLOADS=10
```

### Getting API Keys

| Provider | Get Key At | Rate Limit |
|----------|-----------|------------|
| **Unsplash** | [unsplash.com/developers](https://unsplash.com/developers) | 50 requests/hour |
| **Pexels** | [pexels.com/api](https://www.pexels.com/api/) | 200 requests/hour |
| **Pixabay** | [pixabay.com/api/docs](https://pixabay.com/api/docs/) | 5000 requests/hour |

---

## CLI Commands

### Global Options

All commands support these options:

| Flag | Short | Description |
|------|-------|-------------|
| `--format <FORMAT>` | `-f` | Output format: `text`, `json`, `json-compact`, `tsv` |
| `--verbose` | `-v` | Enable verbose output |
| `--quiet` | `-q` | Suppress output except errors |
| `--help` | `-h` | Show help |
| `--version` | `-V` | Show version |

---

### Search Command

Search for media assets across all configured providers.

```bash
dx search <QUERY> [OPTIONS]
```

#### Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--type <TYPE>` | `-t` | Media type filter | All types |
| `--provider <NAME>` | `-p` | Search specific provider(s) | All available |
| `--count <N>` | `-n` | Number of results | 20 |
| `--page <N>` | | Page number | 1 |
| `--download` | `-d` | Download results immediately | false |
| `--output <DIR>` | `-o` | Download directory | `./downloads` |
| `--orientation <O>` | | `landscape`, `portrait`, `square` | Any |
| `--color <COLOR>` | | Filter by dominant color | Any |

#### Media Types

- `image` - Photos and images
- `video` - Video clips
- `vector` - Vector graphics (SVG, AI)
- `audio` - Music and sound effects
- `gif` - Animated GIFs

#### Examples

```bash
# Basic search
dx search "sunset beach"

# Search for images only
dx search "coffee" --type image

# Search specific provider
dx search "mountains" --provider unsplash

# Get 50 results
dx search "nature" --count 50

# Search and download
dx search "workspace" --type image --download --output ./photos

# Filter by orientation
dx search "banner" --orientation landscape

# Output as JSON
dx search "cat" --format json

# Multiple providers
dx search "flowers" --provider unsplash --provider pexels
```

---

### Download Command

Download a media asset by its ID or URL.

```bash
dx download <ID> [OPTIONS]
```

#### Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--provider <NAME>` | `-p` | Provider name (required with ID) | - |
| `--output <DIR>` | `-o` | Download directory | `./downloads` |

#### Examples

```bash
# Download by provider and ID
dx download abc123 --provider unsplash

# Download to specific directory
dx download xyz789 --provider pexels --output ./assets
```

---

### Scrape Command

Scrape and download media from any website. **No API key required!**

```bash
dx scrape <URL> [OPTIONS]
```

#### Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--output <DIR>` | `-o` | Download directory | `.` |
| `--type <TYPE>` | `-t` | Media type filter | `image` |
| `--count <N>` | `-n` | Maximum assets to find | `20` |
| `--depth <N>` | `-d` | Link-following depth (0 = only URL) | `0` |
| `--pattern <GLOB>` | `-p` | File pattern to match (e.g., `*.jpg`) | All |
| `--dry-run` | | Preview without downloading | false |

#### Examples

```bash
# Scrape images from a website
dx scrape "https://example.com"

# Scrape and save to directory
dx scrape "https://blog.example.com" --output ./scraped-images

# Preview what would be downloaded
dx scrape "https://example.com" --dry-run

# Scrape only JPG files
dx scrape "https://example.com" --pattern "*.jpg"

# Limit to 10 images
dx scrape "https://example.com" --count 10

# Scrape videos
dx scrape "https://example.com" --type video

# Follow links one level deep
dx scrape "https://example.com" --depth 1

# Output as JSON
dx scrape "https://example.com" --dry-run --format json
```

---

### Providers Command

List and manage media providers.

```bash
dx providers [OPTIONS]
```

#### Options

| Option | Description |
|--------|-------------|
| `--available` | Show only providers with valid API keys |
| `--type <TYPE>` | Filter by supported media type |
| `--detailed` | Show detailed provider information |

#### Examples

```bash
# List all providers
dx providers

# Show only available providers
dx providers --available

# Show providers supporting video
dx providers --type video

# Detailed view
dx providers --detailed
```

#### Output Example

```
Available Providers
Stats: 3 total, 2 available, 1 needs API key

  ✓ Unsplash (unsplash)
      Types: image

  ✓ Pexels (pexels)
      Types: image, video

  ✗ Pixabay (pixabay)
      Types: image, video, vector
```

---

### Config Command

Display current configuration.

```bash
dx config
```

Shows:
- API key status for each provider
- Download directory settings
- Rate limiting configuration

---

## Library Usage

Use DX Media as a Rust library in your projects.

### Add Dependency

```toml
[dependencies]
dx-media = { path = "../dx-media" }
tokio = { version = "1", features = ["full"] }
```

### Basic Example

```rust
use dx_media::{DxMedia, MediaType, SearchQuery};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the DX Media engine
    let dx = DxMedia::new()?;
    
    // Simple search
    let results = dx.search("sunset").await?;
    
    println!("Found {} results", results.total_count);
    
    for asset in &results.assets {
        println!("  {} - {} by {}", 
            asset.id, 
            asset.title,
            asset.author.as_deref().unwrap_or("Unknown")
        );
    }
    
    Ok(())
}
```

### Advanced Search with Builder

```rust
use dx_media::{DxMedia, MediaType};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let dx = DxMedia::new()?;
    
    // Use the search builder for advanced queries
    let results = dx.search_builder()
        .query("mountain landscape")
        .media_type(MediaType::Image)
        .providers(vec!["unsplash", "pexels"])
        .count(30)
        .page(1)
        .orientation("landscape")
        .execute()
        .await?;
    
    println!("Found {} assets from {} providers",
        results.assets.len(),
        results.providers_searched.len()
    );
    
    Ok(())
}
```

### Download Assets

```rust
use dx_media::{DxMedia, Downloader, FileManager};
use std::path::Path;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let dx = DxMedia::new()?;
    
    // Search for assets
    let results = dx.search("coffee").await?;
    
    // Set up downloader
    let downloader = Downloader::new(dx.http_client().clone());
    let file_manager = FileManager::new("./downloads")
        .organize_by_provider(true)
        .organize_by_type(true);
    
    // Download first result
    if let Some(asset) = results.assets.first() {
        let target_dir = file_manager.target_dir(asset);
        let filename = downloader.generate_filename(asset);
        let path = target_dir.join(&filename);
        
        downloader.download(asset, &path).await?;
        println!("Downloaded: {}", path.display());
    }
    
    Ok(())
}
```

### List Providers

```rust
use dx_media::DxMedia;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let dx = DxMedia::new()?;
    
    // Get all registered providers
    let providers = dx.list_providers();
    
    for provider in providers {
        println!("{} ({}) - Available: {}", 
            provider.display_name,
            provider.name,
            provider.is_available
        );
        println!("  Types: {:?}", provider.media_types);
        println!("  Rate Limit: {} req/{} sec", 
            provider.rate_limit.requests_per_window(),
            provider.rate_limit.window_secs()
        );
    }
    
    Ok(())
}
```

---

## Provider Setup

### Unsplash

1. Go to [unsplash.com/developers](https://unsplash.com/developers)
2. Create a new application
3. Copy your **Access Key**
4. Set `UNSPLASH_ACCESS_KEY` environment variable

**Rate Limit:** 50 requests/hour (demo), 5000/hour (production)

**License:** [Unsplash License](https://unsplash.com/license) - Free for commercial use, attribution appreciated

### Pexels

1. Go to [pexels.com/api](https://www.pexels.com/api/)
2. Sign up and request API access
3. Copy your API key
4. Set `PEXELS_API_KEY` environment variable

**Rate Limit:** 200 requests/hour, 20,000/month

**License:** [Pexels License](https://www.pexels.com/license/) - Free for commercial use, no attribution required

### Pixabay

1. Go to [pixabay.com/api/docs](https://pixabay.com/api/docs/)
2. Sign up for an account
3. Copy your API key from the docs page
4. Set `PIXABAY_API_KEY` environment variable

**Rate Limit:** 5000 requests/hour

**License:** [Pixabay License](https://pixabay.com/service/license/) - Free for commercial use, no attribution required

---

## Output Formats

### Text (Default)

Human-readable output with colors and formatting.

```bash
dx search "sunset" --format text
```

### JSON

Full JSON output for programmatic use.

```bash
dx search "sunset" --format json | jq '.assets[0]'
```

### JSON Compact

Single-line JSON (useful for piping).

```bash
dx search "sunset" --format json-compact
```

### TSV

Tab-separated values for spreadsheets.

```bash
dx search "sunset" --format tsv > results.tsv
```

---

## Examples

### Build a Photo Gallery

```bash
# Download 50 nature photos organized by provider
dx search "nature" --type image --count 50 --download --output ./gallery
```

### Find Stock Footage

```bash
# Search for video clips
dx search "city timelapse" --type video --provider pexels
```

### Script Integration

```bash
#!/bin/bash
# Download random images for a project

QUERY=${1:-"abstract background"}
COUNT=${2:-10}

dx search "$QUERY" --type image --count $COUNT --format json \
    | jq -r '.assets[].download_url' \
    | xargs -n1 -P5 curl -O
```

### CI/CD Asset Fetching

```yaml
# GitHub Actions example
- name: Fetch placeholder images
  env:
    UNSPLASH_ACCESS_KEY: ${{ secrets.UNSPLASH_KEY }}
  run: |
    dx search "hero background" --type image --count 1 --download
```

---

## Troubleshooting

### "Missing API key" Error

**Problem:** `Error: Missing API key for provider: unsplash`

**Solution:** Set the required environment variable:
```bash
export UNSPLASH_ACCESS_KEY="your-key-here"
```

Or add to `.env` file in your project directory.

### Rate Limiting

**Problem:** `Error: Rate limited by provider`

**Solution:** DX Media automatically handles rate limits with backoff. If you hit limits:
1. Wait for the rate limit window to reset
2. Use a different provider
3. Request a higher rate limit from the provider

### No Results Found

**Problem:** Search returns empty results

**Solutions:**
1. Check if provider API keys are set (`dx providers --available`)
2. Try different search terms
3. Try a different provider
4. Check your internet connection

### Connection Errors

**Problem:** Network or SSL errors

**Solutions:**
1. Check internet connectivity
2. Verify API endpoints are accessible
3. Check for proxy/firewall issues
4. Update to latest version

### Debug Mode

Enable verbose logging for troubleshooting:

```bash
RUST_LOG=debug dx search "test"
```

Or use the verbose flag:

```bash
dx search "test" --verbose
```

---

## Getting Help

- **Documentation:** [docs.dx-media.dev](https://docs.dx-media.dev)
- **Issues:** [GitHub Issues](https://github.com/anthropics/dx-media/issues)
- **Discussions:** [GitHub Discussions](https://github.com/anthropics/dx-media/discussions)

---

*Made with ❤️ and 🦀 by the DX Team*
