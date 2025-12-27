I already have media at crates/media folder but I want to add more features into it!
And also try to do it as efficiently as possible without wasting time and resources!!! As because this is a very big task so we will take me turn in agents to do it properly and efficiently without wasting time and resources!!! And don't ask small questions because this is a very big task so we will take me turn in agents to do it properly and efficiently without wasting time and resources!!!

Here is the plan to add more features into media folder:
```markdown
DX Media Conversion Feature - Implementation Plan
📋 Executive Summary
This document outlines a comprehensive plan to implement media conversion features in the dx-media crate, following the Binary Dawn architecture principles. The goal is to provide 60 practical media tools with minimal dependencies while maintaining excellent functionality.

🎯 Architecture Overview
text

dx/
├── crates/
│   └── dx-media/
│       ├── Cargo.toml
│       ├── src/
│       │   ├── lib.rs                 # Public API re-exports
│       │   ├── core/
│       │   │   ├── mod.rs
│       │   │   ├── pipeline.rs        # Zero-copy processing pipeline
│       │   │   ├── buffer.rs          # Memory-mapped buffers
│       │   │   └── progress.rs        # Progress tracking
│       │   │
│       │   ├── image/
│       │   │   ├── mod.rs
│       │   │   ├── convert.rs         # Format conversion
│       │   │   ├── resize.rs          # Smart resizing
│       │   │   ├── compress.rs        # Compression
│       │   │   ├── watermark.rs       # Watermarking
│       │   │   ├── metadata.rs        # EXIF handling
│       │   │   ├── qr.rs              # QR code gen/read
│       │   │   ├── palette.rs         # Color extraction
│       │   │   ├── filters.rs         # Image filters
│       │   │   ├── ocr.rs             # Text extraction
│       │   │   └── icon.rs            # Icon generation
│       │   │
│       │   ├── video/
│       │   │   ├── mod.rs
│       │   │   ├── transcode.rs       # Format transcoding
│       │   │   ├── extract.rs         # Audio extraction
│       │   │   ├── trim.rs            # Video trimming
│       │   │   ├── gif.rs             # GIF creation
│       │   │   ├── thumbnail.rs       # Thumbnail generation
│       │   │   ├── scale.rs           # Resolution scaling
│       │   │   ├── concat.rs          # Video concatenation
│       │   │   ├── mute.rs            # Audio removal
│       │   │   ├── info.rs            # Metadata inspection
│       │   │   └── subtitle.rs        # Subtitle handling
│       │   │
│       │   ├── audio/
│       │   │   ├── mod.rs
│       │   │   ├── convert.rs         # Format conversion
│       │   │   ├── tags.rs            # ID3 tag editing
│       │   │   ├── normalize.rs       # Volume normalization
│       │   │   ├── silence.rs         # Silence detection/removal
│       │   │   ├── waveform.rs        # Waveform visualization
│       │   │   ├── merge.rs           # Audio merging
│       │   │   ├── speed.rs           # Speed/pitch change
│       │   │   ├── channels.rs        # Mono/stereo conversion
│       │   │   ├── duration.rs        # Duration calculation
│       │   │   └── spectrum.rs        # Frequency analysis
│       │   │
│       │   ├── document/
│       │   │   ├── mod.rs
│       │   │   ├── pdf_merge.rs       # PDF merging
│       │   │   ├── pdf_split.rs       # PDF splitting
│       │   │   ├── markdown.rs        # MD to HTML/PDF
│       │   │   ├── csv_json.rs        # CSV <-> JSON
│       │   │   ├── pdf_text.rs        # PDF text extraction
│       │   │   ├── img_to_pdf.rs      # Images to PDF
│       │   │   ├── word_count.rs      # Word/char counting
│       │   │   ├── minify.rs          # JSON/HTML minification
│       │   │   ├── highlight.rs       # Syntax highlighting
│       │   │   └── diff.rs            # Text diff
│       │   │
│       │   ├── archive/
│       │   │   ├── mod.rs
│       │   │   ├── extract.rs         # Universal extraction
│       │   │   ├── compress.rs        # Batch compression
│       │   │   ├── encrypt.rs         # Encrypted archives
│       │   │   ├── tarball.rs         # Tar.gz handling
│       │   │   ├── list.rs            # Archive listing
│       │   │   ├── verify.rs          # Integrity check
│       │   │   ├── partial.rs         # Partial extraction
│       │   │   ├── convert.rs         # Archive conversion
│       │   │   ├── dedup.rs           # Deduplication
│       │   │   └── flatten.rs         # Structure flattening
│       │   │
│       │   └── utility/
│       │       ├── mod.rs
│       │       ├── rename.rs          # Batch renaming
│       │       ├── duplicate.rs       # Duplicate finder
│       │       ├── base64.rs          # Base64 encode/decode
│       │       ├── watch.rs           # File watching
│       │       ├── checksum.rs        # Hash verification
│       │       ├── size.rs            # Large file finder
│       │       ├── clipboard.rs       # Clipboard operations
│       │       ├── hex.rs             # Hex viewer
│       │       ├── shred.rs           # Secure delete
│       │       └── mime.rs            # MIME detection
│       │
│       ├── binary/
│       │   ├── mod.rs
│       │   ├── format.rs              # Binary media format (.dxm)
│       │   └── cache.rs               # Binary conversion cache
│       │
│       └── cli/
│           ├── mod.rs
│           └── commands.rs            # CLI command definitions
│
├── docs/
│   └── DX_MEDIA_CONVERSION.md
│
└── tests/
    └── dx-media/
        └── ...
📦 Crate Selection Strategy
Philosophy: Minimal Dependencies, Maximum Performance
Following the Binary Dawn principles:

Zero-copy operations where possible
SIMD optimization for batch processing
Pure Rust preferred over FFI bindings
Lazy loading of heavy dependencies via feature flags
Binary caching for repeated conversions
🔧 Dependency Matrix
Core Dependencies (Always Included)
toml

[dependencies]
# Core utilities
thiserror = "2.0"           # Error handling
tracing = "0.1"             # Logging/diagnostics
rayon = "1.10"              # Parallel processing
memmap2 = "0.9"             # Memory-mapped I/O
blake3 = "1.5"              # Fast hashing for cache
walkdir = "2.5"             # Directory traversal
Feature-Gated Dependencies
toml

[features]
default = ["image-core", "archive-core", "utility-core"]

# Image Processing
image-core = ["dep:image", "dep:imageproc"]
image-advanced = ["image-core", "dep:oxipng", "dep:img-parts"]
image-qr = ["dep:qrcode", "dep:rxing"]
image-ocr = ["dep:ocrs"]

# Video Processing  
video = ["dep:ffmpeg-next"]
video-gif = ["image-core", "dep:gif"]

# Audio Processing
audio-core = ["dep:symphonia", "dep:hound"]
audio-advanced = ["audio-core", "dep:rubato", "dep:dasp"]
audio-tags = ["dep:lofty"]

# Document Processing
document-core = ["dep:lopdf", "dep:pulldown-cmark"]
document-advanced = ["document-core", "dep:printpdf", "dep:pdf-extract"]
document-data = ["dep:csv", "dep:serde", "dep:serde_json"]
document-syntax = ["dep:syntect"]

# Archive Processing
archive-core = ["dep:zip", "dep:tar", "dep:flate2"]
archive-advanced = ["archive-core", "dep:compress-tools"]

# Utility
utility-core = ["dep:regex", "dep:sha2", "dep:notify"]
utility-advanced = ["utility-core", "dep:arboard", "dep:infer"]

# Full suite
full = [
    "image-advanced", "image-qr", "image-ocr",
    "video", "video-gif",
    "audio-advanced", "audio-tags",
    "document-advanced", "document-data", "document-syntax",
    "archive-advanced",
    "utility-advanced"
]
📚 Detailed Crate Mapping
1. Image Tools
Feature	Crate(s)	Version	Lines Est.	Notes
Format Converter	image	0.25	~150	PNG, JPEG, WEBP, GIF, ICO, AVIF
Smart Resizer	image	0.25	~100	imageops::resize with aspect ratio
Image Compressor	image, oxipng	0.25, 9.1	~200	Quality control, PNG optimization
Watermarker	imageproc, rusttype	0.25, 0.9	~250	Text/image overlay
EXIF Wiper	img-parts	0.3	~100	Zero-copy metadata removal
QR Generator/Reader	qrcode, rxing	0.14, 0.7	~200	Pure Rust implementations
Color Palette	image + custom	0.25	~150	K-means clustering
Filters	imageproc	0.25	~100	Blur, grayscale, contrast
OCR	ocrs	0.9	~150	Pure Rust OCR (experimental)
Icon Generator	image, ico	0.25, 0.3	~120	Multi-size favicon generation
2. Video Tools
Feature	Crate(s)	Version	Lines Est.	Notes
Format Transcoder	ffmpeg-next	7.1	~300	FFmpeg bindings (best coverage)
Audio Extractor	ffmpeg-next	7.1	~150	Extract to MP3/WAV
Video Trimmer	ffmpeg-next	7.1	~200	Stream copy mode
GIF Maker	gif, image	0.13, 0.25	~250	Frame extraction + encoding
Thumbnail Generator	ffmpeg-next	7.1	~100	Keyframe extraction
Resolution Scaler	ffmpeg-next	7.1	~150	Hardware acceleration
Video Concatenator	ffmpeg-next	7.1	~200	Demux/mux pipeline
Mute Video	ffmpeg-next	7.1	~100	Stream removal
Metadata Inspector	ffmpeg-next	7.1	~150	Probe all streams
Subtitle Burner	ffmpeg-next	7.1	~200	Hardcode .srt/.ass
3. Audio Tools
Feature	Crate(s)	Version	Lines Est.	Notes
Audio Converter	symphonia, hound	0.5, 3.5	~250	Decode any, encode WAV/PCM
Tag Editor	lofty	0.21	~200	ID3v2, Vorbis, APE, MP4
Volume Normalizer	dasp	0.11	~150	Peak/RMS normalization
Silence Remover	symphonia	0.5	~200	Threshold detection
Waveform Visualizer	image + custom	0.25	~300	PNG waveform generation
Audio Merger	dasp	0.11	~150	Sample interleaving
Speed/Pitch Changer	rubato	0.16	~200	High-quality resampling
Channel Converter	dasp	0.11	~100	Mono ↔ Stereo
Duration Calculator	lofty	0.21	~80	Batch duration sum
Spectrum Analyzer	spectrum-analyzer	1.6	~200	FFT analysis
4. Document Tools
Feature	Crate(s)	Version	Lines Est.	Notes
PDF Merger	lopdf	0.34	~200	Page manipulation
PDF Splitter	lopdf	0.34	~150	Range extraction
Markdown Converter	pulldown-cmark	0.12	~200	MD → HTML
CSV ↔ JSON	csv, serde_json	1.3, 1.0	~150	Bidirectional
PDF Text Extractor	pdf-extract	0.7	~100	Unicode extraction
Images to PDF	printpdf	0.7	~200	Page assembly
Word Counter	std + unicode-segmentation	1.12	~100	Grapheme clusters
Minifier	minify-html	0.16	~100	HTML/JSON/CSS
Syntax Highlighter	syntect	5.2	~200	HTML/image output
Diff Viewer	similar	2.6	~150	Unified diff
5. Archive Tools
Feature	Crate(s)	Version	Lines Est.	Notes
Universal Extractor	compress-tools	0.15	~200	Auto-detect format
Batch Compressor	zip, walkdir	2.2, 2.5	~150	Parallel compression
Encrypted Zip	zip	2.2	~150	AES-256 support
Tarball Creator	tar, flate2	0.4, 1.0	~120	.tar.gz/.tar.xz
Archive Lister	zip, tar	2.2, 0.4	~100	Streaming list
Integrity Checker	crc32fast, sha2	1.4, 0.10	~150	CRC/SHA verification
Partial Extractor	zip, tar	2.2, 0.4	~150	Single file extraction
Archive Converter	zip, tar, flate2	-	~200	Format transformation
Deduplicator	sha2, blake3	0.10, 1.5	~200	Content hashing
Flatten Extractor	zip, tar	-	~120	Remove nested structure
6. Utility Tools
Feature	Crate(s)	Version	Lines Est.	Notes
Batch Renamer	regex	1.11	~200	Pattern replacement
Duplicate Finder	sha2, walkdir	0.10, 2.5	~250	Content-based dedup
Base64 Codec	base64	0.22	~80	SIMD-accelerated
File Watcher	notify	7.0	~200	Hot folder processing
Checksum Validator	md-5, sha2, blake3	-	~150	Multi-algorithm
Large File Finder	walkdir	2.5	~120	Size-sorted listing
Clipboard Manager	arboard	3.4	~150	Cross-platform
Hex Viewer	pretty-hex	0.4	~100	Binary inspection
Secure Shredder	std::fs	-	~100	Multi-pass overwrite
MIME Detector	infer	0.16	~80	Magic bytes
🚀 Implementation Plan
Phase 1: Core Infrastructure (Week 1)
Day 1-2: Project Setup & Core Pipeline
Rust

// crates/dx-media/src/core/pipeline.rs

use std::path::Path;
use memmap2::Mmap;
use thiserror::Error;

/// Zero-copy processing pipeline following Binary Dawn principles
pub struct MediaPipeline {
    /// Memory-mapped input buffer
    input: Option<Mmap>,
    /// Conversion cache using Blake3 hashes
    cache: ConversionCache,
    /// Progress callback
    progress: Option<Box<dyn Fn(f32) + Send + Sync>>,
}

#[derive(Error, Debug)]
pub enum MediaError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Format not supported: {0}")]
    UnsupportedFormat(String),
    #[error("Conversion failed: {0}")]
    ConversionFailed(String),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

pub type MediaResult<T> = Result<T, MediaError>;

impl MediaPipeline {
    /// Create a new pipeline with memory-mapped input
    pub fn new(path: impl AsRef<Path>) -> MediaResult<Self> {
        let file = std::fs::File::open(path)?;
        let mmap = unsafe { Mmap::map(&file)? };
        
        Ok(Self {
            input: Some(mmap),
            cache: ConversionCache::new(),
            progress: None,
        })
    }
    
    /// Set progress callback
    pub fn with_progress<F>(mut self, callback: F) -> Self 
    where
        F: Fn(f32) + Send + Sync + 'static,
    {
        self.progress = Some(Box::new(callback));
        self
    }
    
    /// Get input bytes (zero-copy)
    pub fn input_bytes(&self) -> &[u8] {
        self.input.as_ref().map(|m| &m[..]).unwrap_or(&[])
    }
}
Day 3-4: Binary Cache System
Rust

// crates/dx-media/src/binary/cache.rs

use blake3::Hasher;
use std::path::PathBuf;
use std::collections::HashMap;
use std::sync::RwLock;

/// Binary conversion cache for dx-media
/// Follows Binary Dawn philosophy: never repeat work
pub struct ConversionCache {
    /// In-memory cache index
    index: RwLock<HashMap<[u8; 32], CacheEntry>>,
    /// Cache directory
    cache_dir: PathBuf,
}

#[derive(Clone)]
pub struct CacheEntry {
    /// Output file path
    pub path: PathBuf,
    /// Conversion parameters hash
    pub params_hash: [u8; 32],
    /// Creation timestamp
    pub created_at: u64,
}

impl ConversionCache {
    pub fn new() -> Self {
        let cache_dir = dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("dx-media");
        
        std::fs::create_dir_all(&cache_dir).ok();
        
        Self {
            index: RwLock::new(HashMap::new()),
            cache_dir,
        }
    }
    
    /// Generate cache key from input + parameters
    pub fn cache_key(input: &[u8], params: &impl AsRef<[u8]>) -> [u8; 32] {
        let mut hasher = Hasher::new();
        hasher.update(input);
        hasher.update(params.as_ref());
        *hasher.finalize().as_bytes()
    }
    
    /// Check if conversion result is cached
    pub fn get(&self, key: &[u8; 32]) -> Option<CacheEntry> {
        self.index.read().ok()?.get(key).cloned()
    }
    
    /// Store conversion result
    pub fn put(&self, key: [u8; 32], entry: CacheEntry) {
        if let Ok(mut index) = self.index.write() {
            index.insert(key, entry);
        }
    }
}
Day 5: CLI Integration
Rust

// crates/dx-media/src/cli/commands.rs

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "dx-media")]
#[command(about = "Binary-first media processing for DX")]
pub struct MediaCli {
    #[command(subcommand)]
    pub command: MediaCommand,
    
    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,
    
    /// Use cached results if available
    #[arg(long, global = true, default_value = "true")]
    pub cache: bool,
}

#[derive(Subcommand)]
pub enum MediaCommand {
    /// Image processing tools
    Image {
        #[command(subcommand)]
        action: ImageAction,
    },
    /// Video processing tools
    Video {
        #[command(subcommand)]
        action: VideoAction,
    },
    /// Audio processing tools
    Audio {
        #[command(subcommand)]
        action: AudioAction,
    },
    /// Document processing tools
    Doc {
        #[command(subcommand)]
        action: DocAction,
    },
    /// Archive processing tools
    Archive {
        #[command(subcommand)]
        action: ArchiveAction,
    },
    /// Utility tools
    Util {
        #[command(subcommand)]
        action: UtilAction,
    },
}

#[derive(Subcommand)]
pub enum ImageAction {
    /// Convert image format
    Convert {
        /// Input file
        input: PathBuf,
        /// Output file
        output: PathBuf,
        /// Output format (auto-detected from extension if not specified)
        #[arg(short, long)]
        format: Option<String>,
        /// Quality (1-100, for lossy formats)
        #[arg(short, long, default_value = "85")]
        quality: u8,
    },
    /// Resize image
    Resize {
        input: PathBuf,
        output: PathBuf,
        /// Target width
        #[arg(short, long)]
        width: Option<u32>,
        /// Target height  
        #[arg(short, long)]
        height: Option<u32>,
        /// Keep aspect ratio
        #[arg(long, default_value = "true")]
        keep_aspect: bool,
    },
    /// Compress image
    Compress {
        input: PathBuf,
        output: PathBuf,
        /// Target quality (1-100)
        #[arg(short, long, default_value = "75")]
        quality: u8,
    },
    // ... more actions
}
Phase 2: Image Processing (Week 2)
Day 6-7: Core Image Operations
Rust

// crates/dx-media/src/image/convert.rs

use image::{DynamicImage, ImageFormat, ImageReader};
use std::path::Path;
use crate::core::{MediaResult, MediaError};

/// Supported image formats
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImageOutputFormat {
    Png,
    Jpeg { quality: u8 },
    WebP { quality: u8 },
    Gif,
    Ico,
    Avif { quality: u8, speed: u8 },
    Bmp,
    Tiff,
}

impl ImageOutputFormat {
    /// Detect format from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "png" => Some(Self::Png),
            "jpg" | "jpeg" => Some(Self::Jpeg { quality: 85 }),
            "webp" => Some(Self::WebP { quality: 80 }),
            "gif" => Some(Self::Gif),
            "ico" => Some(Self::Ico),
            "avif" => Some(Self::Avif { quality: 80, speed: 6 }),
            "bmp" => Some(Self::Bmp),
            "tiff" | "tif" => Some(Self::Tiff),
            _ => None,
        }
    }
}

/// Convert image between formats
pub fn convert(
    input: impl AsRef<Path>,
    output: impl AsRef<Path>,
    format: Option<ImageOutputFormat>,
) -> MediaResult<()> {
    let img = ImageReader::open(&input)?
        .with_guessed_format()?
        .decode()
        .map_err(|e| MediaError::ConversionFailed(e.to_string()))?;
    
    let format = format.or_else(|| {
        output.as_ref()
            .extension()
            .and_then(|e| e.to_str())
            .and_then(ImageOutputFormat::from_extension)
    }).ok_or_else(|| MediaError::UnsupportedFormat("Unknown output format".into()))?;
    
    save_image(&img, output.as_ref(), format)
}

fn save_image(img: &DynamicImage, path: &Path, format: ImageOutputFormat) -> MediaResult<()> {
    use std::fs::File;
    use std::io::BufWriter;
    
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    
    match format {
        ImageOutputFormat::Png => {
            img.write_to(&mut writer, ImageFormat::Png)?;
        }
        ImageOutputFormat::Jpeg { quality } => {
            let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut writer, quality);
            img.write_with_encoder(encoder)?;
        }
        ImageOutputFormat::WebP { quality } => {
            let encoder = image::codecs::webp::WebPEncoder::new_lossless(&mut writer);
            // Note: For lossy, use WebPEncoder::new_with_quality
            img.write_with_encoder(encoder)?;
        }
        // ... handle other formats
        _ => {
            return Err(MediaError::UnsupportedFormat(format!("{:?}", format)));
        }
    }
    
    Ok(())
}
Day 8-9: Advanced Image Operations
Rust

// crates/dx-media/src/image/resize.rs

use image::{DynamicImage, imageops::FilterType};
use crate::core::MediaResult;

#[derive(Debug, Clone, Copy)]
pub struct ResizeOptions {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub keep_aspect: bool,
    pub filter: FilterType,
}

impl Default for ResizeOptions {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
            keep_aspect: true,
            filter: FilterType::Lanczos3,
        }
    }
}

/// Smart resize maintaining aspect ratio
pub fn resize(img: &DynamicImage, opts: ResizeOptions) -> MediaResult<DynamicImage> {
    let (orig_w, orig_h) = (img.width(), img.height());
    
    let (new_w, new_h) = match (opts.width, opts.height, opts.keep_aspect) {
        (Some(w), Some(h), false) => (w, h),
        (Some(w), Some(h), true) => {
            // Fit within bounds
            let ratio_w = w as f32 / orig_w as f32;
            let ratio_h = h as f32 / orig_h as f32;
            let ratio = ratio_w.min(ratio_h);
            ((orig_w as f32 * ratio) as u32, (orig_h as f32 * ratio) as u32)
        }
        (Some(w), None, _) => {
            let ratio = w as f32 / orig_w as f32;
            (w, (orig_h as f32 * ratio) as u32)
        }
        (None, Some(h), _) => {
            let ratio = h as f32 / orig_h as f32;
            ((orig_w as f32 * ratio) as u32, h)
        }
        (None, None, _) => (orig_w, orig_h),
    };
    
    Ok(img.resize_exact(new_w, new_h, opts.filter))
}
Rust

// crates/dx-media/src/image/metadata.rs

use img_parts::{ImageEXIF, DynImage, jpeg::Jpeg, png::Png};
use std::path::Path;
use std::fs;
use crate::core::MediaResult;

/// Remove all EXIF/metadata from image (privacy-safe)
pub fn strip_metadata(input: impl AsRef<Path>, output: impl AsRef<Path>) -> MediaResult<()> {
    let bytes = fs::read(&input)?;
    
    // Detect format and strip accordingly
    if let Ok(mut jpeg) = Jpeg::from_bytes(bytes.clone().into()) {
        jpeg.set_exif(None);
        fs::write(&output, jpeg.encoder().bytes())?;
    } else if let Ok(mut png) = Png::from_bytes(bytes.clone().into()) {
        // PNG uses tEXt chunks for metadata
        // img-parts handles this
        png.set_exif(None);
        fs::write(&output, png.encoder().bytes())?;
    } else {
        // Fallback: re-encode through image crate (strips metadata)
        let img = image::open(&input)?;
        img.save(&output)?;
    }
    
    Ok(())
}
Day 10: QR and Color Tools
Rust

// crates/dx-media/src/image/qr.rs

use qrcode::{QrCode, EcLevel};
use image::{Luma, ImageBuffer};
use crate::core::MediaResult;

/// Generate QR code image
pub fn generate_qr(
    data: &str,
    size: u32,
    output: impl AsRef<std::path::Path>,
) -> MediaResult<()> {
    let code = QrCode::with_error_correction_level(data, EcLevel::M)
        .map_err(|e| crate::core::MediaError::ConversionFailed(e.to_string()))?;
    
    let image = code.render::<Luma<u8>>()
        .min_dimensions(size, size)
        .build();
    
    image.save(output)?;
    Ok(())
}

/// Read QR code from image
#[cfg(feature = "image-qr")]
pub fn read_qr(input: impl AsRef<std::path::Path>) -> MediaResult<String> {
    use rxing::{BarcodeFormat, DecodeHintType, DecodeHints, RXingResultMetadataType};
    
    let img = image::open(&input)?;
    let luma = img.to_luma8();
    
    let mut hints = DecodeHints::default();
    hints.try_harder = true;
    
    let result = rxing::helpers::detect_in_luma_with_hints(
        luma.into_raw(),
        img.width(),
        img.height(),
        None,
        &mut hints,
    ).map_err(|e| crate::core::MediaError::ConversionFailed(e.to_string()))?;
    
    Ok(result.getText().to_string())
}
Rust

// crates/dx-media/src/image/palette.rs

use image::DynamicImage;
use std::collections::HashMap;

/// Extracted color palette
#[derive(Debug, Clone)]
pub struct ColorPalette {
    pub colors: Vec<PaletteColor>,
}

#[derive(Debug, Clone)]
pub struct PaletteColor {
    pub hex: String,
    pub rgb: (u8, u8, u8),
    pub percentage: f32,
}

/// Extract dominant colors using k-means clustering
pub fn extract_palette(img: &DynamicImage, num_colors: usize) -> ColorPalette {
    let rgba = img.to_rgba8();
    let mut color_counts: HashMap<(u8, u8, u8), usize> = HashMap::new();
    
    // Quantize to reduce color space (5-bit per channel)
    for pixel in rgba.pixels() {
        let r = (pixel[0] / 8) * 8;
        let g = (pixel[1] / 8) * 8;
        let b = (pixel[2] / 8) * 8;
        *color_counts.entry((r, g, b)).or_insert(0) += 1;
    }
    
    // Sort by frequency
    let mut sorted: Vec<_> = color_counts.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    
    let total: usize = sorted.iter().map(|(_, c)| c).sum();
    
    let colors = sorted
        .into_iter()
        .take(num_colors)
        .map(|((r, g, b), count)| PaletteColor {
            hex: format!("#{:02X}{:02X}{:02X}", r, g, b),
            rgb: (r, g, b),
            percentage: (count as f32 / total as f32) * 100.0,
        })
        .collect();
    
    ColorPalette { colors }
}
Phase 3: Audio Processing (Week 3)
Day 11-12: Audio Core
Rust

// crates/dx-media/src/audio/convert.rs

use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use hound::{WavSpec, WavWriter};
use std::path::Path;
use std::fs::File;
use crate::core::MediaResult;

/// Supported audio output formats
#[derive(Debug, Clone, Copy)]
pub enum AudioOutputFormat {
    Wav { sample_rate: u32, bits: u16 },
    Flac,
    // Note: MP3/OGG encoding requires additional crates
}

impl Default for AudioOutputFormat {
    fn default() -> Self {
        Self::Wav {
            sample_rate: 44100,
            bits: 16,
        }
    }
}

/// Convert audio file to WAV
pub fn convert_to_wav(
    input: impl AsRef<Path>,
    output: impl AsRef<Path>,
    sample_rate: Option<u32>,
) -> MediaResult<()> {
    // Create media source
    let file = File::open(&input)?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());
    
    // Probe the file
    let mut hint = Hint::new();
    if let Some(ext) = input.as_ref().extension().and_then(|e| e.to_str()) {
        hint.with_extension(ext);
    }
    
    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &FormatOptions::default(), &MetadataOptions::default())
        .map_err(|e| crate::core::MediaError::ConversionFailed(e.to_string()))?;
    
    let mut format = probed.format;
    let track = format.default_track().ok_or_else(|| {
        crate::core::MediaError::InvalidInput("No audio track found".into())
    })?;
    
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &DecoderOptions::default())
        .map_err(|e| crate::core::MediaError::ConversionFailed(e.to_string()))?;
    
    let sample_rate = sample_rate.unwrap_or(
        track.codec_params.sample_rate.unwrap_or(44100)
    );
    let channels = track.codec_params.channels
        .map(|c| c.count() as u16)
        .unwrap_or(2);
    
    let spec = WavSpec {
        channels,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    
    let mut writer = WavWriter::create(&output, spec)?;
    
    // Decode and write samples
    loop {
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(_) => break,
        };
        
        let decoded = match decoder.decode(&packet) {
            Ok(decoded) => decoded,
            Err(_) => continue,
        };
        
        // Convert samples to i16 and write
        // This is simplified - real implementation needs buffer handling
        let mut buf = symphonia::core::audio::SampleBuffer::<i16>::new(
            decoded.capacity() as u64,
            *decoded.spec(),
        );
        buf.copy_interleaved_ref(decoded);
        
        for sample in buf.samples() {
            writer.write_sample(*sample)?;
        }
    }
    
    writer.finalize()?;
    Ok(())
}
Day 13-14: Audio Tags and Analysis
Rust

// crates/dx-media/src/audio/tags.rs

use lofty::{Accessor, AudioFile, Probe, Tag, TagExt, TaggedFileExt};
use std::path::Path;
use crate::core::MediaResult;

/// Audio metadata
#[derive(Debug, Clone, Default)]
pub struct AudioMetadata {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub year: Option<u32>,
    pub track: Option<u32>,
    pub genre: Option<String>,
    pub duration_secs: f64,
    pub sample_rate: Option<u32>,
    pub channels: Option<u8>,
    pub bitrate: Option<u32>,
}

/// Read audio metadata
pub fn read_metadata(path: impl AsRef<Path>) -> MediaResult<AudioMetadata> {
    let tagged_file = Probe::open(&path)?
        .read()
        .map_err(|e| crate::core::MediaError::ConversionFailed(e.to_string()))?;
    
    let tag = tagged_file.primary_tag().or_else(|| tagged_file.first_tag());
    let properties = tagged_file.properties();
    
    let mut meta = AudioMetadata {
        duration_secs: properties.duration().as_secs_f64(),
        sample_rate: properties.sample_rate(),
        channels: properties.channels(),
        bitrate: properties.audio_bitrate(),
        ..Default::default()
    };
    
    if let Some(tag) = tag {
        meta.title = tag.title().map(|s| s.to_string());
        meta.artist = tag.artist().map(|s| s.to_string());
        meta.album = tag.album().map(|s| s.to_string());
        meta.year = tag.year();
        meta.track = tag.track();
        meta.genre = tag.genre().map(|s| s.to_string());
    }
    
    Ok(meta)
}

/// Write audio metadata
pub fn write_metadata(
    path: impl AsRef<Path>,
    meta: &AudioMetadata,
) -> MediaResult<()> {
    let mut tagged_file = Probe::open(&path)?
        .read()
        .map_err(|e| crate::core::MediaError::ConversionFailed(e.to_string()))?;
    
    let tag = tagged_file.primary_tag_mut().ok_or_else(|| {
        crate::core::MediaError::InvalidInput("No tag found".into())
    })?;
    
    if let Some(ref title) = meta.title {
        tag.set_title(title.clone());
    }
    if let Some(ref artist) = meta.artist {
        tag.set_artist(artist.clone());
    }
    if let Some(ref album) = meta.album {
        tag.set_album(album.clone());
    }
    if let Some(year) = meta.year {
        tag.set_year(year);
    }
    if let Some(track) = meta.track {
        tag.set_track(track);
    }
    if let Some(ref genre) = meta.genre {
        tag.set_genre(genre.clone());
    }
    
    tag.save_to_path(&path)
        .map_err(|e| crate::core::MediaError::ConversionFailed(e.to_string()))?;
    
    Ok(())
}
Rust

// crates/dx-media/src/audio/waveform.rs

use image::{ImageBuffer, Rgb};
use symphonia::core::audio::SampleBuffer;
use std::path::Path;
use crate::core::MediaResult;

/// Waveform visualization options
#[derive(Debug, Clone)]
pub struct WaveformOptions {
    pub width: u32,
    pub height: u32,
    pub background: Rgb<u8>,
    pub foreground: Rgb<u8>,
    pub center_line: bool,
}

impl Default for WaveformOptions {
    fn default() -> Self {
        Self {
            width: 1800,
            height: 280,
            background: Rgb([255, 255, 255]),
            foreground: Rgb([66, 133, 244]),
            center_line: true,
        }
    }
}

/// Generate waveform PNG from audio file
pub fn generate_waveform(
    input: impl AsRef<Path>,
    output: impl AsRef<Path>,
    opts: WaveformOptions,
) -> MediaResult<()> {
    // Read audio samples (simplified - real impl uses symphonia)
    let samples = read_audio_samples(&input)?;
    
    // Create image
    let mut img = ImageBuffer::from_pixel(opts.width, opts.height, opts.background);
    
    let center_y = opts.height / 2;
    let samples_per_pixel = samples.len() / opts.width as usize;
    
    for x in 0..opts.width {
        let start = x as usize * samples_per_pixel;
        let end = (start + samples_per_pixel).min(samples.len());
        
        if start >= samples.len() {
            break;
        }
        
        // Find min/max in this slice
        let slice = &samples[start..end];
        let (min, max) = slice.iter().fold((0.0f32, 0.0f32), |(min, max), &s| {
            (min.min(s), max.max(s))
        });
        
        // Scale to image height
        let y_top = ((1.0 - max) * center_y as f32) as u32;
        let y_bottom = ((1.0 - min) * center_y as f32) as u32;
        
        for y in y_top.min(center_y)..=y_bottom.max(center_y) {
            if y < opts.height {
                img.put_pixel(x, y, opts.foreground);
            }
        }
    }
    
    // Draw center line
    if opts.center_line {
        for x in 0..opts.width {
            img.put_pixel(x, center_y, Rgb([200, 200, 200]));
        }
    }
    
    img.save(&output)?;
    Ok(())
}

fn read_audio_samples(path: impl AsRef<Path>) -> MediaResult<Vec<f32>> {
    // Implementation using symphonia to decode audio
    // Returns normalized samples in range [-1.0, 1.0]
    todo!("Implement with symphonia")
}
Phase 4: Video Processing (Week 4)
Day 15-17: FFmpeg Integration
Rust

// crates/dx-media/src/video/mod.rs

#[cfg(feature = "video")]
use ffmpeg_next as ffmpeg;

use std::path::Path;
use crate::core::MediaResult;

/// Initialize FFmpeg (call once at startup)
#[cfg(feature = "video")]
pub fn init() -> MediaResult<()> {
    ffmpeg::init()
        .map_err(|e| crate::core::MediaError::ConversionFailed(e.to_string()))
}

/// Video metadata
#[derive(Debug, Clone)]
pub struct VideoInfo {
    pub duration_secs: f64,
    pub width: u32,
    pub height: u32,
    pub fps: f64,
    pub video_codec: Option<String>,
    pub audio_codec: Option<String>,
    pub bitrate: Option<u64>,
    pub format: String,
}

/// Get video information
#[cfg(feature = "video")]
pub fn get_info(path: impl AsRef<Path>) -> MediaResult<VideoInfo> {
    let input = ffmpeg::format::input(&path.as_ref())
        .map_err(|e| crate::core::MediaError::ConversionFailed(e.to_string()))?;
    
    let duration = input.duration() as f64 / ffmpeg::ffi::AV_TIME_BASE as f64;
    
    let video_stream = input.streams().best(ffmpeg::media::Type::Video);
    let audio_stream = input.streams().best(ffmpeg::media::Type::Audio);
    
    let (width, height, fps, video_codec) = if let Some(stream) = video_stream {
        let codec = stream.parameters();
        let decoder = ffmpeg::codec::context::Context::from_parameters(codec)
            .ok()
            .and_then(|ctx| ctx.decoder().video().ok());
        
        let fps = stream.avg_frame_rate();
        let fps = if fps.denominator() > 0 {
            fps.numerator() as f64 / fps.denominator() as f64
        } else {
            0.0
        };
        
        if let Some(dec) = decoder {
            (dec.width(), dec.height(), fps, Some(codec.id().name().to_string()))
        } else {
            (0, 0, fps, None)
        }
    } else {
        (0, 0, 0.0, None)
    };
    
    let audio_codec = audio_stream.map(|s| s.parameters().id().name().to_string());
    
    Ok(VideoInfo {
        duration_secs: duration,
        width,
        height,
        fps,
        video_codec,
        audio_codec,
        bitrate: Some(input.bit_rate() as u64),
        format: input.format().name().to_string(),
    })
}
Rust

// crates/dx-media/src/video/transcode.rs

#[cfg(feature = "video")]
use ffmpeg_next as ffmpeg;
use std::path::Path;
use crate::core::MediaResult;

/// Video transcoding options
#[derive(Debug, Clone)]
pub struct TranscodeOptions {
    /// Video codec (e.g., "libx264", "libx265", "vp9")
    pub video_codec: Option<String>,
    /// Audio codec (e.g., "aac", "libmp3lame", "opus")
    pub audio_codec: Option<String>,
    /// Video bitrate in kbps
    pub video_bitrate: Option<u32>,
    /// Audio bitrate in kbps
    pub audio_bitrate: Option<u32>,
    /// CRF quality (0-51, lower = better)
    pub crf: Option<u8>,
    /// Preset (ultrafast, fast, medium, slow, veryslow)
    pub preset: Option<String>,
    /// Output resolution (width, height) - None for same as input
    pub resolution: Option<(u32, u32)>,
}

impl Default for TranscodeOptions {
    fn default() -> Self {
        Self {
            video_codec: Some("libx264".to_string()),
            audio_codec: Some("aac".to_string()),
            video_bitrate: None,
            audio_bitrate: Some(128),
            crf: Some(23),
            preset: Some("medium".to_string()),
            resolution: None,
        }
    }
}

/// Transcode video to different format
#[cfg(feature = "video")]
pub fn transcode(
    input: impl AsRef<Path>,
    output: impl AsRef<Path>,
    opts: TranscodeOptions,
) -> MediaResult<()> {
    // This is a high-level API - real implementation uses ffmpeg-next
    // For complex transcoding, consider using std::process::Command with ffmpeg CLI
    
    use std::process::Command;
    
    let mut cmd = Command::new("ffmpeg");
    cmd.args(["-i", input.as_ref().to_str().unwrap()]);
    cmd.args(["-y"]); // Overwrite output
    
    if let Some(ref vcodec) = opts.video_codec {
        cmd.args(["-c:v", vcodec]);
    }
    
    if let Some(ref acodec) = opts.audio_codec {
        cmd.args(["-c:a", acodec]);
    }
    
    if let Some(crf) = opts.crf {
        cmd.args(["-crf", &crf.to_string()]);
    }
    
    if let Some(ref preset) = opts.preset {
        cmd.args(["-preset", preset]);
    }
    
    if let Some(bitrate) = opts.audio_bitrate {
        cmd.args(["-b:a", &format!("{}k", bitrate)]);
    }
    
    if let Some((w, h)) = opts.resolution {
        cmd.args(["-vf", &format!("scale={}:{}", w, h)]);
    }
    
    cmd.arg(output.as_ref().to_str().unwrap());
    
    let status = cmd.status()?;
    
    if status.success() {
        Ok(())
    } else {
        Err(crate::core::MediaError::ConversionFailed(
            "FFmpeg transcode failed".into()
        ))
    }
}
Day 18-19: Video Utilities
Rust

// crates/dx-media/src/video/thumbnail.rs

use std::path::Path;
use crate::core::MediaResult;

/// Extract thumbnail at specific timestamp
pub fn extract_thumbnail(
    input: impl AsRef<Path>,
    output: impl AsRef<Path>,
    timestamp_secs: f64,
    size: Option<(u32, u32)>,
) -> MediaResult<()> {
    use std::process::Command;
    
    let mut cmd = Command::new("ffmpeg");
    cmd.args(["-i", input.as_ref().to_str().unwrap()]);
    cmd.args(["-ss", &timestamp_secs.to_string()]);
    cmd.args(["-vframes", "1"]);
    
    if let Some((w, h)) = size {
        cmd.args(["-vf", &format!("scale={}:{}", w, h)]);
    }
    
    cmd.args(["-y"]);
    cmd.arg(output.as_ref().to_str().unwrap());
    
    let status = cmd.status()?;
    
    if status.success() {
        Ok(())
    } else {
        Err(crate::core::MediaError::ConversionFailed(
            "Thumbnail extraction failed".into()
        ))
    }
}

/// Extract audio from video
pub fn extract_audio(
    input: impl AsRef<Path>,
    output: impl AsRef<Path>,
    format: Option<&str>,
) -> MediaResult<()> {
    use std::process::Command;
    
    let mut cmd = Command::new("ffmpeg");
    cmd.args(["-i", input.as_ref().to_str().unwrap()]);
    cmd.args(["-vn"]); // No video
    
    // Detect output format from extension
    let format = format.or_else(|| {
        output.as_ref()
            .extension()
            .and_then(|e| e.to_str())
    }).unwrap_or("mp3");
    
    match format {
        "mp3" => {
            cmd.args(["-acodec", "libmp3lame"]);
            cmd.args(["-b:a", "192k"]);
        }
        "wav" => {
            cmd.args(["-acodec", "pcm_s16le"]);
        }
        "flac" => {
            cmd.args(["-acodec", "flac"]);
        }
        "aac" | "m4a" => {
            cmd.args(["-acodec", "aac"]);
            cmd.args(["-b:a", "192k"]);
        }
        _ => {}
    }
    
    cmd.args(["-y"]);
    cmd.arg(output.as_ref().to_str().unwrap());
    
    let status = cmd.status()?;
    
    if status.success() {
        Ok(())
    } else {
        Err(crate::core::MediaError::ConversionFailed(
            "Audio extraction failed".into()
        ))
    }
}
Rust

// crates/dx-media/src/video/gif.rs

use image::{Frame, Delay, ImageBuffer, Rgba, AnimationEncoder};
use gif::{Encoder, Repeat};
use std::path::Path;
use std::fs::File;
use crate::core::MediaResult;

/// GIF creation options
#[derive(Debug, Clone)]
pub struct GifOptions {
    pub fps: u8,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub start_time: f64,
    pub duration: f64,
    pub loop_count: Option<u16>, // None = infinite
}

impl Default for GifOptions {
    fn default() -> Self {
        Self {
            fps: 15,
            width: Some(480),
            height: None,
            start_time: 0.0,
            duration: 5.0,
            loop_count: None,
        }
    }
}

/// Convert video segment to GIF
pub fn video_to_gif(
    input: impl AsRef<Path>,
    output: impl AsRef<Path>,
    opts: GifOptions,
) -> MediaResult<()> {
    use std::process::Command;
    use tempfile::tempdir;
    
    // Create temp directory for frames
    let temp_dir = tempdir()?;
    let frame_pattern = temp_dir.path().join("frame_%04d.png");
    
    // Extract frames using ffmpeg
    let mut cmd = Command::new("ffmpeg");
    cmd.args(["-i", input.as_ref().to_str().unwrap()]);
    cmd.args(["-ss", &opts.start_time.to_string()]);
    cmd.args(["-t", &opts.duration.to_string()]);
    cmd.args(["-r", &opts.fps.to_string()]);
    
    let filter = if let (Some(w), Some(h)) = (opts.width, opts.height) {
        format!("scale={}:{}:flags=lanczos", w, h)
    } else if let Some(w) = opts.width {
        format!("scale={}:-1:flags=lanczos", w)
    } else {
        "scale=-1:-1".to_string()
    };
    cmd.args(["-vf", &filter]);
    
    cmd.arg(frame_pattern.to_str().unwrap());
    
    let status = cmd.status()?;
    if !status.success() {
        return Err(crate::core::MediaError::ConversionFailed(
            "Frame extraction failed".into()
        ));
    }
    
    // Read frames and encode GIF
    let mut frames: Vec<_> = std::fs::read_dir(temp_dir.path())?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map(|e| e == "png").unwrap_or(false))
        .collect();
    frames.sort();
    
    if frames.is_empty() {
        return Err(crate::core::MediaError::ConversionFailed(
            "No frames extracted".into()
        ));
    }
    
    // Load first frame to get dimensions
    let first_frame = image::open(&frames[0])?;
    let (width, height) = (first_frame.width() as u16, first_frame.height() as u16);
    
    let file = File::create(&output)?;
    let mut encoder = Encoder::new(file, width, height, &[])?;
    encoder.set_repeat(match opts.loop_count {
        None => Repeat::Infinite,
        Some(n) => Repeat::Finite(n),
    })?;
    
    let delay = (100.0 / opts.fps as f32) as u16;
    
    for frame_path in frames {
        let img = image::open(&frame_path)?;
        let rgba = img.to_rgba8();
        
        let mut frame = gif::Frame::from_rgba_speed(
            width,
            height,
            &mut rgba.into_raw(),
            10,
        );
        frame.delay = delay;
        
        encoder.write_frame(&frame)?;
    }
    
    Ok(())
}
Phase 5: Document & Archive Processing (Week 5)
Day 20-21: Document Tools
Rust

// crates/dx-media/src/document/pdf_merge.rs

use lopdf::{Document, Object, ObjectId};
use std::path::Path;
use crate::core::MediaResult;

/// Merge multiple PDFs into one
pub fn merge_pdfs(
    inputs: &[impl AsRef<Path>],
    output: impl AsRef<Path>,
) -> MediaResult<()> {
    if inputs.is_empty() {
        return Err(crate::core::MediaError::InvalidInput(
            "No input files provided".into()
        ));
    }
    
    // Load first document as base
    let mut merged = Document::load(inputs[0].as_ref())
        .map_err(|e| crate::core::MediaError::ConversionFailed(e.to_string()))?;
    
    // Merge remaining documents
    for input in inputs.iter().skip(1) {
        let doc = Document::load(input.as_ref())
            .map_err(|e| crate::core::MediaError::ConversionFailed(e.to_string()))?;
        
        // Get pages from source document
        let pages = doc.get_pages();
        
        for &page_id in pages.values() {
            // Clone page to merged document
            if let Ok(page) = doc.get_object(page_id) {
                let new_id = merged.add_object(page.clone());
                // Add page to pages tree
                // (simplified - real implementation needs proper page tree handling)
            }
        }
    }
    
    merged.save(&output)
        .map_err(|e| crate::core::MediaError::ConversionFailed(e.to_string()))?;
    
    Ok(())
}

/// Split PDF into individual pages or ranges
pub fn split_pdf(
    input: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
    ranges: Option<&[(u32, u32)]>,
) -> MediaResult<Vec<std::path::PathBuf>> {
    let doc = Document::load(input.as_ref())
        .map_err(|e| crate::core::MediaError::ConversionFailed(e.to_string()))?;
    
    let pages = doc.get_pages();
    let page_count = pages.len() as u32;
    
    let ranges = ranges.map(|r| r.to_vec()).unwrap_or_else(|| {
        (1..=page_count).map(|i| (i, i)).collect()
    });
    
    std::fs::create_dir_all(&output_dir)?;
    
    let mut outputs = Vec::new();
    
    for (start, end) in ranges {
        let output_path = output_dir.as_ref().join(format!("pages_{}-{}.pdf", start, end));
        
        // Create new document with selected pages
        let mut new_doc = Document::with_version("1.5");
        
        for page_num in start..=end {
            if let Some(&page_id) = pages.get(&page_num) {
                if let Ok(page) = doc.get_object(page_id) {
                    new_doc.add_object(page.clone());
                }
            }
        }
        
        new_doc.save(&output_path)
            .map_err(|e| crate::core::MediaError::ConversionFailed(e.to_string()))?;
        
        outputs.push(output_path);
    }
    
    Ok(outputs)
}
Rust

// crates/dx-media/src/document/markdown.rs

use pulldown_cmark::{Parser, Options, html};
use std::path::Path;
use crate::core::MediaResult;

/// Markdown conversion options
#[derive(Debug, Clone)]
pub struct MarkdownOptions {
    pub tables: bool,
    pub footnotes: bool,
    pub strikethrough: bool,
    pub tasklists: bool,
    pub smart_punctuation: bool,
    pub heading_attributes: bool,
}

impl Default for MarkdownOptions {
    fn default() -> Self {
        Self {
            tables: true,
            footnotes: true,
            strikethrough: true,
            tasklists: true,
            smart_punctuation: true,
            heading_attributes: true,
        }
    }
}

/// Convert Markdown to HTML
pub fn markdown_to_html(
    input: impl AsRef<Path>,
    output: impl AsRef<Path>,
    opts: MarkdownOptions,
    template: Option<&str>,
) -> MediaResult<()> {
    let markdown = std::fs::read_to_string(&input)?;
    
    let mut options = Options::empty();
    if opts.tables { options.insert(Options::ENABLE_TABLES); }
    if opts.footnotes { options.insert(Options::ENABLE_FOOTNOTES); }
    if opts.strikethrough { options.insert(Options::ENABLE_STRIKETHROUGH); }
    if opts.tasklists { options.insert(Options::ENABLE_TASKLISTS); }
    if opts.smart_punctuation { options.insert(Options::ENABLE_SMART_PUNCTUATION); }
    if opts.heading_attributes { options.insert(Options::ENABLE_HEADING_ATTRIBUTES); }
    
    let parser = Parser::new_ext(&markdown, options);
    
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    
    // Wrap in template if provided
    let final_html = if let Some(template) = template {
        template.replace("{{content}}", &html_output)
    } else {
        format!(r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <style>
        body {{ font-family: system-ui, sans-serif; max-width: 800px; margin: 2rem auto; padding: 0 1rem; }}
        pre {{ background: #f4f4f4; padding: 1rem; overflow-x: auto; }}
        code {{ background: #f4f4f4; padding: 0.2rem 0.4rem; }}
        table {{ border-collapse: collapse; width: 100%; }}
        th, td {{ border: 1px solid #ddd; padding: 0.5rem; text-align: left; }}
    </style>
</head>
<body>
{html_output}
</body>
</html>"#)
    };
    
    std::fs::write(&output, final_html)?;
    Ok(())
}
Day 22-23: Archive Tools
Rust

// crates/dx-media/src/archive/extract.rs

use std::path::Path;
use std::fs::File;
use std::io::{Read, BufReader};
use crate::core::MediaResult;

/// Detected archive format
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArchiveFormat {
    Zip,
    TarGz,
    TarXz,
    TarBz2,
    Tar,
    SevenZip,
    Rar,
    Unknown,
}

impl ArchiveFormat {
    /// Detect format from file magic bytes
    pub fn detect(path: impl AsRef<Path>) -> MediaResult<Self> {
        let mut file = File::open(&path)?;
        let mut magic = [0u8; 8];
        file.read_exact(&mut magic).ok();
        
        Ok(match &magic {
            [0x50, 0x4B, 0x03, 0x04, ..] => Self::Zip,
            [0x1F, 0x8B, ..] => Self::TarGz,
            [0xFD, 0x37, 0x7A, 0x58, 0x5A, 0x00, ..] => Self::TarXz,
            [0x42, 0x5A, 0x68, ..] => Self::TarBz2,
            [0x75, 0x73, 0x74, 0x61, 0x72, ..] => Self::Tar,
            [0x37, 0x7A, 0xBC, 0xAF, 0x27, 0x1C, ..] => Self::SevenZip,
            [0x52, 0x61, 0x72, 0x21, ..] => Self::Rar,
            _ => Self::Unknown,
        })
    }
}

/// Extract archive to directory
pub fn extract(
    input: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
    flatten: bool,
) -> MediaResult<Vec<std::path::PathBuf>> {
    let format = ArchiveFormat::detect(&input)?;
    
    std::fs::create_dir_all(&output_dir)?;
    
    match format {
        ArchiveFormat::Zip => extract_zip(&input, &output_dir, flatten),
        ArchiveFormat::TarGz => extract_tar_gz(&input, &output_dir, flatten),
        ArchiveFormat::Tar => extract_tar(&input, &output_dir, flatten),
        _ => Err(crate::core::MediaError::UnsupportedFormat(
            format!("{:?}", format)
        )),
    }
}

fn extract_zip(
    input: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
    flatten: bool,
) -> MediaResult<Vec<std::path::PathBuf>> {
    let file = File::open(&input)?;
    let mut archive = zip::ZipArchive::new(file)?;
    
    let mut extracted = Vec::new();
    
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        
        let out_path = if flatten {
            output_dir.as_ref().join(
                Path::new(file.name())
                    .file_name()
                    .unwrap_or_default()
            )
        } else {
            output_dir.as_ref().join(file.name())
        };
        
        if file.name().ends_with('/') {
            std::fs::create_dir_all(&out_path)?;
        } else {
            if let Some(parent) = out_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let mut out_file = File::create(&out_path)?;
            std::io::copy(&mut file, &mut out_file)?;
            extracted.push(out_path);
        }
    }
    
    Ok(extracted)
}

fn extract_tar_gz(
    input: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
    flatten: bool,
) -> MediaResult<Vec<std::path::PathBuf>> {
    let file = File::open(&input)?;
    let decoder = flate2::read::GzDecoder::new(file);
    extract_tar_inner(decoder, output_dir, flatten)
}

fn extract_tar(
    input: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
    flatten: bool,
) -> MediaResult<Vec<std::path::PathBuf>> {
    let file = File::open(&input)?;
    extract_tar_inner(file, output_dir, flatten)
}

fn extract_tar_inner<R: Read>(
    reader: R,
    output_dir: impl AsRef<Path>,
    flatten: bool,
) -> MediaResult<Vec<std::path::PathBuf>> {
    let mut archive = tar::Archive::new(reader);
    let mut extracted = Vec::new();
    
    for entry in archive.entries()? {
        let mut entry = entry?;
        
        let out_path = if flatten {
            output_dir.as_ref().join(
                entry.path()?
                    .file_name()
                    .unwrap_or_default()
            )
        } else {
            output_dir.as_ref().join(entry.path()?)
        };
        
        entry.unpack(&out_path)?;
        
        if out_path.is_file() {
            extracted.push(out_path);
        }
    }
    
    Ok(extracted)
}
Rust

// crates/dx-media/src/archive/compress.rs

use std::path::Path;
use std::fs::File;
use std::io::{Write, BufWriter};
use walkdir::WalkDir;
use crate::core::MediaResult;

/// Compression level (1-9)
#[derive(Debug, Clone, Copy)]
pub struct CompressionLevel(pub u32);

impl Default for CompressionLevel {
    fn default() -> Self {
        Self(6)
    }
}

/// Create ZIP archive from directory or files
pub fn create_zip(
    inputs: &[impl AsRef<Path>],
    output: impl AsRef<Path>,
    level: CompressionLevel,
    password: Option<&str>,
) -> MediaResult<()> {
    let file = File::create(&output)?;
    let writer = BufWriter::new(file);
    let mut zip = zip::ZipWriter::new(writer);
    
    let options = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .compression_level(Some(level.0 as i32));
    
    // Handle password-protected archives
    #[cfg(feature = "archive-advanced")]
    let options = if let Some(pwd) = password {
        options.with_aes_encryption(
            zip::AesMode::Aes256,
            pwd,
        )
    } else {
        options
    };
    
    for input in inputs {
        let path = input.as_ref();
        
        if path.is_dir() {
            // Add directory contents recursively
            for entry in WalkDir::new(path) {
                let entry = entry?;
                let entry_path = entry.path();
                let rel_path = entry_path.strip_prefix(path)
                    .unwrap_or(entry_path);
                
                if entry_path.is_file() {
                    zip.start_file(
                        rel_path.to_string_lossy(),
                        options.clone(),
                    )?;
                    let mut f = File::open(entry_path)?;
                    std::io::copy(&mut f, &mut zip)?;
                } else if entry_path.is_dir() && entry_path != path {
                    zip.add_directory(
                        rel_path.to_string_lossy(),
                        options.clone(),
                    )?;
                }
            }
        } else if path.is_file() {
            zip.start_file(
                path.file_name().unwrap().to_string_lossy(),
                options.clone(),
            )?;
            let mut f = File::open(path)?;
            std::io::copy(&mut f, &mut zip)?;
        }
    }
    
    zip.finish()?;
    Ok(())
}

/// Create tarball (.tar.gz)
pub fn create_tarball(
    inputs: &[impl AsRef<Path>],
    output: impl AsRef<Path>,
    level: CompressionLevel,
) -> MediaResult<()> {
    let file = File::create(&output)?;
    let encoder = flate2::write::GzEncoder::new(
        file,
        flate2::Compression::new(level.0),
    );
    let mut tar = tar::Builder::new(encoder);
    
    for input in inputs {
        let path = input.as_ref();
        
        if path.is_dir() {
            tar.append_dir_all(
                path.file_name().unwrap_or_default(),
                path,
            )?;
        } else if path.is_file() {
            tar.append_path_with_name(
                path,
                path.file_name().unwrap_or_default(),
            )?;
        }
    }
    
    tar.into_inner()?.finish()?;
    Ok(())
}
Phase 6: Utility Tools (Week 6)
Day 24-25: File Utilities
Rust

// crates/dx-media/src/utility/rename.rs

use regex::Regex;
use std::path::{Path, PathBuf};
use crate::core::MediaResult;

/// Batch rename result
#[derive(Debug, Clone)]
pub struct RenameResult {
    pub original: PathBuf,
    pub renamed: PathBuf,
    pub success: bool,
}

/// Batch rename files using regex pattern
pub fn batch_rename(
    dir: impl AsRef<Path>,
    pattern: &str,
    replacement: &str,
    dry_run: bool,
) -> MediaResult<Vec<RenameResult>> {
    let regex = Regex::new(pattern)
        .map_err(|e| crate::core::MediaError::InvalidInput(e.to_string()))?;
    
    let mut results = Vec::new();
    
    for entry in std::fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if !path.is_file() {
            continue;
        }
        
        let filename = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default();
        
        if regex.is_match(filename) {
            let new_name = regex.replace_all(filename, replacement);
            let new_path = path.parent().unwrap().join(new_name.as_ref());
            
            let success = if dry_run {
                true
            } else {
                std::fs::rename(&path, &new_path).is_ok()
            };
            
            results.push(RenameResult {
                original: path,
                renamed: new_path,
                success,
            });
        }
    }
    
    Ok(results)
}
Rust

// crates/dx-media/src/utility/duplicate.rs

use sha2::{Sha256, Digest};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use walkdir::WalkDir;
use rayon::prelude::*;
use crate::core::MediaResult;

/// Duplicate file group
#[derive(Debug, Clone)]
pub struct DuplicateGroup {
    pub hash: String,
    pub size: u64,
    pub files: Vec<PathBuf>,
}

/// Find duplicate files by content hash
pub fn find_duplicates(
    dir: impl AsRef<Path>,
    min_size: Option<u64>,
) -> MediaResult<Vec<DuplicateGroup>> {
    let min_size = min_size.unwrap_or(1);
    
    // First pass: group by size
    let mut size_groups: HashMap<u64, Vec<PathBuf>> = HashMap::new();
    
    for entry in WalkDir::new(&dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        
        if path.is_file() {
            if let Ok(meta) = path.metadata() {
                let size = meta.len();
                if size >= min_size {
                    size_groups
                        .entry(size)
                        .or_default()
                        .push(path.to_path_buf());
                }
            }
        }
    }
    
    // Second pass: hash files with same size
    let groups: Vec<DuplicateGroup> = size_groups
        .into_par_iter()
        .filter(|(_, files)| files.len() > 1)
        .flat_map(|(size, files)| {
            let mut hash_groups: HashMap<String, Vec<PathBuf>> = HashMap::new();
            
            for path in files {
                if let Ok(hash) = hash_file(&path) {
                    hash_groups
                        .entry(hash)
                        .or_default()
                        .push(path);
                }
            }
            
            hash_groups
                .into_iter()
                .filter(|(_, files)| files.len() > 1)
                .map(|(hash, files)| DuplicateGroup {
                    hash,
                    size,
                    files,
                })
                .collect::<Vec<_>>()
        })
        .collect();
    
    Ok(groups)
}

fn hash_file(path: &Path) -> MediaResult<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];
    
    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }
    
    Ok(format!("{:x}", hasher.finalize()))
}
Rust

// crates/dx-media/src/utility/watch.rs

use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent};
use std::path::Path;
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;
use crate::core::MediaResult;

/// File watcher for hot folder processing
pub struct FileWatcher {
    watcher: notify::RecommendedWatcher,
    receiver: Receiver<DebouncedEvent>,
}

impl FileWatcher {
    /// Create a new file watcher
    pub fn new(debounce_ms: u64) -> MediaResult<Self> {
        let (tx, rx) = channel();
        let watcher = watcher(tx, Duration::from_millis(debounce_ms))
            .map_err(|e| crate::core::MediaError::ConversionFailed(e.to_string()))?;
        
        Ok(Self {
            watcher,
            receiver: rx,
        })
    }
    
    /// Watch a directory for changes
    pub fn watch(&mut self, path: impl AsRef<Path>) -> MediaResult<()> {
        self.watcher
            .watch(path.as_ref(), RecursiveMode::Recursive)
            .map_err(|e| crate::core::MediaError::ConversionFailed(e.to_string()))
    }
    
    /// Get next file event (blocking)
    pub fn next(&self) -> Option<FileEvent> {
        self.receiver.recv().ok().and_then(|event| match event {
            DebouncedEvent::Create(path) => Some(FileEvent::Created(path)),
            DebouncedEvent::Write(path) => Some(FileEvent::Modified(path)),
            DebouncedEvent::Remove(path) => Some(FileEvent::Deleted(path)),
            DebouncedEvent::Rename(from, to) => Some(FileEvent::Renamed { from, to }),
            _ => None,
        })
    }
    
    /// Try to get next event (non-blocking)
    pub fn try_next(&self) -> Option<FileEvent> {
        self.receiver.try_recv().ok().and_then(|event| match event {
            DebouncedEvent::Create(path) => Some(FileEvent::Created(path)),
            DebouncedEvent::Write(path) => Some(FileEvent::Modified(path)),
            DebouncedEvent::Remove(path) => Some(FileEvent::Deleted(path)),
            DebouncedEvent::Rename(from, to) => Some(FileEvent::Renamed { from, to }),
            _ => None,
        })
    }
}

#[derive(Debug, Clone)]
pub enum FileEvent {
    Created(std::path::PathBuf),
    Modified(std::path::PathBuf),
    Deleted(std::path::PathBuf),
    Renamed { from: std::path::PathBuf, to: std::path::PathBuf },
}
Day 26: Checksums and MIME Detection
Rust

// crates/dx-media/src/utility/checksum.rs

use std::path::Path;
use std::fs::File;
use std::io::{BufReader, Read};
use crate::core::MediaResult;

/// Checksum algorithms
#[derive(Debug, Clone, Copy)]
pub enum ChecksumAlgorithm {
    Md5,
    Sha1,
    Sha256,
    Sha512,
    Blake3,
}

/// Calculate file checksum
pub fn calculate(
    path: impl AsRef<Path>,
    algorithm: ChecksumAlgorithm,
) -> MediaResult<String> {
    let file = File::open(&path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = [0u8; 8192];
    
    match algorithm {
        ChecksumAlgorithm::Md5 => {
            use md5::{Md5, Digest};
            let mut hasher = Md5::new();
            loop {
                let n = reader.read(&mut buffer)?;
                if n == 0 { break; }
                hasher.update(&buffer[..n]);
            }
            Ok(format!("{:x}", hasher.finalize()))
        }
        ChecksumAlgorithm::Sha256 => {
            use sha2::{Sha256, Digest};
            let mut hasher = Sha256::new();
            loop {
                let n = reader.read(&mut buffer)?;
                if n == 0 { break; }
                hasher.update(&buffer[..n]);
            }
            Ok(format!("{:x}", hasher.finalize()))
        }
        ChecksumAlgorithm::Blake3 => {
            let mut hasher = blake3::Hasher::new();
            loop {
                let n = reader.read(&mut buffer)?;
                if n == 0 { break; }
                hasher.update(&buffer[..n]);
            }
            Ok(hasher.finalize().to_hex().to_string())
        }
        _ => {
            Err(crate::core::MediaError::UnsupportedFormat(
                format!("{:?} not implemented", algorithm)
            ))
        }
    }
}

/// Verify file against known checksum
pub fn verify(
    path: impl AsRef<Path>,
    expected: &str,
    algorithm: ChecksumAlgorithm,
) -> MediaResult<bool> {
    let actual = calculate(&path, algorithm)?;
    Ok(actual.eq_ignore_ascii_case(expected))
}
Rust

// crates/dx-media/src/utility/mime.rs

use infer::Infer;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use crate::core::MediaResult;

/// Detected file type
#[derive(Debug, Clone)]
pub struct FileType {
    pub mime: String,
    pub extension: String,
    pub category: FileCategory,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileCategory {
    Image,
    Video,
    Audio,
    Document,
    Archive,
    Font,
    Application,
    Unknown,
}

/// Detect file type by magic bytes (not extension)
pub fn detect(path: impl AsRef<Path>) -> MediaResult<FileType> {
    let mut file = File::open(&path)?;
    let mut buffer = [0u8; 8192];
    let n = file.read(&mut buffer)?;
    
    let info = Infer::new();
    
    if let Some(kind) = info.get(&buffer[..n]) {
        let category = match kind.mime_type().split('/').next() {
            Some("image") => FileCategory::Image,
            Some("video") => FileCategory::Video,
            Some("audio") => FileCategory::Audio,
            Some("font") => FileCategory::Font,
            Some("application") => {
                match kind.mime_type() {
                    "application/pdf" => FileCategory::Document,
                    "application/zip" | "application/x-tar" | "application/gzip" => FileCategory::Archive,
                    _ => FileCategory::Application,
                }
            }
            _ => FileCategory::Unknown,
        };
        
        Ok(FileType {
            mime: kind.mime_type().to_string(),
            extension: kind.extension().to_string(),
            category,
        })
    } else {
        // Try to infer from extension as fallback
        let ext = path.as_ref()
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        
        Ok(FileType {
            mime: mime_guess::from_ext(ext)
                .first_or_octet_stream()
                .to_string(),
            extension: ext.to_string(),
            category: FileCategory::Unknown,
        })
    }
}
Phase 7: Integration & CLI (Week 7)
Day 27-28: Full CLI Implementation
Rust

// crates/dx-media/src/cli/mod.rs

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "dx")]
#[command(about = "Binary-first media processing for DX")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
    
    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,
    
    /// Disable caching
    #[arg(long, global = true)]
    pub no_cache: bool,
    
    /// Number of parallel workers
    #[arg(short = 'j', long, global = true)]
    pub jobs: Option<usize>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Image processing
    #[command(subcommand)]
    Image(ImageCommand),
    
    /// Video processing
    #[command(subcommand)]
    Video(VideoCommand),
    
    /// Audio processing
    #[command(subcommand)]
    Audio(AudioCommand),
    
    /// Document processing
    #[command(subcommand)]
    Doc(DocCommand),
    
    /// Archive operations
    #[command(subcommand)]
    Archive(ArchiveCommand),
    
    /// File utilities
    #[command(subcommand)]
    Util(UtilCommand),
    
    /// Process a file (auto-detect type)
    Process {
        /// Input file
        input: PathBuf,
        /// Output file
        output: PathBuf,
        /// Processing options (JSON)
        #[arg(short, long)]
        options: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum ImageCommand {
    /// Convert image format
    Convert {
        input: PathBuf,
        output: PathBuf,
        #[arg(short, long)]
        quality: Option<u8>,
    },
    /// Resize image
    Resize {
        input: PathBuf,
        output: PathBuf,
        #[arg(short, long)]
        width: Option<u32>,
        #[arg(short = 'H', long)]
        height: Option<u32>,
        #[arg(long)]
        no_aspect: bool,
    },
    /// Compress image
    Compress {
        input: PathBuf,
        output: PathBuf,
        #[arg(short, long, default_value = "75")]
        quality: u8,
    },
    /// Add watermark
    Watermark {
        input: PathBuf,
        output: PathBuf,
        #[arg(short, long)]
        text: Option<String>,
        #[arg(short, long)]
        image: Option<PathBuf>,
        #[arg(long, default_value = "0.5")]
        opacity: f32,
    },
    /// Strip metadata
    Strip {
        input: PathBuf,
        output: PathBuf,
    },
    /// Generate QR code
    Qr {
        /// Text to encode
        data: String,
        output: PathBuf,
        #[arg(short, long, default_value = "256")]
        size: u32,
    },
    /// Extract color palette
    Palette {
        input: PathBuf,
        #[arg(short, long, default_value = "5")]
        colors: usize,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Apply filter
    Filter {
        input: PathBuf,
        output: PathBuf,
        /// Filter name: grayscale, blur, sharpen, contrast, brightness
        #[arg(short, long)]
        filter: String,
        #[arg(long)]
        intensity: Option<f32>,
    },
    /// Generate favicons
    Favicon {
        input: PathBuf,
        #[arg(short, long, default_value = ".")]
        output_dir: PathBuf,
    },
}

// Similar patterns for Video, Audio, Doc, Archive, Util commands...
Day 29-30: Testing & Documentation
Rust

// crates/dx-media/tests/image_tests.rs

use dx_media::image::*;
use tempfile::tempdir;

#[test]
fn test_image_convert_png_to_jpeg() {
    let temp = tempdir().unwrap();
    let input = "tests/fixtures/test.png";
    let output = temp.path().join("output.jpg");
    
    convert(input, &output, Some(ImageOutputFormat::Jpeg { quality: 85 }))
        .expect("Conversion failed");
    
    assert!(output.exists());
    
    // Verify it's actually a JPEG
    let mime = dx_media::utility::mime::detect(&output).unwrap();
    assert_eq!(mime.mime, "image/jpeg");
}

#[test]
fn test_image_resize_keep_aspect() {
    let img = image::open("tests/fixtures/test.png").unwrap();
    
    let resized = resize(&img, ResizeOptions {
        width: Some(100),
        height: None,
        keep_aspect: true,
        ..Default::default()
    }).unwrap();
    
    assert_eq!(resized.width(), 100);
    // Height should be proportional
}

#[test]
fn test_color_palette_extraction() {
    let img = image::open("tests/fixtures/test.png").unwrap();
    let palette = extract_palette(&img, 5);
    
    assert_eq!(palette.colors.len(), 5);
    for color in &palette.colors {
        assert!(color.hex.starts_with('#'));
        assert!(color.percentage > 0.0);
    }
}
📊 Final Cargo.toml
toml

[package]
name = "dx-media"
version = "0.1.0"
edition = "2024"
description = "Binary-first media processing for DX"
license = "MIT OR Apache-2.0"
repository = "https://github.com/dx-www/dx"
keywords = ["media", "image", "video", "audio", "conversion"]
categories = ["multimedia", "command-line-utilities"]

[lib]
name = "dx_media"
path = "src/lib.rs"

[[bin]]
name = "dx-media"
path = "src/main.rs"
required-features = ["cli"]

[features]
default = ["cli", "image-core", "archive-core", "utility-core"]

# CLI
cli = ["dep:clap"]

# Image Processing
image-core = ["dep:image", "dep:imageproc"]
image-advanced = ["image-core", "dep:oxipng", "dep:img-parts"]
image-qr = ["dep:qrcode", "dep:rxing"]
image-ocr = ["dep:ocrs"]

# Video Processing
video = ["dep:ffmpeg-next"]
video-gif = ["image-core", "dep:gif"]

# Audio Processing
audio-core = ["dep:symphonia", "dep:hound"]
audio-advanced = ["audio-core", "dep:rubato", "dep:dasp"]
audio-tags = ["dep:lofty"]

# Document Processing
document-core = ["dep:lopdf", "dep:pulldown-cmark"]
document-advanced = ["document-core", "dep:printpdf", "dep:pdf-extract"]
document-data = ["dep:csv", "dep:serde", "dep:serde_json"]
document-syntax = ["dep:syntect"]

# Archive Processing
archive-core = ["dep:zip", "dep:tar", "dep:flate2"]
archive-advanced = ["archive-core", "dep:compress-tools"]

# Utility
utility-core = ["dep:regex", "dep:sha2", "dep:notify"]
utility-advanced = ["utility-core", "dep:arboard", "dep:infer"]

# Full suite
full = [
    "cli",
    "image-advanced", "image-qr", "image-ocr",
    "video", "video-gif",
    "audio-advanced", "audio-tags",
    "document-advanced", "document-data", "document-syntax",
    "archive-advanced",
    "utility-advanced"
]

[dependencies]
# Core (always included)
thiserror = "2.0"
tracing = "0.1"
rayon = "1.10"
memmap2 = "0.9"
blake3 = "1.5"
walkdir = "2.5"
tempfile = "3.12"

# CLI
clap = { version = "4.5", features = ["derive"], optional = true }

# Image
image = { version = "0.25", optional = true }
imageproc = { version = "0.25", optional = true }
oxipng = { version = "9.1", optional = true }
img-parts = { version = "0.3", optional = true }
qrcode = { version = "0.14", optional = true }
rxing = { version = "0.7", optional = true }
ocrs = { version = "0.9", optional = true }

# Video
ffmpeg-next = { version = "7.1", optional = true }
gif = { version = "0.13", optional = true }

# Audio
symphonia = { version = "0.5", features = ["all"], optional = true }
hound = { version = "3.5", optional = true }
rubato = { version = "0.16", optional = true }
dasp = { version = "0.11", optional = true }
lofty = { version = "0.21", optional = true }

# Document
lopdf = { version = "0.26", optional = true }


And here is the left!!!


This is the complete, professional implementation plan for the **Dx Media Engine**. It is structured as a standalone architectural document ready to be dropped into your `docs/` folder or used as the roadmap for `crates/dx-media`.

It strictly adheres to the **Binary Dawn** philosophy: *Zero-Copy where possible, Feature-Gated for size, and High Performance.*

```markdown
# Dx Media Conversion Engine: Implementation Plan

