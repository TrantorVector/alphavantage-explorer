// Golden Copy Validation Tests
// These tests validate API responses against golden copy outputs from Alpha Vantage documentation
//
// IMPORTANT: These tests are marked with #[ignore] to prevent accidental execution
// Run with: cargo test --test golden_copy_validation -- --ignored --nocapture
//
// RATE LIMITING: Tests include 12-second delays between calls to respect API rate limits

use alphavantage_client::http_client::AlphaVantageClient;
use alphavantage_core::domain::{ApiKey, EndpointName, TickerSymbol};
use alphavantage_core::ports::ApiClient;
use anyhow::Result;

use std::collections::HashMap;
use std::env;
use std::time::Duration;
use tokio::time::sleep;

mod golden_copy_helper;
use golden_copy_helper::{compare_json_exact, load_golden_copy, parse_input_parameters};

/// Helper function to create an API client with API key from environment
fn get_api_key() -> Result<ApiKey> {
    let key_str = env::var("ALPHAVANTAGE_API_KEY")
        .expect("ALPHAVANTAGE_API_KEY environment variable must be set");

    Ok(ApiKey::new(key_str))
}

/// Helper function to create the HTTP client
fn create_client() -> AlphaVantageClient {
    // Use default settings (25 daily limit, 1000ms delay)
    AlphaVantageClient::default()
}

/// Helper function to add rate limit delay between tests
async fn rate_limit_delay() {
    // Wait 12 seconds to respect Alpha Vantage free tier rate limits (5 calls/min)
    println!("⏳ Waiting 12 seconds for rate limit...");
    sleep(Duration::from_secs(12)).await;
}

