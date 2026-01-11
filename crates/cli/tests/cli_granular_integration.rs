#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]
#![allow(clippy::option_if_let_else)]
//! Integration tests for granular mode

use alphavantage_cli::cli_args::{Commands, LogFormat};
use alphavantage_cli::config::Config;
use alphavantage_cli::granular_executor::GranularExecutor;
use alphavantage_client::{ClientMode, MockClient};
use alphavantage_core::domain::{ApiKey, TickerSymbol};
use std::path::PathBuf;
use tempfile::TempDir;
use tracing::Level;

/// Helper function to create a test config with temporary output directory
fn create_test_config(temp_dir: &TempDir) -> Config {
    let api_key = ApiKey::new("demo".to_string());
    let out_dir = PathBuf::from(temp_dir.path());

    Config {
        symbols: vec![],
        api_key,
        out_dir,
        client_mode: ClientMode::Mock,
        save_raw: true,
        daily_limit: 25,
        min_delay_ms: 0,
        log_format: LogFormat::Human,
        log_level: Level::INFO,
    }
}

#[tokio::test]
async fn test_granular_json_workflow() {
    let temp_dir = TempDir::new().unwrap();
    let config = create_test_config(&temp_dir);
    let client = MockClient::new();

    let executor = GranularExecutor::new(&config, &client);

    let symbol = TickerSymbol::new("AAPL").unwrap();
    let cmd = Commands::Overview {
        symbol,
        output: None,
    };

    // Execute the command
    let result = executor.execute(&cmd).await;

    // Should succeed
    assert!(
        result.is_ok(),
        "Granular execution failed: {:?}",
        result.err()
    );

    // Verify output files exist (raw JSON and markdown)
    let entries: Vec<_> = std::fs::read_dir(temp_dir.path()).unwrap().collect();
    assert!(!entries.is_empty(), "No output files generated");

    // Check for markdown file
    let has_markdown = entries.iter().any(|entry| {
        if let Ok(entry) = entry {
            entry.path().extension().and_then(|ext| ext.to_str()) == Some("md")
        } else {
            false
        }
    });

    assert!(has_markdown, "Markdown file not generated");
}

#[tokio::test]
async fn test_granular_multiple_endpoints() {
    let temp_dir = TempDir::new().unwrap();
    let config = create_test_config(&temp_dir);
    let client = MockClient::new();

    let executor = GranularExecutor::new(&config, &client);
    let symbol = TickerSymbol::new("NVDA").unwrap();

    // Test multiple different endpoints
    let commands = vec![
        Commands::Overview {
            symbol: symbol.clone(),
            output: None,
        },
        Commands::IncomeStatement {
            symbol: symbol.clone(),
            output: None,
        },
        Commands::Earnings {
            symbol: symbol.clone(),
            output: None,
        },
    ];

    for cmd in commands {
        let result = executor.execute(&cmd).await;
        assert!(result.is_ok(), "Command failed: {:?}", result.err());
    }

    // Verify multiple markdown files were created
    let md_count = std::fs::read_dir(temp_dir.path())
        .unwrap()
        .filter_map(std::result::Result::ok)
        .filter(|e| e.path().extension().and_then(|ext| ext.to_str()) == Some("md"))
        .count();

    assert!(md_count >= 3, "Expected at least 3 markdown files");
}

#[tokio::test]
async fn test_granular_custom_output_directory() {
    let temp_dir = TempDir::new().unwrap();
    let custom_output = temp_dir.path().join("custom_reports");

    let config = create_test_config(&temp_dir);
    let client = MockClient::new();

    let executor = GranularExecutor::new(&config, &client);
    let symbol = TickerSymbol::new("AAPL").unwrap();

    let cmd = Commands::Overview {
        symbol,
        output: Some(custom_output.clone()),
    };

    let result = executor.execute(&cmd).await;
    assert!(result.is_ok());

    // Verify custom output directory was used
    assert!(
        custom_output.exists(),
        "Custom output directory not created"
    );
}

#[tokio::test]
async fn test_granular_news_sentiment_with_limit() {
    let temp_dir = TempDir::new().unwrap();
    let config = create_test_config(&temp_dir);
    let client = MockClient::new();

    let executor = GranularExecutor::new(&config, &client);
    let symbol = TickerSymbol::new("AAPL").unwrap();

    let cmd = Commands::NewsSentiment {
        symbol,
        limit: 50, // Changed from Some(50) to 50
        output: None,
    };

    let result = executor.execute(&cmd).await;
    assert!(
        result.is_ok(),
        "NewsSentiment with limit failed: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn test_granular_earnings_calendar_with_horizon() {
    use alphavantage_core::domain::HorizonParam;

    let temp_dir = TempDir::new().unwrap();
    let config = create_test_config(&temp_dir);
    let client = MockClient::new();

    let executor = GranularExecutor::new(&config, &client);
    let symbol = TickerSymbol::new("AAPL").unwrap();

    let cmd = Commands::EarningsCalendar {
        symbol,
        horizon: Some(HorizonParam::ThreeMonth),
        output: None,
    };

    let result = executor.execute(&cmd).await;
    assert!(
        result.is_ok(),
        "EarningsCalendar with horizon failed: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn test_granular_earnings_transcript_with_params() {
    use alphavantage_core::domain::QuarterParam;

    let temp_dir = TempDir::new().unwrap();
    let config = create_test_config(&temp_dir);
    let client = MockClient::new();

    let executor = GranularExecutor::new(&config, &client);
    let symbol = TickerSymbol::new("AAPL").unwrap();

    let cmd = Commands::EarningsCallTranscript {
        symbol,
        year: 2024,
        quarter: QuarterParam::Q1,
        output: None,
    };

    let result = executor.execute(&cmd).await;
    assert!(
        result.is_ok(),
        "EarningsCallTranscript with params failed: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn test_full_output_retention() {
    let temp_dir = TempDir::new().unwrap();
    let config = create_test_config(&temp_dir);
    let client = MockClient::new();

    let executor = GranularExecutor::new(&config, &client);
    let symbol = TickerSymbol::new("AAPL").unwrap();

    let cmd = Commands::Earnings {
        symbol,
        output: None,
    };

    let result = executor.execute(&cmd).await;
    assert!(result.is_ok());

    // Find the generated markdown file
    let md_files: Vec<_> = std::fs::read_dir(temp_dir.path())
        .unwrap()
        .filter_map(std::result::Result::ok)
        .filter(|e| e.path().extension().and_then(|ext| ext.to_str()) == Some("md"))
        .collect();

    assert!(!md_files.is_empty(), "No markdown files generated");

    // Read the markdown content to verify it's not truncated
    let md_file = md_files.first().expect("should have at least one file");
    let content = std::fs::read_to_string(md_file.path()).unwrap();

    // The mock data should have multiple rows
    // Verify content is not empty
    assert!(!content.is_empty(), "Markdown file is empty");

    // Should have table markers
    assert!(content.contains('|'), "Markdown doesn't contain table");
}
