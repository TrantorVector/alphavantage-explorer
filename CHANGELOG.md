# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-01-10

### Added
- **Granular Per-Endpoint Commands**: 13 new subcommands for targeted API calls (`overview`, `income-statement`, `balance-sheet`, etc.)
- **Full Output Retention**: Granular mode retains all rows (no truncation) for complete dataset analysis
- **CSV Passthrough**: Raw CSV output for endpoints like `earnings-calendar` without conversion
- **Dual-Mode Operation**: Seamless switching between bulk mode (all endpoints) and granular mode (single endpoint)
- **Timestamp Utilities**: Auto-generated timestamps for unique file naming per execution
- **Enhanced Documentation**: 
  - Updated README.md with usage modes and endpoint reference
  - Created `docs/manual/commands.md` with comprehensive command reference
  - Created `docs/manual/output-formats.md` explaining JSON vs CSV outputs
- **Parameter Support**: 
  - QuarterParam (Q1-Q4) for earnings data
  - HorizonParam (3month, 6month, 12month) for calendar views
  - Year validation for transcript queries

### Changed
- **CLI Structure**: Migrated to `clap` subcommands for granular endpoint access
- **Markdown Generation**: Configurable row truncation (bulk: 3 rows, granular: all rows)
- **Output Files**: Granular commands use timestamped filenames (e.g., `overview_AAPL_20260110_230045.md`)

### Backward Compatibility
- ✅ **No Breaking Changes**: Existing bulk mode (`--symbols`) fully preserved
- ✅ All v0.1.0 features remain functional

## [0.1.0] - 2026-01-10

### Added
- **Core Logic**: Domain models for Ticker, Endpoint, and API Key validation.
- **CLI**: Feature-rich command-line interface with `clap`.
- **Client**: `reqwest`-based HTTP client with retry policies and rate limiting.
- **Mock Mode**: Simulation mode for testing without API keys.
- **Reporting**: Markdown report generation with tables and schema analysis.
- **Rate Limiting**:
  - Daily limit enforcement (default 25 calls/day).
  - Configurable minimum delay (default 1000ms).
  - Persistent token state tracking.
- **Configuration**: `alphavantage.toml` support for API keys and rate limits.
- **Coverage**: Support for 17+ Alpha Vantage endpoints (Market & Ticker data).

### Changed
- Initial project structure setup with Workspace (cli, client, core).
- Implemented rigorous error handling and structured logging.
