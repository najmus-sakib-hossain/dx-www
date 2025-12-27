//! Validation modules for different aspects of crate standards

mod metadata;
mod naming;
mod documentation;
mod license;
mod structure;
mod dependency;

pub use metadata::*;
pub use naming::*;
pub use documentation::*;
pub use license::*;
pub use structure::*;
pub use dependency::*;
