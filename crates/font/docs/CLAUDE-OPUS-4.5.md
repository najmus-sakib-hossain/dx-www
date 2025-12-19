# 🎬 DX Media - Universal Media Asset Manager

I understand! DX Media will **exclude** icons and fonts since you have dedicated projects for those:
- **dx-icons** - Dedicated icon management
- **dx-fonts** - Dedicated font management
- **dx-media** - Everything else!

Let me create the complete professional README and updated project structure.

---

## README.md

```markdown
<div align="center">

# 🎬 DX Media

### Universal Media Asset Manager

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/dx-media.svg)](https://crates.io/crates/dx-media)
[![Downloads](https://img.shields.io/crates/d/dx-media.svg)](https://crates.io/crates/dx-media)

**Search, download, and convert media assets from 50+ free sources across the internet.**

[Features](#-features) •
[Installation](#-installation) •
[Quick Start](#-quick-start) •
[Providers](#-providers) •
[Documentation](#-documentation)

---

<img src="assets/demo.gif" alt="DX Media Demo" width="800">

</div>

## 🌟 Overview

**DX Media** is a powerful command-line tool and Rust library for managing media assets. It provides a unified interface to search, download, and convert virtually any type of media from free sources across the internet.

Whether you need stock photos for a website, video footage for a project, audio for a podcast, 3D models for a game, or datasets for machine learning — DX Media has you covered.

### 🎯 Why DX Media?

- **One Tool, All Media**: No more juggling between different websites and tools
- **50+ Free Sources**: Access millions of free assets from trusted providers
- **Format Conversion**: Convert between formats without leaving the terminal
- **Batch Operations**: Download and convert multiple assets simultaneously
- **Offline Catalog**: Browse and manage your downloaded assets
- **Rust-Powered**: Fast, safe, and memory-efficient

### 📦 Part of the DX Ecosystem

| Project | Description |
|---------|-------------|
| **dx-media** | Universal media asset manager (this project) |
| [dx-icons](https://github.com/user/dx-icons) | Dedicated icon management and search |
| [dx-fonts](https://github.com/user/dx-fonts) | Dedicated font management and search |

> **Note**: Icons and fonts are handled by their dedicated tools. DX Media focuses on all other media types.

---

## ✨ Features

### 📁 Supported Media Types

<table>
<tr>
<td width="50%">

#### 🖼️ Visual
- Photos & Images
- Illustrations & Artwork
- Vectors & SVGs
- Wallpapers & Backgrounds
- Screenshots & Mockups
- Memes & Infographics
- GIFs & Animations

#### 🎬 Video
- Stock Footage
- Video Templates
- Animations
- Motion Graphics
- Tutorials

#### 🎵 Audio
- Music & Songs
- Sound Effects (SFX)
- Podcasts
- Audiobooks
- Voice Recordings
- Ambient Sounds

</td>
<td width="50%">

#### 📄 Text & Documents
- Articles & Blog Posts
- Books & E-books
- Research Papers
- PDFs & Documents
- Word Documents (DOCX)
- Spreadsheets (XLSX, CSV)
- Presentations (PPTX)

#### 📊 Data
- JSON & XML Files
- CSV & TSV Datasets
- YAML & TOML Configs
- Database Dumps
- API Response Samples
- Machine Learning Datasets

#### 🎨 3D Assets
- 3D Models (GLB, GLTF, OBJ, FBX)
- Textures & Materials
- HDRIs & Skyboxes
- Environment Maps

</td>
</tr>
<tr>
<td>

#### 💻 Code
- Code Snippets
- Project Templates
- Boilerplates
- Configuration Files
- Scripts & Utilities

</td>
<td>

#### 🎮 Game Assets
- Sprites & Spritesheets
- Tilesets & Tilemaps
- UI Kits
- Character Assets
- Backgrounds
- Game Templates

</td>
</tr>
</table>

### 🔧 Core Features

| Feature | Description |
|---------|-------------|
| 🔍 **Universal Search** | Search across multiple providers simultaneously |
| ⬇️ **Smart Download** | Resume interrupted downloads, parallel downloading |
| 🔄 **Format Conversion** | Convert between formats using FFmpeg & ImageMagick |
| 📋 **Asset Catalog** | Browse and manage downloaded assets locally |
| 🏷️ **Metadata Extraction** | Automatic extraction of asset metadata |
| 🔐 **License Tracking** | Track attribution requirements for each asset |
| 📦 **Batch Operations** | Process multiple assets with a single command |
| 🌐 **Web Scraping** | Extract assets from websites when APIs unavailable |
| 💾 **Caching** | Intelligent caching for faster repeated searches |
| 🎨 **TUI Mode** | Beautiful terminal user interface (optional) |

---

## 📥 Installation

### From Cargo (Recommended)

```bash
cargo install dx-media
```

### From Source

```bash
git clone https://github.com/user/dx-media
cd dx-media
cargo build --release
```

### Dependencies Setup

```bash
# Create a new project
cargo new dx-media
cd dx-media

# Add all dependencies (latest versions)
cargo add tokio --features full
cargo add reqwest --features "json stream multipart cookies rustls-tls"
cargo add serde --features derive
cargo add serde_json
cargo add toml
cargo add clap --features "derive env wrap_help color"
cargo add ratatui
cargo add crossterm
cargo add indicatif
cargo add thiserror
cargo add anyhow
cargo add tracing
cargo add tracing-subscriber --features "env-filter json"
cargo add dotenvy
cargo add tokio-util --features io
cargo add futures
cargo add futures-util
cargo add image
cargo add lopdf
cargo add zip
cargo add quick-xml
cargo add calamine
cargo add scraper
cargo add url
cargo add async-trait
cargo add governor
cargo add tokio-retry
cargo add backoff --features tokio
cargo add uuid --features v4
cargo add chrono --features serde
cargo add directories
cargo add colored
cargo add owo-colors
cargo add dialoguer
cargo add console
cargo add mime_guess
cargo add infer
cargo add rayon
cargo add sanitize-filename
cargo add bytes
cargo add sha2
cargo add hex
cargo add flate2
cargo add tar
cargo add base64
cargo add regex
cargo add lazy_static
cargo add html2text
cargo add pulldown-cmark
cargo add csv
cargo add semver
cargo add humansize
cargo add glob
cargo add tempfile
cargo add walkdir
cargo add open
cargo add webbrowser
cargo add urlencoding
cargo add percent-encoding
cargo add rand
cargo add once_cell
cargo add parking_lot
cargo add dashmap
cargo add strum --features derive
cargo add derive_more --features full
cargo add bon

# Dev dependencies
cargo add --dev wiremock
cargo add --dev assert_fs
cargo add --dev predicates
cargo add --dev tokio-test
cargo add --dev pretty_assertions
cargo add --dev mockall
```

### System Dependencies

For full format conversion support, install these optional system tools:

```bash
# macOS
brew install ffmpeg imagemagick

# Ubuntu/Debian
sudo apt install ffmpeg imagemagick

# Arch Linux
sudo pacman -S ffmpeg imagemagick

# Windows (using Chocolatey)
choco install ffmpeg imagemagick
```

---

## 🚀 Quick Start

### 1. Configure API Keys

Create a `.env` file in your project directory:

```env
# Image Providers
UNSPLASH_ACCESS_KEY=your_key_here
PEXELS_API_KEY=your_key_here
PIXABAY_API_KEY=your_key_here

# Audio Providers
FREESOUND_API_KEY=your_key_here

# 3D Providers
SKETCHFAB_API_KEY=your_key_here

# Code Providers (optional)
GITHUB_TOKEN=your_token_here
```

> 💡 **Tip**: Many providers offer free API keys. Check the [Providers](#-providers) section for signup links.

### 2. Basic Commands

```bash
# Search for images
dx search "mountain sunset" --type image

# Search for videos
dx search "ocean waves" --type video

# Search for music
dx search "ambient piano" --type music

# Search for 3D models
dx search "low poly tree" --type 3d

# Search for datasets
dx search "weather data" --type dataset

# Download an asset
dx download <asset-id> --provider unsplash

# Convert an image
dx convert photo.jpg --format webp --quality 85

# Convert a video
dx convert video.mp4 --format webm --width 1280

# List downloaded assets
dx list --type image

# Show available providers
dx providers

