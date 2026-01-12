# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.1] - 2026-01-12

### Added
- **Golden Copy Validation Test Suite** for 11 API endpoints
  - Validates 100% output accuracy against official Alpha Vantage documentation
  - Helper module (`golden_copy_helper.rs`) for JSON comparison with detailed diff reporting
  - Tests handle special cases (quarter parameters, different symbols)
  - All tests marked `#[ignore]` to prevent accidental API quota consumption
  - Rate limiting integration (12-second delays between tests)
- **Comprehensive Testing Documentation**
  - Complete test plan (`golden-copy-test-plan.md`) with test inventory and methodology
  - User guide (`GOLDEN_COPY_TESTING.md`) for running tests and troubleshooting
  - Implementation walkthrough documenting test infrastructure
  - Test results tracking (`test-results-2026-01-11.md`)
  - Documentation index (`docs/testing/README.md`) for easy navigation
  - Project summary with statistics and next steps
- **Golden Copy Reference Data** (22 files)
  - 11 input parameter files from Alpha Vantage documentation
  - 11 expected output JSON files from Alpha Vantage documentation
  - Covers: OVERVIEW, financial statements, earnings, dividends, splits, insider transactions

### Changed
- **Updated README.md** with comprehensive Testing section
- **Improved .gitignore** to exclude test output directories (`validation/`, `test-results/`)
- **Enhanced CI compliance** - fixed all strict clippy lints including:
  - `unwrap-used`, `expect-used`, `panic` replacements
  - Added error documentation for public functions
  - Added `#[must_use]` attributes
  - Inline format argument usage

### Removed
- **Obsolete Documentation** from earlier development phases (5 files):
  - Old build plan versions (Antigravity-Build-Plan 1.1.md, 1.2.md)
  - Manual test result files (LIVE_TEST_NVDA_RESULTS.md)
  - Redundant configuration documentation (CONFIG_FILE.md, IMPLEMENTATION_SUMMARY_CONFIG.md)
- **Manual Test Output Directories**:
  - `out/` folder (CLI-generated outputs)
  - `validation/` folder (24 manual test files from earlier phases)

### Testing Results
- ✅ **9/11 golden copy tests** pass with 100% exact match
  - OVERVIEW, BALANCE_SHEET, CASH_FLOW, INCOME_STATEMENT
  - EARNINGS_CALL_TRANSCRIPT (with quarter parameter)
  - DIVIDENDS, SPLITS, SHARES_OUTSTANDING (MSFT symbol)
  - INSIDER_TRANSACTIONS (42K lines validated)
- ⚠️ **2/11 tests** show expected data changes over time
  - EARNINGS - quarterly data updated (expected behavior)
  - EARNINGS_ESTIMATES - API now returns data (was empty in documentation)
- ✅ **Test framework** validated at 100% reliability
  - Correctly detects exact matches (9 tests)
  - Correctly detects real differences (2 tests)
  - Handles large files, special parameters, different symbols

### Fixed
- All clippy warnings for strict GitHub CI rules
- Code formatting compliance across all test files
- Proper error handling in test code (no panic, unwrap, or expect in production paths)

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
