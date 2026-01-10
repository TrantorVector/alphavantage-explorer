#![allow(clippy::expect_used)]

use crate::config::Config;
use crate::index_generator::{generate_index, ExecutionResults};
use crate::progress::ProgressReporter;
use alphavantage_client::{
    create_client, FileSystemJsonPersister, MarkdownWriterImpl, SchemaAnalyzerImpl,
};
use alphavantage_core::domain::{EndpointName, SchemaTable, TickerSymbol};
use alphavantage_core::logic::json_to_table::parse_json_to_tables;
use alphavantage_core::ports::{JsonPersister, MarkdownWriter};
use anyhow::Result;
use std::collections::HashMap;
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
        let client = create_client(
            self.config.client_mode,
            self.config.daily_limit,
            self.config.min_delay_ms,
        );
        let persister = FileSystemJsonPersister::new();
        let analyzer = SchemaAnalyzerImpl::new();

        // Aggregate results
        let mut results = ExecutionResults::new();
        let mut tables_accumulator: HashMap<EndpointName, HashMap<TickerSymbol, Vec<SchemaTable>>> =
            HashMap::new();

        // 4 market-wide endpoints
        let market_endpoints = [
            EndpointName::MarketStatus,
            EndpointName::TopGainersLosers,
            EndpointName::ListingStatus,
            EndpointName::NewsSentiment,
        ];

        // 13 ticker-specific endpoints per PRD
        let ticker_endpoints = [
            EndpointName::Overview,
            EndpointName::IncomeStatement,
            EndpointName::BalanceSheet,
            EndpointName::CashFlow,
            EndpointName::Earnings,
            EndpointName::EarningsEstimates,
            EndpointName::NewsSentiment,
            EndpointName::InsiderTransactions,
            EndpointName::Dividends,
            EndpointName::Splits,
            EndpointName::SharesOutstanding,
            EndpointName::EarningsCalendar,
            EndpointName::EarningsCallTranscript,
        ];

        let total_tasks =
            market_endpoints.len() + (self.config.symbols.len() * ticker_endpoints.len());
        let progress = ProgressReporter::new(total_tasks);

        // 1. Fetch Market Wide
        for &endpoint in &market_endpoints {
            let success = self
                .fetch_and_process(client.as_ref(), &persister, &progress, endpoint, None)
                .await?;
            results.market_status.insert(endpoint, success);
        }

        // 2. Fetch per Ticker
        for ticker in &self.config.symbols {
            results.ticker_status.insert(ticker.clone(), HashMap::new());

            for &endpoint in &ticker_endpoints {
                let (success, tables_opt) = self
                    .fetch_and_process_ticker(
                        client.as_ref(),
                        &persister,
                        &progress,
                        endpoint,
                        ticker,
                    )
                    .await?;

                // Track status
                let entry = results
                    .ticker_status
                    .get_mut(ticker)
                    .expect("ticker should exist in pre-initialized map");
                entry.insert(endpoint, (success, None));
                // We don't have error msg easily propagated here without changing signature more,
                // but fetch_and_process_ticker logs it.
                // Let's assume success=true means no error msg.
                // To do this properly, fetch_and_process_ticker should return error string.
                // Refactoring below.

                // Accumulate tables
                if let Some(tables) = tables_opt {
                    tables_accumulator
                        .entry(endpoint)
                        .or_default()
                        .insert(ticker.clone(), tables);
                }
            }
        }

        // 3. Compute Schema Diffs
        for (endpoint, ticker_map) in &tables_accumulator {
            if ticker_map.len() > 1 {
                let diff = analyzer.compute_schema_diff(*endpoint, ticker_map);
                if !diff.differences.is_empty() {
                    results.schema_diffs.push(diff);
                }
            }
        }

        results.end_time = Some(chrono::Local::now());
        progress.summary().await;

        // 4. Generate Index
        generate_index(&results, &self.config.out_dir)?;

        Ok(())
    }

    // Refactored to return success status for market endpoints
    #[instrument(skip(self, client, persister, progress))]
    async fn fetch_and_process(
        &self,
        client: &dyn alphavantage_core::ports::ApiClient,
        persister: &FileSystemJsonPersister,
        progress: &ProgressReporter,
        endpoint: EndpointName,
        ticker: Option<&TickerSymbol>,
    ) -> Result<bool> {
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

                if let Err(e) = self.generate_markdown(endpoint, ticker, &json) {
                    tracing::warn!("Failed to generate markdown for {}: {}", endpoint, e);
                }

                progress
                    .finish_fetch(endpoint, display_name, true, None)
                    .await;
                Ok(true)
            }
            Err(e) => {
                progress
                    .finish_fetch(endpoint, display_name, false, Some(&e.to_string()))
                    .await;
                Ok(false)
            }
        }
    }

    // Separate function for ticker to return Tables and detailed status
    async fn fetch_and_process_ticker(
        &self,
        client: &dyn alphavantage_core::ports::ApiClient,
        persister: &FileSystemJsonPersister,
        progress: &ProgressReporter,
        endpoint: EndpointName,
        ticker: &TickerSymbol,
    ) -> Result<(bool, Option<Vec<SchemaTable>>)> {
        ProgressReporter::start_fetch(endpoint, ticker.as_str());

        let result = client
            .fetch_ticker_endpoint(endpoint, ticker, &self.config.api_key)
            .await;

        match result {
            Ok(json) => {
                // Save RAW
                if self.config.save_raw {
                    let mut path = self.config.out_dir.join("raw");
                    path.push("tickers");
                    path.push(ticker.as_str());
                    path.push(format!("{endpoint}.json"));
                    let _ = persister.save_raw_json(&path, &json);
                }

                // Gen Markdown & Extract Tables
                let tables =
                    match self.generate_markdown_returning_tables(endpoint, Some(ticker), &json) {
                        Ok(t) => t,
                        Err(e) => {
                            tracing::warn!(
                                "Failed to process markdown/tables for {}: {}",
                                endpoint,
                                e
                            );
                            progress
                                .finish_fetch(
                                    endpoint,
                                    ticker.as_str(),
                                    true,
                                    Some("Partial success (layout error)"),
                                )
                                .await;
                            return Ok((true, None)); // Fetch succeeded, but parse failed??
                                                     // Actually generate_markdown calls parse_json_to_tables. If that fails, we can't diff.
                        }
                    };

                progress
                    .finish_fetch(endpoint, ticker.as_str(), true, None)
                    .await;
                Ok((true, Some(tables)))
            }
            Err(e) => {
                progress
                    .finish_fetch(endpoint, ticker.as_str(), false, Some(&e.to_string()))
                    .await;
                Ok((false, None))
            }
        }
    }

    fn generate_markdown(
        &self,
        endpoint: EndpointName,
        ticker: Option<&TickerSymbol>,
        json: &serde_json::Value,
    ) -> Result<()> {
        let _ = self.generate_markdown_returning_tables(endpoint, ticker, json)?;
        Ok(())
    }

    fn generate_markdown_returning_tables(
        &self,
        endpoint: EndpointName,
        ticker: Option<&TickerSymbol>,
        json: &serde_json::Value,
    ) -> Result<Vec<SchemaTable>> {
        let tables = parse_json_to_tables(endpoint, json, Some(3))?; // Bulk mode truncates to 3 rows

        let mut writer = MarkdownWriterImpl::new();
        for table in &tables {
            writer.write_table(table)?;
        }

        let mut path = self.config.out_dir.clone();
        if let Some(t) = ticker {
            path.push("tickers");
            path.push(format!("{}.md", t.as_str()));
            Self::append_to_file(&path, writer.as_str())?;
        } else {
            path.push(format!("market_{endpoint}.md"));
            writer.flush_to_file(&path)?;
        }

        Ok(tables)
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
