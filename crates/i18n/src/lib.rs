//! # i18n - Internationalization Library
//!
//! A Rust library for translation and text-to-speech, inspired by deep-translator, edge-tts, and gTTS.
//!
//! ## Modules
//! - `locale`: Translation functionality supporting multiple providers
//! - `tts`: Text-to-speech functionality supporting Microsoft Edge TTS and Google TTS

pub mod locale;
pub mod tts;
pub mod error;

pub use error::{I18nError, Result};
