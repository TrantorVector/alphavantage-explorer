mod api_config;
mod cli_args;
mod config;
mod executor;
mod granular_executor;
mod index_generator;
mod progress;

use crate::executor::Executor;
use clap::Parser;
use cli_args::{CliArgs, LogFormat};
use config::Config;
use std::process;
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    // Setup Tracing
    let log_level = tracing::Level::from(args.log_level);
    let subscriber_builder = tracing_subscriber::fmt()
        .with_max_level(log_level)
        .with_span_events(FmtSpan::CLOSE);

    match args.log_format {
        LogFormat::Json => {
            subscriber_builder.json().init();
        }
        LogFormat::Human => {
            subscriber_builder.with_ansi(true).init();
        }
    }

    // Load Config
    let config = match Config::from_args(args.clone()) {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Configuration error: {}", e);
            process::exit(1);
        }
    };

    tracing::info!("Starting Alpha Vantage Explorer");
    tracing::debug!("Loaded configuration: {:?}", config);

    // Check if granular command or bulk mode
    if let Some(command) = args.command {
        // Granular mode: single endpoint
        let client = alphavantage_client::create_client(
            config.client_mode,
            config.daily_limit,
            config.min_delay_ms,
        );
        let granular_exec = granular_executor::GranularExecutor::new(&config, client.as_ref());

        if let Err(e) = granular_exec.execute(&command).await {
            tracing::error!("Granular execution failed: {}", e);
            process::exit(1);
        }
    } else {
        // Bulk mode: all endpoints for provided symbols
        let executor = Executor::new(config);

        if let Err(e) = executor.run().await {
            tracing::error!("Execution failed: {}", e);
            process::exit(1);
        }
    }

    tracing::info!("Done.");
}
