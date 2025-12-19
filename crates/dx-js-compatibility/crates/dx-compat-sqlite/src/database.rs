//! SQLite database implementation.

use crate::error::{SqliteError, SqliteResult};
use std::path::Path;

/// SQLite database connection.
pub struct Database {
    _conn: rusqlite::Connection,
}

impl Database {
    /// Open or create a database at the given path.
    pub fn new(path: impl AsRef<Path>) -> SqliteResult<Self> {
        let conn = rusqlite::Connection::open(path)?;
        // Enable WAL mode for performance
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA synchronous=NORMAL;")?;
        Ok(Self { _conn: conn })
    }

    /// Open an in-memory database.
    pub fn memory() -> SqliteResult<Self> {
        let conn = rusqlite::Connection::open_in_memory()?;
        Ok(Self { _conn: conn })
    }

    /// Execute SQL without returning results.
    pub fn exec(&self, _sql: &str) -> SqliteResult<()> {
        // TODO: Implement
        Ok(())
    }
}
