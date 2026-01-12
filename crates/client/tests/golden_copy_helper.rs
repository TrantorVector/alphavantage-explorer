// Helper module for golden copy validation tests
use anyhow::{Context, Result};
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

/// Parameters extracted from input files for API calls
#[derive(Debug, Clone)]
pub struct TestParameters {
    pub symbol: String,
    pub quarter: Option<String>,
    #[allow(dead_code)]
    pub datatype: Option<String>,
}

/// Result of JSON comparison
#[derive(Debug)]
pub struct ComparisonResult {
    pub matches: bool,
    pub differences: Vec<String>,
}

/// Load golden copy JSON output file
pub fn load_golden_copy(function_name: &str) -> Result<Value> {
    let filename = if function_name == "income-statement" {
        "inome-statement-output.json"
    } else {
        &format!("{function_name}-output.json")
    };

    let path = PathBuf::from(format!("../../docs/golden-copy/{filename}"));

    let content = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read golden copy file: {:?}", path))?;

    let json: Value = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse JSON from: {:?}", path))?;

    Ok(json)
}

/// Parse input parameters from input.txt file
pub fn parse_input_parameters(function_name: &str) -> Result<TestParameters> {
    let input_path = PathBuf::from(format!("../../docs/golden-copy/{function_name}-input.txt"));

    let content = fs::read_to_string(&input_path)
        .with_context(|| format!("Failed to read input file: {:?}", input_path))?;

    // Extract symbol from content
    let symbol = content
        .lines()
        .find(|line| line.starts_with("symbol="))
        .map(|line| line.split('=').nth(1).unwrap_or_default().to_string())
        .unwrap_or_else(|| "IBM".to_string());

    // Extract quarter if present (only for EARNINGS_CALL_TRANSCRIPT)
    let quarter = content
        .lines()
        .find(|line| line.starts_with("quarter="))
        .map(|line| line.split('=').nth(1).unwrap_or_default().to_string());

    // Check if datatype is mentioned (optional parameter)
    let datatype = content
        .lines()
        .find(|line| line.starts_with("datatype="))
        .map(|line| line.split('=').nth(1).unwrap_or_default().to_string());

    Ok(TestParameters {
        symbol,
        quarter,
        datatype,
    })
}

/// Compare two JSON values for exact equality
pub fn compare_json_exact(actual: &Value, expected: &Value) -> ComparisonResult {
    let mut differences = Vec::new();
    compare_json_recursive(actual, expected, "", &mut differences);

    ComparisonResult {
        matches: differences.is_empty(),
        differences,
    }
}

/// Recursive JSON comparison helper
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

            // Compare common keys
            for (key, expected_val) in expected_obj {
                if let Some(actual_val) = actual_obj.get(key) {
                    let key_path = if path.is_empty() {
                        key.to_string()
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
                return;
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
                    "{path}: Value mismatch - expected {:?}, got {:?}",
                    expected_val, actual_val
                ));
            }
        }
    }
}

/// Get the path to the golden-copy directory
fn get_golden_copy_dir() -> PathBuf {
    // Navigate from crates/client/tests to docs/golden-copy
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop(); // Remove client
    path.pop(); // Remove crates
    path.push("docs");
    path.push("golden-copy");
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_golden_copy() {
        // Test loading a known golden copy file
        let result = load_golden_copy("overview");
        assert!(result.is_ok(), "Failed to load overview golden copy");
    }

    #[test]
    fn test_parse_input_parameters_basic() {
        let params = parse_input_parameters("overview").unwrap();
        assert_eq!(params.symbol, "IBM");
        assert!(params.quarter.is_none());
    }

    #[test]
    fn test_parse_input_parameters_with_quarter() {
        let params = parse_input_parameters("earnings-call-transcript").unwrap();
        assert_eq!(params.symbol, "IBM");
        assert_eq!(params.quarter, Some("2024Q1".to_string()));
    }

    #[test]
    fn test_parse_input_parameters_msft() {
        let params = parse_input_parameters("shares-outstanding").unwrap();
        assert_eq!(params.symbol, "MSFT");
    }

    #[test]
    fn test_compare_json_exact_equal() {
        let json1 = serde_json::json!({"a": 1, "b": "test"});
        let json2 = serde_json::json!({"a": 1, "b": "test"});

        let result = compare_json_exact(&json1, &json2);
        assert!(result.matches);
        assert!(result.differences.is_empty());
    }

    #[test]
    fn test_compare_json_exact_different() {
        let json1 = serde_json::json!({"a": 1, "b": "test"});
        let json2 = serde_json::json!({"a": 2, "b": "test"});

        let result = compare_json_exact(&json1, &json2);
        assert!(!result.matches);
        assert!(!result.differences.is_empty());
    }
}
