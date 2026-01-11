use crate::cli_args::Commands;
use crate::config::Config;
use alphavantage_client::{CsvHandler, FileSystemJsonPersister, MarkdownWriterImpl};
use alphavantage_core::domain::{EndpointName, QuarterParam, TickerSymbol};
use alphavantage_core::error::Result;
use alphavantage_core::logic::json_to_table::parse_json_to_tables;
use alphavantage_core::ports::{ApiClient, JsonPersister, MarkdownWriter};
use alphavantage_core::util::generate_timestamp;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Executor for granular (single-endpoint) commands
pub struct GranularExecutor<'a> {
    config: &'a Config,
    client: &'a dyn ApiClient,
}

impl<'a> GranularExecutor<'a> {
    #[must_use]
    pub const fn new(config: &'a Config, client: &'a dyn ApiClient) -> Self {
        Self { config, client }
    }

    /// Execute a granular command (single endpoint fetch)
    ///
    /// # Errors
    /// Returns error if API call fails, file I/O fails, or parsing fails
    pub async fn execute(&self, command: &Commands) -> Result<()> {
        let (endpoint, symbol, params, output_dir) = Self::route_command(command);

        // Make API call (note: API client returns JSON Value, not raw string for now)
        let json_value = self
            .client
            .fetch_ticker_endpoint(endpoint, &symbol, Some(&params), &self.config.api_key)
            .await?;

        // Generate timestamped filename
        let timestamp = generate_timestamp();
        let endpoint_name = format!("{endpoint}").to_lowercase().replace('_', "-");
        let base_filename = format!("{endpoint_name}_{symbol}_{timestamp}");

        // Determine output directory (use custom or default)
        let out_dir = output_dir.unwrap_or_else(|| self.config.out_dir.clone());

        // Check for wrapped CSV content from standard client
        if let Some(csv_content) = json_value.get("csv_content").and_then(|v| v.as_str()) {
            Self::handle_csv_output(csv_content, &base_filename, &out_dir)?;
            return Ok(());
        }

        // Convert to string for consistency with bulk mode
        let response = serde_json::to_string_pretty(&json_value)?;

        // Handle response based on type (JSON vs CSV)
        #[allow(clippy::single_match_else)]
        match Self::detect_content_type(&response) {
            ContentType::Json => {
                self.handle_json_output(&json_value, endpoint, &base_filename, &out_dir)?;
            }
            ContentType::Csv => {
                Self::handle_csv_output(&response, &base_filename, &out_dir)?;
            }
        }

        Ok(())
    }

    /// Route command to endpoint and extract parameters
    fn route_command(
        command: &Commands,
    ) -> (
        EndpointName,
        TickerSymbol,
        HashMap<String, String>,
        Option<PathBuf>,
    ) {
        let mut params = HashMap::new();

        let (endpoint, symbol, output) = match command {
            Commands::Overview { symbol, output } => {
                (EndpointName::Overview, symbol.clone(), output.clone())
            }
            Commands::IncomeStatement { symbol, output } => (
                EndpointName::IncomeStatement,
                symbol.clone(),
                output.clone(),
            ),
            Commands::BalanceSheet { symbol, output } => {
                (EndpointName::BalanceSheet, symbol.clone(), output.clone())
            }
            Commands::CashFlow { symbol, output } => {
                (EndpointName::CashFlow, symbol.clone(), output.clone())
            }
            Commands::Earnings { symbol, output } => {
                (EndpointName::Earnings, symbol.clone(), output.clone())
            }
            Commands::EarningsEstimates { symbol, output } => (
                EndpointName::EarningsEstimates,
                symbol.clone(),
                output.clone(),
            ),
            Commands::NewsSentiment {
                symbol,
                limit,
                output,
            } => {
                params.insert("limit".to_string(), limit.to_string());
                (EndpointName::NewsSentiment, symbol.clone(), output.clone())
            }
            Commands::InsiderTransactions { symbol, output } => (
                EndpointName::InsiderTransactions,
                symbol.clone(),
                output.clone(),
            ),
            Commands::Dividends { symbol, output } => {
                (EndpointName::Dividends, symbol.clone(), output.clone())
            }
            Commands::Splits { symbol, output } => {
                (EndpointName::Splits, symbol.clone(), output.clone())
            }
            Commands::SharesOutstanding { symbol, output } => (
                EndpointName::SharesOutstanding,
                symbol.clone(),
                output.clone(),
            ),
            Commands::EarningsCalendar {
                symbol,
                horizon,
                output,
            } => {
                if let Some(h) = horizon {
                    params.insert("horizon".to_string(), h.to_string());
                }
                (
                    EndpointName::EarningsCalendar,
                    symbol.clone(),
                    output.clone(),
                )
            }
            Commands::EarningsCallTranscript {
                symbol,
                year,
                quarter,
                output,
            } => {
                let q_num = match quarter {
                    QuarterParam::Q1 => "1",
                    QuarterParam::Q2 => "2",
                    QuarterParam::Q3 => "3",
                    QuarterParam::Q4 => "4",
                };
                // API expects quarter=YYYYQx (e.g., 2024Q1)
                let combined_quarter = format!("{year}Q{q_num}");
                params.insert("quarter".to_string(), combined_quarter);
                (
                    EndpointName::EarningsCallTranscript,
                    symbol.clone(),
                    output.clone(),
                )
            }
        };

        (endpoint, symbol, params, output)
    }

    /// Detect content type from response
    const fn detect_content_type(_response: &str) -> ContentType {
        // For now, assume JSON (CSV detection would require checking actual content-type headers)
        // In a real implementation, this would check the HTTP response headers
        ContentType::Json
    }

    /// Handle JSON output: save raw JSON and generate markdown
    fn handle_json_output(
        &self,
        json: &serde_json::Value,
        endpoint: EndpointName,
        base_filename: &str,
        out_dir: &Path,
    ) -> Result<()> {
        // Save raw JSON
        if self.config.save_raw {
            let raw_path = out_dir.join("raw").join(format!("{base_filename}.json"));
            let persister = FileSystemJsonPersister::new();
            persister.save_raw_json(&raw_path, json)?;
            println!("✓ Saved raw JSON: {}", raw_path.display());
        }

        // Generate markdown with FULL output (None = all rows)
        let tables = parse_json_to_tables(endpoint, json, None)?;
        let mut writer = MarkdownWriterImpl::with_truncation(false); // Full output for granular mode

        for table in &tables {
            writer.write_table(table)?;
        }

        let md_path = out_dir.join(format!("{base_filename}.md"));
        writer.flush_to_file(&md_path)?;
        println!("✓ Saved markdown: {}", md_path.display());

        Ok(())
    }

    /// Handle CSV output: save raw CSV only
    fn handle_csv_output(response: &str, base_filename: &str, out_dir: &Path) -> Result<()> {
        let csv_path = out_dir.join("raw").join(format!("{base_filename}.csv"));
        CsvHandler::save_raw(response, &csv_path)?;
        println!(
            "✓ Saved CSV: {} (CSV format, no markdown conversion)",
            csv_path.display()
        );
        Ok(())
    }
}

enum ContentType {
    Json,
    #[allow(dead_code)]
    Csv,
}
