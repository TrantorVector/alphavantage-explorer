use crate::domain::{ApiKey, EndpointName, TickerSymbol};
use crate::error::Result;
use async_trait::async_trait;

/// Port for fetching raw data from the API provider.
#[async_trait]
pub trait ApiClient: Send + Sync {
    /// Fetches data for a specific ticker (e.g., `TIME_SERIES_DAILY`).
    ///
    /// Returns the raw JSON value.
    async fn fetch_ticker_endpoint(
        &self,
        endpoint: EndpointName,
        ticker: &TickerSymbol,
        api_key: &ApiKey,
    ) -> Result<serde_json::Value>;

    /// Fetches data for a market-wide endpoint (e.g., `MARKET_STATUS`).
    async fn fetch_market_endpoint(
        &self,
        endpoint: EndpointName,
        api_key: &ApiKey,
    ) -> Result<serde_json::Value>;
}
