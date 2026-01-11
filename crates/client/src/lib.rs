pub mod csv_handler;
pub mod http_client;
pub mod json_persister;
pub mod markdown_writer;
pub mod mock_client;
pub mod rate_limiter;
pub mod retry_policy;
pub mod schema_analyzer;

pub use csv_handler::CsvHandler;
pub use json_persister::FileSystemJsonPersister;
pub use markdown_writer::MarkdownWriterImpl;
pub use mock_client::MockAlphaVantageClient as MockClient;
pub use rate_limiter::RateLimiter;
pub use schema_analyzer::SchemaAnalyzerImpl;

use alphavantage_core::ports::ApiClient;
use http_client::AlphaVantageClient;
use mock_client::MockAlphaVantageClient;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClientMode {
    Live,
    Mock,
}

#[must_use]
pub fn create_client(mode: ClientMode, daily_limit: u32, min_delay_ms: u64) -> Box<dyn ApiClient> {
    match mode {
        ClientMode::Live => Box::new(AlphaVantageClient::new(daily_limit, min_delay_ms)),
        ClientMode::Mock => Box::new(MockAlphaVantageClient::new()),
    }
}
