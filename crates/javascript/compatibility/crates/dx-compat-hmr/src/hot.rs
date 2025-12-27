//! import.meta.hot API implementation.

use std::collections::HashMap;

/// Hot module API.
pub struct HotModule {
    /// Preserved data between updates
    pub data: HashMap<String, String>,
}

impl HotModule {
    /// Create a new hot module.
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// Accept updates for this module.
    pub fn accept<F: Fn()>(&self, _callback: F) {
        // TODO: Implement
    }

    /// Dispose callback before replacement.
    pub fn dispose<F: Fn(&mut HashMap<String, String>)>(&self, _callback: F) {
        // TODO: Implement
    }

    /// Decline updates (force full reload).
    pub fn decline(&self) {
        // TODO: Implement
    }

    /// Invalidate this module.
    pub fn invalidate(&self) {
        // TODO: Implement
    }
}

impl Default for HotModule {
    fn default() -> Self {
        Self::new()
    }
}
