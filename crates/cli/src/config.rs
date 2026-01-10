use crate::api_config::ApiConfig;
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
    pub daily_limit: u32,
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

        // Determine API key and rate limit
        let (api_key_str, daily_limit) = match client_mode {
            ClientMode::Live => {
                // Try to load from config file first, then fall back to CLI args
                if let Ok(config_file) = ApiConfig::load() {
                    tracing::info!(
                        "Loaded API configuration from alphavantage.toml (daily limit: {})",
                        config_file.rate_limit.daily_limit
                    );
                    let key = args.api_key.unwrap_or(config_file.api.api_key);
                    let limit = config_file.rate_limit.daily_limit;
                    (key, limit)
                } else {
                    // Fall back to CLI args/env vars
                    tracing::warn!(
                        "Could not load alphavantage.toml, using CLI args/env vars. \
                         Create alphavantage.toml from alphavantage.toml.template for easier configuration."
                    );
                    let key = args.api_key.context(
                        "API key required for live mode. \
                         Set ALPHA_VANTAGE_API_KEY env var, use --api-key flag, \
                         or create alphavantage.toml from alphavantage.toml.template",
                    )?;
                    (key, 25) // Default to free tier limit
                }
            }
            ClientMode::Mock => {
                // For mock mode, use dummy values
                (args.api_key.unwrap_or_else(|| "mock_key".to_string()), 25)
            }
        };

        let api_key = ApiKey::new(api_key_str);

        Ok(Self {
            symbols: args.symbols,
            out_dir: args.out_dir,
            api_key,
            client_mode,
            save_raw: !args.no_raw,
            daily_limit,
            log_format: args.log_format,
            log_level: args.log_level.into(),
        })
    }
}
