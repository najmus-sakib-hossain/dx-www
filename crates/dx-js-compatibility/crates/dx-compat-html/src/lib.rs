//! # dx-compat-html
//!
//! HTML Rewriter compatibility layer using lol_html.

#![warn(missing_docs)]

mod element;
mod error;
mod rewriter;

pub use element::{ContentType, Element};
pub use error::{HtmlError, HtmlResult};
pub use rewriter::HTMLRewriter;
