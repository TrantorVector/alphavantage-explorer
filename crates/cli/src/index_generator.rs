use alphavantage_core::domain::{EndpointName, TickerSymbol};
use alphavantage_client::schema_analyzer::SchemaDiff;
use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;
use std::io::Write;

#[derive(Debug, Default)]
pub struct ExecutionResults {
    pub market_status: HashMap<EndpointName, bool>,
    // (success, error_msg)
    pub ticker_status: HashMap<TickerSymbol, HashMap<EndpointName, (bool, Option<String>)>>,
    pub schema_diffs: Vec<SchemaDiff>,
    pub start_time: chrono::DateTime<chrono::Local>,
    pub end_time: Option<chrono::DateTime<chrono::Local>>,
}

impl ExecutionResults {
    pub fn new() -> Self {
        Self {
            market_status: HashMap::new(),
            ticker_status: HashMap::new(),
            schema_diffs: Vec::new(),
            start_time: chrono::Local::now(),
            end_time: None,
        }
    }
}

pub fn generate_index(results: &ExecutionResults, out_dir: &Path) -> Result<()> {
    let mut path = out_dir.to_path_buf();
    path.push("index.md");
    tracing::info!("Writing index to {:?}", path);
    
    let mut file = std::fs::File::create(&path)?;
    
    writeln!(file, "# Alpha Vantage Explorer Report")?;
    writeln!(file, "")?;
    writeln!(file, "**Generated:** {}", results.start_time.format("%Y-%m-%d %H:%M:%S"))?;
    if let Some(end) = results.end_time {
        let duration = end.signed_duration_since(results.start_time);
        writeln!(file, "**Duration:** {}s", duration.num_seconds())?;
    }
    writeln!(file, "")?;
    
    // Market Section
    writeln!(file, "## Market Overview")?;
    writeln!(file, "| Endpoint | Status | Link |")?;
    writeln!(file, "|---|---|---|")?;
    
    // Sort endpoints for deterministic output
    let mut market_endpoints: Vec<_> = results.market_status.keys().collect();
    market_endpoints.sort_by_key(|e| e.to_string());
    
    for &ep in market_endpoints {
        let success = results.market_status.get(&ep).unwrap_or(&false);
        let status_icon = if *success { "✅" } else { "❌" };
        let link = format!("[View Report](market_{}.md)", ep);
        writeln!(file, "| {} | {} | {} |", ep, status_icon, link)?;
    }
    writeln!(file, "")?;
    
    // Ticker Section
    writeln!(file, "## Ticker Reports")?;
    writeln!(file, "| Ticker | Success Rate | Link |")?;
    writeln!(file, "|---|---|---|")?;
    
    let mut tickers: Vec<_> = results.ticker_status.keys().collect();
    tickers.sort_by(|a, b| a.as_str().cmp(b.as_str()));
    
    for ticker in tickers {
        let endpoint_map = results.ticker_status.get(ticker).unwrap();
        let total = endpoint_map.len();
        let passed = endpoint_map.values().filter(|(s, _)| *s).count();
        let link = format!("[View Report](tickers/{}.md)", ticker.as_str());
        
        writeln!(file, "| **{}** | {}/{} | {} |", ticker.as_str(), passed, total, link)?;
    }
    writeln!(file, "")?;
    
    // Schema Diff Section
    if !results.schema_diffs.is_empty() {
        writeln!(file, "## Schema Discrepancies")?;
        
        for diff in &results.schema_diffs {
            if diff.differences.is_empty() {
                continue;
            }
            
            writeln!(file, "### {}", diff.endpoint)?;
            writeln!(file, "> **Union Headers:** {}", diff.union_headers.len())?;
            writeln!(file, "")?;
            
            writeln!(file, "| Ticker | Missing Fields |")?;
            writeln!(file, "|---|---|")?;
            
            let mut diff_tickers: Vec<&TickerSymbol> = diff.differences.keys().collect();
            diff_tickers.sort_by(|a, b| a.as_str().cmp(b.as_str()));
            
            for t in diff_tickers {
                let (missing, _) = diff.differences.get(t).unwrap();
                let missing_str = if missing.is_empty() { 
                    "-".to_string() 
                } else {
                    missing.join(", ")
                };
                writeln!(file, "| {} | {} |", t.as_str(), missing_str)?;
            }
            writeln!(file, "")?;
        }
    }
    
    Ok(())
}
