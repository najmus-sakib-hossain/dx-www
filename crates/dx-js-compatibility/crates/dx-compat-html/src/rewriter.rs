//! HTML Rewriter implementation.

use crate::error::HtmlResult;

/// HTML Rewriter.
pub struct HTMLRewriter {
    _handlers: Vec<()>,
}

impl HTMLRewriter {
    /// Create a new HTML rewriter.
    pub fn new() -> Self {
        Self {
            _handlers: Vec::new(),
        }
    }

    /// Register an element handler.
    pub fn on(&mut self, _selector: &str, _handler: impl Fn()) -> &mut Self {
        // TODO: Implement
        self
    }

    /// Register a document handler.
    pub fn on_document(&mut self, _handler: impl Fn()) -> &mut Self {
        // TODO: Implement
        self
    }

    /// Transform HTML content.
    pub fn transform(&self, _html: &str) -> HtmlResult<String> {
        // TODO: Implement using lol_html
        Ok(String::new())
    }
}

impl Default for HTMLRewriter {
    fn default() -> Self {
        Self::new()
    }
}
