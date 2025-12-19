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

// Allow dead_code for API completeness
#![allow(dead_code)]
#![allow(unused_variables)]

pub mod cdn;
pub mod cli;
pub mod config;
pub mod download;
pub mod models;
pub mod providers;
pub mod search;

pub use cdn::{CdnProvider, CdnUrlGenerator, FontCdnUrls};
pub use download::FontDownloader;
pub use models::{Font, FontFamily, FontProvider, FontStyle, FontWeight};
pub use search::FontSearch;