> **Status:** Architecture Phase  
> **Target:** High-Performance Media Pipeline  
> **Philosophy:** Binary First, Zero Parse, Minimal Dependencies

## 📋 Executive Summary

The **Dx Media Engine** (`dx-media`) is a high-performance, modular crate designed to handle 60+ media manipulation tasks across images, video, audio, documents, and archives. 

Unlike traditional media tools that spawn heavy external processes for every task, `dx-media` integrates natively into the Rust runtime, leveraging **SIMD optimizations**, **memory mapping (`mmap`)**, and **parallel processing (`rayon`)** to achieve throughputs significantly higher than Node.js or Python equivalents.

---

## 🏗️ Architecture & Project Structure

The crate is organized to allow "pay-for-what-you-use" via Cargo features. A user needing only image resizing will not pay the binary size cost of FFmpeg bindings.

```text
dx/
├── crates/
│   └── dx-media/
│       ├── Cargo.toml                 # Feature-gated dependency matrix
│       ├── src/
│       │   ├── lib.rs                 # Public API
│       │   │
│       │   ├── core/                  # Shared Infrastructure
│       │   │   ├── pipeline.rs        # Zero-copy Mmap logic
│       │   │   ├── cache.rs           # Blake3-based artifact caching
│       │   │   └── error.rs           # Unified error handling
│       │   │
│       │   ├── image/                 # Image Engine
│       │   │   ├── convert.rs         # Format transcoding
│       │   │   ├── ops.rs             # Resize, Crop, Filter
│       │   │   ├── meta.rs            # EXIF/Metadata stripping
│       │   │   ├── qr.rs              # QR Code Gen/Read
│       │   │   └── analysis.rs        # Palette extraction, OCR
│       │   │
│       │   ├── video/                 # Video Engine (FFmpeg)
│       │   │   ├── transcode.rs       # Format conversion
│       │   │   ├── process.rs         # Trim, Concat, Mute
│       │   │   └── frames.rs          # Thumbnail, GIF extraction
│       │   │
│       │   ├── audio/                 # Audio Engine
│       │   │   ├── codec.rs           # Decode/Encode pipeline
│       │   │   ├── tags.rs            # ID3/Metadata manipulation
│       │   │   └── dsp.rs             # Waveforms, Normalization, Silence
│       │   │
│       │   ├── document/              # Document Engine
│       │   │   ├── pdf.rs             # Merge, Split, Extract
│       │   │   ├── text.rs            # MD->HTML, Syntax Highlight
│       │   │   └── data.rs            # CSV<->JSON, Minification
│       │   │
│       │   ├── archive/               # Archive Engine
│       │   │   ├── manager.rs         # Zip/Tar abstraction
│       │   │   └── stream.rs          # Streaming extraction
│       │   │
│       │   ├── utility/               # System Utilities
│       │   │   ├── fs.rs              # Rename, Dedupe, Watch
│       │   │   └── crypto.rs          # Checksums, Base64
│       │   │
│       │   └── cli/                   # CLI Command Definitions
│       │       └── mod.rs             # Clap subcommands
│       │
│       └── tests/                     # Integration Tests
```

