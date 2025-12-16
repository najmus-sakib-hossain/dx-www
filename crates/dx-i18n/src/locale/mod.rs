//! Translation module
//!
//! This module provides translation functionality through multiple providers.

pub mod base;
pub mod google;
pub mod microsoft;
pub mod constants;

pub use base::Translator;
pub use google::GoogleTranslator;
pub use microsoft::MicrosoftTranslator;
