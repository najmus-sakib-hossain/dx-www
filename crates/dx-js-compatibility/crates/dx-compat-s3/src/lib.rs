//! # dx-compat-s3
//!
//! S3-compatible object storage compatibility layer.

#![warn(missing_docs)]

mod client;
mod error;
mod file;

pub use client::{S3Client, S3Config};
pub use error::{S3Error, S3Result};
pub use file::S3File;