---

## 🛠️ Feature Matrix (The 60 Tools)

We map the requested features to specific, high-performance Rust crates.

### 1. Image Tools (Feature: `image-processing`)
| # | Feature | Crate / Strategy | Performance Note |
|---|---------|------------------|------------------|
| 1 | Format Converter | `image` | Native Rust decoders |
| 2 | Smart Resizer | `image` (Lanczos3) | Parallelized |
| 3 | Compressor | `oxipng`, `image` | Optimized DEFLATE |
| 4 | Watermarker | `imageproc` | Layer composition |
| 5 | Metadata Wiper | `img-parts` | **Zero-copy** header rewrite |
| 6 | QR Generator | `qrcode` | Pure Rust |
| 7 | Palette Extractor | Custom K-Means | Pixel sampling |
| 8 | Filters | `imageproc` | Gauss, Sharpen, Grayscale |
| 9 | Text (OCR) | `ocrs` | Experimental pure Rust |
| 10 | Icon Generator | `image`, `ico` | Multi-size generation |

### 2. Video Tools (Feature: `video-processing`)
| # | Feature | Crate / Strategy | Performance Note |
|---|---------|------------------|------------------|
| 11 | Transcoder | `ffmpeg-next` | Hardware Accel bindings |
| 12 | Audio Extractor | `ffmpeg-next` | Stream copy (no re-encode) |
| 13 | Trimmer | `ffmpeg-next` | Keyframe seeking |
| 14 | GIF Maker | `image`, `gif` | Palette quantization |
| 15 | Thumbnailer | `ffmpeg-next` | I-Frame extraction |
| 16 | Scaler | `ffmpeg-next` | Filter graph |
| 17 | Concatenator | `ffmpeg-next` | Demuxer chaining |
| 18 | Mute | `ffmpeg-next` | Drop audio stream |
| 19 | Inspector | `ffmpeg-next` | Probe API |
| 20 | Subtitle Burner | `ffmpeg-next` | Filter graph overlay |

