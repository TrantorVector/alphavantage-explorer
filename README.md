# Alpha Vantage API Explorer

**A high-performance CLI tool to validate, explore, and report on Alpha Vantage API data.**

Built with Rust, this tool enables developers and financial analysts to inspect API responses, track rate limits, and generate detailed Markdown reports for stock market data. It follows a Hexagonal Architecture for logic isolation and testability.

## 🚀 Features

- **📊 Comprehensive Reporting**: Generates Markdown reports with tables, JSON schema analysis, and API health checks.
- **⚡ Live & Mock Modes**: Switch seamlessly between live API data and built-in mock data for testing without burning credits.
- **🛡️ Rate Limiting**: Smart token bucket algorithm enforces API limits (default 25 calls/day) with configurable per-request delays.
- **🔍 Schema Validation**: Automatically detects and highlights schema changes or missing fields in API responses.
- **⚙️ Configurable**: Flexible configuration via `alphavantage.toml` for API keys and rate limit settings.
- **🔁 Robust Client**: Features auto-retry policies, timeout handling, and structured logging.

## 📦 Installation

Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.

```bash
# Clone the repository
git clone https://github.com/TrantorVector/alphavantage-explorer.git
cd alphavantage-explorer

# Install the binary
cargo install --path crates/cli
```

## 🚀 Quick Start

### 1. Configure
Create a configuration file:
```bash
cp alphavantage.toml.template alphavantage.toml
# Edit alphavantage.toml to add your API Key
```

### 2. Run (Mock Mode)
Test the tool without using your API key:
```bash
alphavantage_cli --symbols AAPL --out-dir ./report
```

### 3. Run (Live Mode)
Fetch real data (consumes API credits):
```bash
alphavantage_cli --live-api --symbols NVDA,IBM --out-dir ./live-report
```

## 📖 Usage Guide

### Command Line Options

```bash
alphavantage_cli [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `-s, --symbols <SYMBOLS>` | Comma-separated list of stock tickers (e.g., `AAPL,MSFT`). |
| `--live-api` | Enable live API calls. If omitted, uses Mock mode. |
| `-o, --out-dir <PATH>` | directory to save reports (default: `./out`). |
| `--log-format <TYPE>` | Log format: `full`, `compact`, `pretty`, or `json` (default: `pretty`). |

### Rate Limiting

The tool respects the **25 calls/day** limit of the free tier by default.
- It tracks usage in a local state file.
- You can configure custom limits in `alphavantage.toml`:

```toml
[rate_limit]
daily_limit = 25
min_delay_ms = 1000 # 1 second delay between calls
```

## 📂 Output Structure

The tool generates a structured report directory:

```text
out/
├── index.md                 # Dashboard summary
├── market_MARKET_STATUS.md  # Global market endpoint reports
├── tickers/
│   ├── AAPL.md              # Detailed report for AAPL
│   └── IBM.md               # Detailed report for IBM
└── raw/                     # Raw JSON responses (for debugging)
```

## ⚙️ Configuration

You can configure the tool using `alphavantage.toml` in the current directory:

```toml
[api]
api_key = "YOUR_KEY_HERE"

[rate_limit]
daily_limit = 25
min_delay_ms = 1000
```

*Alternatively, the standard `check` validation mode is active during builds.*

## 🏗️ Architecture

- **crates/core**: Domain entities (Ticker, ApiKey) and business logic.
- **crates/client**: `reqwest` HTTP client, Rate Limiter, and Persistence.
- **crates/cli**: Command-line interface and Orchestration.

## 🤝 Contributing

Contributions are welcome! Please ensure you run the quality checks before submitting a PR:

```bash
cargo fmt
cargo clippy
cargo test
```

## 📄 License

This project is licensed under the [MIT License](LICENSE).
