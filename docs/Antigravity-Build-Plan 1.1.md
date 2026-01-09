# Antigravity Build Plan: Alpha Vantage API Explorer
## Business-Focused Implementation Guide

**Document Status:** Build Execution Plan for Google Antigravity  
**Version:** 1.0  
**Date:** 2026-01-08  
**Git Repository:** https://github.com/TrantorVector/alphavantage-explorer.git  
**SSH:** git@github.com:TrantorVector/alphavantage-explorer.git

---

## Executive Summary

This build plan is designed for **Google Antigravity** (AI coding agent) to implement the Alpha Vantage API Explorer as a production-ready CLI tool. The plan is structured in **8 logical phases**, each representing a complete, testable unit of work.

### Critical Constraint: API Rate Limit
**Free tier: 25 API calls per day** (updated from previous assumptions).

**Impact**: With 4 market-wide endpoints + 13 company-specific endpoints × 3 tickers = **43 total API calls**, a full run exceeds the daily limit. Therefore:
- Development must use **mock/stub mode** by default
- Real API mode is opt-in via `--live-api` flag
- Testing primarily uses fixtures and mocked HTTP responses
- Real API validation is limited to smoke tests (1-2 endpoints per day)

### Business Outcomes
1. **Validate API integration** before building the larger investment platform
2. **Schema discovery**: Understand all fields returned by each endpoint
3. **Schema consistency**: Detect field differences across tickers
4. **Reusable foundation**: Core domain logic ready for platform expansion

---

## Table of Contents

