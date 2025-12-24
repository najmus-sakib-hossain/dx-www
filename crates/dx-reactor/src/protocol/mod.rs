//! HBTP (Hyper-Binary Transfer Protocol) implementation.
//!
//! This module provides the binary protocol for dx-www client-server communication,
//! replacing HTTP with 8-byte headers and O(1) routing.

mod hbtp;
mod response;

pub use hbtp::{HbtpOpcode, HbtpHeader, HbtpFlags, HbtpProtocol, HbtpError};
pub use response::ResponseBuffer;
