# i18n - Rust Internationalization Library

A comprehensive Rust library for translation and text-to-speech, inspired by Python's `deep-translator`, `edge-tts`, and `gTTS`.

## 🌟 Features

### 📝 Translation (Locale Module)
- **Multiple Translation Providers**: Google Translate, Microsoft Translator
- **Language Auto-Detection**: Automatic source language detection
- **Batch Translation**: Translate multiple texts efficiently
- **100+ Languages**: Support for a wide range of languages
- **Async/Await**: Built with modern async Rust

### 🔊 Text-to-Speech (TTS Module)
- **Multiple TTS Engines**: Google TTS and Microsoft Edge TTS
- **High-Quality Audio**: Generate MP3 audio files
- **Custom Voice Control**: Choose from various voices and accents
- **Speech Parameters**: Control rate, pitch, and volume
- **Streaming Support**: Generate audio on-the-fly

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
i18n = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## 🚀 Quick Start

### Translation Example

```rust
use i18n::locale::{GoogleTranslator, Translator};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a translator from English to Spanish
    let translator = GoogleTranslator::new("en", "es")?;
    
    // Translate text
    let result = translator.translate("Hello, world!").await?;
    println!("Translation: {}", result);
    
    Ok(())
}
```

### Text-to-Speech Example

```rust
use i18n::tts::{GoogleTTS, TextToSpeech};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a Google TTS instance
    let tts = GoogleTTS::new("en", "com", false);
    
    // Generate audio file
    tts.save("Hello, world!", Path::new("output.mp3")).await?;
    
    Ok(())
}
```

## 📖 Modules

### Locale Module

The `locale` module provides translation functionality through multiple providers.

#### Google Translator

```rust
use i18n::locale::{GoogleTranslator, Translator};

let translator = GoogleTranslator::new("auto", "fr")?;
let translation = translator.translate("Good morning").await?;
```

**Features:**
- Auto language detection
- 100+ supported languages
- Free to use (rate limited)

#### Microsoft Translator

```rust
use i18n::locale::{MicrosoftTranslator, Translator};

let translator = MicrosoftTranslator::new(
    "en", 
    "de", 
    Some("your-api-key".to_string()),
    None
)?;

let translation = translator.translate("Hello").await?;
```

**Features:**
- High-quality translations
- API key required
- Custom region support

#### Batch Translation

```rust
let texts = vec!["Hello", "Goodbye", "Thank you"];
let results = translator.translate_batch(&texts).await?;
```

### TTS Module

The `tts` module provides text-to-speech functionality.

#### Google TTS

```rust
use i18n::tts::{GoogleTTS, TextToSpeech};

let tts = GoogleTTS::new("en", "com", false);
let audio = tts.synthesize("Welcome!").await?;
```

**Parameters:**
- `lang`: Language code (e.g., "en", "es", "fr")
- `tld`: Top-level domain (e.g., "com", "co.uk")
- `slow`: Speak slowly (true/false)

#### Edge TTS

```rust
use i18n::tts::{EdgeTTS, TextToSpeech, TTSConfig};

// Basic usage
let tts = EdgeTTS::new("en-US-AriaNeural");

// Custom configuration
let config = TTSConfig {
    voice: "en-US-GuyNeural".to_string(),
    rate: "+20%".to_string(),
    volume: "+10%".to_string(),
    pitch: "+5Hz".to_string(),
};
let tts = EdgeTTS::with_config(config);

let audio = tts.synthesize("Hello!").await?;
```

**Features:**
- Multiple voice options
- Custom speech parameters (rate, pitch, volume)
- High-quality neural voices

## 🎯 JSON Examples

### Translating JSON Locale Files

```rust
use i18n::locale::{GoogleTranslator, Translator};
use serde_json::Value;

// Load your locale file
let locale: Value = serde_json::from_str(locale_json)?;

// Translate all text values
let translator = GoogleTranslator::new("en", "es")?;
// ... recursively translate JSON values
```

See `playgrounds/translate_locale.rs` for a complete example.

### Generating Audio from JSON Scripts

```rust
use i18n::tts::{GoogleTTS, TextToSpeech};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Script {
    lines: Vec<ScriptLine>,
}

#[derive(Deserialize)]
struct ScriptLine {
    speaker: String,
    text: String,
}

// Generate audio for each line
for line in script.lines {
    tts.save(&line.text, Path::new(&format!("{}.mp3", line.speaker))).await?;
}
```

See `playgrounds/generate_audio.rs` for a complete example.

## 🎨 Demo Application

Run the included demo to see both translation and TTS in action:

```bash
cargo run --bin i18n-demo
```

This will:
1. Translate JSON messages to Spanish
2. Generate audio files using both Google TTS and Edge TTS
3. Save results and metadata to the `playgrounds/` folder

## 📚 Examples

The `playgrounds/` directory contains several examples:

- **translate_locale.rs**: Translate JSON localization files to multiple languages
- **generate_audio.rs**: Generate audio from JSON scripts with multiple voices
- **translate_and_speak.rs**: Combined workflow - translate and generate audio

Run examples with:

```bash
cd playgrounds
cargo run --example translate_locale
```

## 🌍 Supported Languages

### Translation
The library supports 249 languages through Google Translate including:
- English, Spanish, French, German, Italian, Portuguese
- Chinese (Simplified & Traditional), Japanese, Korean
- Arabic, Hindi, Russian, Turkish
- And many more...

### Text-to-Speech
- **Google TTS**: 249 languages with various accents
- **Edge TTS**: 200+ voices across 70+ languages with neural voice quality

## 🔧 Configuration

### Environment Variables

**Microsoft Translator:**
```bash
export MICROSOFT_API_KEY="your-api-key-here"
```

### Error Handling

The library uses custom error types for better error handling:

```rust
use i18n::error::{I18nError, Result};

match translator.translate("text").await {
    Ok(translation) => println!("{}", translation),
    Err(I18nError::LanguageNotSupported(lang)) => {
        eprintln!("Language {} not supported", lang);
    }
    Err(I18nError::TooManyRequests(msg)) => {
        eprintln!("Rate limited: {}", msg);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## 🏗️ Architecture

The library is organized into two main modules:

```
i18n/
├── locale/          # Translation functionality
│   ├── base.rs      # Translator trait
│   ├── google.rs    # Google Translate implementation
│   ├── microsoft.rs # Microsoft Translator implementation
│   └── constants.rs # Language codes and URLs
│
├── tts/             # Text-to-speech functionality
│   ├── base.rs      # TextToSpeech trait
│   ├── google.rs    # Google TTS implementation
│   ├── edge.rs      # Edge TTS implementation
│   └── constants.rs # TTS configurations
│
└── error.rs         # Error types
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📝 License

This project is inspired by:
- [deep-translator](https://github.com/nidhaloff/deep-translator) - Python translation library
- [edge-tts](https://github.com/rany2/edge-tts) - Microsoft Edge TTS library
- [gTTS](https://github.com/pndurette/gTTS) - Google Text-to-Speech library

## 🙏 Acknowledgments

Special thanks to the creators of the original Python libraries that inspired this project:
- Nidhal Baccouri (deep-translator)
- rany2 (edge-tts)
- Pierre Nicolas Durette (gTTS)

## 📞 Support

For issues, questions, or suggestions, please open an issue on GitHub.

---

Made with ❤️ in Rust

