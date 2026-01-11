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
    pub min_delay_ms: u64,
    #[allow(dead_code)]
    pub log_format: LogFormat,
    #[allow(dead_code)]
    pub log_level: Level,
}

impl Config {
    /// Creates a configuration from CLI arguments
    ///
    /// # Errors
    /// Returns error if API key is required but not provided in live mode
    pub fn from_args(args: CliArgs) -> Result<Self> {
        let client_mode = if args.live_api {
            ClientMode::Live
        } else {
            ClientMode::Mock
        };

        // Determine API key, rate limit, and min delay
        let (api_key_str, daily_limit, min_delay_ms) = match client_mode {
            ClientMode::Live => {
                // Try to load from config file first, then fall back to CLI args
                if let Ok(config_file) = ApiConfig::load() {
                    tracing::info!(
                        "Loaded API configuration from alphavantage.toml (daily limit: {}, min delay: {}ms)",
                        config_file.rate_limit.daily_limit,
                        config_file.rate_limit.min_delay_ms
                    );
                    let key = args.api_key.unwrap_or(config_file.api.api_key);
                    let limit = config_file.rate_limit.daily_limit;
                    let delay = config_file.rate_limit.min_delay_ms;
                    (key, limit, delay)
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
                    (key, 25, 1000) // Default to free tier limit and delay
                }
            }
            ClientMode::Mock => {
                // For mock mode, use dummy values
                (
                    args.api_key.unwrap_or_else(|| "mock_key".to_string()),
                    25,
                    0, // No delay in mock mode
                )
            }
        };

        let api_key = ApiKey::new(api_key_str);

        // For bulk mode (no subcommand), use symbols or default to AAPL,NVDA,MU
        // For granular mode (has subcommand), symbols field won't be used
        let symbols = args.symbols.unwrap_or_else(|| {
            // Safe to use from_static since we control these known-valid symbols
            const DEFAULT_SYMBOLS: &[&str] = &["AAPL", "NVDA", "MU"];
            DEFAULT_SYMBOLS
                .iter()
                .filter_map(|s| TickerSymbol::new(*s).ok())
                .collect()
        });

        Ok(Self {
            symbols,
            out_dir: args.out_dir,
            api_key,
            client_mode,
            save_raw: !args.no_raw,
            daily_limit,
            min_delay_ms,
            log_format: args.log_format,
            log_level: args.log_level.into(),
        })
    }
}
