use crate::cli_args::{CliArgs, LogFormat};
use alphavantage_client::ClientMode;
use alphavantage_core::domain::{ApiKey, TickerSymbol};
use anyhow::{Context, Result};
use std::path::PathBuf;
use tracing::Level;

#[derive(Debug, Clone)]
pub struct Config {
    pub symbols: Vec<TickerSymbol>,
    pub out_dir: PathBuf,
    pub api_key: ApiKey,
    pub client_mode: ClientMode,
    pub save_raw: bool,
    #[allow(dead_code)]
    pub log_format: LogFormat,
    #[allow(dead_code)]
    pub log_level: Level,
}

impl Config {
    pub fn from_args(args: CliArgs) -> Result<Self> {
        let client_mode = if args.live_api {
            ClientMode::Live
        } else {
            ClientMode::Mock
        };

        // If mock, we can just use a dummy key if none provided
        let api_key_str = match client_mode {
            ClientMode::Live => args
                .api_key
                .context("API key required for live mode. Set ALPHA_VANTAGE_API_KEY env var or --api-key flag.")?,
            ClientMode::Mock => args.api_key.unwrap_or_else(|| "mock_key".to_string()),
        };

        let api_key = ApiKey::new(api_key_str);

        Ok(Self {
            symbols: args.symbols,
            out_dir: args.out_dir,
            api_key,
            client_mode,
            save_raw: !args.no_raw,
            log_format: args.log_format,
            log_level: args.log_level.into(),
        })
    }
}
