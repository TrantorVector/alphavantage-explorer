use alphavantage_core::domain::TickerSymbol;
use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use tracing::Level;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Comma-separated list of ticker symbols to query
    #[arg(long, default_value = "AAPL,NVDA,MU", value_delimiter = ',', value_parser = parse_ticker)]
    pub symbols: Vec<TickerSymbol>,

    /// Output directory for reports and raw data
    #[arg(long, default_value = "out")]
    pub out_dir: PathBuf,

    /// Alpha Vantage API Key
    #[arg(long, env = "ALPHA_VANTAGE_API_KEY")]
    pub api_key: Option<String>,

    /// Enable live API calls (consumes quota)
    #[arg(long, default_value_t = false)]
    pub live_api: bool,

    /// Disable raw JSON persistence
    #[arg(long, default_value_t = false)]
    pub no_raw: bool,

    /// Log output format
    #[arg(long, value_enum, default_value_t = LogFormat::Human)]
    pub log_format: LogFormat,

    /// Log verbosity level
    #[arg(long, value_enum, default_value_t = LogLevel::Info)]
    pub log_level: LogLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum LogFormat {
    Human,
    Json,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

// Convert our enum to tracing::Level
impl From<LogLevel> for Level {
    fn from(val: LogLevel) -> Self {
        match val {
            LogLevel::Error => Self::ERROR,
            LogLevel::Warn => Self::WARN,
            LogLevel::Info => Self::INFO,
            LogLevel::Debug => Self::DEBUG,
            LogLevel::Trace => Self::TRACE,
        }
    }
}

fn parse_ticker(s: &str) -> std::result::Result<TickerSymbol, String> {
    TickerSymbol::new(s).map_err(|e| e.to_string())
}
