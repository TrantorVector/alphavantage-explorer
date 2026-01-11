use alphavantage_core::domain::{HorizonParam, QuarterParam, TickerSymbol};
use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;
use tracing::Level;

/// Alpha Vantage Explorer - API validation and reporting tool
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Comma-separated list of ticker symbols to query (bulk mode if no subcommand)
    #[arg(long, value_delimiter = ',', value_parser = parse_ticker, global = true)]
    pub symbols: Option<Vec<TickerSymbol>>,

    /// Output directory for reports and raw data
    #[arg(long, default_value = "out", global = true)]
    pub out_dir: PathBuf,

    /// Alpha Vantage API Key
    #[arg(long, env = "ALPHA_VANTAGE_API_KEY", global = true)]
    pub api_key: Option<String>,

    /// Enable live API calls (consumes quota)
    #[arg(long, default_value_t = false, global = true)]
    pub live_api: bool,

    /// Disable raw JSON persistence
    #[arg(long, default_value_t = false, global = true)]
    pub no_raw: bool,

    /// Log output format
    #[arg(long, value_enum, default_value_t = LogFormat::Human, global = true)]
    pub log_format: LogFormat,

    /// Log verbosity level
    #[arg(long, value_enum, default_value_t = LogLevel::Info, global = true)]
    pub log_level: LogLevel,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Fetch company overview (`OVERVIEW` endpoint)
    Overview {
        /// Stock ticker symbol
        #[arg(short, long, value_parser = parse_ticker)]
        symbol: TickerSymbol,

        /// Output file path (optional, defaults to `out_dir`)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Fetch income statement (`INCOME_STATEMENT` endpoint)
    IncomeStatement {
        /// Stock ticker symbol
        #[arg(short, long, value_parser = parse_ticker)]
        symbol: TickerSymbol,

        /// Output file path (optional)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Fetch balance sheet (`BALANCE_SHEET` endpoint)
    BalanceSheet {
        /// Stock ticker symbol
        #[arg(short, long, value_parser = parse_ticker)]
        symbol: TickerSymbol,

        /// Output file path (optional)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Fetch cash flow statement (`CASH_FLOW` endpoint)
    CashFlow {
        /// Stock ticker symbol
        #[arg(short, long, value_parser = parse_ticker)]
        symbol: TickerSymbol,

        /// Output file path (optional)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Fetch earnings data (`EARNINGS` endpoint)
    Earnings {
        /// Stock ticker symbol
        #[arg(short, long, value_parser = parse_ticker)]
        symbol: TickerSymbol,

        /// Output file path (optional)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Fetch earnings estimates (`EARNINGS_ESTIMATES` endpoint)
    EarningsEstimates {
        /// Stock ticker symbol
        #[arg(short, long, value_parser = parse_ticker)]
        symbol: TickerSymbol,

        /// Output file path (optional)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Fetch news sentiment (`NEWS_SENTIMENT` endpoint)
    NewsSentiment {
        /// Stock ticker symbol
        #[arg(short, long, value_parser = parse_ticker)]
        symbol: TickerSymbol,

        /// Maximum number of news items to fetch
        #[arg(short, long, default_value_t = 50)]
        limit: u32,

        /// Output file path (optional)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Fetch insider transactions (`INSIDER_TRANSACTIONS` endpoint)
    InsiderTransactions {
        /// Stock ticker symbol
        #[arg(short, long, value_parser = parse_ticker)]
        symbol: TickerSymbol,

        /// Output file path (optional)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Fetch dividend history (`DIVIDENDS` endpoint)
    Dividends {
        /// Stock ticker symbol
        #[arg(short, long, value_parser = parse_ticker)]
        symbol: TickerSymbol,

        /// Output file path (optional)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Fetch stock splits (`SPLITS` endpoint)
    Splits {
        /// Stock ticker symbol
        #[arg(short, long, value_parser = parse_ticker)]
        symbol: TickerSymbol,

        /// Output file path (optional)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Fetch shares outstanding (`SHARES_OUTSTANDING` endpoint)
    SharesOutstanding {
        /// Stock ticker symbol
        #[arg(short, long, value_parser = parse_ticker)]
        symbol: TickerSymbol,

        /// Output file path (optional)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Fetch earnings calendar (`EARNINGS_CALENDAR` endpoint)
    EarningsCalendar {
        /// Stock ticker symbol
        #[arg(short, long, value_parser = parse_ticker)]
        symbol: TickerSymbol,

        /// Time horizon (3month, 6month, 12month)
        #[arg(short = 'H', long)]
        horizon: Option<HorizonParam>,

        /// Output file path (optional)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Fetch earnings call transcript (`EARNINGS_CALL_TRANSCRIPT` endpoint)
    EarningsCallTranscript {
        /// Stock ticker symbol
        #[arg(short, long, value_parser = parse_ticker)]
        symbol: TickerSymbol,

        /// Year of earnings call
        #[arg(short, long)]
        year: u16,

        /// Quarter (Q1, Q2, Q3, Q4)
        #[arg(short, long)]
        quarter: QuarterParam,

        /// Output file path (optional)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
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
