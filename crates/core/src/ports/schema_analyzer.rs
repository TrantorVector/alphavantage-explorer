use crate::domain::SchemaTable;
use crate::error::Result;

/// Port for analyzing and diffing schemas (for future use).
pub trait SchemaAnalyzer: Send + Sync {
    /// Computes the difference between two schema tables.
    ///
    /// Returns a textual description of the diff, or None if identical.
    ///
    /// # Errors
    /// Returns `ExplorerError` if analysis fails.
    fn compute_schema_diff(
        &self,
        current: &SchemaTable,
        expected: &SchemaTable,
    ) -> Result<Option<String>>;
}
