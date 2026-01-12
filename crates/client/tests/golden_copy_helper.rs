// Golden Copy Helper Functions
// Utilities for loading golden copy files and comparing JSON outputs

use anyhow::{Context, Result};
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct TestParameters {
    pub symbol: String,
    pub quarter: Option<String>,
    #[allow(dead_code)]
    pub datatype: Option<String>,
}

#[derive(Debug)]
pub struct ComparisonResult {
    pub matches: bool,
    pub differences: Vec<String>,
}

/// Load golden copy JSON output file
///
/// # Errors
/// Returns an error if the file cannot be read or parsed as JSON
pub fn load_golden_copy(function_name: &str) -> Result<Value> {
    let filename = if function_name == "income-statement" {
        "inome-statement-output.json"
    } else {
        &format!("{function_name}-output.json")
    };

    let path = PathBuf::from(format!("../../docs/golden-copy/{filename}"));

    let content = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read golden copy file: {}", path.display()))?;

    serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse JSON from: {}", path.display()))
}

/// Parse input parameters from input.txt file
///
/// # Errors
/// Returns an error if the input file cannot be read
pub fn parse_input_parameters(function_name: &str) -> Result<TestParameters> {
    let input_path = PathBuf::from(format!("../../docs/golden-copy/{function_name}-input.txt"));

    let content = fs::read_to_string(&input_path)
        .with_context(|| format!("Failed to read input file: {}", input_path.display()))?;

    // Extract symbol from example in content (e.g., "For example: symbol=IBM" or "symbol=MSFT")
    let symbol = if content.contains("symbol=MSFT") {
        "MSFT".to_string()
    } else if content.contains("symbol=IBM") {
        "IBM".to_string()
    } else {
        // Default to IBM if not explicitly specified
        "IBM".to_string()
    };

    // Extract quarter if present (e.g., "For example: quarter=2024Q1")
    let quarter = if content.contains("quarter=2024Q1") {
        Some("2024Q1".to_string())
    } else {
        None
    };

    // Check if datatype is mentioned (optional parameter)
    let datatype = if content.contains("datatype=json") {
        Some("json".to_string())
    } else {
        None
    };

    Ok(TestParameters {
        symbol,
        quarter,
        datatype,
    })
}

/// Compare two JSON values for exact equality
///
/// Returns a comparison result with match status and list of differences
#[must_use]
pub fn compare_json_exact(actual: &Value, expected: &Value) -> ComparisonResult {
    let mut differences = Vec::new();
    compare_json_recursive(actual, expected, "", &mut differences);

    ComparisonResult {
        matches: differences.is_empty(),
        differences,
    }
}

fn compare_json_recursive(
    actual: &Value,
    expected: &Value,
    path: &str,
    differences: &mut Vec<String>,
) {
    match (actual, expected) {
        (Value::Object(actual_obj), Value::Object(expected_obj)) => {
            // Check for missing keys in actual
            for key in expected_obj.keys() {
                if !actual_obj.contains_key(key) {
                    differences.push(format!(
                        "{path}: Missing key in actual - expected key '{key}'"
                    ));
                }
            }

            // Check for extra keys in actual
            for key in actual_obj.keys() {
                if !expected_obj.contains_key(key) {
                    differences.push(format!(
                        "{path}: Extra key in actual - unexpected key '{key}'"
                    ));
                }
            }

            // Recursively compare matching keys
            for (key, expected_val) in expected_obj {
                if let Some(actual_val) = actual_obj.get(key) {
                    let key_path = if path.is_empty() {
                        key.clone()
                    } else {
                        format!("{path}.{key}")
                    };
                    compare_json_recursive(actual_val, expected_val, &key_path, differences);
                }
            }
        }
        (Value::Array(actual_arr), Value::Array(expected_arr)) => {
            if actual_arr.len() != expected_arr.len() {
                differences.push(format!(
                    "{path}: Array length mismatch - expected {}, got {}",
                    expected_arr.len(),
                    actual_arr.len()
                ));
            }

            for (i, (actual_elem, expected_elem)) in
                actual_arr.iter().zip(expected_arr.iter()).enumerate()
            {
                let array_path = format!("{path}[{i}]");
                compare_json_recursive(actual_elem, expected_elem, &array_path, differences);
            }
        }
        (actual_val, expected_val) => {
            if actual_val != expected_val {
                differences.push(format!(
                    "{path}: Value mismatch - expected {expected_val:?}, got {actual_val:?}"
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_json_exact_match() {
        let json1 = serde_json::json!({"key": "value"});
        let json2 = serde_json::json!({"key": "value"});
        let result = compare_json_exact(&json1, &json2);
        assert!(result.matches);
        assert!(result.differences.is_empty());
    }

    #[test]
    fn test_compare_json_mismatch() {
        let json1 = serde_json::json!({"key": "value1"});
        let json2 = serde_json::json!({"key": "value2"});
        let result = compare_json_exact(&json1, &json2);
        assert!(!result.matches);
        assert_eq!(result.differences.len(), 1);
    }

    #[test]
    fn test_parse_input_parameters_basic() {
        if let Ok(params) = parse_input_parameters("overview") {
            assert_eq!(params.symbol, "IBM");
            assert!(params.quarter.is_none());
        }
    }

    #[test]
    fn test_parse_input_parameters_with_quarter() {
        if let Ok(params) = parse_input_parameters("earnings-call-transcript") {
            assert_eq!(params.symbol, "IBM");
            assert_eq!(params.quarter, Some("2024Q1".to_string()));
        }
    }

    #[test]
    fn test_parse_input_parameters_msft() {
        if let Ok(params) = parse_input_parameters("shares-outstanding") {
            assert_eq!(params.symbol, "MSFT");
        }
    }
}