### 3. Audio Tools (Feature: `audio-processing`)
| # | Feature | Crate / Strategy | Performance Note |
|---|---------|------------------|------------------|
| 21 | Converter | `symphonia`, `hound` | SIMD Decoding |
| 22 | Tag Editor | `lofty` | Zero-copy tag writing |
| 23 | Normalizer | `dasp` | Single-pass RMS |
| 24 | Silence Remover | `symphonia` | Amplitude thresholding |
| 25 | Waveform Gen | `symphonia` + `image` | Downsampling visualization |
| 26 | Merger | `dasp` | Sample interleaving |
| 27 | Speed/Pitch | `rubato` | Async resampling |
| 28 | Mono/Stereo | `dasp` | Channel mixing |
| 29 | Duration Calc | `lofty` | Header parsing only |
| 30 | Analyzer | `spectrum-analyzer` | FFT processing |

### 4. Document Tools (Feature: `doc-processing`)
| # | Feature | Crate / Strategy | Performance Note |
|---|---------|------------------|------------------|
| 31 | PDF Merger | `lopdf` | Object ID remapping |
| 32 | PDF Splitter | `lopdf` | Page tree pruning |
| 33 | MD -> HTML | `pulldown-cmark` | Event-based parser |
| 34 | CSV <-> JSON | `csv`, `serde` | Zero-copy deserialization |
| 35 | PDF Text | `pdf-extract` | Content stream parsing |
| 36 | Img -> PDF | `printpdf` | Image embedding |
| 37 | Word Count | `unicode-segmentation`| Grapheme cluster count |
| 38 | Minifier | `minify-html` | Fast C-bindings |
| 39 | Syntax Highlight| `syntect` | Sublime Text format |
| 40 | Diff Viewer | `similar` | Myers algorithm |