# Interactive mode
dx interactive
```

### 3. Library Usage

```rust
use dx_media::{DxMedia, MediaType, SearchQuery};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize DX Media
    let dx = DxMedia::new()?;

    // Search for images
    let results = dx.search("nature landscape", MediaType::Image).await?;
    
    println!("Found {} assets from {} providers", 
        results.total_assets,
        results.providers_searched.len()
    );

    // Download the first result
    if let Some(asset) = results.all_assets().first() {
        let path = dx.download(asset).await?;
        println!("Downloaded to: {}", path.display());
    }

    // Advanced search with filters
    let query = SearchQuery::new("abstract art", MediaType::Illustration)
        .with_page(1)
        .with_per_page(20)
        .with_providers(vec!["unsplash".into(), "pexels".into()]);
    
    let results = dx.search_with_query(query).await?;

    Ok(())
}
```

---

## 🌐 Providers

### Image Providers

| Provider | API Key Required | Rate Limit | License |
|----------|-----------------|------------|---------|
| [Unsplash](https://unsplash.com/developers) | ✅ Yes (Free) | 50/hour | Unsplash License |
| [Pexels](https://www.pexels.com/api/) | ✅ Yes (Free) | 200/hour | Pexels License |
| [Pixabay](https://pixabay.com/api/docs/) | ✅ Yes (Free) | 100/min | Pixabay License |
| [Lorem Picsum](https://picsum.photos/) | ❌ No | Unlimited | Unsplash License |
| [Placeholder.com](https://placeholder.com/) | ❌ No | Unlimited | Free |

### Video Providers

| Provider | API Key Required | Rate Limit | License |
|----------|-----------------|------------|---------|
| [Pexels Videos](https://www.pexels.com/api/) | ✅ Yes (Free) | 200/hour | Pexels License |
| [Pixabay Videos](https://pixabay.com/api/docs/) | ✅ Yes (Free) | 100/min | Pixabay License |
| [Coverr](https://coverr.co/) | ❌ No | - | CC0 |

### Audio Providers

| Provider | API Key Required | Rate Limit | License |
|----------|-----------------|------------|---------|
| [Freesound](https://freesound.org/apiv2/) | ✅ Yes (Free) | 60/min | Various CC |
| [Jamendo](https://developer.jamendo.com/) | ✅ Yes (Free) | 120/min | CC Licenses |
| [Free Music Archive](https://freemusicarchive.org/) | ❌ No | - | Various CC |
| [BBC Sound Effects](https://sound-effects.bbcrewind.co.uk/) | ❌ No | - | Personal Use |

### Text Providers

| Provider | API Key Required | Rate Limit | License |
|----------|-----------------|------------|---------|
| [Wikipedia](https://www.mediawiki.org/wiki/API:Main_page) | ❌ No | Reasonable | CC BY-SA |
| [Project Gutenberg](https://gutendex.com/) | ❌ No | Reasonable | Public Domain |
| [arXiv](https://arxiv.org/help/api/) | ❌ No | Reasonable | Various |
| [Open Library](https://openlibrary.org/developers/api) | ❌ No | Reasonable | Various |

### Document Providers

| Provider | API Key Required | Rate Limit | License |
|----------|-----------------|------------|---------|
| [Internet Archive](https://archive.org/developers/) | ❌ No | Reasonable | Various |
| [Scribd (Free)](https://www.scribd.com/) | ❌ No (Scraping) | - | Various |
| [SlideShare](https://www.slideshare.net/) | ❌ No (Scraping) | - | Various |

### 3D Model Providers

| Provider | API Key Required | Rate Limit | License |
|----------|-----------------|------------|---------|
| [Sketchfab](https://sketchfab.com/developers) | ✅ Yes (Free) | 50/10s | Various CC |
| [Poly Pizza](https://poly.pizza/) | ❌ No | Reasonable | CC0 |
| [Kenney](https://kenney.nl/) | ❌ No | - | CC0 |
| [OpenGameArt](https://opengameart.org/) | ❌ No | - | Various CC |

### Data Providers

| Provider | API Key Required | Rate Limit | License |
|----------|-----------------|------------|---------|
| [Kaggle](https://www.kaggle.com/docs/api) | ✅ Yes (Free) | Reasonable | Various |
| [Data.gov](https://www.data.gov/) | ❌ No | Reasonable | Public Domain |
| [GitHub Datasets](https://github.com/) | ⚪ Optional | 60/hour | Various |
| [Awesome Public Datasets](https://github.com/awesomedata/awesome-public-datasets) | ❌ No | - | Various |

### Code Providers

| Provider | API Key Required | Rate Limit | License |
|----------|-----------------|------------|---------|
| [GitHub Gists](https://docs.github.com/en/rest/gists) | ⚪ Optional | 60/hour | Various |
| [GitLab Snippets](https://docs.gitlab.com/ee/api/snippets.html) | ⚪ Optional | Reasonable | Various |
| [CodePen](https://codepen.io/) | ❌ No (Scraping) | - | Various |

### Game Asset Providers

| Provider | API Key Required | Rate Limit | License |
|----------|-----------------|------------|---------|
| [OpenGameArt](https://opengameart.org/) | ❌ No | Reasonable | Various CC |
| [itch.io](https://itch.io/game-assets/free) | ❌ No | Reasonable | Various |
| [Kenney](https://kenney.nl/assets) | ❌ No | - | CC0 |
| [Game-Icons.net](https://game-icons.net/) | ❌ No | - | CC BY 3.0 |

---

## 📖 Documentation

### Command Reference

#### Search Command

```bash
dx search <query> [options]

Arguments:
  <query>               Search keywords

Options:
  -t, --type <TYPE>     Media type [default: image]
                        Values: image, video, audio, music, 3d, text, 
                                document, pdf, spreadsheet, presentation,
                                dataset, json, csv, code, template, 
                                game-asset, sprite, tileset, etc.
  
  -n, --limit <N>       Results per page [default: 10]
  -p, --page <N>        Page number [default: 1]
  -P, --providers <P>   Specific providers (comma-separated)
  -d, --download        Download results immediately
  -o, --output <DIR>    Output directory
  --orientation <O>     Image orientation (landscape, portrait, square)
  --color <COLOR>       Dominant color filter
  --min-width <W>       Minimum width
  --min-height <H>      Minimum height
  --safe-search         Enable safe search [default: true]
  --format <F>          Output format (text, json, table)

Examples:
  dx search "sunset beach" --type image --limit 20
  dx search "epic orchestral" --type music --providers freesound,jamendo
  dx search "neural network" --type dataset --download
```

#### Download Command

```bash
dx download <source> [options]

Arguments:
  <source>              Asset URL, ID, or search result index

Options:
  -p, --provider <P>    Provider name (required if using ID)
  -o, --output <DIR>    Output directory [default: ./media/<type>]
  -c, --convert <FMT>   Convert to format after download
  --quality <Q>         Quality for conversion (1-100)
  --overwrite           Overwrite existing files
  --no-progress         Hide progress bar

Examples:
  dx download abc123 --provider unsplash
  dx download https://unsplash.com/photos/abc123
  dx download 1 --convert webp --quality 90
```

#### Convert Command

```bash
dx convert <input> [options]

Arguments:
  <input>               Input file path or glob pattern

Options:
  -f, --format <FMT>    Target format (required)
  -o, --output <PATH>   Output path or directory
  -q, --quality <Q>     Quality (1-100) [default: 85]
  --width <W>           Target width
  --height <H>          Target height
  --keep-aspect         Maintain aspect ratio [default: true]
  --keep-original       Keep original file [default: true]
  --batch               Process multiple files

Supported Conversions:
  Images: jpg, png, webp, gif, bmp, tiff, avif, ico
  Videos: mp4, webm, mov, avi, mkv, gif
  Audio:  mp3, wav, ogg, flac, aac, m4a
  Docs:   pdf (from docx, xlsx, pptx, etc.)

Examples:
  dx convert photo.png --format webp --quality 90
  dx convert video.mp4 --format webm --width 1280
  dx convert "*.png" --format jpg --batch
```

#### List Command

```bash
dx list [options]

Options:
  -t, --type <TYPE>     Filter by media type
  -p, --provider <P>    Filter by provider
  -d, --detailed        Show detailed information
  --sort <FIELD>        Sort by field (date, name, size, type)
  --format <F>          Output format (text, json, table)

Examples:
  dx list --type image --detailed
  dx list --provider unsplash --sort date
```

#### Providers Command

```bash
dx providers [options]

Options:
  -a, --available       Show only available providers
  -t, --type <TYPE>     Show providers for media type
  --check               Check API connectivity

