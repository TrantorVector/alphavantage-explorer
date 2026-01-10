#!/bin/bash
# Basic usage example using Mock Mode (no API key required)

echo "Running Alpha Vantage Explorer in MOCK MODE..."
echo "This will verify the report generation logic without hitting the API."

# Run for Apple and NVDA
cargo run --bin alphavantage_cli -- --symbols AAPL,NVDA --out-dir ./out_mock

echo "Done! Check ./out_mock/index.md for the report."
