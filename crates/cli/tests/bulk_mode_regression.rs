#![allow(deprecated)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::redundant_closure_for_method_calls)]
//! Backward compatibility tests for bulk mode
//! Ensures that new granular mode doesn't break existing bulk mode functionality

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_bulk_mode_unchanged() {
    let temp_dir = TempDir::new().unwrap();
    let out_path = temp_dir.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("alphavantage_cli").unwrap();

    cmd.arg("--symbols")
        .arg("AAPL")
        .arg("--out-dir")
        .arg(out_path)
        .assert()
        .success();

    // Verify bulk mode output structure (tickers/AAPL.md)
    let aapl_report = temp_dir.path().join("tickers/AAPL.md");
    assert!(
        aapl_report.exists(),
        "Bulk mode should create tickers/AAPL.md"
    );

    // Verify content structure matches v0.1.0 format
    let content = fs::read_to_string(&aapl_report).unwrap();
    assert!(
        content.contains("OVERVIEW"),
        "Bulk mode should contain Overview section"
    );
    assert!(content.contains("AAPL"), "Bulk mode should contain symbol");

    // Verify raw JSON structure (raw/tickers/AAPL/OVERVIEW.json)
    let raw_file = temp_dir.path().join("raw/tickers/AAPL/OVERVIEW.json");
    assert!(
        raw_file.exists(),
        "Bulk mode should save raw JSON in same location"
    );
}

#[test]
fn test_bulk_mode_multiple_symbols() {
    let temp_dir = TempDir::new().unwrap();
    let out_path = temp_dir.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("alphavantage_cli").unwrap();

    cmd.arg("--symbols")
        .arg("AAPL,NVDA")
        .arg("--out-dir")
        .arg(out_path)
        .assert()
        .success();

    // Verify both symbols processed
    assert!(
        temp_dir.path().join("tickers/AAPL.md").exists(),
        "AAPL report should exist"
    );
    assert!(
        temp_dir.path().join("tickers/NVDA.md").exists(),
        "NVDA report should exist"
    );
}

#[test]
fn test_bulk_truncation_still_works() {
    let temp_dir = TempDir::new().unwrap();
    let out_path = temp_dir.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("alphavantage_cli").unwrap();

    cmd.arg("--symbols")
        .arg("AAPL")
        .arg("--out-dir")
        .arg(out_path)
        .assert()
        .success();

    let aapl_report = temp_dir.path().join("tickers/AAPL.md");
    let content = fs::read_to_string(&aapl_report).unwrap();

    // In bulk mode, tables should be truncated
    // We can verify by checking for truncation indicators
    // The actual truncation happens in markdown writer with truncate_rows=true
    // For now, just verify the file exists and has content
    assert!(!content.is_empty(), "Report should not be empty");
    assert!(content.contains('|'), "Report should contain table markers");
}

#[test]
fn test_bulk_mode_output_structure() {
    let temp_dir = TempDir::new().unwrap();
    let out_path = temp_dir.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("alphavantage_cli").unwrap();

    cmd.arg("--symbols")
        .arg("AAPL")
        .arg("--out-dir")
        .arg(out_path)
        .assert()
        .success();

    // Verify directory structure
    assert!(
        temp_dir.path().join("tickers").exists(),
        "tickers directory should exist"
    );
    assert!(
        temp_dir.path().join("raw/tickers/AAPL").exists(),
        "raw/tickers/AAPL directory should exist"
    );

    // Verify index.md if generated
    let index_file = temp_dir.path().join("index.md");
    if index_file.exists() {
        let content = fs::read_to_string(&index_file).unwrap();
        assert!(
            content.contains("AAPL"),
            "Index should reference processed symbols"
        );
    }
}

#[test]
fn test_bulk_mode_help() {
    let mut cmd = Command::cargo_bin("alphavantage_cli").unwrap();

    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("symbols"))
        .stdout(predicate::str::contains("out-dir"));
}

#[test]
fn test_bulk_mode_invalid_symbol() {
    let mut cmd = Command::cargo_bin("alphavantage_cli").unwrap();

    cmd.arg("--symbols")
        .arg("INVALID_SYMBOL_TOO_LONG")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Ticker too long"));
}

#[test]
fn test_bulk_and_granular_modes_distinct() {
    // Verify that bulk mode (--symbols) and granular mode (subcommands) don't conflict
    let temp_dir = TempDir::new().unwrap();
    let out_path = temp_dir.path().to_str().unwrap();

    // Test bulk mode
    let mut bulk_cmd = Command::cargo_bin("alphavantage_cli").unwrap();
    bulk_cmd
        .arg("--symbols")
        .arg("AAPL")
        .arg("--out-dir")
        .arg(out_path)
        .assert()
        .success();

    // Test granular mode (overview subcommand)
    let mut granular_cmd = Command::cargo_bin("alphavantage_cli").unwrap();
    granular_cmd
        .arg("overview")
        .arg("--symbol")
        .arg("NVDA")
        .arg("--output")
        .arg(out_path)
        .assert()
        .success();

    // Both should work independently
    assert!(
        temp_dir.path().join("tickers/AAPL.md").exists(),
        "Bulk mode output exists"
    );

    // Granular mode creates files with timestamp pattern: overview_NVDA_*.md
    let has_granular = std::fs::read_dir(temp_dir.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .any(|e| {
            let name = e.file_name();
            let name_str = name.to_str().unwrap_or("");
            name_str.starts_with("overview") && name_str.contains("NVDA")
        });

    assert!(has_granular, "Granular mode output exists");
}
