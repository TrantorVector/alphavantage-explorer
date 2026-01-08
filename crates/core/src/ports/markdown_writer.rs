use crate::domain::SchemaTable;
use crate::error::{ExplorerError, Result};
use std::path::Path;

/// Port for writing human-readable markdown reports.
pub trait MarkdownWriter: Send + Sync {
    /// Writes a structured table to the buffer.
    ///
    /// # Errors
    /// Returns `ExplorerError::Io` or `ExplorerError::Validation` if writing fails.
    fn write_table(&mut self, table: &SchemaTable) -> Result<()>;

    /// Writes an error message to the buffer.
    ///
    /// # Errors
    /// Returns `ExplorerError::Io` if writing fails.
    fn write_error(&mut self, error: &ExplorerError) -> Result<()>;

    /// Persists the internal buffer to a file.
    ///
    /// # Errors
    /// Returns `ExplorerError::Io` if the file cannot be created or written to.
    fn flush_to_file(&mut self, path: &Path) -> Result<()>;
}
