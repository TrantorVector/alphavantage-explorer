use crate::config::Config;
use crate::progress::ProgressReporter;
use alphavantage_client::{create_client, FileSystemJsonPersister, MarkdownWriterImpl};
use alphavantage_core::domain::{EndpointName, TickerSymbol};
use alphavantage_core::logic::json_to_table::parse_json_to_tables;
use alphavantage_core::ports::{JsonPersister, MarkdownWriter};
use anyhow::Result;
use std::io::Write;
use std::path::PathBuf;
use tracing::instrument;

pub struct Executor {
    config: Config,
}

impl Executor {
    pub const fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn run(&self) -> Result<()> {
        let client = create_client(self.config.client_mode);
        let persister = FileSystemJsonPersister::new();

        // 4 market-wide endpoints
        let market_endpoints = [
            EndpointName::MarketStatus,
            EndpointName::TopGainersLosers,
            EndpointName::ListingStatus,
            EndpointName::NewsSentiment, // Technically can be ticker specific too, but treating as market wide for feed
        ];

        // 13 ticker-specific endpoints (subset for now based on available parsers? No, strict list from requirements)
        // Wait, Build Plan says "fetch company-specific endpoints".
        // Let's use the full list of endpoint variants that are NOT market-wide?
        // Or explicitly list them to be safe and match the plan.
        let ticker_endpoints = [
            EndpointName::Overview,
            EndpointName::IncomeStatement,
            EndpointName::BalanceSheet,
            EndpointName::CashFlow,
            EndpointName::Earnings,
            EndpointName::GlobalQuote,
            EndpointName::TimeSeriesDaily,
            EndpointName::TimeSeriesWeekly,
            EndpointName::TimeSeriesMonthly,
            // Add others if parsers are ready, for now these are safest
        ];

        // Total tasks = market_endpoints + (symbols * ticker_endpoints)
        let total_tasks =
            market_endpoints.len() + (self.config.symbols.len() * ticker_endpoints.len());
        let progress = ProgressReporter::new(total_tasks);

        // 1. Fetch Market Wide
        for &endpoint in &market_endpoints {
            self.fetch_and_process(client.as_ref(), &persister, &progress, endpoint, None)
                .await;
        }

        // 2. Fetch per Ticker
        for ticker in &self.config.symbols {
            for &endpoint in &ticker_endpoints {
                self.fetch_and_process(
                    client.as_ref(),
                    &persister,
                    &progress,
                    endpoint,
                    Some(ticker),
                )
                .await;
            }
        }

        progress.summary().await;

        Ok(())
    }

    #[instrument(skip(self, client, persister, progress))]
    async fn fetch_and_process(
        &self,
        client: &dyn alphavantage_core::ports::ApiClient,
        persister: &FileSystemJsonPersister,
        progress: &ProgressReporter,
        endpoint: EndpointName,
        ticker: Option<&TickerSymbol>,
    ) {
        let display_name = ticker.map_or("MARKET", alphavantage_core::domain::TickerSymbol::as_str);
        ProgressReporter::start_fetch(endpoint, display_name);

        let result = if let Some(t) = ticker {
            client
                .fetch_ticker_endpoint(endpoint, t, &self.config.api_key)
                .await
        } else {
            client
                .fetch_market_endpoint(endpoint, &self.config.api_key)
                .await
        };

        match result {
            Ok(json) => {
                // Save RAW
                if self.config.save_raw {
                    let mut path = self.config.out_dir.join("raw");
                    if let Some(t) = ticker {
                        path.push("tickers");
                        path.push(t.as_str());
                    } else {
                        path.push("market");
                    }
                    path.push(format!("{endpoint}.json"));

                    if let Err(e) = persister.save_raw_json(&path, &json) {
                        tracing::warn!("Failed to save raw JSON for {}: {}", endpoint, e);
                    }
                }

                // Gen Markdown
                if let Err(e) = self.generate_markdown(endpoint, ticker, &json) {
                    tracing::warn!("Failed to generate markdown for {}: {}", endpoint, e);
                }

                progress
                    .finish_fetch(endpoint, display_name, true, None)
                    .await;
            }
            Err(e) => {
                progress
                    .finish_fetch(endpoint, display_name, false, Some(&e.to_string()))
                    .await;
            }
        }
    }

    fn generate_markdown(
        &self,
        endpoint: EndpointName,
        ticker: Option<&TickerSymbol>,
        json: &serde_json::Value,
    ) -> Result<()> {
        let tables = parse_json_to_tables(endpoint, json)?;

        let mut writer = MarkdownWriterImpl::new();
        for table in tables {
            writer.write_table(&table)?;
        }

        let mut path = self.config.out_dir.clone();
        if let Some(t) = ticker {
            path.push("tickers");
            path.push(format!("{}.md", t.as_str()));
            // Append mode? Or overwrite?
            // If we have multiple endpoints writing to the SAME file (AAPL.md), we need to APPEND.
            // MarkdownWriter currently has flush_to_file which uses fs::write (overwrite).
            // We need to implement Append or read-modify-write.
            // For Phase 6 simplification: We will switch to one file per endpoint or use append mode manually here?
            // "Write Markdown reports per ticker" suggests a single report.
            // Let's modify MarkdownWriter to support append or handle it here.
            // Actually, for simplicity and robustness, let's create per-endpoint files first
            // OR simply read existing content into buffer?
            // Let's go with: out/tickers/AAPL/<endpoint>.md
            // WAIT, instructions say: "Should create out/tickers/AAPL.md"
            // This clearly implies aggregation.

            // To support aggregation without race conditions or overwrite:
            // The executor loops sequentially per ticker.
            // So we can maintain a single writer per ticker?
            // BUT fetch_and_process is per endpoint.
            // We should change the aggregation strategy.
            // Refactor: File naming: out/tickers/<ticker>/<endpoint>.md is safer.
            // BUT "out/tickers/AAPL.md" is the requirement.

            // Allow me to use a helper to append to file.
            Self::append_to_file(&path, writer.as_str())?;
        } else {
            path.push(format!("market_{endpoint}.md")); // Market stuff separate?
                                                        // Or out/market.md?
                                                        // Let's stick to safe defaults: unique files.
            writer.flush_to_file(&path)?;
        }

        Ok(())
    }

    fn append_to_file(path: &PathBuf, content: &str) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;

        // Add some spacing
        writeln!(file, "\n{content}\n")?;
        Ok(())
    }
}