### 5. Archive Tools (Feature: `archive-processing`)
| # | Feature | Crate / Strategy | Performance Note |
|---|---------|------------------|------------------|
| 41 | Extractor | `zip`, `tar`, `flate2`| Streamed extraction |
| 42 | Compressor | `zip`, `walkdir` | Parallel file reading |
| 43 | Encryption | `zip` (AES) | AES-256 support |
| 44 | Tarball | `tar` | POSIX standard |
| 45 | Lister | `zip` | Central Directory read only |
| 46 | Integrity | `crc32fast` | Hardware CRC instructions |
| 47 | Partial Extract | `zip` | Seek and stream |
| 48 | Converter | Pipe logic | Decompress -> Compress pipe |
| 49 | Deduplicator | `sha2` | Content hashing inside archive|
| 50 | Flattener | Path logic | Path rewriting |

### 6. Utility Tools (Feature: `utility-processing`)
| # | Feature | Crate / Strategy | Performance Note |
|---|---------|------------------|------------------|
| 51 | Batch Renamer | `regex` | Compiled regex engine |
| 52 | Duplicate Finder| `sha2`, `walkdir` | **Rayon** parallel hashing |
| 53 | Base64 | `base64` | SIMD encoding |
| 54 | File Watcher | `notify` | OS native events |
| 55 | Checksum | `blake3` | Fastest cryptographic hash |
| 56 | Large File | `walkdir` | Metadata scanning only |
| 57 | Clipboard | `arboard` | Cross-platform |
| 58 | Hex Viewer | `pretty-hex` | Binary view |
| 59 | Shredder | `std::fs` | Multi-pass overwrite |
| 60 | MIME Detect | `infer` | Magic byte checking |