Examples:
  dx providers --available
  dx providers --type audio --check
```

### Configuration

DX Media can be configured via:

1. **Environment Variables** (`.env` file)
2. **Config File** (`~/.config/dx-media/config.toml`)
3. **Command Line Arguments**

#### Config File Example

```toml
# ~/.config/dx-media/config.toml

[general]
media_dir = "~/Media/dx-media"
cache_dir = "~/.cache/dx-media"
default_type = "image"

[download]
concurrent = 5
retry_attempts = 3
timeout_seconds = 300
show_progress = true
verify_integrity = true
overwrite = false

[conversion]
default_image_quality = 85
default_video_crf = 23
default_audio_bitrate = 192
keep_original = true

[providers]
# Enabled providers (empty = all available)
enabled = []
# Disabled providers
disabled = []
# Custom rate limits
[providers.rate_limits]
unsplash = 50
pexels = 200

[display]
colors = true
verbose = false
format = "text"  # text, json, table
```

---

## 🏗️ Architecture

```
dx-media/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library exports
│   │
│   ├── core/                # Core types and utilities
│   │   ├── mod.rs
│   │   ├── media_type.rs    # Media type definitions
│   │   ├── asset.rs         # Asset structs
│   │   ├── error.rs         # Error types
│   │   └── downloader.rs    # Download manager
│   │
│   ├── config/              # Configuration
│   │   ├── mod.rs
│   │   └── settings.rs
│   │
│   ├── providers/           # Provider implementations
│   │   ├── mod.rs
│   │   ├── traits.rs        # Provider traits
│   │   ├── registry.rs      # Provider registry
│   │   ├── images/          # Image providers
│   │   ├── videos/          # Video providers
│   │   ├── audio/           # Audio providers
│   │   ├── text/            # Text providers
│   │   ├── documents/       # Document providers
│   │   ├── models3d/        # 3D model providers
│   │   ├── code/            # Code providers
│   │   ├── data/            # Data providers
│   │   └── game_assets/     # Game asset providers
│   │
│   ├── converters/          # Format converters
│   │   ├── mod.rs
│   │   ├── traits.rs
│   │   ├── image.rs
│   │   ├── video.rs
│   │   ├── audio.rs
│   │   └── document.rs
│   │
│   ├── scraper/             # Web scraping utilities
│   │   ├── mod.rs
│   │   └── extractors/
│   │
│   ├── cli/                 # CLI implementation
│   │   ├── mod.rs
│   │   └── commands/
│   │
│   └── utils/               # Utilities
│       ├── mod.rs
│       ├── file.rs
│       ├── cache.rs
│       └── progress.rs
│
├── media/                   # Downloaded assets
├── cache/                   # Cache directory
└── tests/                   # Integration tests
```

---

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run with logging
RUST_LOG=debug cargo test

# Run specific test
cargo test test_unsplash_search

# Run integration tests
cargo test --test integration
```

---

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Adding a New Provider

1. Create a new file in the appropriate `providers/` subdirectory
2. Implement the `AssetProvider` trait
3. Register the provider in `providers/registry.rs`
4. Add tests in `tests/`
5. Update documentation

---

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- All the amazing free asset providers that make this possible
- The Rust community for excellent crates
- Contributors and users of DX Media

---

## 📊 Crate Dependencies

| Category | Crate | Purpose |
|----------|-------|---------|
| **Runtime** | `tokio` | Async runtime |
| **HTTP** | `reqwest` | HTTP client |
| **Serialization** | `serde`, `serde_json`, `toml` | Data serialization |
| **CLI** | `clap` | Command-line parsing |
| **TUI** | `ratatui`, `crossterm` | Terminal UI |
| **Progress** | `indicatif` | Progress bars |
| **Errors** | `thiserror`, `anyhow` | Error handling |
| **Logging** | `tracing`, `tracing-subscriber` | Logging |
| **Images** | `image` | Image processing |
| **Documents** | `lopdf`, `calamine`, `zip`, `quick-xml` | Document handling |
| **Scraping** | `scraper` | HTML parsing |
| **Utilities** | `uuid`, `chrono`, `regex`, `glob` | Various utilities |
| **Concurrency** | `rayon`, `dashmap`, `parking_lot` | Parallel processing |

---

<div align="center">

**Made with ❤️ and 🦀**

