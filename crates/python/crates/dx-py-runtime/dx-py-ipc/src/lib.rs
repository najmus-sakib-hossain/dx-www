//! DX-Py IPC - Binary Protocol IPC (HBTP for Python)
//!
//! High-performance binary protocol for inter-process communication
//! with zero-copy array transfer via shared memory.

pub mod protocol;
pub mod shared_memory;
pub mod channel;

pub use protocol::{HbtpHeader, MessageType, HbtpFlags};
pub use shared_memory::{SharedMemoryArena, SharedArrayHandle};
pub use channel::HbtpChannel;
