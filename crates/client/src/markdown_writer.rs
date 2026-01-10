use alphavantage_core::domain::SchemaTable;
use alphavantage_core::error::{ExplorerError, Result};
use alphavantage_core::ports::MarkdownWriter;
use std::fmt::Write;
use std::path::Path;

/// Implementation of `MarkdownWriter` that builds a report in memory
/// before flushing to disk.
pub struct MarkdownWriterImpl {
    buffer: String,
    #[allow(dead_code)] // Will be used in Phase 11 granular mode
    truncate_rows: bool,
}

impl MarkdownWriterImpl {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            buffer: String::new(),
            truncate_rows: true, // Default to truncation for backward compatibility
        }
    }

    /// Create a writer with specific truncation behavior
    #[must_use]
    pub const fn with_truncation(truncate: bool) -> Self {
        Self {
            buffer: String::new(),
            truncate_rows: truncate,
        }
    }

    /// Helper to write a heading.
    pub fn write_heading(&mut self, text: &str, level: usize) {
        let hashes = "#".repeat(level);
        writeln!(self.buffer, "{hashes} {text}\n").ok();
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.buffer
    }
}

impl Default for MarkdownWriterImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl MarkdownWriter for MarkdownWriterImpl {
    fn write_table(&mut self, table: &SchemaTable) -> Result<()> {
        // 1. Write Title
        self.write_heading(&table.title, 2);

        if table.headers.is_empty() {
            writeln!(self.buffer, "_No data available._\n")
                .map_err(|_| ExplorerError::Io(std::io::Error::other("fmt write error")))?;
            return Ok(());
        }

        // 2. Connector for headers
        // | Header 1 | Header 2 |
        let header_line = format!("| {} |", table.headers.join(" | "));
        writeln!(self.buffer, "{header_line}")
            .map_err(|_| ExplorerError::Io(std::io::Error::other("fmt write error")))?;

        // 3. Separator
        // |---|---|
        let separators: Vec<&str> = table.headers.iter().map(|_| "---").collect();
        let separator_line = format!("| {} |", separators.join(" | "));
        writeln!(self.buffer, "{separator_line}")
            .map_err(|_| ExplorerError::Io(std::io::Error::other("fmt write error")))?;

        // 4. Rows
        for row in &table.rows {
            // Basic escaping of pipes might be needed, but sticking to simple strings for now
            let row_line = format!("| {} |", row.join(" | "));
            writeln!(self.buffer, "{row_line}")
                .map_err(|_| ExplorerError::Io(std::io::Error::other("fmt write error")))?;
        }

        writeln!(self.buffer).ok(); // Blank line after table

        // 5. Truncation Note
        if table.total_records > table.rows.len() {
            writeln!(
                self.buffer,
                "_(Showing {} of {} total records)_\n",
                table.rows.len(),
                table.total_records
            )
            .map_err(|_| ExplorerError::Io(std::io::Error::other("fmt write error")))?;
        }

        Ok(())
    }

    fn write_error(&mut self, error: &ExplorerError) -> Result<()> {
        writeln!(self.buffer, "> **Error**\n> {error}\n")
            .map_err(|_| ExplorerError::Io(std::io::Error::other("fmt write error")))?;
        Ok(())
    }

    fn flush_to_file(&mut self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, &self.buffer)?;
        Ok(())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_write_table_formatting() {
        let mut writer = MarkdownWriterImpl::new();
        let table = SchemaTable {
            title: "TEST TABLE".to_string(),
            headers: vec!["Col1".to_string(), "Col2".to_string()],
            rows: vec![
                vec!["Val1".to_string(), "Val2".to_string()],
                vec!["A".to_string(), "B".to_string()],
            ],
            total_records: 2,
        };

        writer.write_table(&table).unwrap();

        insta::assert_snapshot!(writer.buffer, @r###"
        ## TEST TABLE

        | Col1 | Col2 |
        | --- | --- |
        | Val1 | Val2 |
        | A | B |

        "###);
    }

    #[test]
    fn test_truncation_note() {
        let mut writer = MarkdownWriterImpl::new();
        let table = SchemaTable {
            title: "Big Table".to_string(),
            headers: vec!["H".to_string()],
            rows: vec![vec!["1".to_string()]],
            total_records: 100, // much larger than rows
        };

        writer.write_table(&table).unwrap();
        assert!(writer.buffer.contains("Showing 1 of 100 total records"));
    }

    #[test]
    fn test_flush_to_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("subdir/report.md");

        let mut writer = MarkdownWriterImpl::new();
        writer.write_heading("Report", 1);
        writer.flush_to_file(&file_path).unwrap();

        assert!(file_path.exists());
        let content = std::fs::read_to_string(file_path).unwrap();
        assert_eq!(content, "# Report\n\n");
    }
}
