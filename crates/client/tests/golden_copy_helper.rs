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
    let mut path = get_golden_copy_dir();
    
    // Handle special case: income statement has a typo in the filename
    let filename = if function_name == "income-statement" {
        "inome-statement-output.json"
    } else {
        &format!("{}-output.json", function_name)
    };
    
    path.push(filename);
    
    let content = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read golden copy file: {:?}", path))?;
    
    let json: Value = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse JSON from: {:?}", path))?;
    
    Ok(json)
}

/// Parse input parameters from input.txt file
pub fn parse_input_parameters(function_name: &str) -> Result<TestParameters> {
    let mut path = get_golden_copy_dir();
    path.push(format!("{}-input.txt", function_name));
    
    let content = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read input file: {:?}", path))?;
    
    // Extract symbol from content
    let symbol = if content.contains("symbol=IBM") {
        "IBM".to_string()
    } else if content.contains("symbol=MSFT") {
        "MSFT".to_string()
    } else {
        // Default to IBM if not explicitly specified
        "IBM".to_string()
    };
    
    // Extract quarter if present (only for EARNINGS_CALL_TRANSCRIPT)
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
                        "{}.{}: Missing key in actual response",
                        path, key
                    ));
                }
            }
            
            // Check for extra keys in actual
            for key in actual_obj.keys() {
                if !expected_obj.contains_key(key) {
                    differences.push(format!(
                        "{}.{}: Extra key in actual response",
                        path, key
                    ));
                }
            }
            
            // Compare common keys
            for (key, expected_val) in expected_obj {
                if let Some(actual_val) = actual_obj.get(key) {
                    let new_path = if path.is_empty() {
                        key.to_string()
                    } else {
                        format!("{}.{}", path, key)
                    };
                    compare_json_recursive(actual_val, expected_val, &new_path, differences);
                }
            }
        }
        (Value::Array(actual_arr), Value::Array(expected_arr)) => {
            if actual_arr.len() != expected_arr.len() {
                differences.push(format!(
                    "{}: Array length mismatch - expected {}, got {}",
                    path,
                    expected_arr.len(),
                    actual_arr.len()
                ));
                return;
            }
            
            for (i, (actual_elem, expected_elem)) in
                actual_arr.iter().zip(expected_arr.iter()).enumerate()
            {
                let new_path = format!("{}[{}]", path, i);
                compare_json_recursive(actual_elem, expected_elem, &new_path, differences);
            }
        }
        (actual_val, expected_val) => {
            if actual_val != expected_val {
                differences.push(format!(
                    "{}: Value mismatch - expected {:?}, got {:?}",
                    path, expected_val, actual_val
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
        let params = parse_input_parameters("overview").expect("Failed to parse");
        assert_eq!(params.symbol, "IBM");
        assert!(params.quarter.is_none());
    }

    #[test]
    fn test_parse_input_parameters_with_quarter() {
        let params = parse_input_parameters("earnings-call-transcript").expect("Failed to parse");
        assert_eq!(params.symbol, "IBM");
        assert_eq!(params.quarter, Some("2024Q1".to_string()));
    }

    #[test]
    fn test_parse_input_parameters_msft() {
        let params = parse_input_parameters("shares-outstanding").expect("Failed to parse");
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
