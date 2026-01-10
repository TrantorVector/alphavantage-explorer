use alphavantage_client::create_client;
use alphavantage_client::ClientMode;
use alphavantage_core::domain::{ApiKey, EndpointName, TickerSymbol};

#[allow(clippy::unwrap_used, clippy::indexing_slicing)]
#[tokio::test]
async fn test_mock_client_fixtures() {
    let api_key = ApiKey::new("dummy");
    let client = create_client(ClientMode::Mock, 25, 0);

    // Test AAPL OVERVIEW
    let ticker = TickerSymbol::new("AAPL").unwrap();
    let resp = client
        .fetch_ticker_endpoint(EndpointName::Overview, &ticker, &api_key)
        .await;
    if let Err(e) = &resp {
        println!("Error fetching AAPL OVERVIEW: {e:?}");
    }
    assert!(resp.is_ok());
    let json = resp.unwrap();
    assert_eq!(json["Symbol"], "AAPL");
    assert_eq!(json["Sector"], "Technology");

    // Test AAPL INCOME_STATEMENT
    let resp = client
        .fetch_ticker_endpoint(EndpointName::IncomeStatement, &ticker, &api_key)
        .await;
    if let Err(e) = &resp {
        println!("Error fetching AAPL INCOME_STATEMENT: {e:?}");
    }
    assert!(resp.is_ok());
    let json = resp.unwrap();
    assert_eq!(json["symbol"], "AAPL");
    assert!(!json["annualReports"].as_array().unwrap().is_empty());

    // Test Market TOP_GAINERS_LOSERS
    let resp = client
        .fetch_market_endpoint(EndpointName::TopGainersLosers, &api_key)
        .await;
    assert!(resp.is_ok());
    let json = resp.unwrap();
    assert_eq!(json["endpoint"], "TopGainersLosers");
    assert!(!json["top_gainers"].as_array().unwrap().is_empty());
}
