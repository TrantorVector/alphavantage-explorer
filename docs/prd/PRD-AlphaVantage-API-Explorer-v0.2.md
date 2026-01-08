# PRD: Alpha Vantage API Explorer (CLI) v0.2

**Status:** Draft (updated with product decisions)  
**Owner:** User (Product/Architecture)  
**Target language:** Rust  
**Doc-as-code location (recommended):** `docs/prd/alpha-vantage-api-explorer-v0.2.md`  

## Problem statement
Before building a larger investing analysis platform, there is a need for a deterministic, repeatable way to validate a constrained set of Alpha Vantage endpoints, inspect returned schemas (field names + sample values), and produce a human-readable Markdown report.

## Goals
- Validate that each selected endpoint responds successfully (HTTP + JSON parse) for three representative tickers: `AAPL`, `NVDA`, `MU`.
- Produce human-readable Markdown tables that expose the **full set of fields/columns** for each endpoint payload while truncating the data rows to **top 3**.
- Continue execution on errors: log the error and move to the next endpoint / ticker.
- Remain free-tier friendly with strict rate limiting and backoff.
- Align with the “planet-scale” engineering constraints: clear crate boundaries, typed errors, structured logging, no panics, and maintainability.

## Non-goals
- Not building the full investing platform (no persistence DB, no backtesting engine, no web UI).
- Not guaranteeing completeness/accuracy of financial data (this tool explores schema and endpoint behavior).
- No advanced filtering/parameter tuning in v0.2 (except minimal defaults necessary to make endpoints callable).
- No ETF endpoints (e.g., `ETF_PROFILE`) in this phase.

## Product decisions (locked)
- Output organization: **one Markdown file per ticker + a generated index Markdown file**.
- Default usage: **ad-hoc CLI** (not a scheduled job).
- Compliance constraints for `NEWS_SENTIMENT`: **none**.

## Scope: endpoints
Only endpoints explicitly listed below are in scope for v0.2.

### Market-wide (run once)
- `TOP_GAINERS_LOSERS`
- `LISTING_STATUS`
- `EARNINGS_CALENDAR`
- `IPO_CALENDAR`

### Company-specific (iterate tickers: AAPL, NVDA, MU)
- `OVERVIEW`
- `INCOME_STATEMENT`
- `BALANCE_SHEET`
- `CASH_FLOW`
- `EARNINGS`
- `EARNINGS_ESTIMATES`
- `NEWS_SENTIMENT` (no filters)
- `INSIDER_TRANSACTIONS`
- `DIVIDENDS`
- `SPLITS`
- `SHARES_OUTSTANDING`
- `EARNINGS_CALENDAR` (ticker-specific when applicable)
- `EARNINGS_CALL_TRANSCRIPT` (default to “latest available” selection logic)

## API interaction requirements
### Base URL
- `https://www.alphavantage.co/query`

### Authentication
- Accept API key via env var (preferred) and optionally via CLI flag.
- Never write API keys to the report, raw JSON artifacts, or logs.

### Rate limiting
- Enforce a conservative limiter at the API-client boundary to avoid free-tier throttling.
- Implementation: token bucket / leaky bucket; serialize calls (no parallel fanout) unless the limiter is upgraded.

### Timeouts & retries
- Default request timeout: 10s.
- Retry policy:
  - Retry transient network failures up to 2 times with exponential backoff + jitter.
  - On HTTP 429: sleep/backoff and retry once.
  - Do not retry other 4xx.

### Provider-side “soft errors”
- Alpha Vantage sometimes returns error payloads inside JSON even with HTTP 200.
- Detect and treat these as failures (render as error block and continue).

## Output & artifacts
### Output directory layout (locked)
Default output root: `out/`.

- `out/index.md`
- `out/market/market.md` (optional consolidation file for market-wide endpoints)
- `out/tickers/AAPL.md`
- `out/tickers/NVDA.md`
- `out/tickers/MU.md`

### Raw JSON storage (recommendation: YES, default ON)
**Recommendation:** store raw JSON payloads by default under `out/raw/` for auditing, reproducibility, and debugging table parsing. This is especially useful when Alpha Vantage returns partial data or schema changes unexpectedly.

- Proposed layout:
  - `out/raw/market/<endpoint>.json`
  - `out/raw/tickers/<ticker>/<endpoint>.json`

- Controls:
  - Default: ON.
  - Flag to disable: `--no-raw`.
  - Flag to cap size: `--max-raw-bytes <n>` (default e.g., 5MB) to avoid runaway output on large endpoints.

