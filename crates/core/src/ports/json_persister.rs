use crate::error::Result;
use serde_json::Value;
use std::path::Path;

/// Port for saving raw JSON responses for debugging/archival.
pub trait JsonPersister: Send + Sync {
    /// Saves a raw JSON value to a file.
    ///
    /// # Errors
    /// Returns `ExplorerError::Io` if the file cannot be created or written to.
    fn save_raw_json(&self, path: &Path, data: &Value) -> Result<()>;
}
