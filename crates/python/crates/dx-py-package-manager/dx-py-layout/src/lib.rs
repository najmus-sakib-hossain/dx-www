//! dx-py-layout: O(1) virtual environment layout cache for DX-Py package manager
//!
//! This crate provides a layout cache that enables instant virtual environment
//! setup through pre-built layouts indexed by project hash.

mod error;
mod headers;
mod index;
mod cache;

pub use error::{LayoutError, LayoutResult};
pub use headers::{LayoutEntry, LayoutIndexHeader};
pub use index::LayoutIndex;
pub use cache::{LayoutCache, ResolvedPackage};

/// Magic number for DX-Py Layout Cache format
pub const DXLC_MAGIC: &[u8; 4] = b"DXLC";

/// Current layout cache format version
pub const LAYOUT_VERSION: u16 = 1;
