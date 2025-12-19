//! HMR server implementation.

use crate::error::HmrResult;
use std::path::Path;

/// HMR server.
pub struct HmrServer {
    _root: std::path::PathBuf,
}

impl HmrServer {
    /// Create a new HMR server.
    pub fn new(root: impl AsRef<Path>) -> HmrResult<Self> {
        Ok(Self {
            _root: root.as_ref().to_path_buf(),
        })
    }

    /// Start watching for changes.
    pub async fn start(&mut self) -> HmrResult<()> {
        // TODO: Implement file watching
        Ok(())
    }
}