---

## 💻 Implementation Details (Code Plan)

### Phase 1: Dependency Configuration (`Cargo.toml`)

We use a highly granular feature setup to keep build times fast and binary sizes small.

```toml
[package]
name = "dx-media"
version = "0.1.0"
edition = "2024"

[features]
default = ["cli", "image-core", "archive-core", "utility-core"]

# --- Feature Groups ---
cli = ["dep:clap"]
image-core = ["dep:image", "dep:imageproc", "dep:img-parts"]
image-extra = ["image-core", "dep:qrcode", "dep:ocrs", "dep:oxipng"]
video-core = ["dep:ffmpeg-next"]
audio-core = ["dep:symphonia", "dep:hound", "dep:lofty"]
doc-core = ["dep:lopdf", "dep:pulldown-cmark", "dep:csv", "dep:serde", "dep:serde_json"]
archive-core = ["dep:zip", "dep:tar", "dep:flate2"]
utility-core = ["dep:regex", "dep:sha2", "dep:walkdir", "dep:memmap2", "dep:blake3"]

[dependencies]
# Core
anyhow = "1.0"
rayon = "1.8"       # Parallel processing
tracing = "0.1"
memmap2 = { version = "0.9", optional = true }
blake3 = { version = "1.5", optional = true }

# CLI
clap = { version = "4.5", features = ["derive"], optional = true }

# Image
image = { version = "0.24", default-features = false, features = ["png", "jpeg", "webp"], optional = true }
imageproc = { version = "0.23", optional = true }
img-parts = { version = "0.3", optional = true }
qrcode = { version = "0.12", optional = true }
ocrs = { version = "0.1", optional = true } # Experimental
oxipng = { version = "9.0", optional = true }

# Video
ffmpeg-next = { version = "6.1", optional = true }

# Audio
symphonia = { version = "0.5", features = ["mp3", "pcm", "wav"], optional = true }
hound = { version = "3.5", optional = true }
lofty = { version = "0.17", optional = true }

# Document
lopdf = { version = "0.31", optional = true }
pulldown-cmark = { version = "0.9", optional = true }
csv = { version = "1.3", optional = true }
serde = { version = "1.0", features = ["derive"], optional = true }
serde_json = { version = "1.0", optional = true }

# Archive
zip = { version = "0.6", default-features = false, features = ["deflate", "aes-crypto"], optional = true }
tar = { version = "0.4", optional = true }
flate2 = { version = "1.0", optional = true }

# Utility
regex = { version = "1.10", optional = true }
sha2 = { version = "0.10", optional = true }
walkdir = { version = "2.4", optional = true }
infer = "0.15"
```

