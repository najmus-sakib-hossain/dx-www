//! DX-Py GC - Lock-Free Parallel Garbage Collector
//!
//! This crate implements a lock-free garbage collector with:
//! - Atomic reference counting for immediate reclamation
//! - Epoch-based reclamation for safe memory deallocation
//! - Concurrent cycle detection without stop-the-world pauses
//! - Sub-100μs maximum pause times

pub mod refcount;
pub mod epoch;
pub mod cycle;

pub use refcount::LockFreeRefCount;
pub use epoch::EpochGc;
pub use cycle::CycleDetector;

/// GC configuration
#[derive(Debug, Clone)]
pub struct GcConfig {
    /// Number of epochs to keep before reclaiming
    pub epoch_count: usize,
    /// Maximum objects per garbage list before triggering collection
    pub max_garbage_per_epoch: usize,
    /// Enable cycle detection
    pub enable_cycle_detection: bool,
}

impl Default for GcConfig {
    fn default() -> Self {
        Self {
            epoch_count: 3,
            max_garbage_per_epoch: 10000,
            enable_cycle_detection: true,
        }
    }
}