[Report Bug](https://github.com/user/dx-media/issues) •
[Request Feature](https://github.com/user/dx-media/issues)

</div>
```

---

## Updated Media Types (Excluding Icons & Fonts)

Now let me update the `media_type.rs` to **exclude icons and fonts**:

```rust
// src/core/media_type.rs

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use strum::{EnumIter, IntoEnumIterator};

/// All supported media types in DX Media
/// 
/// NOTE: Icons and Fonts are NOT included here as they are handled by:
/// - dx-icons: Dedicated icon management
/// - dx-fonts: Dedicated font management
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
#[serde(rename_all = "snake_case")]
pub enum MediaType {
    // ===== IMAGES & VISUAL =====
    Image,
    Photo,
    Illustration,
    Vector,
    Wallpaper,
    Screenshot,
    Meme,
    Infographic,
    Artwork,
    Painting,
    Drawing,
    Sketch,
    Diagram,
    Chart,
    Graph,
    Map,
    Poster,
    Banner,
    Thumbnail,
    Avatar,
    Cover,
    
    // ===== VIDEO =====
    Video,
    Animation,
    Gif,
    Footage,
    StockFootage,
    Tutorial,
    Timelapse,
    SlowMotion,
    Drone,
    MotionGraphics,
    Intro,
    Outro,
    Transition,
    Overlay,
    LowerThird,
    
    // ===== AUDIO =====
    Audio,
    Music,
    Song,
    Track,
    SoundEffect,
    Sfx,
    Podcast,
    Audiobook,
    Voice,
    VoiceOver,
    Narration,
    Ambient,
    Loop,
    Jingle,
    Ringtone,
    
    // ===== TEXT CONTENT =====
    Text,
    Article,
    BlogPost,
    NewsArticle,
    Essay,
    Book,
    Ebook,
    Novel,
    ShortStory,
    Story,
    Poem,
    Poetry,
    Quote,
    Lyrics,
    Script,
    Screenplay,
    Subtitle,
    Caption,
    Documentation,
    Readme,
    Wiki,
    Manual,
    Guide,
    Tutorial as TutorialText,
    HowTo,
    Faq,
    License as LicenseText,
    Changelog,
    ReleaseNotes,
    
    // ===== DOCUMENTS =====
    Document,
    Pdf,
    Word,
    Docx,
    Doc,
    Odt,
    Rtf,
    Report,
    Whitepaper,
    Research,
    Paper,
    Thesis,
    Resume,
    Cv,
    CoverLetter,
    Contract,
    Invoice,
    Receipt,
    Form,
    Certificate,
    Diploma,
    Letter,
    Memo,
    Minutes,
    Proposal,
    BusinessPlan,
    
    // ===== SPREADSHEETS =====
    Spreadsheet,
    Excel,
    Xlsx,
    Xls,
    Csv,
    Tsv,
    Ods,
    Table,
    DataTable,
    PivotTable,
    Budget,
    FinancialStatement,
    Inventory,
    Schedule,
    Calendar,
    Gantt,
    
    // ===== PRESENTATIONS =====
    Presentation,
    Powerpoint,
    Pptx,
    Ppt,
    Odp,
    Keynote,
    Slides,
    Deck,
    Pitch,
    PitchDeck,
    
    // ===== DATA & DATASETS =====
    Data,
    Dataset,
    Json,
    JsonLines,
    Ndjson,
    GeoJson,
    Xml,
    Yaml,
    Toml,
    Ini,
    Env,
    Parquet,
    Avro,
    Orc,
    Sqlite,
    Database,
    Sql,
    GraphQl,
    Api,
    ApiResponse,
    MockData,
    SampleData,
    TestData,
    TrainingData,
    
    // ===== 3D ASSETS =====
    Model3D,
    Mesh,
    Geometry,
    Texture,
    Material,
    Shader,
    Hdri,
    Skybox,
    Environment,
    Cubemap,
    NormalMap,
    HeightMap,
    DisplacementMap,
    AoMap,
    RoughnessMap,
    MetallicMap,
    Rig,
    Animation3D,
    MotionCapture,
    Mocap,
    
    // ===== CODE & DEVELOPMENT =====
    Code,
    SourceCode,
    Snippet,
    Gist,
    CodeBlock,
    Function,
    Class,
    Module,
    Library,
    Package,
    Crate,
    Template,
    Boilerplate,
    Starter,
    Scaffold,
    Example,
    Demo,
    Sample,
    Algorithm,
    DataStructure,
    DesignPattern,
    
    // ===== CONFIGURATION =====
    Config,
    Configuration,
    Settings,
    Preferences,
    Dotfile,
    EnvFile,
    DockerFile,
    Makefile,
    BuildFile,
    Manifest,
    Lockfile,
    
    // ===== SCRIPTS =====
    Script,
    ShellScript,
    Bash,
    PowerShell,
    Python as PythonScript,
    JavaScript as JsScript,
    Automation,
    
    // ===== GAME ASSETS =====
    GameAsset,
    Sprite,
    Spritesheet,
    SpriteAtlas,
    Tileset,
    Tilemap,
    TileAtlas,
    UiKit,
    UiElement,
    Button,
    Panel,
    Window,
    Hud,
    CharacterAsset,
    Character,
    Enemy,
    Npc,
    Player,
    Background,
    Parallax,
    Platform,
    Prop,
    Item,
    Collectible,
    PowerUp,
    Weapon,
    Effect,
    Particle,
    Explosion,
    VFX,
    LevelDesign,
    MapData,
    Dialogue,
    QuestData,
    
    // ===== DESIGN ASSETS =====
    Design,
    Mockup,
    Wireframe,
    Prototype,
    UIDesign,
    UXDesign,
    WebDesign,
    AppDesign,
    Logo,
    Branding,
    BrandKit,
    StyleGuide,
    DesignSystem,
    ColorPalette,
    ColorScheme,
    Gradient,
    Pattern,
    Swatch,
    
    // ===== PRINT & MARKETING =====
    Print,
    Flyer,
    Brochure,
    BusinessCard,
    Letterhead,
    Envelope,
    Stationery,
    PackageDesign,
    LabelDesign,
    MenuDesign,
    SignDesign,
    
    // ===== SOCIAL MEDIA =====
    SocialMedia,
    InstagramPost,
    InstagramStory,
    FacebookPost,
    TwitterPost,
    LinkedInPost,
    YouTubeThumbnail,
    TwitchOverlay,
    DiscordEmoji,
    
    // ===== ARCHIVES & BUNDLES =====
    Archive,
    Zip,
    Tarball,
    Gzip,
    SevenZip,
    Rar,
    Package as ArchivePackage,
    Bundle,
    Collection,
    Pack,
    Kit,
    Suite,
    
    // ===== COMICS & CARTOONS =====
    Toon,
    Cartoon,
    Comic,
    ComicStrip,
    ComicBook,
    Manga,
    Manhwa,
    Webtoon,
    GraphicNovel,
    Panel,
    Storyboard,
    
    // ===== MISC =====
    Emoji,
    Sticker,
    Cursor,
    Theme,
    Skin,
    Preset,
    Lut,
    Filter,
    Action,
    Brush,
    
    // ===== MACHINE LEARNING =====
    MLModel,
    TrainedModel,
    Weights,
    Checkpoint,
    Embedding,
    Tokenizer,
    Vocabulary,
    
    Other,
}

impl MediaType {
    /// Check if this media type is handled by external DX tools
    /// Returns the tool name if handled externally, None otherwise
    pub fn external_handler(&self) -> Option<&'static str> {
        // Icons and Fonts are handled by dedicated tools
        // This method exists for future compatibility
        None  // All types in this enum are handled by dx-media
    }

    /// Get the directory name for this media type
    pub fn directory_name(&self) -> &'static str {
        match self {
            // Images
            MediaType::Image | MediaType::Photo | MediaType::Illustration |
            MediaType::Vector | MediaType::Wallpaper | MediaType::Screenshot |
            MediaType::Meme | MediaType::Infographic | MediaType::Artwork |
            MediaType::Painting | MediaType::Drawing | MediaType::Sketch |
            MediaType::Diagram | MediaType::Chart | MediaType::Graph |
            MediaType::Map | MediaType::Poster | MediaType::Banner |
            MediaType::Thumbnail | MediaType::Avatar | MediaType::Cover => "images",
            
            // Video
            MediaType::Video | MediaType::Animation | MediaType::Footage |
            MediaType::StockFootage | MediaType::Tutorial | MediaType::Timelapse |
            MediaType::SlowMotion | MediaType::Drone | MediaType::MotionGraphics => "videos",
            
            MediaType::Gif => "videos/gifs",
            
            MediaType::Intro | MediaType::Outro | MediaType::Transition |
            MediaType::Overlay | MediaType::LowerThird => "videos/templates",
            
            // Audio
            MediaType::Audio | MediaType::Music | MediaType::Song |
            MediaType::Track => "audio/music",
            
            MediaType::SoundEffect | MediaType::Sfx => "audio/sfx",
            
            MediaType::Podcast => "audio/podcasts",
            MediaType::Audiobook => "audio/audiobooks",
            
            MediaType::Voice | MediaType::VoiceOver | MediaType::Narration => "audio/voice",
            
            MediaType::Ambient | MediaType::Loop => "audio/ambient",
            MediaType::Jingle | MediaType::Ringtone => "audio/jingles",
            
            // Text
            MediaType::Text | MediaType::Article | MediaType::BlogPost |
            MediaType::NewsArticle | MediaType::Essay => "text/articles",
            
            MediaType::Book | MediaType::Ebook | MediaType::Novel |
            MediaType::ShortStory | MediaType::Story => "text/books",
            
            MediaType::Poem | MediaType::Poetry | MediaType::Lyrics => "text/poetry",
            
            MediaType::Quote => "text/quotes",
            
            MediaType::Script | MediaType::Screenplay | MediaType::Subtitle |
            MediaType::Caption => "text/scripts",
            
            MediaType::Documentation | MediaType::Readme | MediaType::Wiki |
            MediaType::Manual | MediaType::Guide | MediaType::TutorialText |
            MediaType::HowTo | MediaType::Faq => "text/docs",
            
            MediaType::LicenseText | MediaType::Changelog | 
            MediaType::ReleaseNotes => "text/meta",
            
            // Documents
            MediaType::Document | MediaType::Pdf => "documents/pdf",
            
            MediaType::Word | MediaType::Docx | MediaType::Doc |
            MediaType::Odt | MediaType::Rtf => "documents/word",
            
            MediaType::Report | MediaType::Whitepaper | MediaType::Research |
            MediaType::Paper | MediaType::Thesis => "documents/research",
            
            MediaType::Resume | MediaType::Cv | MediaType::CoverLetter => "documents/resume",
            
            MediaType::Contract | MediaType::Invoice | MediaType::Receipt |
            MediaType::Form | MediaType::Certificate | MediaType::Diploma => "documents/legal",
            
            MediaType::Letter | MediaType::Memo | MediaType::Minutes |
            MediaType::Proposal | MediaType::BusinessPlan => "documents/business",
            
            // Spreadsheets
            MediaType::Spreadsheet | MediaType::Excel | MediaType::Xlsx |
            MediaType::Xls | MediaType::Ods | MediaType::Table |
            MediaType::DataTable | MediaType::PivotTable => "documents/spreadsheets",
            
            MediaType::Csv | MediaType::Tsv => "data/csv",
            
            MediaType::Budget | MediaType::FinancialStatement |
            MediaType::Inventory => "documents/finance",
            
            MediaType::Schedule | MediaType::Calendar | 
            MediaType::Gantt => "documents/planning",
            
            // Presentations
            MediaType::Presentation | MediaType::Powerpoint | MediaType::Pptx |
            MediaType::Ppt | MediaType::Odp | MediaType::Keynote |
            MediaType::Slides | MediaType::Deck | MediaType::Pitch |
            MediaType::PitchDeck => "documents/presentations",
            
            // Data
            MediaType::Data | MediaType::Dataset => "data/datasets",
            MediaType::Json | MediaType::JsonLines | MediaType::Ndjson => "data/json",
            MediaType::GeoJson => "data/geojson",
            MediaType::Xml => "data/xml",
            MediaType::Yaml | MediaType::Toml | MediaType::Ini |
            MediaType::Env => "data/config",
            MediaType::Parquet | MediaType::Avro | MediaType::Orc => "data/columnar",
            MediaType::Sqlite | MediaType::Database | MediaType::Sql => "data/databases",
            MediaType::GraphQl | MediaType::Api | MediaType::ApiResponse => "data/api",
            MediaType::MockData | MediaType::SampleData | MediaType::TestData => "data/samples",
            MediaType::TrainingData => "data/ml",
            
            // 3D
            MediaType::Model3D | MediaType::Mesh | MediaType::Geometry => "3d/models",
            
            MediaType::Texture | MediaType::Material | MediaType::Shader |
            MediaType::NormalMap | MediaType::HeightMap | MediaType::DisplacementMap |
            MediaType::AoMap | MediaType::RoughnessMap | 
            MediaType::MetallicMap => "3d/textures",
            
            MediaType::Hdri | MediaType::Skybox | MediaType::Environment |
            MediaType::Cubemap => "3d/environments",
            
            MediaType::Rig | MediaType::Animation3D | MediaType::MotionCapture |
            MediaType::Mocap => "3d/animations",
            
            // Code
            MediaType::Code | MediaType::SourceCode | MediaType::Snippet |
            MediaType::Gist | MediaType::CodeBlock | MediaType::Function |
            MediaType::Class | MediaType::Module => "code/snippets",
            
            MediaType::Library | MediaType::Package | MediaType::Crate => "code/libraries",
            
            MediaType::Template | MediaType::Boilerplate | MediaType::Starter |
            MediaType::Scaffold => "code/templates",
            
            MediaType::Example | MediaType::Demo | MediaType::Sample => "code/examples",
            
            MediaType::Algorithm | MediaType::DataStructure |
            MediaType::DesignPattern => "code/algorithms",
            
            // Config
            MediaType::Config | MediaType::Configuration | MediaType::Settings |
            MediaType::Preferences | MediaType::Dotfile | MediaType::EnvFile |
            MediaType::DockerFile | MediaType::Makefile | MediaType::BuildFile |
            MediaType::Manifest | MediaType::Lockfile => "code/config",
            
            // Scripts
            MediaType::Script | MediaType::ShellScript | MediaType::Bash |
            MediaType::PowerShell | MediaType::PythonScript | MediaType::JsScript |
            MediaType::Automation => "code/scripts",
            
            // Game Assets
            MediaType::GameAsset => "game-assets",
            
            MediaType::Sprite | MediaType::Spritesheet | 
            MediaType::SpriteAtlas => "game-assets/sprites",
            
            MediaType::Tileset | MediaType::Tilemap | 
            MediaType::TileAtlas => "game-assets/tiles",
            
            MediaType::UiKit | MediaType::UiElement | MediaType::Button |
            MediaType::Panel | MediaType::Window | MediaType::Hud => "game-assets/ui",
            
            MediaType::CharacterAsset | MediaType::Character | MediaType::Enemy |
            MediaType::Npc | MediaType::Player => "game-assets/characters",
            
            MediaType::Background | MediaType::Parallax => "game-assets/backgrounds",
            
            MediaType::Platform | MediaType::Prop | MediaType::Item |
            MediaType::Collectible | MediaType::PowerUp | 
            MediaType::Weapon => "game-assets/objects",
            
            MediaType::Effect | MediaType::Particle | MediaType::Explosion |
            MediaType::VFX => "game-assets/effects",
            
            MediaType::LevelDesign | MediaType::MapData => "game-assets/levels",
            MediaType::Dialogue | MediaType::QuestData => "game-assets/data",
            
            // Design
            MediaType::Design | MediaType::Mockup | MediaType::Wireframe |
            MediaType::Prototype | MediaType::UIDesign | MediaType::UXDesign |
            MediaType::WebDesign | MediaType::AppDesign => "design/mockups",
            
            MediaType::Logo | MediaType::Branding | MediaType::BrandKit |
            MediaType::StyleGuide | MediaType::DesignSystem => "design/branding",
            
            MediaType::ColorPalette | MediaType::ColorScheme | MediaType::Gradient |
            MediaType::Pattern | MediaType::Swatch => "design/colors",
            
            // Print
            MediaType::Print | MediaType::Flyer | MediaType::Brochure |
            MediaType::BusinessCard | MediaType::Letterhead | MediaType::Envelope |
            MediaType::Stationery => "design/print",
            
            MediaType::PackageDesign | MediaType::LabelDesign |
            MediaType::MenuDesign | MediaType::SignDesign => "design/packaging",
            
            // Social Media
            MediaType::SocialMedia | MediaType::InstagramPost | MediaType::InstagramStory |
            MediaType::FacebookPost | MediaType::TwitterPost | MediaType::LinkedInPost |
            MediaType::YouTubeThumbnail | MediaType::TwitchOverlay |
            MediaType::DiscordEmoji => "design/social",
            
            // Archives
            MediaType::Archive | MediaType::Zip | MediaType::Tarball |
            MediaType::Gzip | MediaType::SevenZip | MediaType::Rar |
            MediaType::ArchivePackage | MediaType::Bundle | MediaType::Collection |
            MediaType::Pack | MediaType::Kit | MediaType::Suite => "archives",
            
            // Comics
            MediaType::Toon | MediaType::Cartoon => "images/cartoons",
            
            MediaType::Comic | MediaType::ComicStrip | MediaType::ComicBook |
            MediaType::Manga | MediaType::Manhwa | MediaType::Webtoon |
            MediaType::GraphicNovel => "comics",
            
            MediaType::Panel | MediaType::Storyboard => "comics/panels",
            
            // Misc
            MediaType::Emoji | MediaType::Sticker => "images/stickers",
            MediaType::Cursor => "cursors",
            MediaType::Theme | MediaType::Skin => "themes",
            MediaType::Preset | MediaType::Lut | MediaType::Filter |
            MediaType::Action | MediaType::Brush => "presets",
            
            // ML
            MediaType::MLModel | MediaType::TrainedModel | MediaType::Weights |
            MediaType::Checkpoint | MediaType::Embedding | MediaType::Tokenizer |
            MediaType::Vocabulary => "ml-models",
            
            MediaType::Other => "other",
        }
    }

    /// Get common file extensions for this media type
    pub fn extensions(&self) -> Vec<&'static str> {
        match self {
            // Images (detailed)
            MediaType::Image | MediaType::Photo | MediaType::Wallpaper |
            MediaType::Screenshot | MediaType::Meme | MediaType::Artwork |
            MediaType::Painting | MediaType::Thumbnail | MediaType::Avatar |
            MediaType::Cover => {
                vec!["jpg", "jpeg", "png", "webp", "avif", "heic", "bmp", "tiff"]
            }
            
            MediaType::Illustration | MediaType::Drawing | MediaType::Sketch => {
                vec!["png", "jpg", "psd", "ai", "svg", "webp"]
            }
            
            MediaType::Vector | MediaType::Diagram | MediaType::Chart |
            MediaType::Graph | MediaType::Map | MediaType::Infographic => {
                vec!["svg", "ai", "eps", "pdf"]
            }
            
            MediaType::Poster | MediaType::Banner => {
                vec!["jpg", "png", "psd", "ai", "pdf", "svg"]
            }
            
            // Video (detailed)
            MediaType::Video | MediaType::Footage | MediaType::StockFootage |
            MediaType::Tutorial | MediaType::Timelapse | MediaType::SlowMotion |
            MediaType::Drone => {
                vec!["mp4", "webm", "mov", "avi", "mkv", "m4v", "prores"]
            }
            
            MediaType::Animation | MediaType::MotionGraphics => {
                vec!["mp4", "webm", "mov", "json", "lottie"]
            }
            
            MediaType::Gif => vec!["gif", "webp"],
            
            MediaType::Intro | MediaType::Outro | MediaType::Transition |
            MediaType::Overlay | MediaType::LowerThird => {
                vec!["mp4", "webm", "mov", "prores", "aep"]
            }
            
            // Audio (detailed)
            MediaType::Audio | MediaType::Music | MediaType::Song | MediaType::Track => {
                vec!["mp3", "wav", "flac", "ogg", "m4a", "aac", "opus"]
            }
            
            MediaType::SoundEffect | MediaType::Sfx | MediaType::Ambient |
            MediaType::Loop => {
                vec!["wav", "mp3", "ogg", "flac"]
            }
            
            MediaType::Podcast | MediaType::Audiobook => {
                vec!["mp3", "m4a", "m4b", "ogg"]
            }
            
            MediaType::Voice | MediaType::VoiceOver | MediaType::Narration => {
                vec!["mp3", "wav", "ogg", "m4a"]
            }
            
            MediaType::Jingle | MediaType::Ringtone => {
                vec!["mp3", "m4r", "ogg", "wav"]
            }
            
            // Text files
            MediaType::Text | MediaType::Article | MediaType::BlogPost |
            MediaType::NewsArticle | MediaType::Essay | MediaType::Quote => {
                vec!["txt", "md", "markdown", "rst", "adoc"]
            }
            
            MediaType::Book | MediaType::Ebook | MediaType::Novel |
            MediaType::ShortStory | MediaType::Story => {
                vec!["epub", "mobi", "azw", "azw3", "pdf", "txt"]
            }
            
            MediaType::Poem | MediaType::Poetry | MediaType::Lyrics => {
                vec!["txt", "md", "pdf"]
            }
            
            MediaType::Script | MediaType::Screenplay => {
                vec!["txt", "pdf", "fdx", "fountain"]
            }
            
            MediaType::Subtitle | MediaType::Caption => {
                vec!["srt", "vtt", "ass", "ssa", "sub"]
            }
            
            MediaType::Documentation | MediaType::Readme | MediaType::Wiki |
            MediaType::Manual | MediaType::Guide | MediaType::TutorialText |
            MediaType::HowTo | MediaType::Faq => {
                vec!["md", "rst", "adoc", "txt", "html"]
            }
            
            MediaType::LicenseText => vec!["txt", "md", "LICENSE"],
            MediaType::Changelog | MediaType::ReleaseNotes => vec!["md", "txt", "CHANGELOG"],
            
            // Documents (all formats)
            MediaType::Pdf => vec!["pdf"],
            MediaType::Word | MediaType::Docx => vec!["docx", "doc"],
            MediaType::Doc => vec!["doc"],
            MediaType::Odt => vec!["odt"],
            MediaType::Rtf => vec!["rtf"],
            
            MediaType::Document | MediaType::Report | MediaType::Whitepaper |
            MediaType::Research | MediaType::Paper | MediaType::Thesis |
            MediaType::Resume | MediaType::Cv | MediaType::CoverLetter |
            MediaType::Contract | MediaType::Letter | MediaType::Memo |
            MediaType::Proposal | MediaType::BusinessPlan => {
                vec!["pdf", "docx", "doc", "odt", "rtf"]
            }
            
            MediaType::Invoice | MediaType::Receipt | MediaType::Form => {
                vec!["pdf", "docx", "xlsx"]
            }
            
            MediaType::Certificate | MediaType::Diploma | MediaType::Minutes => {
                vec!["pdf", "docx"]
            }
            
            // Spreadsheets
            MediaType::Spreadsheet | MediaType::Excel | MediaType::Xlsx => {
                vec!["xlsx", "xls", "ods", "csv"]
            }
            MediaType::Xls => vec!["xls"],
            MediaType::Ods => vec!["ods"],
            MediaType::Csv => vec!["csv"],
            MediaType::Tsv => vec!["tsv"],
            
            MediaType::Table | MediaType::DataTable | MediaType::PivotTable |
            MediaType::Budget | MediaType::FinancialStatement | MediaType::Inventory |
            MediaType::Schedule | MediaType::Calendar | MediaType::Gantt => {
                vec!["xlsx", "xls", "ods", "csv"]
            }
            
            // Presentations
            MediaType::Presentation | MediaType::Powerpoint | MediaType::Pptx |
            MediaType::Slides | MediaType::Deck | MediaType::Pitch |
            MediaType::PitchDeck => {
                vec!["pptx", "ppt", "odp", "key", "pdf"]
            }
            MediaType::Ppt => vec!["ppt"],
            MediaType::Odp => vec!["odp"],
            MediaType::Keynote => vec!["key"],
            
            // Data formats
            MediaType::Data | MediaType::Dataset | MediaType::MockData |
            MediaType::SampleData | MediaType::TestData | MediaType::TrainingData => {
                vec!["json", "csv", "xml", "parquet", "sqlite"]
            }
            
            MediaType::Json | MediaType::JsonLines | MediaType::Ndjson => {
                vec!["json", "jsonl", "ndjson"]
            }
            MediaType::GeoJson => vec!["geojson", "json"],
            MediaType::Xml => vec!["xml", "xsd", "xsl"],
            MediaType::Yaml => vec!["yaml", "yml"],
            MediaType::Toml => vec!["toml"],
            MediaType::Ini => vec!["ini", "cfg", "conf"],
            MediaType::Env => vec!["env"],
            
            MediaType::Parquet => vec!["parquet"],
            MediaType::Avro => vec!["avro", "avsc"],
            MediaType::Orc => vec!["orc"],
            
            MediaType::Sqlite | MediaType::Database => vec!["sqlite", "sqlite3", "db"],
            MediaType::Sql => vec!["sql"],
            MediaType::GraphQl => vec!["graphql", "gql"],
            MediaType::Api | MediaType::ApiResponse => vec!["json", "xml"],
            
            // 3D
            MediaType::Model3D | MediaType::Mesh | MediaType::Geometry => {
                vec!["glb", "gltf", "obj", "fbx", "stl", "blend", "dae", "3ds", "usdz"]
            }
            
            MediaType::Texture | MediaType::NormalMap | MediaType::HeightMap |
            MediaType::DisplacementMap | MediaType::AoMap | MediaType::RoughnessMap |
            MediaType::MetallicMap => {
                vec!["png", "jpg", "tga", "exr", "tiff", "dds"]
            }
            
            MediaType::Material | MediaType::Shader => {
                vec!["mtl", "mat", "shader", "glsl", "hlsl"]
            }
            
            MediaType::Hdri | MediaType::Skybox | MediaType::Environment |
            MediaType::Cubemap => {
                vec!["hdr", "exr", "hdri"]
            }
            
            MediaType::Rig => vec!["blend", "fbx", "ma", "mb"],
            MediaType::Animation3D | MediaType::MotionCapture | MediaType::Mocap => {
                vec!["fbx", "bvh", "blend", "ma"]
            }
            
            // Code
            MediaType::Code | MediaType::SourceCode | MediaType::Snippet |
            MediaType::Gist | MediaType::CodeBlock | MediaType::Function |
            MediaType::Class | MediaType::Module | MediaType::Algorithm |
            MediaType::DataStructure | MediaType::DesignPattern => {
                vec![
                    "rs", "py", "js", "ts", "go", "java", "c", "cpp", "h", "hpp",
                    "cs", "rb", "php", "swift", "kt", "scala", "r", "jl", "lua",
                    "zig", "nim", "v", "d", "ex", "exs", "clj", "hs", "ml", "fs"
                ]
            }
            
            MediaType::Library | MediaType::Package | MediaType::Crate => {
                vec!["zip", "tar.gz", "tgz"]
            }
            
            MediaType::Template | MediaType::Boilerplate | MediaType::Starter |
            MediaType::Scaffold => {
                vec!["html", "css", "js", "jsx", "tsx", "vue", "svelte"]
            }
            
            MediaType::Example | MediaType::Demo | MediaType::Sample => {
                vec!["rs", "py", "js", "ts", "go", "java"]
            }
            
            // Config files
            MediaType::Config | MediaType::Configuration | MediaType::Settings |
            MediaType::Preferences => {
                vec!["json", "yaml", "yml", "toml", "ini", "cfg", "conf"]
            }
            
            MediaType::Dotfile => vec![""],
            MediaType::EnvFile => vec!["env"],
            MediaType::DockerFile => vec!["dockerfile", "Dockerfile"],
            MediaType::Makefile => vec!["makefile", "Makefile", "mk"],
            MediaType::BuildFile => vec!["gradle", "cmake", "meson", "ninja"],
            MediaType::Manifest => vec!["json", "yaml", "toml", "xml"],
            MediaType::Lockfile => vec!["lock", "json"],
            
            // Scripts
            MediaType::Script | MediaType::ShellScript | MediaType::Bash => {
                vec!["sh", "bash", "zsh", "fish"]
            }
            MediaType::PowerShell => vec!["ps1", "psm1", "psd1"],
            MediaType::PythonScript => vec!["py"],
            MediaType::JsScript => vec!["js", "mjs", "cjs"],
            MediaType::Automation => vec!["sh", "py", "js", "ps1"],
            
            // Game Assets
            MediaType::GameAsset => vec!["png", "json", "xml"],
            
            MediaType::Sprite | MediaType::Spritesheet | MediaType::SpriteAtlas => {
                vec!["png", "json", "atlas", "xml"]
            }
            
            MediaType::Tileset | MediaType::Tilemap | MediaType::TileAtlas => {
                vec!["png", "json", "tmx", "tsx", "xml"]
            }
            
            MediaType::UiKit | MediaType::UiElement | MediaType::Button |
            MediaType::Panel | MediaType::Window | MediaType::Hud => {
                vec!["png", "svg", "json", "sketch", "fig"]
            }
            
            MediaType::CharacterAsset | MediaType::Character | MediaType::Enemy |
            MediaType::Npc | MediaType::Player => {
                vec!["png", "json", "spine", "dragonbones"]
            }
            
            MediaType::Background | MediaType::Parallax | MediaType::Platform |
            MediaType::Prop | MediaType::Item | MediaType::Collectible |
            MediaType::PowerUp | MediaType::Weapon => {
                vec!["png", "svg", "json"]
            }
            
            MediaType::Effect | MediaType::Particle | MediaType::Explosion |
            MediaType::VFX => {
                vec!["png", "json", "plist", "particle"]
            }
            
            MediaType::LevelDesign | MediaType::MapData => {
                vec!["json", "tmx", "xml", "ldtk"]
            }
            
            MediaType::Dialogue | MediaType::QuestData => {
                vec!["json", "yarn", "ink", "xml"]
            }
            
            // Design
            MediaType::Design | MediaType::Mockup | MediaType::Wireframe |
            MediaType::Prototype | MediaType::UIDesign | MediaType::UXDesign |
            MediaType::WebDesign | MediaType::AppDesign => {
                vec!["fig", "sketch", "xd", "psd", "ai", "png", "pdf"]
            }
            
            MediaType::Logo | MediaType::Branding | MediaType::BrandKit |
            MediaType::StyleGuide | MediaType::DesignSystem => {
                vec!["svg", "ai", "eps", "pdf", "fig", "sketch"]
            }
            
            MediaType::ColorPalette | MediaType::ColorScheme | MediaType::Swatch => {
                vec!["ase", "aco", "gpl", "json", "css", "scss"]
            }
            
            MediaType::Gradient => vec!["css", "json", "svg"],
            MediaType::Pattern => vec!["png", "svg", "pat"],
            
            // Print
            MediaType::Print | MediaType::Flyer | MediaType::Brochure |
            MediaType::BusinessCard | MediaType::Letterhead | MediaType::Envelope |
            MediaType::Stationery | MediaType::PackageDesign | MediaType::LabelDesign |
            MediaType::MenuDesign | MediaType::SignDesign => {
                vec!["pdf", "ai", "psd", "indd", "svg"]
            }
            
            // Social Media
            MediaType::SocialMedia | MediaType::InstagramPost | MediaType::InstagramStory |
            MediaType::FacebookPost | MediaType::TwitterPost | MediaType::LinkedInPost |
            MediaType::YouTubeThumbnail => {
                vec!["png", "jpg", "psd", "fig"]
            }
            
            MediaType::TwitchOverlay => vec!["png", "psd", "html"],
            MediaType::DiscordEmoji => vec!["png", "gif"],
            
            // Archives
            MediaType::Zip | MediaType::Archive | MediaType::ArchivePackage |
            MediaType::Bundle | MediaType::Collection | MediaType::Pack |
            MediaType::Kit | MediaType::Suite => vec!["zip"],
            
            MediaType::Tarball | MediaType::Gzip => vec!["tar", "tar.gz", "tgz"],
            MediaType::SevenZip => vec!["7z"],
            MediaType::Rar => vec!["rar"],
            
            // Comics
            MediaType::Toon | MediaType::Cartoon => vec!["png", "jpg", "svg"],
            
            MediaType::Comic | MediaType::ComicStrip | MediaType::ComicBook |
            MediaType::Manga | MediaType::Manhwa | MediaType::Webtoon |
            MediaType::GraphicNovel => {
                vec!["cbz", "cbr", "pdf", "epub"]
            }
            
            MediaType::Panel | MediaType::Storyboard => vec!["png", "jpg", "psd"],
            
            // Misc
            MediaType::Emoji | MediaType::Sticker => vec!["png", "gif", "webp", "json"],
            MediaType::Cursor => vec!["cur", "ani", "png"],
            MediaType::Theme | MediaType::Skin => vec!["zip", "json", "xml", "css"],
            MediaType::Preset => vec!["json", "xml"],
            MediaType::Lut => vec!["cube", "3dl", "look"],
            MediaType::Filter => vec!["xmp", "lrtemplate", "dcp"],
            MediaType::Action => vec!["atn", "jsx"],
            MediaType::Brush => vec!["abr", "tpl", "gbr"],
            
            // ML
            MediaType::MLModel | MediaType::TrainedModel | MediaType::Weights |
            MediaType::Checkpoint => {
                vec!["pt", "pth", "onnx", "pb", "h5", "keras", "safetensors", "gguf", "bin"]
            }
            
            MediaType::Embedding => vec!["npy", "npz", "pt", "bin"],
            MediaType::Tokenizer | MediaType::Vocabulary => vec!["json", "txt", "model"],
            
            MediaType::Other => vec![],
        }
    }

    /// Get the parent category
    pub fn category(&self) -> MediaCategory {
        match self {
            // Visual
            MediaType::Image | MediaType::Photo | MediaType::Illustration |
            MediaType::Vector | MediaType::Wallpaper | MediaType::Screenshot |
            MediaType::Meme | MediaType::Infographic | MediaType::Artwork |
            MediaType::Painting | MediaType::Drawing | MediaType::Sketch |
            MediaType::Diagram | MediaType::Chart | MediaType::Graph |
            MediaType::Map | MediaType::Poster | MediaType::Banner |
            MediaType::Thumbnail | MediaType::Avatar | MediaType::Cover |
            MediaType::Video | MediaType::Animation | MediaType::Gif |
            MediaType::Footage | MediaType::StockFootage | MediaType::Tutorial |
            MediaType::Timelapse | MediaType::SlowMotion | MediaType::Drone |
            MediaType::MotionGraphics | MediaType::Intro | MediaType::Outro |
            MediaType::Transition | MediaType::Overlay | 
            MediaType::LowerThird => MediaCategory::Visual,
            
            // Audio
            MediaType::Audio | MediaType::Music | MediaType::Song |
            MediaType::Track | MediaType::SoundEffect | MediaType::Sfx |
            MediaType::Podcast | MediaType::Audiobook | MediaType::Voice |
            MediaType::VoiceOver | MediaType::Narration | MediaType::Ambient |
            MediaType::Loop | MediaType::Jingle | 
            MediaType::Ringtone => MediaCategory::Audio,
            
            // Text
            MediaType::Text | MediaType::Article | MediaType::BlogPost |
            MediaType::NewsArticle | MediaType::Essay | MediaType::Book |
            MediaType::Ebook | MediaType::Novel | MediaType::ShortStory |
            MediaType::Story | MediaType::Poem | MediaType::Poetry |
            MediaType::Quote | MediaType::Lyrics | MediaType::Script |
            MediaType::Screenplay | MediaType::Subtitle | MediaType::Caption |
            MediaType::Documentation | MediaType::Readme | MediaType::Wiki |
            MediaType::Manual | MediaType::Guide | MediaType::TutorialText |
            MediaType::HowTo | MediaType::Faq | MediaType::LicenseText |
            MediaType::Changelog | MediaType::ReleaseNotes => MediaCategory::Text,
            
            // Documents
            MediaType::Document | MediaType::Pdf | MediaType::Word |
            MediaType::Docx | MediaType::Doc | MediaType::Odt |
            MediaType::Rtf | MediaType::Report | MediaType::Whitepaper |
            MediaType::Research | MediaType::Paper | MediaType::Thesis |
            MediaType::Resume | MediaType::Cv | MediaType::CoverLetter |
            MediaType::Contract | MediaType::Invoice | MediaType::Receipt |
            MediaType::Form | MediaType::Certificate | MediaType::Diploma |
            MediaType::Letter | MediaType::Memo | MediaType::Minutes |
            MediaType::Proposal | MediaType::BusinessPlan |
            MediaType::Spreadsheet | MediaType::Excel | MediaType::Xlsx |
            MediaType::Xls | MediaType::Csv | MediaType::Tsv |
            MediaType::Ods | MediaType::Table | MediaType::DataTable |
            MediaType::PivotTable | MediaType::Budget | MediaType::FinancialStatement |
            MediaType::Inventory | MediaType::Schedule | MediaType::Calendar |
            MediaType::Gantt | MediaType::Presentation | MediaType::Powerpoint |
            MediaType::Pptx | MediaType::Ppt | MediaType::Odp |
            MediaType::Keynote | MediaType::Slides | MediaType::Deck |
            MediaType::Pitch | MediaType::PitchDeck => MediaCategory::Documents,
            
            // Data
            MediaType::Data | MediaType::Dataset | MediaType::Json |
            MediaType::JsonLines | MediaType::Ndjson | MediaType::GeoJson |
            MediaType::Xml | MediaType::Yaml | MediaType::Toml |
            MediaType::Ini | MediaType::Env | MediaType::Parquet |
            MediaType::Avro | MediaType::Orc | MediaType::Sqlite |
            MediaType::Database | MediaType::Sql | MediaType::GraphQl |
            MediaType::Api | MediaType::ApiResponse | MediaType::MockData |
            MediaType::SampleData | MediaType::TestData |
            MediaType::TrainingData => MediaCategory::Data,
            
            // 3D
            MediaType::Model3D | MediaType::Mesh | MediaType::Geometry |
            MediaType::Texture | MediaType::Material | MediaType::Shader |
            MediaType::Hdri | MediaType::Skybox | MediaType::Environment |
            MediaType::Cubemap | MediaType::NormalMap | MediaType::HeightMap |
            MediaType::DisplacementMap | MediaType::AoMap | MediaType::RoughnessMap |
            MediaType::MetallicMap | MediaType::Rig | MediaType::Animation3D |
            MediaType::MotionCapture | MediaType::Mocap => MediaCategory::ThreeD,
            
            // Code
            MediaType::Code | MediaType::SourceCode | MediaType::Snippet |
            MediaType::Gist | MediaType::CodeBlock | MediaType::Function |
            MediaType::Class | MediaType::Module | MediaType::Library |
            MediaType::Package | MediaType::Crate | MediaType::Template |
            MediaType::Boilerplate | MediaType::Starter | MediaType::Scaffold |
            MediaType::Example | MediaType::Demo | MediaType::Sample |
            MediaType::Algorithm | MediaType::DataStructure | MediaType::DesignPattern |
            MediaType::Config | MediaType::Configuration | MediaType::Settings |
            MediaType::Preferences | MediaType::Dotfile | MediaType::EnvFile |
            MediaType::DockerFile | MediaType::Makefile | MediaType::BuildFile |
            MediaType::Manifest | MediaType::Lockfile | MediaType::Script |
            MediaType::ShellScript | MediaType::Bash | MediaType::PowerShell |
            MediaType::PythonScript | MediaType::JsScript |
            MediaType::Automation => MediaCategory::Code,
            
            // Game Assets
            MediaType::GameAsset | MediaType::Sprite | MediaType::Spritesheet |
            MediaType::SpriteAtlas | MediaType::Tileset | MediaType::Tilemap |
            MediaType::TileAtlas | MediaType::UiKit | MediaType::UiElement |
            MediaType::Button | MediaType::Panel | MediaType::Window |
            MediaType::Hud | MediaType::CharacterAsset | MediaType::Character |
            MediaType::Enemy | MediaType::Npc | MediaType::Player |
            MediaType::Background | MediaType::Parallax | MediaType::Platform |
            MediaType::Prop | MediaType::Item | MediaType::Collectible |
            MediaType::PowerUp | MediaType::Weapon | MediaType::Effect |
            MediaType::Particle | MediaType::Explosion | MediaType::VFX |
            MediaType::LevelDesign | MediaType::MapData | MediaType::Dialogue |
            MediaType::QuestData => MediaCategory::GameAssets,
            
            // Design
            MediaType::Design | MediaType::Mockup | MediaType::Wireframe |
            MediaType::Prototype | MediaType::UIDesign | MediaType::UXDesign |
            MediaType::WebDesign | MediaType::AppDesign | MediaType::Logo |
            MediaType::Branding | MediaType::BrandKit | MediaType::StyleGuide |
            MediaType::DesignSystem | MediaType::ColorPalette | MediaType::ColorScheme |
            MediaType::Gradient | MediaType::Pattern | MediaType::Swatch |
            MediaType::Print | MediaType::Flyer | MediaType::Brochure |
            MediaType::BusinessCard | MediaType::Letterhead | MediaType::Envelope |
            MediaType::Stationery | MediaType::PackageDesign | MediaType::LabelDesign |
            MediaType::MenuDesign | MediaType::SignDesign | MediaType::SocialMedia |
            MediaType::InstagramPost | MediaType::InstagramStory | MediaType::FacebookPost |
            MediaType::TwitterPost | MediaType::LinkedInPost | MediaType::YouTubeThumbnail |
            MediaType::TwitchOverlay | MediaType::DiscordEmoji => MediaCategory::Design,
            
            // Archives
            MediaType::Archive | MediaType::Zip | MediaType::Tarball |
            MediaType::Gzip | MediaType::SevenZip | MediaType::Rar |
            MediaType::ArchivePackage | MediaType::Bundle | MediaType::Collection |
            MediaType::Pack | MediaType::Kit | MediaType::Suite => MediaCategory::Archives,
            
            // ML
            MediaType::MLModel | MediaType::TrainedModel | MediaType::Weights |
            MediaType::Checkpoint | MediaType::Embedding | MediaType::Tokenizer |
            MediaType::Vocabulary => MediaCategory::MachineLearning,
            
            // Other
            MediaType::Toon | MediaType::Cartoon | MediaType::Comic |
            MediaType::ComicStrip | MediaType::ComicBook | MediaType::Manga |
            MediaType::Manhwa | MediaType::Webtoon | MediaType::GraphicNovel |
            MediaType::Panel | MediaType::Storyboard | MediaType::Emoji |
            MediaType::Sticker | MediaType::Cursor | MediaType::Theme |
            MediaType::Skin | MediaType::Preset | MediaType::Lut |
            MediaType::Filter | MediaType::Action | MediaType::Brush |
            MediaType::Other => MediaCategory::Other,
        }
    }

    /// Get all variants
    pub fn all() -> impl Iterator<Item = MediaType> {
        MediaType::iter()
    }
}

