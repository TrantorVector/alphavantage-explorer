#!/bin/bash
# Example showing how to iterate over a custom list of tickers

TICKERS="MSFT,GOOGL,AMZN"

echo "Running reports for custom list: $TICKERS"

# Using Mock mode for safety in this example. Add --live-api to go live.
cargo run --bin alphavantage_cli -- --symbols $TICKERS --out-dir ./out_custom

echo "Report generated at ./out_custom"
