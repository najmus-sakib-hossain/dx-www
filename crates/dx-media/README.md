# DX Media 🎨

> **Universal Digital Asset Acquisition Engine** - One command, any media, from anywhere.

[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

DX Media is a blazing-fast CLI tool and Rust library for searching and downloading royalty-free media assets from 30+ providers. Search images, videos, audio, 3D models, GIFs, and documents—all from a single unified interface.

## ✨ Features

- 🔍 **Unified Search** - Query 30+ providers simultaneously
- ⚡ **Dual Search Modes** - Quantity (fast, early-exit) or Quality (comprehensive)
- 📦 **Multiple Media Types** - Images, Videos, Audio, 3D Models, GIFs, Documents
- 🎨 **Rich Filtering** - By media type, orientation, color, provider
- 💾 **Smart Downloads** - Parallel downloads with proper file naming
- 🌐 **Web Scraping** - Extract media from any website
- 🔧 **Developer Friendly** - Use as CLI or Rust library

## 🚀 Quick Start

```bash
# Search for images
dx search "sunset mountains"

# Search specific media type
dx search -t video "ocean waves"
dx search -t audio "rain ambient"
dx search -t 3d "rock"

# Search specific provider
dx search -P nasa "mars"
dx search -P openverse "coffee shop"

# Download directly
dx search "cat" --download
dx download met:437261 -o ./downloads

# Quality mode (wait for all providers)
dx search -m quality "landscape"
```

## 📋 Supported Media Types

| Type | CLI Flag | Description |
|------|----------|-------------|
| Images | `-t image` | Photos, illustrations, artwork |
| Videos | `-t video` | Video clips, footage |
| Audio | `-t audio` | Music, sound effects, ambient |
| 3D Models | `-t 3d` | GLB, OBJ, FBX models |
| GIFs | `-t gif` | Animated GIFs |
| Documents | `-t document` | PDFs, manuscripts, archives |
| Vector | `-t vector` | SVG graphics |

## 🔌 Providers

### Free (No API Key Required) - 22 Available

| Provider | Types | Content |
|----------|-------|---------|
| **Openverse** | Images, Audio | 700M+ CC-licensed assets |
| **NASA** | Images, Video | Space & astronomy media |
| **Met Museum** | Images | 400K+ public domain artworks |
| **Art Institute Chicago** | Images | CC0 licensed art collection |
| **Rijksmuseum** | Images | Dutch masterpieces |
| **Cleveland Museum** | Images | Open access artworks |
| **Library of Congress** | Images | Historical archives |
| **Europeana** | Images | European cultural heritage |
| **Wikimedia Commons** | Images | 90M+ free media files |
| **Archive.org** | Images | Internet Archive collections |
| **PolyHaven** | 3D, Images | CC0 3D assets & HDRIs |
| **Lorem Picsum** | Images | Placeholder images |
| **LoremFlickr** | Images | Random Flickr images |
| **Lorem Space** | Images | Space placeholder images |
| **Dog CEO** | Images | Random dog photos |
| **Cat API** | Images | Random cat photos |
| **Random Fox** | Images | Random fox photos |
| **DiceBear** | Images | Generative avatars |
| **RoboHash** | Images | Robot/alien avatars |
| **Waifu.pics** | Images, GIF | Anime images & GIFs |
| **XKCD** | Images | 3000+ comics |
| **Scryfall** | Images | MTG card images |

### Premium (API Key Required) - 9 Providers

| Provider | Types | Get API Key |
|----------|-------|-------------|
| Pixabay | Images, Video | [pixabay.com/api](https://pixabay.com/api/docs/) |
| Unsplash | Images | [unsplash.com/developers](https://unsplash.com/developers) |
| Pexels | Images, Video | [pexels.com/api](https://www.pexels.com/api/) |
| Giphy | GIF | [developers.giphy.com](https://developers.giphy.com/) |
| Freesound | Audio | [freesound.org/apiv2](https://freesound.org/apiv2/apply) |
| V&A Museum | Images | [api.vam.ac.uk](https://api.vam.ac.uk/) |
| Smithsonian | Images, 3D | [api.si.edu](https://api.si.edu/) |
| DPLA | Images | [dp.la/info/developers](https://dp.la/info/developers/codex) |
| Open Library | Images | [openlibrary.org/developers](https://openlibrary.org/developers/api) |

### Disabled (Cloudflare Protection)

- **Walters Art Museum** - Blocked by Cloudflare
- **Nekos.best** - Bot detection active

## ⚡ Search Modes

### Quantity Mode (Default)
Fast early-exit search. Stops when sufficient results are found.
```bash
dx search "nature"  # Same as -m quantity
```

**Behavior:**
- 5-second provider timeout
- 3-second scraper timeout
- Early exit at 3× requested count
- Results stream as they arrive

### Quality Mode
Wait for all providers to respond for comprehensive results.
```bash
dx search -m quality "landscape photography"
```

**Behavior:**
- 8-second provider timeout
- 6-second scraper timeout
- Wait for all providers
- Best for exhaustive searches

## 📥 Download Options

```bash
# Download by asset ID
dx download openverse:864ca73c-f8c0-4a53-b9b5-8f2c0ccc8a01

# Download with custom output directory
dx download met:437261 -o ./my-downloads

# Auto-download first search result
dx search "waterfall" --download

# Download to specific folder
dx search "sunset" --download -o ./photos
```

## 🌐 Web Scraping

Scrape media from any website:

```bash
# Basic scrape
dx scrape https://example.com

# Scrape with depth
dx scrape https://example.com --depth 2

# Filter by media type
dx scrape https://example.com -t image

# Limit results
dx scrape https://example.com -n 50
```

## 🛠️ Configuration

Set API keys via environment variables:

```bash
export PIXABAY_API_KEY="your-key"
export UNSPLASH_ACCESS_KEY="your-key"
export PEXELS_API_KEY="your-key"
export GIPHY_API_KEY="your-key"
export FREESOUND_API_KEY="your-key"
```

Or use a config file at `~/.config/dx-media/config.toml`:

```toml
[api_keys]
pixabay = "your-key"
unsplash = "your-key"
pexels = "your-key"

[settings]
timeout_secs = 10
retry_attempts = 3
download_dir = "~/Downloads/dx-media"
```

## 📊 Output Formats

```bash
# Text output (default)
dx search "cat"

# JSON output
dx search "cat" --format json

# Quiet mode (errors only)
dx search "cat" -q
```

## 🔧 As a Library

```rust
use dx_media::{DxMedia, SearchQuery, MediaType, SearchMode};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let dx = DxMedia::new()?;
    
    // Simple search
    let results = dx.search("sunset").await?;
    
    // Advanced search with filters
    let query = SearchQuery::new("nature")
        .media_type(MediaType::Image)
        .count(20)
        .providers(vec!["openverse", "nasa"]);
    
    let results = dx.search_query(&query).await?;
    
    // Search all providers with quality mode
    let results = dx.search_all_with_mode("landscape", 50, SearchMode::Quality).await?;
    
    // Download asset
    let asset = &results.assets[0];
    let path = dx.download(asset).await?;
    
    Ok(())
}
```

## 🏗️ Building from Source

```bash
git clone https://github.com/anthropics/dx-media
cd dx-media
cargo build --release

# Run directly
./target/release/dx search "mountains"

# Install globally
cargo install --path .
```

## 📈 Performance

| Mode | Typical Time | Use Case |
|------|-------------|----------|
| Quantity | ~1-2 seconds | Quick searches |
| Quality | ~5-8 seconds | Comprehensive results |
| Single Provider | ~0.3-1 second | Targeted searches |

**Optimizations:**
- Async concurrent provider queries
- HTTP connection pooling (10 per host)
- TCP_NODELAY for low latency
- Gzip/Brotli compression
- Smart early-exit in quantity mode

## 📝 License

MIT License - see [LICENSE](LICENSE) for details.

## 🤝 Contributing

Contributions welcome! Please read our contributing guidelines before submitting PRs.

## 🔗 Links

- [Documentation](https://docs.rs/dx-media)
- [Changelog](CHANGELOG.md)
- [Issue Tracker](https://github.com/anthropics/dx-media/issues)

---

Made with ❤️ by the DX Team
