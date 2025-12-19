//! # dx-compat-hmr
//!
//! Hot Module Replacement compatibility layer.

#![warn(missing_docs)]

mod error;
mod server;
mod update;

pub use error::{HmrError, HmrResult};
pub use server::HmrServer;
pub use update::{HmrUpdate, UpdateType};

/// import.meta.hot API.
pub mod hot;
