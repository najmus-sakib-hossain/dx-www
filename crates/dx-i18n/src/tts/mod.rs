//! Text-to-Speech module
//!
//! This module provides TTS functionality through multiple providers.

pub mod base;
pub mod google;
pub mod edge;
pub mod constants;

pub use base::TextToSpeech;
pub use google::GoogleTTS;
pub use edge::EdgeTTS;
