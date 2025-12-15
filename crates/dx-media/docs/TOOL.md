Here is the list with **File Watcher** replaced by a **Line Ending Fixer** (essential for cross-platform development), keeping everything else exactly as requested:

### 1. Image Tools
**Primary Crate:** `image` (The de-facto standard for basic manipulation).

1.  **Format Converter:** Convert between PNG, JPEG, WEBP, GIF, and ICO.
    *   *Crates:* `image`
2.  **Smart Resizer:** Resize images maintaining aspect ratio or forcing dimensions.
    *   *Crates:* `image` (specifically `image::imageops::resize`)
3.  **Image Compressor:** Reduce file size with quality loss control.
    *   *Crates:* `image`, `turbojpeg` (for speed), or `oxipng` (for PNG optimization).
4.  **Watermarker:** Overlay text or logos onto images transparently.
    *   *Crates:* `imageproc` (drawing text/shapes), `rusttype` (font handling).
5.  **Metadata/EXIF Wiper:** Remove GPS and camera data for privacy.
    *   *Crates:* `kamadak-exif` or `img-parts`.
6.  **QR Code Generator/Reader:** Generate QRs from text or read them from images.
    *   *Crates:* `qrcode`, `rxing` (port of ZXing).
7.  **Color Palette Extractor:** Analyze an image and output the dominant Hex colors (great for frontend devs).
    *   *Crates:* `color_thief` or `kmeans`.
8.  **Grayscale/Filter Applier:** Apply visual effects (blur, grayscale, contrast).
    *   *Crates:* `imageproc`.
9.  **Text-to-Image (OCR):** Extract text from screenshots or scanned documents.
    *   *Crates:* `tesseract-plumbing` (bindings to Tesseract) or `ocrs` (pure Rust OCR, experimental but promising).
10. **Icon Generator:** Generate `favicon.ico` and Apple Touch Icons from a single image.
    *   *Crates:* `image`, `ico`.

---

### 2. Video Tools
**Note on Video in Rust:** Pure Rust video encoding is still maturing. The industry standard is using bindings to **FFmpeg**.

1.  **Format Transcoder:** Convert MKV to MP4, WebM, etc.
    *   *Crates:* `ffmpeg-next` (safe bindings for FFmpeg).
2.  **Audio Extractor:** Strip audio tracks from video files (save as MP3/WAV).
    *   *Crates:* `ffmpeg-next`.
3.  **Video Trimmer:** Cut video from timestamp A to B without re-encoding (stream copy).
    *   *Crates:* `ffmpeg-next` or calling `std::process::Command` on a system ffmpeg.
4.  **GIF Maker:** Convert short video clips into optimized GIFs.
    *   *Crates:* `gif`, `image` (frame processing).
5.  **Thumbnail Generator:** Extract a frame at specific timestamp to use as a cover.
    *   *Crates:* `ffmpeg-next`.
6.  **Resolution Scaler:** Downscale 4K to 1080p/720p to save space.
    *   *Crates:* `ffmpeg-next`.
7.  **Video Concatenator:** Stitch multiple video clips together.
    *   *Crates:* `gstreamer` or `ffmpeg-next`.
8.  **Mute Video:** Remove the audio stream entirely.
    *   *Crates:* `mp4` (if working specifically with ISO/MP4 files) or `ffmpeg-next`.
9.  **Metadata Inspector:** View codec, bitrate, and frame rate info.
    *   *Crates:* `ffprobe` (wrapper) or `mediainfo`.
10. **Subtitle Burner:** Hardcode subtitles (.srt) into the video stream.
    *   *Crates:* `ffmpeg-next`.

---

### 3. Audio Tools
**Primary Crates:** `symphonia` (decoding) and `rodio` (playback/processing).

1.  **Audio Converter:** Convert lossless (WAV/FLAC) to lossy (MP3/OGG).
    *   *Crates:* `symphonia` (read), `hound` (WAV write), `lame-sys` (MP3 write).
2.  **Tag Editor (ID3):** Edit Title, Artist, and Album Art metadata.
    *   *Crates:* `lofty` (Excellent metadata library).
3.  **Volume Normalizer:** Boost quiet audio or normalize peaks.
    *   *Crates:* `dasp` (Digital Audio Signal Processing).
4.  **Silence Remover:** Detect and trim silence from start/end of clips.
    *   *Crates:* `rodio` or raw sample analysis with `symphonia`.
5.  **Waveform Visualizer:** Generate a PNG waveform representation of the audio (cool for UI).
    *   *Crates:* `waveform`.
6.  **Audio Merger:** Join multiple audio tracks into one.
    *   *Crates:* `dasp` (interleaving samples).
7.  **Speed/Pitch Changer:** Speed up playback (podcast style) or change pitch.
    *   *Crates:* `rubato` (resampling).
8.  **Mono/Stereo Switcher:** Force audio channels to stereo or mono.
    *   *Crates:* `dasp`.
9.  **Duration Calculator:** accurate length check for a folder of audio files.
    *   *Crates:* `lofty`.
