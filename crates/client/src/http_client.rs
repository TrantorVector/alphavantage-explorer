use crate::rate_limiter::RateLimiter;
use crate::retry_policy::execute_with_retry;
use alphavantage_core::domain::{ApiKey, EndpointName, TickerSymbol};
use alphavantage_core::ports::ApiClient;
use alphavantage_core::{ExplorerError, Result};
use async_trait::async_trait;
use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;
use tracing::instrument;

const BASE_URL: &str = "https://www.alphavantage.co/query";

pub struct AlphaVantageClient {
    http_client: Client,
    rate_limiter: Arc<RateLimiter>,
    base_url: String,
}

impl AlphaVantageClient {
    /// # Panics
    /// Panics if the HTTP client builder fails (e.g. TLS backend issue).
    #[must_use]
    #[allow(clippy::expect_used)]
    pub fn new(daily_limit: u32, min_delay_ms: u64) -> Self {
        Self::with_base_url(BASE_URL, daily_limit, min_delay_ms)
    }

    /// # Panics
    /// Panics if the HTTP client builder fails.
    #[must_use]
    #[allow(clippy::expect_used)]
    pub fn with_base_url(base_url: impl Into<String>, daily_limit: u32, min_delay_ms: u64) -> Self {
        Self {
            http_client: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("Failed to build HTTP client"),
            rate_limiter: Arc::new(RateLimiter::new(daily_limit, min_delay_ms)),
            base_url: base_url.into(),
        }
    }
}

impl Default for AlphaVantageClient {
    fn default() -> Self {
        Self::new(25, 1000) // Default to free tier limit and delay
    }
}

#[async_trait]
impl ApiClient for AlphaVantageClient {
    #[instrument(skip(self, api_key), fields(endpoint = %endpoint.function_name(), ticker = %ticker.as_str()))]
    async fn fetch_ticker_endpoint(
        &self,
        endpoint: EndpointName,
        ticker: &TickerSymbol,
        params: Option<&std::collections::HashMap<String, String>>,
        api_key: &ApiKey,
    ) -> Result<serde_json::Value> {
        self.execute_request(endpoint, Some(ticker.as_str()), params, api_key)
            .await
    }

    #[instrument(skip(self, api_key), fields(endpoint = %endpoint.function_name()))]
    async fn fetch_market_endpoint(
        &self,
        endpoint: EndpointName,
        api_key: &ApiKey,
    ) -> Result<serde_json::Value> {
        self.execute_request(endpoint, None, None, api_key).await
    }
}

impl AlphaVantageClient {
    async fn execute_request(
        &self,
        endpoint: EndpointName,
        symbol: Option<&str>,
        extra_params: Option<&std::collections::HashMap<String, String>>,
        api_key: &ApiKey,
    ) -> Result<serde_json::Value> {
        let is_demo = api_key.secret() == "demo";

        // Skip rate limiter only for "demo" key
        if !is_demo {
            self.rate_limiter.wait().await?;
        }

        let url = self.base_url.clone();
        let client = &self.http_client;
        let function = endpoint.function_name();
        let key_str = api_key.secret().to_string();
        let symbol_owned = symbol.map(ToString::to_string);
        // Clone params for usage in closure
        let params_owned = extra_params.cloned();

        // Execute with retry
        execute_with_retry(move || {
            let client = client.clone();
            let url = url.clone();
            let function = function.to_string();
            let key = key_str.clone();
            let sym = symbol_owned.clone();
            let params = params_owned.clone();

            async move {
                let mut req = client
                    .get(&url)
                    .query(&[("function", &function), ("apikey", &key)]);

                if let Some(s) = &sym {
                    req = req.query(&[("symbol", s)]);
                }
                
                if let Some(p) = &params {
                    req = req.query(p);
                }

                let resp = req
                    .try_clone()
                    .unwrap()
                    .send()
                    .await
                    .map_err(|e| ExplorerError::Network(e.to_string()))?;

                let status = resp.status();
                if !status.is_success() {
                    return Err(ExplorerError::HttpStatus(status.as_u16()));
                }

                // Read response as text first
                let content_type = resp
                    .headers()
                    .get(reqwest::header::CONTENT_TYPE)
                    .and_then(|h| h.to_str().ok())
                    .unwrap_or("")
                    .to_string();

                let text = resp
                    .text()
                    .await
                    .map_err(|e| ExplorerError::Network(e.to_string()))?;

                // Try parsing as JSON
                let json: serde_json::Value = match serde_json::from_str(&text) {
                    Ok(v) => v,
                    Err(_) => {
                        // If parsing fails, check if it looks like CSV or if Content-Type indicates CSV
                        if content_type.contains("text/csv")
                            || content_type.contains("application/csv")
                            || text.contains(',')
                        {
                            // Wrap CSV in a special JSON structure so we can pass it through the existing pipeline
                            serde_json::json!({ "csv_content": text })
                        } else {
                            // If it's not JSON and not obviously CSV, return the parse error (or the text wrapped?)
                            // Better to error out if we can't understand it, unless it's a raw error message
                            return Err(ExplorerError::Network(format!(
                                "Failed to parse response as JSON: {text}"
                            )));
                        }
                    }
                };

                // Check for API soft errors (200 OK but body has Error)
                if let Some(err_msg) = json.get("Error Message") {
                    return Err(ExplorerError::ProviderError(err_msg.to_string()));
                }
                if let Some(note) = json.get("Note") {
                    // "Thank you for using Alpha Vantage!..." means rate limit hit
                    if note.to_string().contains("call volume") {
                        return Err(ExplorerError::RateLimited);
                    }
                }
                // Check if it's an Information field (sometimes implies error/limit)
                if json.get("Information").is_some() {
                    // Usually not an error, but worth noting?
                    // For now accept it unless it's clearly a limit message.
                }

                Ok(json)
            }
        })
        .await
    }
}