### Phase 2: Core Infrastructure

**`src/core/pipeline.rs`**: Establish the zero-copy standard.

```rust
use std::path::Path;
use memmap2::Mmap;
use anyhow::Result;

pub struct MediaContext {
    pub input_path: std::path::PathBuf,
    // Memory mapped file for zero-copy reading
    pub source: Option<Mmap>,
}

impl MediaContext {
    pub fn new(path: &Path) -> Result<Self> {
        let file = std::fs::File::open(path)?;
        // Unsafe: We guarantee file isn't modified externally during short process time
        let mmap = unsafe { Mmap::map(&file)? };
        Ok(Self {
            input_path: path.to_path_buf(),
            source: Some(mmap),
        })
    }
}
```

### Phase 3: Image Module Implementation

**`src/image/ops.rs`**: High-performance resizing.

```rust
use image::{DynamicImage, imageops::FilterType};
use anyhow::Result;

pub fn smart_resize(img: &DynamicImage, width: u32, height: u32) -> DynamicImage {
    // Lanczos3 provides best quality/performance ratio for downscaling
    img.resize(width, height, FilterType::Lanczos3)
}

pub fn strip_metadata(data: &[u8]) -> Result<Vec<u8>> {
    use img_parts::jpeg::Jpeg;
    use img_parts::ImageEXIF;

    // Zero-copy parsing of segments
    if let Ok(mut jpeg) = Jpeg::from_bytes(data.into()) {
        jpeg.set_exif(None);
        // Returns the reconstructed bytes without re-encoding the image data
        return Ok(jpeg.encoder().bytes().to_vec());
    }
    // Fallback or other formats...
    Ok(data.to_vec())
}
```