10. **Frequency Analyzer:** Perform FFT to analyze audio spectrum.
    *   *Crates:* `spectrum-analyzer`.

---

### 4. Document Tools
**Primary Focus:** PDF, Markdown, and Text manipulation.

1.  **PDF Merger:** Combine multiple PDFs into a single file.
    *   *Crates:* `lopdf`.
2.  **PDF Splitter:** Extract specific pages from a PDF.
    *   *Crates:* `lopdf`.
3.  **Markdown to HTML/PDF:** Convert readme/docs to viewable formats.
    *   *Crates:* `pulldown-cmark` (MD parsing), `headless_chrome` (for PDF generation via print).
4.  **CSV <-> JSON Converter:** Essential for data handling.
    *   *Crates:* `csv`, `serde`, `serde_json`.
5.  **PDF Text Extractor:** Rip text content from PDFs.
    *   *Crates:* `pdf-extract`.
6.  **Image to PDF:** Combine a folder of images into a PDF document.
    *   *Crates:* `printpdf` or `genpdf`.
7.  **Word Counter/Analyzer:** Count words, chars, and lines in text files.
    *   *Crates:* Standard library (`std::fs`) + `unicode-segmentation`.
8.  **Minifier:** Minify JSON, XML, or HTML files.
    *   *Crates:* `minify-html`, `json-minify`.
9.  **Syntax Highlighter:** Convert code files into images or HTML with syntax colors.
    *   *Crates:* `syntect` (The standard used by Bat).
10. **Diff Viewer:** Compare two text files and show differences.
    *   *Crates:* `similar`.

---

### 5. Archives (Compression)
**Primary Crates:** `flate2`, `zip`, `tar`.

1.  **Universal Extractor:** Auto-detect archive type (zip, tar, gz) and extract.
    *   *Crates:* `compress-tools` (handles multiple formats).
2.  **Batch Compressor:** Compress individual folders into separate zip files.
    *   *Crates:* `zip`, `walkdir`.
3.  **Encrypted Zip Creator:** Create password-protected archives.
    *   *Crates:* `zip` (supports AES encryption).
4.  **Tarball Creator:** Create `.tar.gz` files (Linux standard).
    *   *Crates:* `tar`, `flate2`.
5.  **Archive Lister:** List contents of a huge zip without extracting it.
    *   *Crates:* `zip`.
6.  **Integrity Checker:** Verify CRC/Checksums of archived files.
    *   *Crates:* `zip`, `crc32fast`.
7.  **Partial Extractor:** Extract only one specific file from a massive archive.
    *   *Crates:* `zip` or `tar`.
8.  **Archive Converter:** Convert `.tar.gz` to `.zip`.
    *   *Crates:* Combine `tar` and `zip`.
9.  **Deduplicator:** Find duplicate files inside an archive before extracting.
    *   *Crates:* `sha2` (hashing streams).
10. **Recursion Flattener:** Extract files but remove the internal folder structure.
    *   *Crates:* Standard logic with `std::path`.

---

### 6. Others (System & Utility)
Tools that help manage the filesystem and workflow.

1.  **Batch Renamer:** Rename files using Regex (e.g., `IMG_001.jpg` -> `2023-01.jpg`).
    *   *Crates:* `regex`, `rename`.
2.  **Duplicate File Finder:** Find identical files based on content hashing (SHA256).
    *   *Crates:* `sha2`, `walkdir`.
3.  **Base64 Encoder/Decoder:** Quickly encode files/text for web usage.
    *   *Crates:* `base64`.
4.  **Line Ending Fixer:** Batch convert CRLF (Windows) to LF (Linux) to fix git issues.
    *   *Crates:* `newline-converter`.
5.  **Checksum Validator:** Generate/Verify MD5 or SHA hashes for downloaded files.
    *   *Crates:* `digest`, `md-5`, `sha2`.
6.  **Large File Finder:** Visualize what is taking up space in a directory.
    *   *Crates:* `walkdir`, `filesize`.
7.  **Clipboard Manager:** Copy file content or image directly to system clipboard.
    *   *Crates:* `arboard` (Cross-platform clipboard).
8.  **Hex Viewer:** View the raw binary data of any file.
    *   *Crates:* `hexdump` or `pretty-hex`.
9.  **Secure Delete (Shredder):** Overwrite a file multiple times before deleting it.
    *   *Crates:* Standard `fs` write operations looped.
10. **MIME Type Detector:** Detect real file type by "magic numbers" (not just extension).
    *   *Crates:* `infer`.

### Architecture Tip for DX-Media
Since you are building a CLI (`dx-cli`) and an Extension (`dx-extension`):

1.  **Core Library:** Create a Rust crate `dx-core` that contains all the logic above.
2.  **CLI:** `dx-cli` imports `dx-core` and uses `clap` for the command line arguments.
3.  **Extension:** For VS Code, you can either:
    *   Spawn the `dx-cli` binary as a subprocess (Easier).
    *   Compile `dx-core` to **WebAssembly (WASM)** using `wasm-bindgen` (Faster, but some crates like `ffmpeg` won't work in WASM).