/// Helper function to execute a test and print results
async fn execute_test(
    test_name: &str,
    function_name: &str,
    endpoint: EndpointName,
    symbol: &str,
    extra_params: Option<HashMap<String, String>>,
) -> Result<()> {
    println!("\n{}", "=".repeat(60));
    println!("🧪 Testing: {}", test_name);
    println!("{}", "=".repeat(60));

    // Load golden copy
    println!("📖 Loading golden copy for: {}", function_name);
    let expected = load_golden_copy(function_name)?;

    // Create client and API key
    let client = create_client();
    let api_key = get_api_key()?;
    let ticker = TickerSymbol::new(symbol.to_string())?;

    println!(
        "📝 Parameters: endpoint={}, symbol={}",
        endpoint.function_name(),
        symbol
    );
    if let Some(ref params) = extra_params {
        for (k, v) in params {
            println!("    Extra param: {}={}", k, v);
        }
    }

    // Execute API call
    println!("🌐 Calling Alpha Vantage API...");
    let actual = client
        .fetch_ticker_endpoint(endpoint, &ticker, extra_params.as_ref(), &api_key)
        .await?;

    // Compare results
    println!("🔍 Comparing actual vs expected...");
    let comparison = compare_json_exact(&actual, &expected);

    if comparison.matches {
        println!("✅ PASS: Output matches golden copy exactly!");
    } else {
        println!("❌ FAIL: Output does not match golden copy");
        println!("\n📋 Differences found:");
        for (i, diff) in comparison.differences.iter().enumerate() {
            println!("  {}. {}", i + 1, diff);
            if i >= 9 {
                println!(
                    "  ... and {} more differences",
                    comparison.differences.len() - 10
                );
                break;
            }
        }

        // Write detailed diff to file for analysis
        std::fs::create_dir_all("test-results")?;
        let diff_file = format!("test-results/{}-diff.txt", function_name);
        if let Err(e) = std::fs::write(&diff_file, comparison.differences.join("\n")) {
            eprintln!("Warning: Could not write diff file: {}", e);
        } else {
            println!("\n📄 Full diff written to: {}", diff_file);
        }

        anyhow::bail!("Golden copy validation failed for {}", test_name);
    }

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_overview_golden_copy() -> Result<()> {
    let params = parse_input_parameters("overview")?;

    execute_test(
        "OVERVIEW",
        "overview",
        EndpointName::Overview,
        &params.symbol,
        None,
    )
    .await?;

    rate_limit_delay().await;
    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_balance_sheet_golden_copy() -> Result<()> {
    let params = parse_input_parameters("balance-sheet")?;

    execute_test(
        "BALANCE_SHEET",
        "balance-sheet",
        EndpointName::BalanceSheet,
        &params.symbol,
        None,
    )
    .await?;

    rate_limit_delay().await;
    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_cash_flow_golden_copy() -> Result<()> {
    let params = parse_input_parameters("cash-flow")?;

    execute_test(
        "CASH_FLOW",
        "cash-flow",
        EndpointName::CashFlow,
        &params.symbol,
        None,
    )
    .await?;

    rate_limit_delay().await;
    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_income_statement_golden_copy() -> Result<()> {
    let params = parse_input_parameters("income-statement")?;

    execute_test(
        "INCOME_STATEMENT",
        "income-statement",
        EndpointName::IncomeStatement,
        &params.symbol,
        None,
    )
    .await?;

    rate_limit_delay().await;
    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_earnings_golden_copy() -> Result<()> {
    let params = parse_input_parameters("earnings")?;

    execute_test(
        "EARNINGS",
        "earnings",
        EndpointName::Earnings,
        &params.symbol,
        None,
    )
    .await?;

    rate_limit_delay().await;
    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_earnings_estimates_golden_copy() -> Result<()> {
    let params = parse_input_parameters("earnings-estimates")?;

    execute_test(
        "EARNINGS_ESTIMATES",
        "earnings-estimates",
        EndpointName::EarningsEstimates,
        &params.symbol,
        None,
    )
    .await?;

    rate_limit_delay().await;
    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_earnings_call_transcript_golden_copy() -> Result<()> {
    let params = parse_input_parameters("earnings-call-transcript")?;

    let quarter = params
        .quarter
        .expect("EARNINGS_CALL_TRANSCRIPT requires quarter parameter");

    let mut extra_params = HashMap::new();
    extra_params.insert("quarter".to_string(), quarter);

    execute_test(
        "EARNINGS_CALL_TRANSCRIPT",
        "earnings-call-transcript",
        EndpointName::EarningsCallTranscript,
        &params.symbol,
        Some(extra_params),
    )
    .await?;

    rate_limit_delay().await;
    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_dividends_golden_copy() -> Result<()> {
    let params = parse_input_parameters("dividends")?;

    execute_test(
        "DIVIDENDS",
        "dividends",
        EndpointName::Dividends,
        &params.symbol,
        None,
    )
    .await?;

    rate_limit_delay().await;
    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_splits_golden_copy() -> Result<()> {
    let params = parse_input_parameters("splits")?;

    execute_test(
        "SPLITS",
        "splits",
        EndpointName::Splits,
        &params.symbol,
        None,
    )
    .await?;

    rate_limit_delay().await;
    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_shares_outstanding_golden_copy() -> Result<()> {
    let params = parse_input_parameters("shares-outstanding")?;

    // Note: This test uses MSFT, not IBM
    assert_eq!(
        params.symbol, "MSFT",
        "shares-outstanding should use MSFT symbol"
    );

    execute_test(
        "SHARES_OUTSTANDING",
        "shares-outstanding",
        EndpointName::SharesOutstanding,
        &params.symbol,
        None,
    )
    .await?;

    rate_limit_delay().await;
    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_insider_transactions_golden_copy() -> Result<()> {
    let params = parse_input_parameters("insider-transactions")?;

    execute_test(
        "INSIDER_TRANSACTIONS",
        "insider-transactions",
        EndpointName::InsiderTransactions,
        &params.symbol,
        None,
    )
    .await?;

    rate_limit_delay().await;
    Ok(())
}
