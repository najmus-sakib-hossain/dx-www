//! # dx-compat-sqlite
//!
//! Built-in SQLite database compatibility layer.

#![warn(missing_docs)]

mod database;
mod error;
mod statement;

pub use database::Database;
pub use error::{SqliteError, SqliteResult};
pub use statement::PreparedStatement;
