#![allow(clippy::unwrap_used)]
#![allow(clippy::redundant_clone)]
#![allow(clippy::panic)]
//! Unit tests for granular executor

use alphavantage_cli::cli_args::Commands;
use alphavantage_core::domain::TickerSymbol;
use alphavantage_core::util::generate_timestamp;
use std::path::PathBuf;

#[test]
fn test_endpoint_routing_overview() {
    let symbol = TickerSymbol::new("AAPL").unwrap();
    let cmd = Commands::Overview {
        symbol: symbol.clone(),
        output: None,
    };

    // Test that command can be created and pattern matched
    match cmd {
        Commands::Overview { symbol: s, .. } => {
            assert_eq!(s.to_string(), "AAPL");
        }
        _ => panic!("Expected Overview command"),
    }
}

#[test]
fn test_endpoint_routing_income_statement() {
    let symbol = TickerSymbol::new("NVDA").unwrap();
    let cmd = Commands::IncomeStatement {
        symbol: symbol.clone(),
        output: Some(PathBuf::from("./test_output")),
    };

    match cmd {
        Commands::IncomeStatement { symbol: s, output } => {
            assert_eq!(s.to_string(), "NVDA");
            assert_eq!(output, Some(PathBuf::from("./test_output")));
        }
        _ => panic!("Expected IncomeStatement command"),
    }
}

#[test]
fn test_endpoint_routing_balance_sheet() {
    let symbol = TickerSymbol::new("MSFT").unwrap();
    let cmd = Commands::BalanceSheet {
        symbol,
        output: None,
    };

    match cmd {
        Commands::BalanceSheet { .. } => (),
        _ => panic!("Expected BalanceSheet command"),
    }
}

#[test]
fn test_endpoint_routing_cash_flow() {
    let symbol = TickerSymbol::new("GOOGL").unwrap();
    let cmd = Commands::CashFlow {
        symbol,
        output: None,
    };

    match cmd {
        Commands::CashFlow { .. } => (),
        _ => panic!("Expected CashFlow command"),
    }
}

#[test]
fn test_endpoint_routing_earnings() {
    let symbol = TickerSymbol::new("TSLA").unwrap();
    let cmd = Commands::Earnings {
        symbol,
        output: None,
    };

    match cmd {
        Commands::Earnings { .. } => (),
        _ => panic!("Expected Earnings command"),
    }
}

#[test]
fn test_endpoint_routing_news_sentiment() {
    let symbol = TickerSymbol::new("AAPL").unwrap();
    let cmd = Commands::NewsSentiment {
        symbol: symbol.clone(),
        limit: 50,
        output: None,
    };

    match cmd {
        Commands::NewsSentiment {
            symbol: s,
            limit: l,
            ..
        } => {
            assert_eq!(s.to_string(), "AAPL");
            assert_eq!(l, 50);
        }
        _ => panic!("Expected NewsSentiment command"),
    }
}

#[test]
fn test_endpoint_routing_earnings_calendar() {
    use alphavantage_core::domain::HorizonParam;

    let symbol = TickerSymbol::new("AAPL").unwrap();
    let cmd = Commands::EarningsCalendar {
        symbol: symbol.clone(),
        horizon: Some(HorizonParam::ThreeMonth),
        output: None,
    };

    match cmd {
        Commands::EarningsCalendar {
            symbol: s, horizon, ..
        } => {
            assert_eq!(s.to_string(), "AAPL");
            assert_eq!(horizon, Some(HorizonParam::ThreeMonth));
        }
        _ => panic!("Expected EarningsCalendar command"),
    }
}

#[test]
fn test_endpoint_routing_earnings_call_transcript() {
    use alphavantage_core::domain::QuarterParam;

    let symbol = TickerSymbol::new("AAPL").unwrap();
    let cmd = Commands::EarningsCallTranscript {
        symbol: symbol.clone(),
        year: Some(2024),
        quarter: Some(QuarterParam::Q1),
        output: None,
    };

    match cmd {
        Commands::EarningsCallTranscript {
            symbol: s,
            year,
            quarter,
            ..
        } => {
            assert_eq!(s.to_string(), "AAPL");
            assert_eq!(year, Some(2024));
            assert_eq!(quarter, Some(QuarterParam::Q1));
        }
        _ => panic!("Expected EarningsCallTranscript command"),
    }
}

#[test]
fn test_filename_generation() {
    // Test that timestamps are in the expected format
    let timestamp = generate_timestamp();

    // Timestamp should be YYYYMMDD_HHMMSS format (15 characters)
    assert_eq!(timestamp.len(), 15);

    // Should contain an underscore separator
    assert!(timestamp.contains('_'));

    // First 8 characters should be numeric (YYYYMMDD)
    let date_part = &timestamp[..8];
    assert!(date_part.chars().all(|c| c.is_ascii_digit()));

    // Last 6 characters should be numeric (HHMMSS)
    let time_part = &timestamp[9..];
    assert!(time_part.chars().all(|c| c.is_ascii_digit()));
}

#[test]
fn test_output_path_creation() {
    use std::fs;
    use tempfile::TempDir;

    // Create a temporary directory
    let temp_dir = TempDir::new().unwrap();
    let test_path = temp_dir.path().join("test_output");

    // Test that we can create the output directory
    fs::create_dir_all(&test_path).unwrap();
    assert!(test_path.exists());
    assert!(test_path.is_dir());

    // Test creating nested directories
    let nested_path = test_path.join("raw");
    fs::create_dir_all(&nested_path).unwrap();
    assert!(nested_path.exists());
    assert!(nested_path.is_dir());
}

#[test]
fn test_all_endpoint_commands_exist() {
    // This test ensures all 13 endpoint commands are defined
    let symbol = TickerSymbol::new("AAPL").unwrap();

    let commands = vec![
        Commands::Overview {
            symbol: symbol.clone(),
            output: None,
        },
        Commands::IncomeStatement {
            symbol: symbol.clone(),
            output: None,
        },
        Commands::BalanceSheet {
            symbol: symbol.clone(),
            output: None,
        },
        Commands::CashFlow {
            symbol: symbol.clone(),
            output: None,
        },
        Commands::Earnings {
            symbol: symbol.clone(),
            output: None,
        },
        Commands::EarningsEstimates {
            symbol: symbol.clone(),
            output: None,
        },
        Commands::NewsSentiment {
            symbol: symbol.clone(),
            limit: 50,
            output: None,
        },
        Commands::InsiderTransactions {
            symbol: symbol.clone(),
            output: None,
        },
        Commands::Dividends {
            symbol: symbol.clone(),
            output: None,
        },
        Commands::Splits {
            symbol: symbol.clone(),
            output: None,
        },
        Commands::SharesOutstanding {
            symbol: symbol.clone(),
            output: None,
        },
        Commands::EarningsCalendar {
            symbol: symbol.clone(),
            horizon: None,
            output: None,
        },
        Commands::EarningsCallTranscript {
            symbol: symbol.clone(),
            year: None,
            quarter: None,
            output: None,
        },
    ];

    // Verify we have exactly 13 commands
    assert_eq!(commands.len(), 13);
}
