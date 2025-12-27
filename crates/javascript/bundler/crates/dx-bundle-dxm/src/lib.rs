//! DX Module Binary Format (.dxm)
//! 
//! Pre-compiled binary representation of JavaScript modules.
//! Zero-parse bundling through binary fusion.

mod format;
mod atomizer;
mod fusion;

pub use format::*;
pub use atomizer::*;
pub use fusion::*;