### Schema-diffing between tickers (recommendation: YES, lightweight)
**Recommendation:** implement schema-diffing as a first-class feature, but keep it lightweight and deterministic:

- Compute `headers_by_endpoint_by_ticker` for each endpoint.
- Compute `union_headers_by_endpoint` across tickers.
- In `out/index.md`, render a table per endpoint:
  - Columns: `Ticker | Status | Missing fields count | Extra fields count`
- Additionally, include a compact “missing fields” list per ticker (truncated to top N missing fields, configurable).

This catches field drift without forcing brittle type definitions.

### Index file content (required)
`out/index.md` must include:
- Run metadata: timestamp, tool version, symbols used.
- Market-wide endpoints summary (success/failure + artifact links).
- Per-ticker summary:
  - Endpoint status table.
  - Links to per-ticker markdown reports.
- Schema-diff sections (per endpoint).

### Markdown table formatting rules
- Map-like payloads (e.g., `OVERVIEW`): render as 2-column table: `Field | Value`.
- Array-of-objects payloads (e.g., statement `annualReports` / `quarterlyReports`):
  - Render separate tables per array.
  - Compute headers as the set of keys from the first record (v0.2), but during schema-diff compute union across tickers.
  - Include *all* headers/columns.
  - Include only top 3 rows.
- Nested payloads:
  - Render each significant object/array under its own heading.

### Error rendering
For each endpoint call, produce either:
- A “Success” section with tables, or
- An “Error” section with:
  - Endpoint name
  - URL (with key redaction)
  - Error category (Network / HTTP / ProviderError / Parse)
  - Error message

## CLI requirements
### Command
- `alpha-vantage-explorer run` (default)

### Flags
- `--symbols AAPL,NVDA,MU` (optional override)
- `--out-dir out/`
- `--api-key-env ALPHAVANTAGE_API_KEY` (default)
- `--api-key <key>` (discouraged; allowed for local quick testing)
- `--no-raw`
- `--log-format human|json`
- `--log-level error|warn|info|debug|trace`

### UX requirements
- Clear progress output (endpoint/ticker currently running).
- Rate limit waiting is visible.
- Exit code:
  - `0` if at least one endpoint succeeded and the index was written.
  - Non-zero if nothing could be fetched (e.g., invalid API key / total outage).

## Architecture (FAANG-grade modular monolith)
### Workspace layout
Cargo workspace with strict boundaries:
- `crates/core`
  - Pure domain + ports (traits), NewTypes (TickerSymbol, ApiKey), typed errors.
  - No IO crates.
- `crates/client`
  - HTTP adapter (`reqwest`), limiter, retries, JSON parsing, JSON→table conversion.
- `crates/cli`
  - Application orchestration: endpoint plan, loops over symbols, writes Markdown + raw JSON.

### Ports & adapters
- Core defines an `ApiClient` trait.
- Client implements `ApiClient`.

### Dynamic schema strategy
- Parse responses as `serde_json::Value`.
- Convert into a neutral table representation:
  - `title`
  - `headers`
  - `rows` (top 3 only)

This avoids brittle endpoint-specific structs.

## Logging format (recommendation: human default)
**Recommendation:** default to **human-readable console logs** for ad-hoc CLI use, with an opt-in JSON mode.

- Default: `--log-format human` (fast to read interactively).
- Optional: `--log-format json` for piping to log collectors or later scheduled execution.

Implementation still uses `tracing`; only the subscriber formatting changes.

## Testing requirements
### Unit tests
- NewType validation.
- JSON→table conversion for representative fixtures (map-like and array-like).
- Provider error payload detection.

### Integration tests
- Mocked HTTP server (e.g., `wiremock`/`httpmock`) to validate:
  - index generation
  - per-ticker reports
  - raw JSON persistence

### Snapshot tests
- Snapshot the generated Markdown output (`insta`) for deterministic fixtures.

## Quality gates (robot team)
- `cargo fmt --check`
- `cargo clippy --workspace -- -D warnings` (ban unwrap/expect/panic in prod)
- `cargo deny check`
- `cargo test --workspace`

## Milestones
- M0: Lock endpoint list + output file layout.
- M1: Workspace skeleton + boundaries (core/client/cli).
- M2: HTTP client + limiter + retry policy.
- M3: Markdown writer + per-ticker output + index.
- M4: Raw JSON persistence + schema-diffing summary.
- M5: Tests + CI quality gates.
