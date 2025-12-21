//! # dx-reactor
//!
//! Binary Dawn - Cross-platform I/O reactor with thread-per-core architecture.
//!
//! This crate provides a unified I/O abstraction layer that automatically selects
//! the best platform-specific backend:
//!
//! - **Linux 5.1+**: io_uring with SQPOLL for zero-syscall I/O
//! - **Linux (older)**: epoll fallback
//! - **macOS/BSD**: kqueue
//! - **Windows**: IOCP (I/O Completion Ports)
//!
//! ## Features
//!
//! - Thread-per-core architecture with CPU pinning
//! - Zero-copy I/O operations
//! - HBTP binary protocol support
//! - Memory teleportation for WASM interop

pub mod io;
pub mod protocol;
pub mod memory;
pub mod middleware;

mod reactor;
mod core_state;

pub use reactor::{DxReactor, ReactorBuilder, WorkerStrategy, IoBackend};
pub use core_state::CoreState;
pub use io::{Reactor, ReactorConfig, Completion, Interest, IoHandle, PlatformReactor};

/// Re-export commonly used types
pub mod prelude {
    pub use crate::{
        DxReactor, ReactorBuilder, WorkerStrategy, IoBackend,
        Reactor, ReactorConfig, Completion, Interest,
    };
}
