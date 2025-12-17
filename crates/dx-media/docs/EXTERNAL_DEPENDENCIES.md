# External Dependencies for DX-Media

This document describes the external tools that some dx-media features depend on and how to bundle them for distribution.

## Overview of Dependencies

### Native Rust (No External Dependencies) - 33 Tools

These tools work out of the box without any external binaries:

| Category | Tools | Count |
|----------|-------|-------|
| **Image** | resize, crop, rotate, convert, compress, grayscale, thumbnail, watermark, QR code, palette | 10 |
| **Archive** | zip, tar, gzip, list, extract, encrypted zip, split, tar.gz | 8 |
| **Utility** | hash, base64, URL encode/decode, JSON format, CSV convert, YAML convert, diff, UUID | 10 |
| **Document** | markdown_to_html, text extract, word count | 3 |
| **Audio** | metadata (lofty) | 1 |
| **Video** | - | 0 |

### External Dependencies Required - 27 Tools

| Tool Category | External Dependency | Purpose |
|---------------|---------------------|---------|
| Video (all 10) | FFmpeg | Video processing, transcoding, thumbnails |
| Audio (9 of 10) | FFmpeg | Audio conversion, effects, waveforms |
| Document | wkhtmltopdf | HTML to PDF conversion |
| Document | Pandoc | Universal document conversion |
| Image | Tesseract | OCR (text extraction from images) |
| Image | ExifTool | EXIF metadata reading/writing |
| Archive | 7z | 7-zip archive support |

## Bundling Options

### Option 1: Statically Linked FFmpeg (Recommended)

Using `ffmpeg-sys-next` with static linking:

```toml
# Cargo.toml
[dependencies]
ffmpeg-sys-next = { version = "7", features = ["static"] }
```

This compiles FFmpeg directly into the binary. **Pros:** Single binary, no external deps. **Cons:** Larger binary (~50MB), longer compile times.

### Option 2: Runtime FFmpeg Detection

The current approach - detect FFmpeg at runtime:

```rust
// src/tools/video/mod.rs
fn check_ffmpeg() -> Result<()> {
    Command::new("ffmpeg").arg("-version").output()
        .map_err(|_| DxError::config("FFmpeg not found. Install from https://ffmpeg.org"))?;
    Ok(())
}
```

### Option 3: Bundled Binaries (Platform-Specific)

Bundle pre-compiled binaries with releases:

```
dx-media-windows-x64.zip
├── dx.exe
├── ffmpeg.exe           # ~100MB
├── ffprobe.exe          # ~50MB
└── README.md

dx-media-linux-x64.tar.gz
├── dx
├── ffmpeg               # ~80MB  
├── ffprobe              # ~40MB
└── README.md

dx-media-macos-arm64.tar.gz
├── dx
├── ffmpeg               # ~70MB
├── ffprobe              # ~35MB
└── README.md
```

### Option 4: Pure Rust Alternatives (Future)

Replace external tools with pure Rust implementations as they mature:

| External Tool | Pure Rust Alternative | Status |
|---------------|----------------------|--------|
| FFmpeg | `av1-grain`, `rav1e`, `gstreamer-rs` | Partial |
| Tesseract | `ocrs` | Experimental |
| wkhtmltopdf | `headless_chrome` | Works |
| Pandoc | `pulldown-cmark` | Markdown only |
| ExifTool | `kamadak-exif` | Read only |
| 7z | `sevenz-rust` | Works |

## Implementation Plan

### Phase 1: Graceful Degradation (Current)

- Tools check for dependencies at runtime
- Return helpful error messages with install instructions
- Native tools work without any setup

### Phase 2: Optional Static Linking

Add cargo features for static linking:

```toml
[features]
default = []
static-ffmpeg = ["ffmpeg-sys-next/static"]
static-all = ["static-ffmpeg"]
```

### Phase 3: Binary Distribution

Create GitHub Actions workflow to:
1. Build for each platform
2. Download/bundle pre-compiled FFmpeg
3. Create distributable archives
4. Upload to releases

## Installation Instructions for Users

### Windows
```powershell
# Option 1: winget (recommended)
winget install FFmpeg

# Option 2: chocolatey
choco install ffmpeg

# Option 3: Manual download
# Download from https://www.gyan.dev/ffmpeg/builds/
```

### macOS
```bash
# Homebrew (recommended)
brew install ffmpeg

# Additional tools
brew install tesseract wkhtmltopdf pandoc p7zip
```

### Linux
```bash
# Debian/Ubuntu
sudo apt install ffmpeg tesseract-ocr wkhtmltopdf pandoc p7zip-full

# Fedora
sudo dnf install ffmpeg tesseract wkhtmltopdf pandoc p7zip

# Arch
sudo pacman -S ffmpeg tesseract wkhtmltopdf pandoc p7zip
```

## Checking Dependencies

Run the dependency checker:

```bash
# Check all external dependencies
./dx check-deps

# Example output:
# ✓ ffmpeg 6.1 - Video/Audio processing
# ✓ ffprobe 6.1 - Media info
# ✗ tesseract - OCR (optional)
# ✗ wkhtmltopdf - HTML to PDF (optional)
# ✓ 7z 23.01 - 7-zip archives
```

## Tool Availability Matrix

| Tool | Without FFmpeg | With FFmpeg |
|------|---------------|-------------|
| video::transcode | ✗ | ✓ |
| video::extract_audio | ✗ | ✓ |
| video::trim | ✗ | ✓ |
| video::to_gif | ✗ | ✓ |
| video::thumbnail | ✗ | ✓ |
| video::scale | ✗ | ✓ |
| video::concatenate | ✗ | ✓ |
| video::mute | ✗ | ✓ |
| video::watermark | ✗ | ✓ |
| video::subtitle | ✗ | ✓ |
| audio::convert | ✗ | ✓ |
| audio::trim | ✗ | ✓ |
| audio::merge | ✗ | ✓ |
| audio::normalize | ✗ | ✓ |
| audio::volume | ✗ | ✓ |
| audio::effects | ✗ | ✓ |
| audio::spectrum | ✗ | ✓ |
| audio::split | ✗ | ✓ |
| audio::transcribe | ✗ | ✓ |
| audio::metadata | ✓ | ✓ |

## Summary

- **33 tools** work natively in pure Rust
- **27 tools** require external dependencies (primarily FFmpeg)
- FFmpeg covers 19 tools (all video + 9 audio)
- Other tools are optional enhancements
