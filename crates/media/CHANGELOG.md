# Changelog

All notable changes to DX Media will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-11-30

### Added

- **Core Library (`dx_media`)**
  - `DxMedia` facade for easy library usage with fluent search builder API
  - `SearchEngine` for multi-provider parallel searching
  - `Downloader` with async file downloads and retry logic
  - `FileManager` for organized file storage by provider/type
  - `HttpClient` with built-in rate limiting and exponential backoff

- **Provider Support**
  - Unsplash provider (images) - requires API key
  - Pexels provider (images, videos) - requires API key
  - Pixabay provider (images, videos, vectors) - requires API key
  - `ProviderRegistry` for dynamic provider management
  - `Provider` trait for implementing custom providers

- **CLI (`dx`)**
  - `dx search <query>` - Search across all configured providers
    - `--type` filter (image, video, audio, gif, vector)
    - `--provider` filter for specific providers
    - `--count` and `--page` for pagination
    - `--orientation` filter (landscape, portrait, square)
    - `--color` filter for dominant color
    - `--download` flag to auto-download first result
  - `dx download <provider:id>` - Download specific asset
  - `dx scrape <url>` - Scrape and download media from any website
    - `--type` filter (image, video, audio, gif, vector, all)
    - `--count` limit for number of assets
    - `--depth` for link-following depth
    - `--pattern` for file pattern matching
    - `--dry-run` to preview without downloading
  - `dx providers` - List available providers and their status
  - `dx config` - Show current configuration
  - Multiple output formats: text, json, json-compact, tsv

- **Configuration**
  - Environment variable configuration
  - `.env` file support via dotenvy
  - Configurable download directory, timeouts, retry attempts
  - Per-provider API key configuration

- **Types**
  - `MediaType` enum (Image, Video, Audio, Gif, Vector, Document, Data, Model3D, Code, Text)
  - `MediaAsset` with comprehensive metadata
  - `SearchQuery` with filters and pagination
  - `SearchResult` with aggregated results from multiple providers
  - `License` types (CC0, CC-BY, Unsplash, Pexels, Pixabay, etc.)

### Technical Details

- Built with Rust 2024 Edition
- Async runtime: Tokio with full features
- HTTP client: reqwest with rustls-tls, gzip, brotli compression
- CLI framework: clap with derive macros
- Serialization: serde + serde_json
- Error handling: thiserror + anyhow
- Logging: tracing with env-filter

## [Unreleased]

### Planned

- Video provider implementations (Pexels Videos, Coverr, Mixkit)
- Audio provider implementations (Freesound, Jamendo)
- Caching layer for search results
- Batch download with concurrency control
- Interactive TUI mode (ratatui)
- Format conversion (image/video/audio)

---

[0.1.0]: https://github.com/anthropics/dx-media/releases/tag/v0.1.0
[Unreleased]: https://github.com/anthropics/dx-media/compare/v0.1.0...HEAD
