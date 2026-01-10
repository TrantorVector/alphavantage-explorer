# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
