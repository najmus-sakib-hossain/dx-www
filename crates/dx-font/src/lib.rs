//! dx-font - A comprehensive font search and download tool
//! 
//! Access 50k+ commercial-free fonts from 100+ sources including:
//! - Google Fonts (1,562 fonts)
//! - Bunny Fonts (1,478 fonts)
//! - Fontsource (1,562 fonts)
//! - Font Squirrel (1,082 fonts)
//! - And many more!
//!
//! ## Features
//! - Blazing fast parallel search across all providers
//! - Concurrent downloads with progress indication
//! - CDN URL generation for font preview and usage
//! - Support for multiple font formats (TTF, OTF, WOFF, WOFF2)

pub mod providers;
pub mod models;
pub mod search;
pub mod download;
pub mod cli;
pub mod config;
pub mod cdn;

pub use models::{Font, FontFamily, FontProvider, FontWeight, FontStyle};
pub use search::FontSearch;
pub use download::FontDownloader;
pub use cdn::{CdnUrlGenerator, FontCdnUrls, CdnProvider};
