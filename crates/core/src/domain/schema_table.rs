use serde::{Deserialize, Serialize};

/// Represents a validated, tabular view of API response data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaTable {
    /// Human-readable title
    pub title: String,
    /// Ordered list of column headers
    pub headers: Vec<String>,
    /// Rows of data, where each row is a list of strings
    pub rows: Vec<Vec<String>>,
    /// Total number of records (which might differ from `rows.len()` if paginated)
    pub total_records: usize,
}

impl SchemaTable {
    pub fn new(title: impl Into<String>, headers: Vec<String>, rows: Vec<Vec<String>>) -> Self {
        let rows_len = rows.len();
        Self {
            title: title.into(),
            headers,
            rows,
            total_records: rows_len,
        }
    }
}