1. [Pre-Phase Setup](#pre-phase-setup)
2. [Phase 1: Foundation & Workspace](#phase-1-foundation--workspace)
3. [Phase 2: Core Domain & Ports](#phase-2-core-domain--ports)
4. [Phase 3: HTTP Client & Rate Limiter](#phase-3-http-client--rate-limiter)
5. [Phase 4: JSON Parsing & Schema Conversion](#phase-4-json-parsing--schema-conversion)
6. [Phase 5: Markdown Writer & Artifacts](#phase-5-markdown-writer--artifacts)
7. [Phase 6: CLI Integration & Orchestration](#phase-6-cli-integration--orchestration)
8. [Phase 7: Schema Diffing & Index Generation](#phase-7-schema-diffing--index-generation)
9. [Phase 8: Documentation & Production Release](#phase-8-documentation--production-release)
10. [Appendix: Git Workflow Template](#appendix-git-workflow-template)

---

## Pre-Phase Setup

### Human Actions (You)

1. **Clone the repository**:
   ```bash
   git clone git@github.com:TrantorVector/alphavantage-explorer.git
   cd alphavantage-explorer
   ```

2. **Set API key as environment variable**:
   ```bash
   export ALPHAVANTAGE_API_KEY="T4LXNLR67DZXFQZ3"
   echo 'export ALPHAVANTAGE_API_KEY="T4LXNLR67DZXFQZ3"' >> ~/.bashrc  # or ~/.zshrc
   ```

3. **Verify Antigravity is available**:
   - Ensure Google Antigravity CLI or IDE integration is set up
   - Confirm Rust toolchain is installed (`rustc --version`)

4. **Initial commit** (if repo is empty):
   ```bash
   git checkout -b main
   echo "# Alpha Vantage API Explorer" > README.md
   git add README.md
   git commit -m "Initial commit"
   git push -u origin main
   ```

---

## Phase 1: Foundation & Workspace

**Branch**: `phase-1-foundation`  
**Goal**: Establish Cargo workspace structure, quality tooling, and CI pipeline  
**Duration**: ~2-4 hours (Antigravity execution time)

### 1.1 Antigravity Instructions

```
@Antigravity

CONTEXT:
You are implementing the Alpha Vantage API Explorer, a CLI tool to validate API endpoints 
and generate human-readable Markdown reports. This is Phase 1 of 8.

REQUIREMENTS:
Read these files to understand the full context:
- PRD-AlphaVantage-API-Explorer-v0.2.md
- Architecture-Plan-AlphaVantage-Explorer.md
- Core-tenets.docx (7 core tenets for planet-scale Rust)
- Consolidated-Architecture-Guide.md

CRITICAL CONSTRAINTS:
1. API Rate Limit: 25 calls/day (free tier)
2. No panics allowed (enforced by clippy)
3. Hexagonal architecture (ports & adapters)
4. All secrets must use secrecy crate

PHASE 1 TASKS:

1.1 Create Cargo workspace structure:
   - Root Cargo.toml as workspace
   - crates/core (domain logic, no I/O dependencies)
   - crates/client (HTTP adapters)
   - crates/cli (application layer)

1.2 Configure workspace dependencies in root Cargo.toml:
   - serde, serde_json, tokio, reqwest, thiserror, anyhow
   - tracing, tracing-subscriber, uuid, chrono, secrecy, strum, clap
   - Workspace lints: deny unwrap_used, expect_used, panic, indexing_slicing

1.3 Create quality tooling configs:
   - clippy.toml (strict linting rules from architecture guide)
   - deny.toml (supply chain security policy)
   - rustfmt.toml (code formatting)
   - .gitignore (Rust template + out/ directory)

1.4 Create GitHub Actions CI pipeline (.github/workflows/ci.yml):
   - Run cargo fmt --check
   - Run cargo clippy --workspace -- -D warnings
   - Run cargo deny check
   - Run cargo test --workspace
   - Run cargo build --release

1.5 Create docs structure:
   - docs/prd/alpha-vantage-api-explorer-v0.2.md (copy from attached)
   - docs/architecture/architecture-plan.md (copy from attached)
   - docs/adr/ (empty, for future ADRs)

1.6 Create basic README.md:
   - Project overview
   - Build instructions
   - Usage example (placeholder)
   - API key setup instructions

1.7 Add API key to .env.example (not .env):
   ALPHAVANTAGE_API_KEY=your_key_here

VERIFICATION:
After implementation, run:
1. cargo check --workspace (should compile)
2. cargo clippy --workspace -- -D warnings (should pass)
3. cargo fmt --check (should pass)
4. Tree structure should match Architecture-Plan-AlphaVantage-Explorer.md Section 4.1

OUTPUT:
Report the following in human-readable format:
- ✅ Workspace structure created (list all crates)
- ✅ Tooling configs created (list files)
- ✅ CI pipeline configured
- ✅ Documentation structure created
- ✅ All quality checks passing

If any issues encountered, report them with proposed solutions.
```

### 1.2 Human Verification Checklist

After Antigravity completes Phase 1:

- [ ] Run `tree -L 3` to verify structure matches architecture
- [ ] Run `cargo check --workspace` → should compile with no errors
- [ ] Run `cargo clippy --workspace -- -D warnings` → should pass
- [ ] Review CI workflow file → ensure all gates are present
- [ ] Check README is clear and professional

### 1.3 Git Workflow

```bash
# Review Antigravity's work
git status
git diff

# If satisfied, commit and push
git add .
git commit -m "Phase 1: Foundation and workspace structure

- Created Cargo workspace with core/client/cli crates
- Configured clippy, deny, rustfmt for quality enforcement
- Set up GitHub Actions CI pipeline
- Added project documentation structure"

git push -u origin phase-1-foundation

# Create PR on GitHub, review, merge
# Then sync locally:
git checkout main
git pull origin main
```

---

## Phase 2: Core Domain & Ports

**Branch**: `phase-2-core-domain`  
**Goal**: Implement pure domain logic with NewTypes, error hierarchy, and port traits  
**Duration**: ~3-5 hours

### 2.1 Antigravity Instructions

```
@Antigravity

PHASE 2: Core Domain & Ports

CONTEXT:
Implement the pure domain layer (crates/core) with zero I/O dependencies.
This crate defines the "language" of the system using NewTypes and traits.

REFERENCE:
- Architecture-Plan-AlphaVantage-Explorer.md Section 5 (Domain Model)
- Architecture-Plan-AlphaVantage-Explorer.md Section 7 (Ports & Adapters)
- Core-tenets.docx Tenet 1 (Security via Typestate)

TASKS:

2.1 Implement NewTypes in crates/core/src/domain/:
   - ticker.rs: TickerSymbol (validated: 1-5 uppercase alphanumeric)
   - api_key.rs: ApiKey (wrapped in secrecy::Secret)
   - endpoint.rs: EndpointName enum (all 16 endpoints as variants)
   - schema_table.rs: SchemaTable struct (title, headers, rows, total_records)
   - market_data.rs: ApiResponse, ApiError, ErrorKind enums

2.2 Implement validation logic:
   - TickerSymbol::new() should validate length and characters
   - ApiKey should never be Debug-printed (use secrecy)
   - EndpointName should have function_name() and is_market_wide() methods

2.3 Create error hierarchy in crates/core/src/error.rs:
   - ExplorerError enum using thiserror
   - Variants: Network, HttpStatus, ProviderError, Parse, RateLimited, Validation, Io, Json
   - Implement From<std::io::Error> and From<serde_json::Error>
   - Type alias: pub type Result<T> = std::result::Result<T, ExplorerError>;

2.4 Define port traits in crates/core/src/ports/:
   - api_client.rs: ApiClient trait with fetch_market_endpoint() and fetch_ticker_endpoint()
   - markdown_writer.rs: MarkdownWriter trait with write_table(), write_error(), flush_to_file()
   - json_persister.rs: JsonPersister trait with save_raw_json()
   - schema_analyzer.rs: SchemaAnalyzer trait with compute_schema_diff()

2.5 Write comprehensive unit tests:
   - Test TickerSymbol validation (reject lowercase, special chars, >5 length)
   - Test ApiKey Debug formatting (should show [REDACTED])
   - Property-based tests using proptest: TickerSymbol::new() never panics
   - Test error conversions

2.6 Add doc comments to ALL public types and functions:
   - Explain WHY, not just WHAT
   - Include usage examples in doc tests
   - Reference business requirements where applicable

CRITICAL: crates/core/Cargo.toml MUST NOT include:
- reqwest, tokio::net, sqlx, axum (I/O crates)
- Only pure logic dependencies: serde, thiserror, uuid, chrono, secrecy, strum

VERIFICATION:
1. cargo test -p core (all tests pass)
2. cargo clippy -p core -- -D warnings (no warnings)
3. cargo doc -p core --open (docs are comprehensive)
4. Verify crates/core/Cargo.toml has no I/O dependencies

OUTPUT:
Report:
- ✅ NewTypes implemented (list each with validation rules)
- ✅ Error hierarchy created (list variants)
- ✅ Port traits defined (list each with method signatures)
- ✅ X unit tests passing
- ✅ X property-based tests passing
- ✅ Doc coverage: X% of public items documented
- ✅ Zero clippy warnings
- ⚠️ Any design decisions or trade-offs made
```

### 2.2 Perform the folllowing Verification Checklist

- [ ] Run `cargo test -p core` → all tests pass
- [ ] Run `cargo doc -p core --open` → review documentation quality
- [ ] Spot-check: Open `crates/core/src/domain/ticker.rs` → validate code quality
- [ ] Verify `ApiKey` Debug output shows `[REDACTED]`

### 2.3 Git Workflow

```bash
git checkout main
git pull
git checkout -b phase-2-core-domain

# After Antigravity completes
git add .
git commit -m "Phase 2: Core domain logic and ports

- Implemented validated NewTypes (TickerSymbol, ApiKey, EndpointName)
- Created typed error hierarchy with thiserror
- Defined port traits for hexagonal architecture
- Added comprehensive unit tests and property-based tests
- 100% doc coverage for public APIs"

git push -u origin phase-2-core-domain
# PR, review, merge, sync to main
```

---

## Phase 3: HTTP Client & Rate Limiter

**Branch**: `phase-3-http-client`  
**Goal**: Implement Alpha Vantage HTTP client with rate limiting, retries, and timeout handling  
**Duration**: ~4-6 hours

### 3.1 Antigravity Instructions

```
@Antigravity

PHASE 3: HTTP Client & Rate Limiter

CONTEXT:
Implement the HTTP adapter (crates/client) that calls Alpha Vantage APIs.
This adapter implements the ApiClient port defined in crates/core.

CRITICAL CONSTRAINT:
Free tier rate limit is 25 API calls per day (not per minute).
We need TWO modes:
1. Mock mode (default): Uses JSON fixtures, no real API calls
2. Live mode (--live-api flag): Real API calls, respects 25/day limit

TASKS:

3.1 Implement rate limiter in crates/client/src/rate_limiter.rs:
   - Token bucket algorithm
   - Config: 25 tokens total, refills at midnight UTC
   - wait() method: blocks until token available or aborts if budget exhausted
   - Log remaining token count after each call
   - Persist token state to ~/.alphavantage-explorer-tokens.json to survive restarts
   - Handle file permission errors gracefully. If ~ is not writable, fall back to ./tokens.json in the current directory.

3.2 Implement retry policy in crates/client/src/retry_policy.rs:
   - Exponential backoff with jitter
   - Retry on: Network errors, HTTP 429, HTTP 5xx
   - Do NOT retry on: HTTP 4xx (except 429)
   - Max 2 retries
   - Log each retry attempt

3.3 Implement HTTP client in crates/client/src/http_client.rs:
   - Struct: AlphaVantageClient implementing ApiClient trait
   - Use reqwest with 10s timeout
   - Build URL with query params (function, symbol, apikey)
   - Detect Alpha Vantage "soft errors" in JSON (e.g., "Error Message" field)
   - Parse JSON into ApiResponse or ApiError
   - Instrument with #[tracing::instrument]
   - If the user supplies the literal key string 'demo', bypass the Rate Limiter entirely." This allows you to test "Live Mode" logic endlessly using IBM without burning your personal quota.

3.4 Implement mock client in crates/client/src/mock_client.rs:
   - Struct: MockAlphaVantageClient implementing ApiClient trait
   - Load JSON fixtures from crates/client/fixtures/<endpoint>_<ticker>.json
   - Simulate realistic delays (100-500ms)
   - Return mock ApiResponse with fixture data

3.5 Create JSON fixtures in crates/client/fixtures/:
   - Create 5-6 representative fixtures (not all 43 endpoints):
     - market/TOP_GAINERS_LOSERS.json
     - tickers/AAPL/OVERVIEW.json
     - tickers/AAPL/INCOME_STATEMENT.json (with annualReports array)
     - tickers/AAPL/BALANCE_SHEET.json (with quarterlyReports array)
     - tickers/AAPL/NEWS_SENTIMENT.json (with feed array)
   - Use realistic data structure from Alpha Vantage API docs
   - Do not invent JSON fields. Copy exact JSON examples provided in alpha-vantage-specific-sections-API.md or use the demo API key URL examples to fetch real initial data.

3.6 Write integration tests using wiremock:
   - Test successful API call
   - Test HTTP 429 triggers retry
   - Test timeout handling
   - Test soft error detection
   - Mock tests: verify fixtures load correctly

3.7 Add client factory in crates/client/src/lib.rs:
   - pub fn create_client(mode: ClientMode, api_key: ApiKey) -> Box<dyn ApiClient>
   - ClientMode enum: Mock, Live
   - Returns MockClient or HttpClient based on mode

VERIFICATION:
1. cargo test -p client (all tests pass)
2. cargo clippy -p client -- -D warnings
3. Verify fixtures parse as valid JSON
4. Test rate limiter: simulate 26 calls, verify 26th is rejected

OUTPUT:
Report:
- ✅ Rate limiter implemented (token bucket, 25/day, persists state)
- ✅ Retry policy implemented (exponential backoff, 2 max retries)
- ✅ HTTP client implemented (ApiClient trait)
- ✅ Mock client implemented with X fixtures
- ✅ X integration tests passing
- ✅ Token state persistence tested
- ⚠️ Any issues with rate limit persistence or HTTP handling
```

### 3.2 Preform Verification Checklist

- [x] Run `cargo test -p client` → all tests pass
- [x] cargo clippy -p client -- -D warnings
- [x] cargo fmt -p client -- --check
- [x] Check `crates/client/fixtures/` → verify realistic JSON
- [x] Review rate limiter logs → confirm token tracking
- [x] Test token persistence: run test, kill, restart → tokens preserved

### 3.3 Git Workflow

```bash
git checkout main
git pull
git checkout -b phase-3-http-client

# After Antigravity completes
git add .
git commit -m "Phase 3: HTTP client and rate limiter

- Implemented token bucket rate limiter (25 calls/day)
- Added retry policy with exponential backoff
- Created Alpha Vantage HTTP client with timeout handling
- Implemented mock client with JSON fixtures for development
- Token state persistence across restarts
- Comprehensive integration tests with wiremock"

git push -u origin phase-3-http-client
# PR, review, merge, sync to main
```

---

## Phase 4: JSON Parsing & Schema Conversion

**Branch**: `phase-4-json-parsing`  
**Goal**: Convert dynamic JSON responses into SchemaTable structures  
**Duration**: ~3-4 hours

### 4.1 Antigravity Instructions

```
@Antigravity

PHASE 4: JSON Parsing & Schema Conversion

CONTEXT:
Implement logic to convert serde_json::Value (dynamic JSON) into SchemaTable structures.
This is the core "schema discovery" logic that makes the explorer flexible.

REFERENCE:
- Architecture-Plan-AlphaVantage-Explorer.md Section 5.2 (SchemaTable)
- PRD Section: Output & artifacts → Markdown table formatting rules

TASKS:

4.1 Implement JSON parser in crates/core/src/logic/json_to_table.rs:
   - Function: pub fn parse_json_to_tables(endpoint: EndpointName, json: Value) -> Result<Vec<SchemaTable>>
   - Detect payload type: Map (single object) vs Array (list of objects)
   - For Map: create 2-column table (Field | Value)
   - For Array: extract headers from first object, create multi-column table
   - Handle nested structures: flatten or create separate tables
   - Truncate rows to top 3
   - Record total_records before truncation

4.2 Implement endpoint-specific parsing strategies:
   - OVERVIEW: Map → Field/Value table
   - INCOME_STATEMENT: Extract annualReports and quarterlyReports arrays
   - BALANCE_SHEET: Extract annualReports and quarterlyReports arrays
   - CASH_FLOW: Extract annualReports and quarterlyReports arrays
   - NEWS_SENTIMENT: Extract feed array
   - TOP_GAINERS_LOSERS: Extract top_gainers, top_losers, most_actively_traded arrays
   - Handle missing/null fields gracefully

4.3 Implement helper functions:
   - extract_headers(object: &Map) -> Vec<String>
   - extract_row(object: &Map, headers: &[String]) -> Vec<String>
   - flatten_nested(value: &Value) -> String (for nested objects/arrays)

4.4 Write comprehensive unit tests:
   - Test map-like payload (OVERVIEW fixture)
   - Test array-like payload (INCOME_STATEMENT fixture)
   - Test nested structures
   - Test missing fields (should use empty string or "N/A")
   - Test truncation (verify rows.len() == 3 when total_records > 3)

4.5 Property-based tests:
   - For any valid JSON, parse_json_to_tables() never panics
   - Headers always match row lengths

VERIFICATION:
1. cargo test -p core --lib json_to_table (all tests pass)
2. Test with real fixtures from Phase 3
3. Verify truncation works correctly
4. Verify nested objects are handled gracefully

OUTPUT:
Report:
- ✅ JSON parser implemented for X endpoint types
- ✅ Handles map-like and array-like payloads
- ✅ Nested structure flattening implemented
- ✅ X unit tests passing
- ✅ Property-based tests confirm no panics
- 📊 Sample output for OVERVIEW and INCOME_STATEMENT (show table structure)
```

### 4.2 Run this Verification Checklist

- [ ] Run `cargo test -p core --lib json_to_table` → all tests pass
- [ ] cargo clippy -p core -- -D warnings
- [ ] cargo fmt -p core -- --check
- [ ] Review test output → verify table structures look correct
- [ ] Spot-check: parse INCOME_STATEMENT fixture → verify all columns present

### 4.3 Git Workflow

```bash
git checkout main
git pull
git checkout -b phase-4-json-parsing

# After Antigravity completes
git add .
git commit -m "Phase 4: JSON parsing and schema conversion

- Implemented dynamic JSON to SchemaTable conversion
- Support for map-like and array-like payloads
- Endpoint-specific parsing strategies
- Nested structure handling with flattening
- Comprehensive unit and property-based tests"

git push -u origin phase-4-json-parsing
# PR, review, merge, sync to main
```

---

## Phase 5: Markdown Writer & Artifacts

**Branch**: `phase-5-markdown-writer`  
**Goal**: Implement Markdown report generation and raw JSON persistence  
**Duration**: ~3-4 hours

### 5.1 Antigravity Instructions

```
@Antigravity

PHASE 5: Markdown Writer & Artifacts

CONTEXT:
Implement adapters for writing Markdown reports and persisting raw JSON.
These implement the MarkdownWriter and JsonPersister ports.

REFERENCE:
- Architecture-Plan-AlphaVantage-Explorer.md Section 7.2 (Adapter Implementations)
- PRD Section: Output & artifacts

TASKS:

5.1 Implement Markdown writer in crates/client/src/markdown_writer.rs:
   - Struct: MarkdownWriterImpl with internal buffer (String)
   - write_table(&SchemaTable): Format as Markdown table
     - Write title as ## heading
     - Write table with headers and separator line
     - Write rows
     - If truncated, add note: "(Showing 3 of X total records)"
   - write_error(error_msg): Format as error block
     - Use > blockquote or <details> tag
   - write_heading(text, level): Write Markdown heading (# or ## or ###)
   - flush_to_file(path): Write buffer to file, create parent dirs

5.2 Implement JSON persister in crates/client/src/json_persister.rs:
   - Struct: FileSystemJsonPersister
   - save_raw_json(endpoint, ticker, json):
     - Build path: out/raw/market/<endpoint>.json or out/raw/tickers/<ticker>/<endpoint>.json
     - Create parent directories
     - Write pretty-printed JSON
     - Check size limit (default 5MB), truncate or skip if exceeded
   - Log each save operation

5.3 Write unit tests:
   - Test table formatting matches expected Markdown
   - Test error block formatting
   - Test file writing (use tempdir)
   - Test JSON persistence with size limit
   - Use insta for snapshot testing Markdown output

5.4 Create example reports in tests/snapshots/:
   - Generate sample Markdown for OVERVIEW
   - Generate sample Markdown for INCOME_STATEMENT
   - Snapshot test these outputs

VERIFICATION:
1. cargo test -p client markdown_writer (all tests pass)
2. cargo test -p client json_persister (all tests pass)
3. Review snapshots: cargo insta review
4. Verify Markdown renders correctly (paste into Markdown viewer)

OUTPUT:
Report:
- ✅ Markdown writer implemented
- ✅ JSON persister implemented with size limits
- ✅ X unit tests passing
- ✅ Snapshot tests created for Markdown output
- 📄 Show example Markdown output (paste a sample table)
```

### 5.2 Human Verification Checklist

- [ ] Run `cargo test -p client markdown` → all tests pass
- [ ] cargo clippy -p client -- -D warnings
- [ ] cargo fmt -p client -- --check
- [ ] Run `cargo insta review` → accept snapshots if they look good
- [ ] Copy sample Markdown to viewer (e.g., HackMD) → verify it renders beautifully

### 5.3 Git Workflow

```bash
git checkout main
git pull
git checkout -b phase-5-markdown-writer

# After Antigravity completes
git add .
git commit -m "Phase 5: Markdown writer and artifact persistence

- Implemented MarkdownWriter for human-readable reports
- Implemented JsonPersister with size limits
- Created snapshot tests for output validation
- Comprehensive unit tests with tempdir"

git push -u origin phase-5-markdown-writer
# PR, review, merge, sync to main
```

---

## Phase 6: CLI Integration & Orchestration

**Branch**: `phase-6-cli-integration`  
**Goal**: Wire together all components into a working CLI tool  
**Duration**: ~4-6 hours

### 6.1 Antigravity Instructions

```
@Antigravity

PHASE 6: CLI Integration & Orchestration

CONTEXT:
Implement the CLI application layer (crates/cli) that orchestrates all components.
This is where the main execution loop lives.

REFERENCE:
- Architecture-Plan-AlphaVantage-Explorer.md Section 8 (Data Flow & Execution Model)
- PRD Section: CLI requirements

TASKS:

6.1 Implement CLI args in crates/cli/src/cli_args.rs:
   - Use clap derive macros
   - Flags:
     - --symbols <AAPL,NVDA,MU> (default: AAPL,NVDA,MU)
     - --out-dir <path> (default: out/)
     - --api-key <key> (optional, prefer env var)
     - --live-api (flag, enables real API calls)
     - --no-raw (flag, disables raw JSON persistence)
     - --log-format <human|json> (default: human)
     - --log-level <error|warn|info|debug|trace> (default: info)
   - Validate: symbols must parse as TickerSymbol

6.2 Implement config in crates/cli/src/config.rs:
   - Struct: Config
   - Load from CLI args + environment variables
   - Validate API key is present (from env or flag)
   - Determine ClientMode (Mock or Live based on --live-api)

6.3 Implement executor in crates/cli/src/executor.rs:
   - Struct: Executor
   - run(config) -> Result<()>:
     a. Initialize tracing based on log-format
     b. Create API client (Mock or Live)
     c. Create Markdown writers (one per ticker + one for market)
     d. Create JSON persister (if enabled)
     e. Fetch market-wide endpoints (4 endpoints, once each)
     f. For each ticker: fetch company-specific endpoints (13 endpoints)
     g. Handle errors gracefully: log and continue to next endpoint
     h. Write Markdown reports per ticker
   - Log progress after each endpoint
   - Write Markdown reports and raw JSON immediately after each successful endpoint fetch. Do not wait for the loop to finish. If the rate limit is hit, the artifacts for successful calls must already be on disk.

6.4 Implement main.rs:
   - Parse CLI args
   - Build Config
   - Initialize tracing
   - Run executor
   - Handle errors and exit codes (0 on success, 1 on failure)

6.5 Implement progress reporter in crates/cli/src/progress.rs:
   - Log: "Fetching {endpoint} for {ticker}..."
   - Log: "✓ {endpoint} succeeded" or "✗ {endpoint} failed: {error}"
   - Log: "Rate limit: X/25 calls remaining"
   - Print summary at end: "Completed X/Y endpoints successfully"

6.6 Add tracing initialization:
   - Human format: pretty, colored logs to stdout
   - JSON format: structured JSON logs
   - Set log level from CLI flag
   - Add span for each endpoint fetch

6.7 Write integration tests in tests/integration/:
   - Test full run with mock client (should succeed without API calls)
   - Test error handling (simulate network failure)
   - Test output directory structure
   - Verify Markdown files are created

VERIFICATION:
1. cargo build --release (builds successfully)
1a. run cargo fmt -p cli -- --check
1c. run cargo clippy -p cli -- -D warnings
2. ./target/release/alpha-vantage-explorer --help (shows help)
3. Run in mock mode:
   ./target/release/alpha-vantage-explorer --symbols AAPL
   - Should complete in <5 seconds
   - Should create out/tickers/AAPL.md
   - Should create out/raw/ files (if not --no-raw)
4. cargo test --workspace (all tests pass)

OUTPUT:
Report:
- ✅ CLI implemented with clap
- ✅ Configuration loading from args + env
- ✅ Executor orchestrates all components
- ✅ Tracing initialized (human and JSON formats)
- ✅ Progress reporting implemented
- ✅ X integration tests passing
- 🎯 Sample run output (show first 20 lines of log output)
- 📁 Sample output structure (tree out/)
```

### 6.2 Run the Verification Checklist if not already done

- [ ] Run `cargo build --release` → builds successfully
- [ ] Run `cargo fmt -p cli -- --check` → no changes
- [ ] Run `cargo clippy -p cli -- -D warnings` → no warnings
- [ ] Run `./target/release/alpha-vantage-explorer --help` → shows usage
- [ ] Run mock mode: `./target/release/alpha-vantage-explorer --symbols AAPL`
- [ ] Check `out/tickers/AAPL.md` exists and looks good
- [ ] Run with live API (careful, uses tokens): `./target/release/alpha-vantage-explorer --symbols AAPL --live-api` (test 1 endpoint only)

### 6.3 Git Workflow

```bash
git checkout main
git checkout -b phase-6-cli-integration

# After Antigravity completes
git add .
git commit -m "Phase 6: CLI integration and orchestration

- Implemented clap-based CLI with all required flags
- Created config loading from args and environment
- Built main executor with complete data flow
- Added tracing with human and JSON formats
- Progress reporting with rate limit tracking
- Integration tests for end-to-end validation"

git push -u origin phase-6-cli-integration
# PR, review, merge, sync to main
```

---

## Phase 7: Schema Diffing & Index Generation

**Branch**: `phase-7-schema-diff`  
**Goal**: Implement schema comparison across tickers and generate index report  
**Duration**: ~3-4 hours

### 7.1 Antigravity Instructions

```
@Antigravity

PHASE 7: Schema Diffing & Index Generation

CONTEXT:
Implement schema comparison to detect field differences across tickers.
Generate index.md as the "table of contents" for all reports.

REFERENCE:
- Architecture-Plan-AlphaVantage-Explorer.md Section 7.1 (Ports)
- PRD Section: Schema-diffing between tickers

TASKS:

7.1 Implement schema analyzer in crates/client/src/schema_analyzer.rs:
   - Struct: SchemaAnalyzerImpl implementing SchemaAnalyzer trait
   - compute_schema_diff(endpoint, tables_by_ticker) -> Result<SchemaDiff>:
     - Compute union of all headers across tickers
     - For each ticker, compute missing and extra fields
     - Sort fields alphabetically
   - Helper: compute_union_headers(tables) -> Vec<String>
   - Helper: compute_diff(ticker_headers, union_headers) -> (missing, extra)

7.2 Implement index generator in crates/cli/src/index_generator.rs:
   - Function: generate_index(results: &ExecutionResults, out_dir: &Path) -> Result<()>
   - Structure:
     a. Title and run metadata (timestamp, version, symbols)
     b. Market-wide endpoints summary table (Endpoint | Status | Link)
     c. Per-ticker summary table (Ticker | Successful | Failed | Link)
     d. Schema diff section (one table per endpoint showing field differences)
   - Write to out/index.md

7.3 Update executor to collect results:
   - Struct: ExecutionResults { market_results, ticker_results, schema_diffs }
   - Track success/failure for each endpoint
   - After all fetches, compute schema diffs
   - Generate index.md at end

7.4 Write unit tests:
   - Test schema diff with matching schemas (no differences)
   - Test schema diff with missing fields in one ticker
   - Test schema diff with extra fields in one ticker
   - Test index generation with mock results
   - Create a specific test fixture set where NVDA has an extra field GPU_Segment_Revenue and AAPL is missing DividendYield. Verify compute_schema_diff correctly identifies these exact differences

7.5 Integration test:
   - Run full executor in mock mode
   - Verify index.md is created
   - Verify index contains all expected sections
   - Snapshot test index.md structure

VERIFICATION:
1. cargo test -p client schema_analyzer (tests pass)
2. cargo test -p cli index_generator (tests pass)
3. Run full tool in mock mode with 3 tickers
4. Verify out/index.md exists and is well-formatted
5. Verify schema diff tables show meaningful comparisons

OUTPUT:
Report:
- ✅ Schema analyzer implemented
- ✅ Index generator implemented
- ✅ Executor updated to collect results
- ✅ X unit tests passing
- ✅ Integration test confirms index creation
- 📄 Sample index.md excerpt (show first 50 lines)
```

### 7.2 Human Verification Checklist

- [ ] Run full tool: `cargo run -- --symbols AAPL,NVDA,MU`
- [ ] Open `out/index.md` in Markdown viewer → verify it's beautiful
- [ ] Check schema diff tables → confirm they show meaningful data
- [ ] Verify links in index work (clicking takes you to right report)

### 7.3 Git Workflow

```bash
git checkout main
git pull
git checkout -b phase-7-schema-diff

# After Antigravity completes
git add .
git commit -m "Phase 7: Schema diffing and index generation

- Implemented schema analyzer to detect field differences
- Created index.md generator as navigation hub
- Executor collects results and computes diffs
- Comprehensive unit and integration tests
- Index includes run metadata, summaries, and diff tables"

git push -u origin phase-7-schema-diff
# PR, review, merge, sync to main
```

---

## Phase 8: Documentation & Production Release

**Branch**: `phase-8-documentation`  
**Goal**: Finalize documentation, examples, and prepare for v0.1.0 release  
**Duration**: ~2-3 hours

### 8.1 Antigravity Instructions

```
@Antigravity

PHASE 8: Documentation & Production Release

CONTEXT:
Final phase: create comprehensive documentation, usage examples, and prepare for release.

TASKS:

8.1 Update README.md:
   - Project overview (business purpose)
   - Features list
   - Installation instructions (cargo install --path .)
   - Quick start guide
   - Usage examples:
     - Mock mode (default)
     - Live API mode with rate limit warnings
     - Custom symbols
   - Output structure explanation
   - API key setup instructions
   - Contributing guide placeholder
   - License (add MIT or Apache-2.0)

8.2 Create docs/user-guide.md:
   - Detailed usage scenarios
   - Understanding the output reports
   - Schema diff interpretation
   - Troubleshooting common issues
   - Rate limit management strategies

8.3 Create examples/ directory:
   - examples/basic-usage.sh (script demonstrating mock mode)
   - examples/live-api-usage.sh (script for live API with warnings)
   - examples/custom-symbols.sh (custom ticker list)

8.4 Update all doc comments:
   - Ensure every public function has doc comment with examples
   - Run cargo doc --open and review
   - Fix any broken doc links

8.5 Create CHANGELOG.md:
   - Document v0.1.0 initial release
   - List all features implemented
   - Known limitations (25 calls/day)

8.6 Add LICENSE file:
   - Choose license (MIT recommended for open source)

8.7 Final quality checks:
   - cargo clippy --workspace -- -D warnings (no warnings)
   - cargo test --workspace (all pass)
   - cargo build --release (builds)
   - cargo doc --workspace --no-deps (generates docs)
   - Run full tool in mock mode → verify output
   - Run full tool with live API (1 ticker only) → verify works

8.8 Tag release:
   - git tag v0.1.0
   - Create GitHub release notes

VERIFICATION:
1. README is clear and professional
2. All docs render correctly
3. Examples run without errors
4. cargo doc output is comprehensive
5. Ready for public use

OUTPUT:
Report:
- ✅ README.md updated with comprehensive guide
- ✅ User guide created
- ✅ Usage examples created
- ✅ CHANGELOG and LICENSE added
- ✅ All quality checks passing
- ✅ Documentation complete
- 🎉 Ready for v0.1.0 release
```

### 8.2 Human Verification Checklist

- [ ] Read README.md → ensure it's clear for new users
- [ ] Run examples → verify they work
- [ ] Review generated docs: `cargo doc --workspace --no-deps --open`
- [ ] Run final smoke test with live API (1 endpoint only)
- [ ] Review GitHub repo → ensure it looks professional

### 8.3 Git Workflow & Release

```bash
git checkout main
git pull
git checkout -b phase-8-documentation

# After Antigravity completes
git add .
git commit -m "Phase 8: Documentation and production release

- Comprehensive README with quick start guide
- Detailed user guide and troubleshooting
- Usage examples for common scenarios
- CHANGELOG and LICENSE files
- Complete API documentation
- Ready for v0.1.0 release"

git push -u origin phase-8-documentation

# PR, review, merge
git checkout main
git pull

# Tag release
git tag -a v0.1.0 -m "Initial release: Alpha Vantage API Explorer v0.1.0"
git push origin v0.1.0

# Create GitHub release with release notes
```

---

## Appendix: Git Workflow Template

### Standard Workflow for Each Phase

```bash
# Start of phase
git checkout main
git pull origin main
git checkout -b phase-X-feature-name

# During development (Antigravity working)
# ... Antigravity makes changes ...

# Review Antigravity's work
git status
git diff

# If satisfied, commit
git add .
git commit -m "Phase X: Feature name

- Bullet point of what was done
- Another bullet point
- Specific accomplishments"

# Push to remote
git push -u origin phase-X-feature-name

# On GitHub:
# 1. Create Pull Request
# 2. Review changes (focus on README, output files, test results)
# 3. If CI passes, approve and merge

# After merge, sync locally
git checkout main
git pull origin main

# Delete feature branch
git branch -d phase-X-feature-name
git push origin --delete phase-X-feature-name
```

---

## Business Verification Criteria

After each phase, verify these business outcomes:

### Phase 1-2: Foundation & Domain
- ✅ Code compiles and is well-structured
- ✅ Type system makes invalid states unrepresentable
- ✅ Error messages are clear and actionable

### Phase 3-4: Data Fetching & Parsing
- ✅ Mock mode works without API calls (critical for development)
- ✅ Rate limiter respects 25/day constraint
- ✅ Schema tables show all fields from API responses

### Phase 5-6: Output Generation & CLI
- ✅ Markdown reports are beautiful and easy to read
- ✅ CLI is intuitive and self-documenting
- ✅ Progress feedback is clear during execution

### Phase 7-8: Schema Analysis & Release
- ✅ Schema diffs highlight field inconsistencies across tickers
- ✅ Index report provides perfect navigation
- ✅ Documentation enables new users to succeed in <5 minutes

---

## Critical Success Factors

### For Antigravity

1. **Follow architecture strictly**: Hexagonal boundaries, no I/O in core
2. **No panics**: Enforced by clippy, every error is typed
3. **Comprehensive testing**: Unit, integration, property-based, and snapshot tests
4. **Beautiful output**: Markdown must render perfectly
5. **Clear logging**: Progress and errors are human-readable

### For You (Business Review)

1. **Focus on outputs**: Review generated Markdown reports, not code
2. **Verify user experience**: Can a new user run the tool successfully?
3. **Check documentation**: Is README clear? Are examples helpful?
4. **Validate business value**: Do reports show schema differences?
5. **Test live API sparingly**: Use mock mode primarily, live API only for final validation

---

## Rate Limit Management Strategy

With only **25 API calls per day**, manage usage carefully:

### Development (Phases 1-7)
- **Use mock mode exclusively**: No real API calls during development
- **Create realistic fixtures**: Ensure fixtures match real API responses

### Testing (Phase 8)
- **Day 1**: Test 3 market-wide endpoints (3 calls)
- **Day 2**: Test 5 company-specific endpoints for AAPL (5 calls)
- **Day 3**: Test 5 company-specific endpoints for NVDA (5 calls)
- **Day 4**: Test remaining endpoints for MU (remaining calls)
- **Day 5**: Full production run (if any quota left)

### Production Use
- **For end users**: Document the 25/day limit clearly
- **Recommendation**: Users should run once per day max
- **Future**: Implement caching to reduce redundant calls

---

## Emergency Procedures

### If API Key is Rate Limited
1. Wait until midnight UTC for quota reset
2. Switch to mock mode immediately
3. Continue development without disruption

### If Tests Fail
1. Antigravity should report failure with context
2. Review test output for root cause
3. If architectural issue, escalate to you
4. If implementation bug, instruct Antigravity to fix

### If CI Pipeline Fails
1. Review CI logs on GitHub
2. Common issues: clippy warnings, formatting, test failures
3. Instruct Antigravity to fix and re-push

---

## Final Checklist: Before v0.1.0 Release

- [ ] All 8 phases completed and merged to main
- [ ] `cargo build --release` succeeds
- [ ] `cargo test --workspace` passes (100% success rate)
- [ ] `cargo clippy --workspace -- -D warnings` passes (zero warnings)
- [ ] `cargo doc --workspace --no-deps` generates complete docs
- [ ] README.md is comprehensive and professional
- [ ] Mock mode runs successfully for all 3 tickers
- [ ] Live API mode validated with at least 1 endpoint per ticker
- [ ] Generated reports (Markdown + JSON) are beautiful
- [ ] Schema diff tables show meaningful data
- [ ] Index.md provides perfect navigation
- [ ] Examples run without errors
- [ ] GitHub repo looks professional (README, LICENSE, .gitignore)
- [ ] v0.1.0 tag created and pushed
- [ ] GitHub release created with notes

---

## Success Metrics

### Technical Metrics
- **Code coverage**: >80% for core logic
- **Test suite**: 100+ tests (unit + integration + property-based)
- **Build time**: <2 minutes for full release build
- **Execution time (mock mode)**: <5 seconds for 3 tickers
- **Zero panics**: Enforced by clippy, tested with fuzzing

### Business Metrics
- **Schema discovery**: All fields from 13+ endpoints documented
- **Field drift detection**: Schema diffs highlight inconsistencies
- **Report quality**: Markdown is presentation-ready
- **Usability**: New user can run tool in <5 minutes from README
- **Reusability**: Core domain types ready for platform expansion

---

## Next Steps (Post-v0.1.0)

After Phase 8 completion and v0.1.0 release:

1. **Gather feedback**: Share tool with beta users
2. **Plan Phase 1 extensions**: Add PostgreSQL persistence
3. **Enhance fixtures**: Add more endpoint examples
4. **Optimize**: Profile and improve performance if needed
5. **Scale planning**: Design for premium API tier (higher rate limits)

---

**End of Build Plan**

This plan is designed for **Antigravity execution** with **business-focused verification**. Each phase is self-contained, testable, and delivers concrete business value.
