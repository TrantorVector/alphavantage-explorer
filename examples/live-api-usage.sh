#!/bin/bash
# Live API usage example
# WARNING: This consumes API credits! 
# Ensure you have your API key set in alphavantage.toml or ALPHAVANTAGE_API_KEY env var.

echo "Running Alpha Vantage Explorer in LIVE MODE..."

if [ ! -f "alphavantage.toml" ]; then
    echo "Warning: alphavantage.toml not found. Make sure your API key is set in environment variables."
fi

# Run for a single symbol to save credits
# This runs ~15 endpoints, leaving ~10 credits for other uses on the Data Tier.
cargo run --bin alphavantage_cli -- --live-api --symbols NVDA --out-dir ./out_live

echo "Done! Check ./out_live/index.md for the report."
