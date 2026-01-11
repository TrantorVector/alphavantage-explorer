# Release Notes - v0.2.0: Granular API Commands

**Release Date:** January 11, 2026  
**Type:** Feature Release (Non-Breaking)

## 🎉 What's New

### Granular Command Mode

The biggest feature in v0.2.0 is **granular mode** - you can now fetch individual endpoints for specific symbols with full data retention!

```bash
# Old way (bulk mode - still works!)
alphavantage_cli --symbols AAPL,NVDA --out-dir ./report

# New way (granular mode)
alphavantage_cli overview --symbol AAPL
alphavantage_cli income-statement --symbol NVDA --output ./financials
alphavantage_cli news-sentiment --symbol MSFT --limit 100
```

**Key Benefits:**
- ✅ **Full Output Retention** - No truncation! All rows included in markdown reports
- ✅ **Faster** - Fetch only what you need
- ✅ **Timestamped Files** - Easy to track when data was fetched
- ✅ **Flexible** - Use custom output directories per command

### 13 New Granular Commands

All company-specific endpoints now have dedicated subcommands:

| Command | Description |
|---------|-------------|
| `overview` | Company overview and fundamentals |
| `income-statement` | Income statement (annual/quarterly) |
| `balance-sheet` | Balance sheet data |
| `cash-flow` | Cash flow statement |
| `earnings` | Earnings data |
| `earnings-estimates` | Earnings estimates |
| `news-sentiment` | News sentiment analysis (supports --limit) |
| `insider-transactions` | Insider trading activity |
| `dividends` | Dividend history |
| `splits` | Stock split history |
| `shares-outstanding` | Shares outstanding over time |
| `earnings-calendar` | Upcoming earnings dates (supports --horizon) |
| `earnings-call-transcript` | Earnings call transcripts (supports --year, --quarter) |

### Enhanced Documentation

- 📖 New [commands manual](docs/manual/commands.md) with detailed examples for each endpoint
- 📋 Comprehensive [output formats guide](docs/manual/output-formats.md)
- 🎯 Updated README with usage modes comparison
- ✅ All functions now have proper error and panic documentation

### Testing & Quality

- 🧪 **57 total tests** (up from 20) - 185% increase in test coverage!
- ✅ **100% CI/CD compliance** - All quality gates passing
- 🔍 Comprehensive test suite:
  - 11 granular executor unit tests
  - 7 granular integration tests
  - 8 backward compatibility tests
  - 11 CSV handler tests
  - 21 parameter validation tests

### Developer Experience

- 🪝 Pre-push git hooks to catch issues before CI
- 📝 Enhanced GitHub Actions workflow with fail-fast
- 🎨 All clippy warnings fixed with proper documentation
- 🏗️ Better code organization with lib.rs for testing

## 📊 Comparison: Bulk vs Granular Mode

| Feature | Bulk Mode | Granular Mode |
|---------|-----------|---------------|
| **Endpoints** | All 13 per symbol | Single endpoint |
| **Output** | Top 3 rows (truncated) | **All rows (full data)** |
| **Use Case** | Comprehensive overview | Targeted data retrieval |
| **Speed** | Slower (13 API calls) | Faster (1 API call) |
| **File Naming** | `tickers/AAPL.md` | `overview_AAPL_20260111_094530.md` |
| **Still Supported?** | ✅ Yes! | ✅ New! |

## 🔄 Migration Guide

### No Changes Required!

This release is **100% backward compatible**. Your existing bulk mode commands will continue to work exactly as before:

```bash
# v0.1.0 - Still works in v0.2.0!
alphavantage_cli --symbols AAPL,NVDA --out-dir ./report
```

### When to Use Each Mode

**Use Bulk Mode when:**
- You want a comprehensive overview of multiple tickers
- You need all endpoints for analysis
- Quick summaries (top 3 rows) are sufficient

**Use Granular Mode when:**
- You need complete data (all rows) for a specific endpoint
- You're fetching data for a single ticker
- You want faster, targeted queries
- You need timestamped output files

## 🐛 Bug Fixes

- Fixed clippy warnings in existing source code
- Improved error handling documentation
- Enhanced parameter validation for quarters, years, and horizons

## 💥 Breaking Changes

**None!** This is a fully backward-compatible release.

## 🚀 Upgrading

### From v0.1.0

No code changes required! Simply update:

```bash
cargo install --path crates/cli --force
```

Or pull the latest from git:

```bash
git pull origin main
cargo build --release
```

### New Installation

```bash
git clone https://github.com/TrantorVector/alphavantage-explorer.git
cd alphavantage-explorer
cargo install --path crates/cli
```

## ⚠️ Known Issues / Limitations

- CSV endpoints (e.g., `earnings-calendar`) do not generate markdown tables - raw CSV only
- Rate limiting is per-application, not per-mode (both modes share the same limit)
- Granular mode uses mock data when `--live-api` is not specified (same as bulk mode)

## 🔮 What's Next (v0.3.0 Ideas)

- Hybrid mode: Fetch specific endpoints for multiple symbols
- Excel export support
- Schema diff tracking for granular mode
- Custom output formatters (e.g., JSON-to-CSV conversion)

## 📚 Documentation

- **README**: https://github.com/TrantorVector/alphavantage-explorer/blob/main/README.md
- **Commands Manual**: docs/manual/commands.md
- **Output Formats**: docs/manual/output-formats.md
- **User Guide**: docs/user-guide.md

## 🙏 Contributors

- **TrantorVector** - Implementation, testing, documentation

## 📦 Release Assets

- `alphavantage_cli` - Release binary for Linux (x86_64)
- `alphavantage_cli.sha256` - Checksum file

## 🔗 Links

- **GitHub Release**: https://github.com/TrantorVector/alphavantage-explorer/releases/tag/v0.2.0
- **Issues**: https://github.com/TrantorVector/alphavantage-explorer/issues
- **Pull Requests**: https://github.com/TrantorVector/alphavantage-explorer/pulls

---

**Questions or Issues?** Please open an issue on GitHub!

**Enjoying the tool?** Give us a ⭐ on GitHub!
