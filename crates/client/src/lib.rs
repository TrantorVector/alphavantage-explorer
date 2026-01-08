pub mod http_client;
pub mod mock_client;
pub mod rate_limiter;
pub mod retry_policy;

use alphavantage_core::ports::ApiClient;
use http_client::AlphaVantageClient;
use mock_client::MockAlphaVantageClient;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClientMode {
    Live,
    Mock,
}

#[must_use]
pub fn create_client(mode: ClientMode) -> Box<dyn ApiClient> {
    match mode {
        ClientMode::Live => Box::new(AlphaVantageClient::new()),
        ClientMode::Mock => Box::new(MockAlphaVantageClient::new()),
    }
}
