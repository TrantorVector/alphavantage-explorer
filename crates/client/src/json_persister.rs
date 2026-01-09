use alphavantage_core::error::{ExplorerError, Result};
use alphavantage_core::ports::JsonPersister;
use serde_json::{json, Value};
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};

const MAX_FILE_SIZE_BYTES: usize = 5 * 1024 * 1024; // 5MB

/// Persists raw JSON to the filesystem.
pub struct FileSystemJsonPersister {
    // Optional: track total bytes written?
    bytes_written: AtomicUsize,
}

impl FileSystemJsonPersister {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            bytes_written: AtomicUsize::new(0),
        }
    }
}

impl Default for FileSystemJsonPersister {
    fn default() -> Self {
        Self::new()
    }
}

impl JsonPersister for FileSystemJsonPersister {
    fn save_raw_json(&self, path: &Path, data: &Value) -> Result<()> {
        // Serialize to bytes to check size
        let bytes = serde_json::to_vec_pretty(data).map_err(ExplorerError::Json)?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        if bytes.len() > MAX_FILE_SIZE_BYTES {
            // Write placeholder
            let placeholder = json!({
                "skipped": true,
                "reason": "File size limit exceeded",
                "max_size_bytes": MAX_FILE_SIZE_BYTES,
                "actual_size_bytes": bytes.len()
            });
            let placeholder_bytes =
                serde_json::to_vec_pretty(&placeholder).map_err(ExplorerError::Json)?;
            std::fs::write(path, placeholder_bytes)?;
        } else {
            std::fs::write(path, bytes)?;
        }

        self.bytes_written.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_save_normal_json() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.json");
        let persister = FileSystemJsonPersister::new();
        let data = json!({"foo": "bar"});

        persister.save_raw_json(&path, &data).unwrap();

        let content = std::fs::read_to_string(path).unwrap();
        assert!(content.contains("foo"));
        assert!(content.contains("bar"));
    }

    #[test]
    fn test_size_limit_exceeded() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("large.json");
        let persister = FileSystemJsonPersister::new();

        // Create huge string > 5MB
        let huge_string = "a".repeat(MAX_FILE_SIZE_BYTES + 100);
        let data = json!({"content": huge_string});

        persister.save_raw_json(&path, &data).unwrap();

        let content = std::fs::read_to_string(path).unwrap();
        assert!(content.contains("File size limit exceeded"));
        assert!(!content.contains("aaaaa")); // verify content wasn't written
    }
}
