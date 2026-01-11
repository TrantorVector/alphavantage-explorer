use alphavantage_core::domain::{ApiKey, EndpointName, TickerSymbol};
use alphavantage_core::ports::ApiClient;
use alphavantage_core::{ExplorerError, Result};
use async_trait::async_trait;
use std::path::PathBuf;
use std::time::Duration;
use tokio::fs;
use tokio::time::sleep;

pub struct MockAlphaVantageClient {
    fixture_dir: PathBuf,
}

impl Default for MockAlphaVantageClient {
    fn default() -> Self {
        Self::new()
    }
}

impl MockAlphaVantageClient {
    #[must_use]
    pub fn new() -> Self {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let path = std::path::PathBuf::from(manifest_dir).join("fixtures");
        Self { fixture_dir: path }
    }

    async fn load_fixture(&self, filename: &str) -> Result<serde_json::Value> {
        let path = self.fixture_dir.join(filename);
        if !path.exists() {
            return Err(ExplorerError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Fixture not found: {}", path.display()),
            )));
        }
        let content = fs::read_to_string(path).await.map_err(ExplorerError::Io)?;
        serde_json::from_str(&content).map_err(ExplorerError::Json)
    }
}

#[async_trait]
impl ApiClient for MockAlphaVantageClient {
    async fn fetch_ticker_endpoint(
        &self,
        endpoint: EndpointName,
        ticker: &TickerSymbol,
        _params: Option<&std::collections::HashMap<String, String>>,
        _api_key: &ApiKey,
    ) -> Result<serde_json::Value> {
        // Simulate network latency
        sleep(Duration::from_millis(150)).await;

        let filename = format!(
            "tickers/{}/{}.json",
            ticker.as_str(),
            endpoint.function_name()
        );
        self.load_fixture(&filename).await
    }

    async fn fetch_market_endpoint(
        &self,
        endpoint: EndpointName,
        _api_key: &ApiKey,
    ) -> Result<serde_json::Value> {
        sleep(Duration::from_millis(150)).await;

        let filename = format!("market/{}.json", endpoint.function_name());
        self.load_fixture(&filename).await
    }
}