### Phase 4: Audio Module Implementation

**`src/audio/codec.rs`**: Leveraging Symphonia for pure Rust decoding.

```rust
use symphonia::core::io::MediaSourceStream;
use symphonia::core::probe::Hint;
use std::fs::File;

pub fn inspect_audio(path: &std::path::Path) -> anyhow::Result<()> {
    let src = File::open(path)?;
    let mss = MediaSourceStream::new(Box::new(src), Default::default());

    let mut hint = Hint::new();
    if let Some(ext) = path.extension() {
        hint.with_extension(ext.to_str().unwrap());
    }

    // Probe without decoding the whole file
    let probed = symphonia::default::get_probe().format(
        &hint,
        mss,
        &Default::default(),
        &Default::default()
    )?;

    println!("Format: {:?}", probed.format.next_packet()?.track_id());
    Ok(())
}
```

### Phase 5: Utility Module Implementation

**`src/utility/fs.rs`**: Parallel duplicate finder.

```rust
use rayon::prelude::*;
use walkdir::WalkDir;
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub fn find_duplicates(dir: &str) -> Vec<Vec<String>> {
    let files: Vec<_> = WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .collect();

    let hashes = Arc::new(Mutex::new(HashMap::new()));

    // Parallel processing of file hashing
    files.par_iter().for_each(|entry| {
        let mut file = std::fs::File::open(entry.path()).unwrap();
        let mut hasher = Sha256::new();
        std::io::copy(&mut file, &mut hasher).unwrap();
        let hash = format!("{:x}", hasher.finalize());

        let mut map = hashes.lock().unwrap();
        map.entry(hash).or_insert_with(Vec::new).push(entry.path().to_string_lossy().into_owned());
    });

    let map = Arc::try_unwrap(hashes).unwrap().into_inner().unwrap();
    map.into_values().filter(|v| v.len() > 1).collect()
}
```

---

## 🚀 CLI Integration Plan

The `dx` binary will expose these features via the `dx media` subcommand.

```rust
// crates/dx-media/src/cli/mod.rs

use clap::{Subcommand, Parser};

#[derive(Parser)]
pub struct MediaCli {
    #[command(subcommand)]
    pub command: MediaCommand,
}

#[derive(Subcommand)]
pub enum MediaCommand {
    /// Image operations (Resize, Convert, Optimize)
    Image {
        #[arg(short, long)]
        input: String,
        #[arg(long)]
        resize: Option<String>, // "1920x1080"
        #[arg(long)]
        format: Option<String>, // "webp"
        #[arg(long)]
        strip_meta: bool,
    },
    /// Recursive duplicate finder
    Dedup {
        target_dir: String,
    },
    /// PDF Operations
    Pdf {
        #[arg(short, long)]
        merge: Vec<String>,
        #[arg(short, long)]
        output: String,
    }
}
```

## 📅 Roadmap to Completion

1.  **Day 1: Foundation.** Set up `Cargo.toml`, feature flags, and the `core` module (Mmap pipeline).
2.  **Day 2: Image Engine.** Implement the `image` wrapper, `imageproc` filters, and `img-parts` stripper.
3.  **Day 3: Archive & Utils.** Implement Zip/Tar logic and the parallel Deduplicator (easy wins, high value).
4.  **Day 4: Audio.** Integrate `symphonia` for reading and `hound` for writing WAVs.
5.  **Day 5: Documents.** Integrate `lopdf` and Markdown parsers.
6.  **Day 6: Video.** Set up the optional `ffmpeg-next` bindings. *Note: Ensure graceful fallback if FFmpeg is missing.*
7.  **Day 7: CLI Polish.** Wire everything into `dx-cli` with nice progress bars (`indicatif`).

This plan provides a professional, scalable, and highly performant media architecture compatible with the **Dx Binary Dawn** ecosystem.
```