// ... (Display and FromStr implementations remain similar but updated for new types)

/// Categories for grouping media types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
pub enum MediaCategory {
    Visual,
    Audio,
    Text,
    Documents,
    Data,
    ThreeD,
    Code,
    GameAssets,
    Design,
    Archives,
    MachineLearning,
    Other,
}

impl fmt::Display for MediaCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            MediaCategory::Visual => "🖼️  Visual (Images & Videos)",
            MediaCategory::Audio => "🎵 Audio (Music & Sounds)",
            MediaCategory::Text => "📄 Text (Articles & Books)",
            MediaCategory::Documents => "📁 Documents (Office Files)",
            MediaCategory::Data => "📊 Data (Datasets & APIs)",
            MediaCategory::ThreeD => "🎨 3D (Models & Textures)",
            MediaCategory::Code => "💻 Code (Snippets & Templates)",
            MediaCategory::GameAssets => "🎮 Game Assets",
            MediaCategory::Design => "✏️  Design (UI/UX & Branding)",
            MediaCategory::Archives => "📦 Archives (Bundles & Packs)",
            MediaCategory::MachineLearning => "🤖 Machine Learning",
            MediaCategory::Other => "📎 Other",
        };
        write!(f, "{}", s)
    }
}
```

---

## Summary

I've created:

1. **Professional README.md** with:
   - Full feature documentation
   - Installation instructions (cargo add commands)
   - Quick start guide
   - Complete provider list with API info
   - Command reference
   - Architecture overview
   - Contribution guidelines

2. **Updated MediaType enum** that:
   - **Excludes fonts and icons** (handled by dx-fonts and dx-icons)
   - Includes 150+ media types across all categories
   - Covers everything from images to ML models
   - Supports all document types (PDF, Word, Excel, PowerPoint)
   - Includes data formats (JSON, CSV, XML, Parquet, etc.)
   - Has game assets, 3D models, code snippets, and more

Would you like me to continue with:
1. **Provider implementations** (Unsplash, Pexels, Wikipedia, etc.)
2. **CLI commands** (search, download, convert)
3. **Converter implementations** (image, video, audio, document)
4. **Core infrastructure** (downloader, error handling, caching)