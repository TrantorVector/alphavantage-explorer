#![allow(clippy::unwrap_used)]
#![allow(clippy::format_push_string)]
//! CSV handler integration tests

use alphavantage_client::CsvHandler;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_csv_detection() {
    // Test various Content-Type headers
    assert!(CsvHandler::detect_csv("text/csv"));
    assert!(CsvHandler::detect_csv("application/csv"));
    assert!(CsvHandler::detect_csv("text/csv; charset=utf-8"));
    assert!(CsvHandler::detect_csv("text/csv;charset=UTF-8"));

    // Should not detect JSON
    assert!(!CsvHandler::detect_csv("application/json"));
    assert!(!CsvHandler::detect_csv("application/json; charset=utf-8"));

    // Should not detect plain text
    assert!(!CsvHandler::detect_csv("text/plain"));
    assert!(!CsvHandler::detect_csv("text/html"));
}

#[test]
fn test_csv_save() {
    let temp_dir = TempDir::new().unwrap();
    let csv_path = temp_dir.path().join("test_output.csv");

    let csv_content = "Symbol,Price,Volume\nAAPL,150.00,1000000\nNVDA,500.00,2000000";

    let result = CsvHandler::save_raw(csv_content, &csv_path);
    assert!(result.is_ok(), "CSV save failed: {:?}", result.err());

    // Verify file exists
    assert!(csv_path.exists(), "CSV file was not created");

    // Verify content is correct
    let saved_content = fs::read_to_string(&csv_path).unwrap();
    assert_eq!(saved_content, csv_content, "CSV content doesn't match");
}

#[test]
fn test_csv_no_conversion() {
    let temp_dir = TempDir::new().unwrap();
    let csv_path = temp_dir.path().join("raw_data.csv");

    // CSV content with special characters and various data types
    let csv_content = r#"Symbol,Name,Price,Change %,Volume
AAPL,Apple Inc.,150.50,+2.5%,50000000
GOOGL,"Alphabet, Inc.",2800.00,-0.8%,1000000
MSFT,Microsoft Corp,300.25,+1.2%,30000000"#;

    CsvHandler::save_raw(csv_content, &csv_path).unwrap();

    // Read back and verify NO transformations occurred
    let saved_content = fs::read_to_string(&csv_path).unwrap();
    assert_eq!(
        saved_content, csv_content,
        "CSV should be saved exactly as-is"
    );

    // Verify no markdown file was created
    let md_path = temp_dir.path().join("raw_data.md");
    assert!(!md_path.exists(), "Markdown file should not be created");
}

#[test]
fn test_csv_large_file() {
    let temp_dir = TempDir::new().unwrap();
    let csv_path = temp_dir.path().join("large_data.csv");

    // Create a large CSV with many rows
    let mut csv_content = String::from("Symbol,Price,Volume\n");
    for i in 0..1000 {
        csv_content.push_str(&format!("SYM{i},100.{i},1000000\n"));
    }

    CsvHandler::save_raw(&csv_content, &csv_path).unwrap();

    // Verify file exists and has correct size
    assert!(csv_path.exists());
    let saved_content = fs::read_to_string(&csv_path).unwrap();

    // Count lines (header + 1000 data rows)
    let line_count = saved_content.lines().count();
    assert_eq!(line_count, 1001, "Should have 1001 lines");
}

#[test]
fn test_csv_save_creates_directories() {
    let temp_dir = TempDir::new().unwrap();
    let nested_path = temp_dir.path().join("raw/data/subdir/output.csv");

    let csv_content = "A,B,C\n1,2,3";

    CsvHandler::save_raw(csv_content, &nested_path).unwrap();

    assert!(nested_path.exists(), "Nested directories should be created");

    // Verify all parent directories were created
    assert!(temp_dir.path().join("raw").exists());
    assert!(temp_dir.path().join("raw/data").exists());
    assert!(temp_dir.path().join("raw/data/subdir").exists());
}

#[test]
fn test_csv_empty_content() {
    let temp_dir = TempDir::new().unwrap();
    let csv_path = temp_dir.path().join("empty.csv");

    let csv_content = "";

    let result = CsvHandler::save_raw(csv_content, &csv_path);
    assert!(result.is_ok());
    assert!(csv_path.exists());

    let saved_content = fs::read_to_string(&csv_path).unwrap();
    assert_eq!(saved_content, "");
}

#[test]
fn test_csv_unicode_content() {
    let temp_dir = TempDir::new().unwrap();
    let csv_path = temp_dir.path().join("unicode.csv");

    let csv_content = "Symbol,Name,Country\nAPPL,Apple,🇺🇸\nTOY,Toyota,🇯🇵\nSAP,SAP,🇩🇪";

    CsvHandler::save_raw(csv_content, &csv_path).unwrap();

    let saved_content = fs::read_to_string(&csv_path).unwrap();
    assert_eq!(saved_content, csv_content);
    assert!(saved_content.contains("🇺🇸"));
}

#[test]
fn test_csv_special_characters() {
    let temp_dir = TempDir::new().unwrap();
    let csv_path = temp_dir.path().join("special.csv");

    // CSV with quotes, commas in values, newlines in quoted fields
    let csv_content = r#"Symbol,Description
AAPL,"Apple, Inc."
MSFT,"Microsoft
Corporation"
TEST,"Value with ""quotes"""#;

    CsvHandler::save_raw(csv_content, &csv_path).unwrap();

    let saved_content = fs::read_to_string(&csv_path).unwrap();
    assert_eq!(saved_content, csv_content);
}
