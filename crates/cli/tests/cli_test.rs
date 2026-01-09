use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn test_mock_mode_run() {
    let mut cmd = Command::cargo_bin("alphavantage_cli").unwrap();
    
    // We use a temp dir for output to avoid clutter
    let temp_dir = tempfile::tempdir().unwrap();
    let out_path = temp_dir.path().to_str().unwrap();

    cmd.arg("--symbols").arg("AAPL")
       .arg("--out-dir").arg(out_path)
       // Implicitly uses mock mode if no API key/live flag, but let's be sure invalid key doesn't break mock
       // Actually Config requires key OR mock mode. 
       // If --live-api is NOT set, we are in Mock mode.
       // In Mock mode, key is optional (defaults to "mock_key").
       .assert()
       .success();

    // Verify output files exist
    let aapl_report = temp_dir.path().join("tickers/AAPL.md");
    assert!(aapl_report.exists(), "AAPL.md should be created");
    
    // Verify content
    let content = fs::read_to_string(aapl_report).unwrap();
    assert!(content.contains("OVERVIEW"), "Report should contain Overview section");
    assert!(content.contains("AAPL"), "Report should contain symbol");
    
    // Verify Raw JSON (default enabled)
    let raw_file = temp_dir.path().join("raw/tickers/AAPL/OVERVIEW.json");
    assert!(raw_file.exists(), "Raw JSON should be saved");
}

#[test]
fn test_help() {
    let mut cmd = Command::cargo_bin("alphavantage_cli").unwrap();
    cmd.arg("--help")
       .assert()
       .success()
       .stdout(predicate::str::contains("Usage:"));
}

#[test]
fn test_invalid_symbol() {
    let mut cmd = Command::cargo_bin("alphavantage_cli").unwrap();
    cmd.arg("--symbols").arg("INVALID_SYMBOL_TOO_LONG")
       .assert()
       .failure()
       .stderr(predicate::str::contains("Ticker too long"));
}
