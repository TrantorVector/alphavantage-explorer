use alphavantage_core::error::Result;
use std::fs;
use std::path::Path;

/// Handler for CSV response data - provides raw CSV passthrough without conversion
pub struct CsvHandler;

impl CsvHandler {
    /// Saves raw CSV content to the specified path
    ///
    /// # Errors
    /// Returns error if file creation or writing fails
    pub fn save_raw(content: &str, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, content)?;
        Ok(())
    }

    /// Detects if content-type indicates CSV format
    ///
    /// Checks for "text/csv" and "application/csv" MIME types
    #[must_use]
    pub fn detect_csv(content_type: &str) -> bool {
        content_type.contains("text/csv") || content_type.contains("application/csv")
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_csv_detection() {
        assert!(CsvHandler::detect_csv("text/csv"));
        assert!(CsvHandler::detect_csv("application/csv"));
        assert!(CsvHandler::detect_csv("text/csv; charset=utf-8"));
        assert!(!CsvHandler::detect_csv("application/json"));
        assert!(!CsvHandler::detect_csv("text/plain"));
    }

    #[test]
    fn test_csv_save() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.csv");

        let csv_content = "Symbol,Price,Volume\nAAPL,150.00,1000000\nNVDA,500.00,2000000";

        CsvHandler::save_raw(csv_content, &file_path).unwrap();

        assert!(file_path.exists());
        let saved_content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(saved_content, csv_content);
    }

    #[test]
    fn test_csv_save_creates_subdirs() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("subdir/nested/test.csv");

        CsvHandler::save_raw("A,B,C", &file_path).unwrap();

        assert!(file_path.exists());
    }
}
