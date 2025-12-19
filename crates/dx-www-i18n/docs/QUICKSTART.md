# Quick Start Guide

## Installation & Setup

1. **Prerequisites:**
   - Rust toolchain installed (https://rustup.rs/)
   - For Microsoft Translator: API key from Azure

2. **Clone and build:**
   ```bash
   cd f:/Code/i18n
   cargo build
   ```

## Running the Demo

```bash
# Run the main demo application
cargo run --bin i18n-demo

# This will:
# - Translate sample messages to Spanish
# - Generate audio files with both Google TTS and Edge TTS
# - Save JSON output files
```

## Quick Examples

### 1. Simple Translation

Create a file `examples/simple_translate.rs`:

```rust
use i18n::locale::{GoogleTranslator, Translator};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let translator = GoogleTranslator::new("en", "es")?;
    let result = translator.translate("Hello, world!").await?;
    println!("Spanish: {}", result);
    Ok(())
}
```

Run it:
```bash
cargo run --example simple_translate
```

### 2. Simple TTS

Create a file `examples/simple_tts.rs`:

```rust
use i18n::tts::{GoogleTTS, TextToSpeech};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tts = GoogleTTS::new("en", "com", false);
    tts.save("Hello, world!", Path::new("hello.mp3")).await?;
    println!("Audio saved to hello.mp3");
    Ok(())
}
```

Run it:
```bash
cargo run --example simple_tts
```

## Playground Examples

The `playgrounds/` directory has ready-to-run examples:

```bash
# Translate JSON localization files
cargo run --bin translate_locale

# Generate audio from JSON scripts
cargo run --bin generate_audio

# Combined workflow: translate and generate audio
cargo run --bin translate_and_speak
```

Note: To run playground examples, you may need to add them as binaries in Cargo.toml:

```toml
[[bin]]
name = "translate_locale"
path = "playgrounds/translate_locale.rs"
```

## Next Steps

1. Read the [README.md](README.md) for full API documentation
2. Check [ARCHITECTURE.md](ARCHITECTURE.md) for design details
3. Browse the source code in `src/` to understand implementation
4. Modify the playground examples for your use case

## Troubleshooting

**Problem:** `cargo: command not found`
- **Solution:** Install Rust from https://rustup.rs/

**Problem:** Rate limiting errors with Google Translate
- **Solution:** Add delays between requests or use batch translation

**Problem:** WebSocket errors with Edge TTS
- **Solution:** Check internet connection and firewall settings

**Problem:** Microsoft Translator API key error
- **Solution:** Set environment variable: `export MICROSOFT_API_KEY="your-key"`

## Support

For issues or questions, refer to the documentation or check the source code comments.
