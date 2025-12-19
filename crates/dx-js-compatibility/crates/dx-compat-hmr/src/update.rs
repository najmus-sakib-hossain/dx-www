//! HMR update types.

/// HMR update message.
#[derive(Debug, Clone)]
pub struct HmrUpdate {
    /// File path
    pub path: String,
    /// Content hash
    pub hash: String,
    /// Update type
    pub update_type: UpdateType,
}

/// Type of HMR update.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateType {
    /// JavaScript update
    Js,
    /// CSS update
    Css,
    /// Full page reload required
    FullReload,
}
