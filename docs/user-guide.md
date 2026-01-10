# User Guide

This guide provides detailed instructions on how to use the Alpha Vantage API Explorer validation tool effectively.

## Table of Contents
1. [Core Concepts](#core-concepts)
2. [Workflow Scenarios](#workflow-scenarios)
3. [Understanding the Output](#understanding-the-output)
4. [Rate Limits & Optimization](#rate-limits--optimization)
5. [Troubleshooting](#troubleshooting)

---

## Core Concepts

### Validation vs. Exploration
The tool is designed for two main purposes:
- **Validation**: Ensuring the API contract (JSON schema) hasn't changed.
- **Exploration**: Quickly viewing the data returned by multiple endpoints for a set of tickers.

### Mock Mode vs. Live Mode
- **Mock Mode**: Uses embedded sample JSON responses. This is the default. Use this to test the report generation, verify file outputs, and understand the tool's behavior without needing an internet connection or API key.
- **Live Mode**: Connects to the real Alpha Vantage API. Requires an API key and consumes your daily rate limit quota.

---

## Workflow Scenarios

### Scenario 1: Initial Setup check
Before using your API key, run a mock validation to ensure the tool is working correctly on your system.
```bash
alphavantage_cli --symbols TEST --out-dir ./test_output
```
Open `./test_output/index.md` in your browser or Markdown viewer to see the result.

### Scenario 2: Validating a New Strategy
If you want to build a strategy around `NVDA` and `AMD`, you first need to know what data fields are available.
Run a live check:
```bash
alphavantage_cli --live-api --symbols NVDA,AMD --out-dir ./chip_stocks
```
This will fetch all supported endpoints (Income Statement, Balance Sheet, Cash Flow, Earnings, etc.) for both tickers.

### Scenario 3: Continuous Integration
You can run the tool in mock mode as part of your CI pipeline to ensure that your reporting logic handles standard API responses correctly.

---

## Understanding the Output

The tool generates a static site structure in the output directory.

### `index.md` (Dashboard)
The main entry point. It lists:
- **Global Market Status**: Status of endpoints that aren't stock-specific (e.g., Top Gainers/Losers).
- **Ticker Summary**: A table showing the success rate (e.g., "13/15 endpoints") for each requested ticker.

### Ticker Reports (e.g., `tickers/AAPL.md`)
For each ticker, a dedicated file is created containing:
- **Table of Contents**: Quick links to each endpoint.
- **Data Tables**: Key financial data parsed into readable Markdown tables.
- **Schema Analysis**:
  - **Missing Fields**: Fields that the tool expects but the API didn't return.
  - **New Fields**: Unexpected fields that appeared in the JSON.
  - **Type Mismatches**: If a field changed from a string to a number, for example.

---

## Rate Limits & Optimization

The Free Tier of Alpha Vantage allows **25 requests per day**. This tool is aggressive—it fetches ~15 endpoints per ticker.

**Warning**: inspecting 2 tickers (NVDA, AMD) involves 30+ requests, which exceeds the daily free limit!

### Management Strategy
1. **Configurable Delay**: The tool waits 1 second between requests by default. You can increase this in `alphavantage.toml` if you have a higher tier that still rate-limits per second.
2. **State Tracking**: The tool remembers how many calls you made today in `~/.alphavantage-explorer-tokens.json`. It will stop automatically when you hit the limit.
3. **Plan Ahead**: Only request 1 ticker at a time if you are on the free tier.

---

## Troubleshooting

### "Rate check failed"
- **Cause**: You exceeded your daily quota or the API is overloaded.
- **Solution**: Wait until tomorrow (UTC midnight) or upgrade your API key.

### "Deserialization Error"
- **Cause**: The API returned data in a format the tool didn't expect (e.g., an error message instead of JSON data, or a changed schema).
- **Solution**: Check the `raw/` directory in the output buffer. Open the JSON file corresponding to the failed endpoint to see the raw error message.

### "Network Error"
- **Cause**: Connection timeout or DNS failure.
- **Solution**: The tool automatically retries 3 times. If it fails, check your internet connection.
