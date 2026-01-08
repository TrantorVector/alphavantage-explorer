# Architecture Plan: Alpha Vantage API Explorer
## FAANG-Grade, Reusable Foundation for Investment Analysis Platform

**Document Status:** Architecture Design Document (ADD)  
**Version:** 1.0  
**Date:** 2026-01-08  
**Target:** Alpha Vantage API Explorer v0.2 → Investment Analysis Platform v1.0+

---

## Table of Contents
1. [Executive Summary](#executive-summary)
2. [Design Philosophy](#design-philosophy)
3. [System Architecture](#system-architecture)
4. [Workspace Structure](#workspace-structure)
5. [Domain Model](#domain-model)
6. [Error Handling Strategy](#error-handling-strategy)
7. [Ports & Adapters](#ports--adapters)
8. [Data Flow & Execution Model](#data-flow--execution-model)
9. [Observability & Logging](#observability--logging)
10. [Testing Strategy](#testing-strategy)
11. [Extensibility for Investment Platform](#extensibility-for-investment-platform)
12. [Technology Stack](#technology-stack)
13. [Implementation Phases](#implementation-phases)

---

## 1. Executive Summary

This architecture implements the Alpha Vantage API Explorer as a **modular monolith** following **hexagonal architecture** principles, designed to be the foundational data ingestion layer for a larger investment analysis platform.

### Key Design Decisions

| Decision | Rationale | Trade-off |
|----------|-----------|-----------|
| Modular monolith over microservices | Single binary, easier to develop/test/deploy for solo dev | Scaling requires horizontal replication |
| Hexagonal architecture (ports & adapters) | Clean boundaries enable future platform extensions | Initial boilerplate overhead |
| Dynamic JSON parsing over strict schemas | Resilient to API schema drift; exploratory tool | Runtime schema validation required |
| Token-bucket rate limiting | Respects free-tier constraints; predictable | Slows execution (15s between calls) |
| Markdown + raw JSON artifacts | Human-readable + machine-parseable outputs | Disk space for raw JSON |
| Workspace crates (core/client/cli) | Enforced architectural boundaries via Cargo | More build complexity |

### Alignment with 7 Core Tenets

| Tenet | Implementation |
|-------|----------------|
| **Security** | `ApiKey` wrapped in `secrecy::Secret`, NewTypes for all domain concepts, no raw primitives |
| **Performance** | `#[repr(transparent)]` NewTypes, stack allocation, zero-copy JSON parsing where possible |
| **Scalability** | Stateless CLI, hexagonal ports enable horizontal scaling adapters (Redis cache, DB, etc.) |
| **Readability** | Explicit types, comprehensive doc comments, no clever macros |
| **Observability** | `tracing` spans for all I/O, correlation IDs, structured logs |
| **Reliability** | Typed errors (`thiserror`), no panics, graceful degradation on failures |
| **Simplicity** | Boring tech stack (`tokio`, `reqwest`, `serde`), single binary, automated linting |

---

## 2. Design Philosophy

### 2.1 Core Principles

1. **Boundary Enforcement via Rust Modules**: The compiler prevents `core` from importing `reqwest`, `sqlx`, or any I/O crate.
2. **Parse, Don't Validate**: All inputs (ticker symbols, API keys) are parsed into validated NewTypes at the boundary.
3. **Fail Fast, Fail Loudly**: Invalid states are compile-time errors; runtime failures are explicit typed errors.
4. **Designed for Reuse**: Domain types and ports are platform-agnostic; adapters are swappable.

### 2.2 Future-Proofing for Investment Platform

This explorer is **Phase 0** of a larger platform:

- **Phase 0 (this)**: CLI tool to validate APIs and generate reports.
- **Phase 1**: Add persistence (Postgres) to store time-series financial data.
- **Phase 2**: Add REST API layer for web/mobile frontends.
- **Phase 3**: Add analytics engine (backtesting, portfolio optimization).
- **Phase 4**: Add ML pipelines for sentiment analysis and forecasting.

**Architectural constraints ensure**:
- Domain logic (`core`) remains pure and testable.
- Adapters (HTTP, DB, cache) can be swapped without touching business logic.
- NewTypes and ports are shared across all phases.

---

## 3. System Architecture

### 3.1 Hexagonal Architecture (Ports & Adapters)

```
┌────────────────────────────────────────────────────────────────┐
│                         CLI Layer                              │
│  (Application orchestration: endpoint loop, markdown writer)   │
└─────────────────────────┬──────────────────────────────────────┘
                          │
                          │ depends on
                          ▼
┌────────────────────────────────────────────────────────────────┐
│                        Core Domain                             │
│                                                                │
│  ┌──────────────────────────────────────────────────────┐    │
│  │  Domain Models (NewTypes)                            │    │
│  │  • TickerSymbol, ApiKey, EndpointName                │    │
│  │  • MarketDataResponse, FinancialStatement            │    │
│  │  • SchemaTable (headers + rows)                      │    │
│  └──────────────────────────────────────────────────────┘    │
│                                                                │
│  ┌──────────────────────────────────────────────────────┐    │
│  │  Ports (Traits)                                      │    │
│  │  • ApiClient: fetch_endpoint()                       │    │
│  │  • MarkdownWriter: write_report()                    │    │
│  │  • JsonPersister: save_raw()                         │    │
│  │  • SchemaAnalyzer: compute_diff()                    │    │
│  └──────────────────────────────────────────────────────┘    │
│                                                                │
│  ┌──────────────────────────────────────────────────────┐    │
│  │  Business Logic                                      │    │
│  │  • JSON → SchemaTable conversion                     │    │
│  │  • Schema diffing algorithm                          │    │
│  │  • Error classification                              │    │
│  └──────────────────────────────────────────────────────┘    │
└────────────────────────────────────────────────────────────────┘
                          ▲
                          │ implements ports
                          │
┌────────────────────────────────────────────────────────────────┐
│                      Client Adapters                           │
│                                                                │
│  ┌──────────────────────────────────────────────────────┐    │
│  │  HTTP Client (reqwest)                               │    │
│  │  • Rate limiter (token bucket)                       │    │
│  │  • Retry logic (exponential backoff)                 │    │
│  │  • Timeout handling (10s default)                    │    │
│  └──────────────────────────────────────────────────────┘    │
│                                                                │
│  ┌──────────────────────────────────────────────────────┐    │
│  │  Markdown Writer                                     │    │
│  │  • Table formatting                                  │    │
│  │  • Index generation                                  │    │
│  └──────────────────────────────────────────────────────┘    │
│                                                                │
│  ┌──────────────────────────────────────────────────────┐    │
│  │  JSON Persister (filesystem)                         │    │
│  │  • Raw JSON storage                                  │    │
│  │  • Size cap enforcement                              │    │
│  └──────────────────────────────────────────────────────┘    │
└────────────────────────────────────────────────────────────────┘
```

### 3.2 Data Flow Diagram

```
┌──────────┐
│   User   │
└────┬─────┘
     │ runs CLI: `alpha-vantage-explorer run --symbols AAPL,NVDA,MU`
     ▼
┌──────────────────────────────────────────────────────────────┐
│  CLI: Parse args, initialize tracing, build execution plan  │
└────────────────────┬─────────────────────────────────────────┘
                     │
                     ▼
     ┌───────────────────────────────────────────────┐
     │  FOR EACH market-wide endpoint:               │
     │    1. Call ApiClient::fetch_endpoint()        │
     │    2. Parse JSON → SchemaTable               │
     │    3. Write to out/market/market.md          │
     │    4. Save raw JSON (if enabled)             │
     │    5. Wait (rate limiter)                     │
     └───────────────────────────────────────────────┘
                     │
                     ▼
     ┌───────────────────────────────────────────────┐
     │  FOR EACH ticker in [AAPL, NVDA, MU]:         │
     │    FOR EACH company-specific endpoint:        │
     │      1. Call ApiClient::fetch_endpoint()      │
     │      2. Parse JSON → SchemaTable             │
     │      3. Write to out/tickers/<TICKER>.md     │
     │      4. Save raw JSON (if enabled)           │
     │      5. Wait (rate limiter)                   │
     └───────────────────────────────────────────────┘
                     │
                     ▼
     ┌───────────────────────────────────────────────┐
     │  Compute schema diffs across tickers          │
     │  Generate out/index.md with:                  │
     │    • Run metadata                             │
     │    • Market endpoint summary                  │
     │    • Per-ticker summary + links               │
     │    • Schema diff tables                       │
     └───────────────────────────────────────────────┘
                     │
                     ▼
            ┌────────────────┐
            │  Exit code: 0  │
            └────────────────┘
```

---

## 4. Workspace Structure

### 4.1 Repository Layout

```
alpha-vantage-explorer/               # ← REPOSITORY ROOT
├── .cursorrules                      # AI agent instructions
├── .gitignore
├── Cargo.toml                        # Workspace root
├── Cargo.lock
├── clippy.toml                       # Linter config (strict)
├── deny.toml                         # Supply chain security
├── rustfmt.toml                      # Code formatting
├── README.md
│
├── .github/
│   └── workflows/
│       └── ci.yml                    # Quality gates
│
├── docs/
│   ├── prd/
│   │   └── alpha-vantage-api-explorer-v0.2.md
│   ├── architecture/
│   │   └── architecture-plan.md     # This document
│   ├── adr/                          # Architectural Decision Records
│   │   ├── 001-use-hexagonal-architecture.md
│   │   ├── 002-dynamic-json-parsing.md
│   │   └── 003-markdown-output-format.md
│   └── api-docs/
│       └── alpha-vantage-specific-sections-API.md
│
├── crates/
│   ├── core/                         # 🧠 DOMAIN LOGIC (Pure Rust)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── domain/               # Domain models
│   │       │   ├── mod.rs
│   │       │   ├── ticker.rs         # TickerSymbol NewType
│   │       │   ├── api_key.rs        # ApiKey (secrecy-wrapped)
│   │       │   ├── endpoint.rs       # EndpointName enum
│   │       │   ├── schema_table.rs   # SchemaTable struct
│   │       │   └── market_data.rs    # Response envelopes
│   │       ├── ports/                # Trait definitions
│   │       │   ├── mod.rs
│   │       │   ├── api_client.rs     # ApiClient trait
│   │       │   ├── markdown_writer.rs
│   │       │   ├── json_persister.rs
│   │       │   └── schema_analyzer.rs
│   │       ├── logic/                # Business logic
│   │       │   ├── mod.rs
│   │       │   ├── json_to_table.rs  # JSON → SchemaTable
│   │       │   ├── schema_diff.rs    # Schema diffing algorithm
│   │       │   └── error_classifier.rs
│   │       ├── error.rs              # Typed error hierarchy
│   │       └── tests/
│   │           ├── mod.rs
│   │           ├── ticker_tests.rs
│   │           └── schema_tests.rs
│   │
│   ├── client/                       # 🔌 HTTP ADAPTER
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── http_client.rs        # ApiClient impl (reqwest)
│   │       ├── rate_limiter.rs       # Token bucket
│   │       ├── retry_policy.rs       # Exponential backoff
│   │       ├── markdown_writer.rs    # MarkdownWriter impl
│   │       ├── json_persister.rs     # JsonPersister impl
│   │       ├── schema_analyzer.rs    # SchemaAnalyzer impl
│   │       └── tests/
│   │           ├── mod.rs
│   │           ├── http_client_tests.rs
│   │           └── rate_limiter_tests.rs
│   │
│   └── cli/                          # 🖥️ APPLICATION LAYER
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs               # Entry point
│           ├── cli_args.rs           # clap config
│           ├── executor.rs           # Main execution loop
│           ├── progress.rs           # Progress reporting
│           └── config.rs             # Configuration struct
│
├── tests/                            # 🧪 INTEGRATION TESTS
│   ├── common/
│   │   ├── mod.rs
│   │   └── fixtures.rs               # Test data
│   └── integration/
│       ├── mod.rs
│       ├── market_endpoints_test.rs
│       ├── ticker_endpoints_test.rs
│       └── schema_diff_test.rs
│
├── snapshots/                        # 📸 INSTA SNAPSHOTS
│   ├── integration__market_report.snap
│   └── integration__ticker_report.snap
│
└── out/                              # 📁 OUTPUT (gitignored)
    ├── index.md
    ├── market/
    │   └── market.md
    ├── tickers/
    │   ├── AAPL.md
    │   ├── NVDA.md
    │   └── MU.md
    └── raw/
        ├── market/
        │   ├── TOP_GAINERS_LOSERS.json
        │   └── LISTING_STATUS.json
        └── tickers/
            ├── AAPL/
            │   ├── OVERVIEW.json
            │   ├── INCOME_STATEMENT.json
            │   └── ...
            ├── NVDA/
            └── MU/
```

### 4.2 Crate Dependency Graph

```
┌─────────────────┐
│      cli        │  (bin crate)
└────────┬────────┘
         │ depends on
         ├──────────────────┬─────────────────┐
         ▼                  ▼                 ▼
┌─────────────┐    ┌────────────┐    ┌──────────────┐
│    core     │◄───│   client   │    │   tokio      │
│  (domain)   │    │ (adapters) │    │   clap       │
└─────────────┘    └────────────┘    │   tracing    │
                                      └──────────────┘
         ▲
         │ depends on (workspace-level)
         │
┌────────────────────────────────────────────────┐
│  serde, thiserror, secrecy, uuid, chrono, ...  │
└────────────────────────────────────────────────┘
```

**Key Constraints**:
- `core` has **zero dependencies** on I/O crates (`reqwest`, `tokio::net`, `sqlx`).
- `client` implements ports defined in `core`.
- `cli` orchestrates by calling `client` via `core` traits.

---

## 5. Domain Model

### 5.1 NewTypes (Security & Clarity)

All domain concepts are wrapped in NewTypes following **Tenet 1 (Security)** and **Tenet 4 (Readability)**.

```rust
// crates/core/src/domain/ticker.rs

use serde::{Deserialize, Serialize};
use std::fmt;

/// Validated ticker symbol (1-5 uppercase alphanumeric characters).
/// 
/// # Examples
/// ```
/// let ticker = TickerSymbol::new("AAPL")?;
/// assert_eq!(ticker.as_str(), "AAPL");
/// ```
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TickerSymbol(String);

impl TickerSymbol {
    /// Parses and validates a ticker symbol.
    ///
    /// # Errors
    /// - Returns `ValidationError::InvalidLength` if not 1-5 chars.
    /// - Returns `ValidationError::InvalidCharacters` if contains lowercase or non-alphanumeric.
    pub fn new(raw: impl Into<String>) -> Result<Self, ValidationError> {
        let raw = raw.into().to_uppercase();

        if raw.is_empty() || raw.len() > 5 {
            return Err(ValidationError::InvalidLength);
        }

        if !raw.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Err(ValidationError::InvalidCharacters);
        }

        Ok(Self(raw))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TickerSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Ticker symbol must be 1-5 characters")]
    InvalidLength,

    #[error("Ticker symbol must be alphanumeric")]
    InvalidCharacters,
}
```

```rust
// crates/core/src/domain/api_key.rs

use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};

/// API key (protected by secrecy crate to prevent logging).
///
/// # Security
/// - Automatically redacted from logs.
/// - Memory zeroed on drop.
#[derive(Clone, Serialize, Deserialize)]
pub struct ApiKey(Secret<String>);

impl ApiKey {
    pub fn new(key: impl Into<String>) -> Self {
        Self(Secret::new(key.into()))
    }

    /// Exposes the underlying secret (use sparingly).
    pub fn expose(&self) -> &str {
        self.0.expose_secret()
    }
}

impl std::fmt::Debug for ApiKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ApiKey([REDACTED])")
    }
}
```

```rust
// crates/core/src/domain/endpoint.rs

use serde::{Deserialize, Serialize};
use strum::{EnumIter, EnumString, Display};

/// All supported Alpha Vantage endpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter, EnumString, Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum EndpointName {
    // Market-wide
    TopGainersLosers,
    ListingStatus,
    EarningsCalendar,
    IpoCalendar,

    // Company-specific
    Overview,
    IncomeStatement,
    BalanceSheet,
    CashFlow,
    Earnings,
    EarningsEstimates,
    NewsSentiment,
    InsiderTransactions,
    Dividends,
    Splits,
    SharesOutstanding,
    EarningsCallTranscript,
}

impl EndpointName {
    /// Returns the Alpha Vantage function parameter value.
    pub fn function_name(&self) -> &'static str {
        match self {
            Self::TopGainersLosers => "TOP_GAINERS_LOSERS",
            Self::ListingStatus => "LISTING_STATUS",
            Self::EarningsCalendar => "EARNINGS_CALENDAR",
            Self::IpoCalendar => "IPO_CALENDAR",
            Self::Overview => "OVERVIEW",
            Self::IncomeStatement => "INCOME_STATEMENT",
            Self::BalanceSheet => "BALANCE_SHEET",
            Self::CashFlow => "CASH_FLOW",
            Self::Earnings => "EARNINGS",
            Self::EarningsEstimates => "EARNINGS_ESTIMATES",
            Self::NewsSentiment => "NEWS_SENTIMENT",
            Self::InsiderTransactions => "INSIDER_TRANSACTIONS",
            Self::Dividends => "DIVIDENDS",
            Self::Splits => "SPLITS",
            Self::SharesOutstanding => "SHARES_OUTSTANDING",
            Self::EarningsCallTranscript => "EARNINGS_CALL_TRANSCRIPT",
        }
    }

    /// Returns true if this endpoint is market-wide (not ticker-specific).
    pub fn is_market_wide(&self) -> bool {
        matches!(
            self,
            Self::TopGainersLosers | Self::ListingStatus | Self::EarningsCalendar | Self::IpoCalendar
        )
    }
}
```

### 5.2 SchemaTable (Dynamic Schema Representation)

```rust
// crates/core/src/domain/schema_table.rs

use serde::{Deserialize, Serialize};

/// Neutral representation of tabular data (headers + rows).
///
/// Designed to capture the "shape" of API responses without
/// brittle type definitions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaTable {
    /// Human-readable title for the table (e.g., "Annual Income Statements").
    pub title: String,

    /// Column headers (ordered).
    pub headers: Vec<String>,

    /// Data rows (truncated to top 3 for reports).
    /// Each row is an ordered list of values corresponding to headers.
    pub rows: Vec<Vec<String>>,

    /// Total record count before truncation.
    pub total_records: usize,
}

impl SchemaTable {
    /// Creates a new SchemaTable.
    pub fn new(title: impl Into<String>, headers: Vec<String>, rows: Vec<Vec<String>>) -> Self {
        let total_records = rows.len();
        let truncated_rows = rows.into_iter().take(3).collect();

        Self {
            title: title.into(),
            headers,
            rows: truncated_rows,
            total_records,
        }
    }

    /// Returns true if the table was truncated.
    pub fn is_truncated(&self) -> bool {
        self.total_records > self.rows.len()
    }
}
```

### 5.3 Response Envelopes

```rust
// crates/core/src/domain/market_data.rs

use serde::{Deserialize, Serialize};
use serde_json::Value;
use chrono::{DateTime, Utc};
use super::{EndpointName, TickerSymbol};

/// Envelope for a successful API response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse {
    pub endpoint: EndpointName,
    pub ticker: Option<TickerSymbol>,
    pub timestamp: DateTime<Utc>,
    pub raw_json: Value,
}

/// Envelope for a failed API call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub endpoint: EndpointName,
    pub ticker: Option<TickerSymbol>,
    pub timestamp: DateTime<Utc>,
    pub error_kind: ErrorKind,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorKind {
    Network,
    HttpStatus,
    ProviderError,
    Parse,
    RateLimited,
}
```

---

## 6. Error Handling Strategy

### 6.1 Typed Error Hierarchy (Tenet 6: Reliability)

```rust
// crates/core/src/error.rs

use thiserror::Error;
use super::domain::TickerSymbol;

/// Top-level error type for the explorer.
#[derive(Error, Debug)]
pub enum ExplorerError {
    #[error("Network error: {0}")]
    Network(String),

    #[error("HTTP {status}: {message}")]
    HttpStatus {
        status: u16,
        message: String,
    },

    #[error("Alpha Vantage API error: {0}")]
    ProviderError(String),

    #[error("Failed to parse response: {0}")]
    Parse(String),

    #[error("Rate limit exceeded; retrying after {seconds}s")]
    RateLimited {
        seconds: u64,
    },

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

/// Result type alias for convenience.
pub type Result<T> = std::result::Result<T, ExplorerError>;
```

### 6.2 Error Handling Policy

- **No Panics**: Enforced by `clippy.toml` (`unwrap_used = "deny"`).
- **Explicit Context**: Use `.map_err()` or `thiserror` to wrap lower-level errors with context.
- **Graceful Degradation**: On per-endpoint failures, log error and continue to next endpoint.
- **Structured Logging**: All errors are emitted as structured `tracing` events.

---

## 7. Ports & Adapters

### 7.1 Core Ports (Traits)

```rust
// crates/core/src/ports/api_client.rs

use async_trait::async_trait;
use crate::{
    domain::{ApiResponse, ApiError, EndpointName, TickerSymbol},
    error::Result,
};

/// Port for fetching data from Alpha Vantage API.
///
/// This trait is implemented by the HTTP adapter (`crates/client`).
#[async_trait]
pub trait ApiClient: Send + Sync {
    /// Fetches data for a market-wide endpoint.
    async fn fetch_market_endpoint(
        &self,
        endpoint: EndpointName,
    ) -> Result<ApiResponse>;

    /// Fetches data for a company-specific endpoint.
    async fn fetch_ticker_endpoint(
        &self,
        endpoint: EndpointName,
        ticker: &TickerSymbol,
    ) -> Result<ApiResponse>;
}
```

```rust
// crates/core/src/ports/markdown_writer.rs

use crate::{
    domain::SchemaTable,
    error::Result,
};

/// Port for writing Markdown reports.
pub trait MarkdownWriter: Send + Sync {
    /// Writes a single table to a markdown buffer.
    fn write_table(&mut self, table: &SchemaTable) -> Result<()>;

    /// Writes an error section.
    fn write_error(&mut self, error_msg: &str) -> Result<()>;

    /// Flushes the buffer to a file.
    fn flush_to_file(&mut self, path: &std::path::Path) -> Result<()>;
}
```

```rust
// crates/core/src/ports/json_persister.rs

use serde_json::Value;
use crate::{
    domain::{EndpointName, TickerSymbol},
    error::Result,
};

/// Port for persisting raw JSON payloads.
pub trait JsonPersister: Send + Sync {
    /// Saves a raw JSON response to disk.
    ///
    /// # Arguments
    /// - `endpoint`: The endpoint name (used for file naming).
    /// - `ticker`: Optional ticker (None for market-wide endpoints).
    /// - `json`: The raw JSON payload.
    async fn save_raw_json(
        &self,
        endpoint: EndpointName,
        ticker: Option<&TickerSymbol>,
        json: &Value,
    ) -> Result<()>;
}
```

```rust
// crates/core/src/ports/schema_analyzer.rs

use std::collections::HashMap;
use crate::{
    domain::{EndpointName, TickerSymbol, SchemaTable},
    error::Result,
};

/// Port for computing schema diffs across tickers.
pub trait SchemaAnalyzer: Send + Sync {
    /// Computes schema differences for a given endpoint across tickers.
    ///
    /// Returns a map: ticker -> (missing_fields, extra_fields).
    fn compute_schema_diff(
        &self,
        endpoint: EndpointName,
        tables_by_ticker: &HashMap<TickerSymbol, SchemaTable>,
    ) -> Result<SchemaDiff>;
}

#[derive(Debug, Clone)]
pub struct SchemaDiff {
    pub union_headers: Vec<String>,
    pub diffs: HashMap<TickerSymbol, TickerDiff>,
}

#[derive(Debug, Clone)]
pub struct TickerDiff {
    pub missing_fields: Vec<String>,
    pub extra_fields: Vec<String>,
}
```

### 7.2 Adapter Implementations

Adapters live in `crates/client` and implement the ports defined in `crates/core`.

```rust
// crates/client/src/http_client.rs

use async_trait::async_trait;
use reqwest::Client;
use std::time::Duration;
use core::{
    domain::{ApiKey, ApiResponse, EndpointName, TickerSymbol},
    error::{ExplorerError, Result},
    ports::ApiClient,
};
use crate::rate_limiter::RateLimiter;

/// HTTP adapter implementing ApiClient port.
pub struct AlphaVantageClient {
    base_url: String,
    api_key: ApiKey,
    http_client: Client,
    rate_limiter: RateLimiter,
}

impl AlphaVantageClient {
    pub fn new(api_key: ApiKey) -> Result<Self> {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .map_err(|e| ExplorerError::Network(e.to_string()))?;

        let rate_limiter = RateLimiter::new(4, Duration::from_secs(60)); // 4 calls/min

        Ok(Self {
            base_url: "https://www.alphavantage.co/query".to_string(),
            api_key,
            http_client,
            rate_limiter,
        })
    }
}

#[async_trait]
impl ApiClient for AlphaVantageClient {
    async fn fetch_market_endpoint(
        &self,
        endpoint: EndpointName,
    ) -> Result<ApiResponse> {
        self.rate_limiter.wait().await;

        let url = format!(
            "{}?function={}&apikey={}",
            self.base_url,
            endpoint.function_name(),
            self.api_key.expose()
        );

        // Implementation details: retry logic, error handling, JSON parsing...
        todo!()
    }

    async fn fetch_ticker_endpoint(
        &self,
        endpoint: EndpointName,
        ticker: &TickerSymbol,
    ) -> Result<ApiResponse> {
        self.rate_limiter.wait().await;

        let url = format!(
            "{}?function={}&symbol={}&apikey={}",
            self.base_url,
            endpoint.function_name(),
            ticker.as_str(),
            self.api_key.expose()
        );

        // Implementation details...
        todo!()
    }
}
```

---

## 8. Data Flow & Execution Model

### 8.1 Main Execution Loop

```rust
// crates/cli/src/executor.rs

use core::{
    domain::{EndpointName, TickerSymbol},
    ports::{ApiClient, MarkdownWriter, JsonPersister, SchemaAnalyzer},
};
use std::collections::HashMap;
use strum::IntoEnumIterator;

pub struct Executor {
    api_client: Box<dyn ApiClient>,
    markdown_writer: Box<dyn MarkdownWriter>,
    json_persister: Option<Box<dyn JsonPersister>>,
    schema_analyzer: Box<dyn SchemaAnalyzer>,
}

impl Executor {
    pub async fn run(&self, tickers: Vec<TickerSymbol>) -> Result<()> {
        tracing::info!("Starting Alpha Vantage API Explorer");

        // Phase 1: Fetch market-wide endpoints
        self.fetch_market_endpoints().await?;

        // Phase 2: Fetch ticker-specific endpoints
        let ticker_results = self.fetch_ticker_endpoints(&tickers).await?;

        // Phase 3: Compute schema diffs
        let schema_diffs = self.compute_schema_diffs(&ticker_results)?;

        // Phase 4: Generate index.md
        self.generate_index(&tickers, &ticker_results, &schema_diffs)?;

        tracing::info!("Explorer run completed successfully");
        Ok(())
    }

    async fn fetch_market_endpoints(&self) -> Result<()> {
        for endpoint in EndpointName::iter().filter(|e| e.is_market_wide()) {
            tracing::info!(endpoint = %endpoint, "Fetching market endpoint");

            match self.api_client.fetch_market_endpoint(endpoint).await {
                Ok(response) => {
                    // Convert to SchemaTable, write markdown, save raw JSON...
                }
                Err(e) => {
                    tracing::error!(endpoint = %endpoint, error = %e, "Failed to fetch endpoint");
                    // Write error to markdown, continue...
                }
            }
        }
        Ok(())
    }

    async fn fetch_ticker_endpoints(
        &self,
        tickers: &[TickerSymbol],
    ) -> Result<HashMap<TickerSymbol, TickerResults>> {
        // Implementation: loop over tickers and endpoints...
        todo!()
    }
}
```

### 8.2 Rate Limiting Strategy

```rust
// crates/client/src/rate_limiter.rs

use std::time::{Duration, Instant};
use tokio::sync::Mutex;

/// Token bucket rate limiter.
///
/// Enforces free-tier constraints (e.g., 5 calls/minute).
pub struct RateLimiter {
    max_tokens: u32,
    refill_interval: Duration,
    state: Mutex<RateLimiterState>,
}

struct RateLimiterState {
    tokens: u32,
    last_refill: Instant,
}

impl RateLimiter {
    pub fn new(max_calls_per_interval: u32, interval: Duration) -> Self {
        Self {
            max_tokens: max_calls_per_interval,
            refill_interval: interval,
            state: Mutex::new(RateLimiterState {
                tokens: max_calls_per_interval,
                last_refill: Instant::now(),
            }),
        }
    }

    /// Waits until a token is available, then consumes it.
    pub async fn wait(&self) {
        loop {
            let mut state = self.state.lock().await;

            // Refill tokens if interval elapsed
            let elapsed = state.last_refill.elapsed();
            if elapsed >= self.refill_interval {
                state.tokens = self.max_tokens;
                state.last_refill = Instant::now();
            }

            if state.tokens > 0 {
                state.tokens -= 1;
                tracing::debug!(remaining_tokens = state.tokens, "Consumed rate limit token");
                break;
            }

            // Wait and retry
            drop(state);
            let wait_time = self.refill_interval.checked_sub(elapsed).unwrap_or(Duration::from_secs(1));
            tracing::info!(wait_seconds = wait_time.as_secs(), "Rate limit exceeded; waiting...");
            tokio::time::sleep(wait_time).await;
        }
    }
}
```

---

## 9. Observability & Logging

### 9.1 Tracing Initialization

```rust
// crates/cli/src/main.rs

use tracing_subscriber::{layer::SubscriberExt, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing (before any other code)
    let log_format = std::env::var("LOG_FORMAT").unwrap_or_else(|_| "human".to_string());

    let subscriber = tracing_subscriber::registry()
        .with(EnvFilter::from_env("RUST_LOG").unwrap_or_else(|_| EnvFilter::new("info")));

    let subscriber = match log_format.as_str() {
        "json" => subscriber.with(tracing_subscriber::fmt::layer().json()).boxed(),
        _ => subscriber.with(tracing_subscriber::fmt::layer().pretty()).boxed(),
    };

    tracing::subscriber::set_global_default(subscriber)?;

    // Run CLI
    cli::run().await
}
```

### 9.2 Instrumentation Example

```rust
#[tracing::instrument(skip(self), fields(endpoint = %endpoint, ticker = ?ticker))]
async fn fetch_ticker_endpoint(
    &self,
    endpoint: EndpointName,
    ticker: &TickerSymbol,
) -> Result<ApiResponse> {
    tracing::info!("Fetching endpoint");

    let response = self.http_client.get(&url).send().await
        .map_err(|e| {
            tracing::error!(error = %e, "HTTP request failed");
            ExplorerError::Network(e.to_string())
        })?;

    tracing::debug!(status = %response.status(), "Received HTTP response");

    // ... rest of implementation
}
```

---

## 10. Testing Strategy

### 10.1 Unit Tests (Core Logic)

```rust
// crates/core/src/domain/tests/ticker_tests.rs

#[test]
fn ticker_symbol_rejects_lowercase() {
    let result = TickerSymbol::new("aapl");
    assert!(result.is_ok()); // Constructor uppercases automatically
    assert_eq!(result.unwrap().as_str(), "AAPL");
}

#[test]
fn ticker_symbol_rejects_too_long() {
    let result = TickerSymbol::new("TOOLONG");
    assert!(result.is_err());
}

#[test]
fn ticker_symbol_rejects_special_chars() {
    let result = TickerSymbol::new("AA$L");
    assert!(result.is_err());
}
```

### 10.2 Property-Based Tests

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn ticker_symbol_never_panics(s in ".*") {
        let _ = TickerSymbol::new(s);
        // Should return Ok or Err, never panic
    }
}
```

### 10.3 Integration Tests (Mocked HTTP)

```rust
// tests/integration/http_client_tests.rs

use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, query_param};

#[tokio::test]
async fn test_fetch_overview_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(query_param("function", "OVERVIEW"))
        .and(query_param("symbol", "AAPL"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "Symbol": "AAPL",
            "Name": "Apple Inc",
            "MarketCapitalization": "3000000000000"
        })))
        .mount(&mock_server)
        .await;

    let client = AlphaVantageClient::new_with_base_url(
        ApiKey::new("test_key"),
        mock_server.uri(),
    )?;

    let response = client.fetch_ticker_endpoint(EndpointName::Overview, &TickerSymbol::new("AAPL")?).await?;
    assert_eq!(response.endpoint, EndpointName::Overview);
}
```

### 10.4 Snapshot Tests

```rust
use insta::assert_snapshot;

#[test]
fn test_markdown_table_format() {
    let table = SchemaTable::new(
        "Test Table",
        vec!["Field".into(), "Value".into()],
        vec![vec!["Symbol".into(), "AAPL".into()]],
    );

    let mut writer = MarkdownWriterImpl::new();
    writer.write_table(&table).unwrap();

    assert_snapshot!(writer.to_string());
}
```

---

## 11. Extensibility for Investment Platform

### 11.1 Phase 0 → Phase 1: Adding Persistence

**Current (Phase 0)**: CLI writes to filesystem (Markdown + raw JSON).

**Phase 1 Extension**: Add Postgres persistence layer.

**Steps**:
1. Add `crates/storage` (new crate).
2. Define new port in `crates/core/src/ports/data_repository.rs`:

```rust
#[async_trait]
pub trait DataRepository: Send + Sync {
    async fn store_market_data(&self, response: &ApiResponse) -> Result<()>;
    async fn query_historical_data(&self, ticker: &TickerSymbol, start_date: DateTime<Utc>) -> Result<Vec<ApiResponse>>;
}
```

3. Implement `SqlxDataRepository` in `crates/storage`.
4. CLI remains unchanged; just inject different adapter.

### 11.2 Phase 1 → Phase 2: Adding REST API

**Phase 2 Extension**: Wrap CLI logic in HTTP API (Axum).

**Steps**:
1. Add `crates/api` (new crate).
2. Expose endpoints:
   - `GET /api/v1/tickers/{ticker}/overview`
   - `GET /api/v1/tickers/{ticker}/financials/income`
   - `POST /api/v1/refresh` (trigger data refresh job)

3. Reuse `core` domain logic and `client` HTTP adapter.
4. Use `storage` adapter for persistence.

### 11.3 Shared Domain Language

All phases share the same domain types from `crates/core`:
- `TickerSymbol`
- `ApiResponse`
- `SchemaTable`
- Error types

This ensures **zero duplication** and **type safety across phases**.

---

## 12. Technology Stack

### 12.1 Core Dependencies

| Crate | Purpose | Justification |
|-------|---------|---------------|
| `tokio` | Async runtime | De facto standard for async Rust |
| `reqwest` | HTTP client | Mature, well-maintained, supports retries |
| `serde` / `serde_json` | Serialization | Industry standard for JSON in Rust |
| `thiserror` | Error types | Ergonomic derive macros for errors |
| `anyhow` | Error aggregation (CLI only) | Simplifies error handling in application layer |
| `tracing` | Structured logging | Best-in-class observability for Rust |
| `secrecy` | Secret management | Auto-redacts secrets from logs/debug |
| `clap` | CLI parsing | Modern, derive-based CLI builder |
| `uuid` | Unique IDs | Widely used, supports v4 generation |
| `chrono` | Date/time | Standard date/time library |
| `strum` | Enum utilities | Derive macros for enum iteration/parsing |

### 12.2 Dev/Test Dependencies

| Crate | Purpose |
|-------|---------|
| `wiremock` | HTTP mocking for integration tests |
| `insta` | Snapshot testing |
| `proptest` | Property-based testing |
| `criterion` | Benchmarking (optional) |
| `cargo-nextest` | Faster test runner |
| `cargo-deny` | Supply chain security |
| `cargo-llvm-cov` | Code coverage |

---

## 13. Implementation Phases

### Phase 0: Foundation (Week 1)
- [ ] Set up Cargo workspace structure
- [ ] Implement core NewTypes (`TickerSymbol`, `ApiKey`, `EndpointName`)
- [ ] Define port traits
- [ ] Write unit tests for domain logic

### Phase 1: HTTP Adapter (Week 2)
- [ ] Implement `AlphaVantageClient` with `reqwest`
- [ ] Implement token bucket rate limiter
- [ ] Add retry logic with exponential backoff
- [ ] Add integration tests with `wiremock`

### Phase 2: JSON → Table Conversion (Week 3)
- [ ] Implement dynamic JSON parsing logic
- [ ] Handle map-like payloads (e.g., `OVERVIEW`)
- [ ] Handle array-like payloads (e.g., statements)
- [ ] Add property tests for edge cases

### Phase 3: Markdown Writer (Week 4)
- [ ] Implement `MarkdownWriter` adapter
- [ ] Format tables, error blocks, headings
- [ ] Generate per-ticker reports
- [ ] Generate index.md

### Phase 4: Schema Diffing (Week 5)
- [ ] Implement `SchemaAnalyzer` adapter
- [ ] Compute union headers across tickers
- [ ] Detect missing/extra fields
- [ ] Add diff summaries to index

### Phase 5: CLI Integration (Week 6)
- [ ] Implement `clap` arg parsing
- [ ] Wire together all adapters
- [ ] Add progress reporting
- [ ] Add tracing initialization

### Phase 6: Testing & CI (Week 7)
- [ ] Write comprehensive integration tests
- [ ] Add snapshot tests for reports
- [ ] Set up GitHub Actions CI pipeline
- [ ] Configure clippy, deny, fmt checks

### Phase 7: Documentation & Release (Week 8)
- [ ] Write comprehensive README
- [ ] Document all public APIs
- [ ] Create example runs
- [ ] Tag v0.1.0 release

---

## Appendix A: Cargo.toml Templates

### Workspace Root

```toml
[workspace]
resolver = "2"
members = ["crates/core", "crates/client", "crates/cli"]

[workspace.dependencies]
# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Async runtime
tokio = { version = "1", features = ["full"] }
async-trait = "0.1"

# Error handling
thiserror = "1.0"
anyhow = "1.0"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["json", "fmt", "env-filter"] }

# Domain types
uuid = { version = "1.0", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
secrecy = { version = "0.8", features = ["serde"] }
strum = { version = "0.26", features = ["derive"] }

# HTTP client
reqwest = { version = "0.12", features = ["json"] }

# CLI
clap = { version = "4", features = ["derive"] }

[workspace.lints.clippy]
unwrap_used = "deny"
expect_used = "deny"
panic = "deny"
indexing_slicing = "deny"

[workspace.lints.rust]
unsafe_code = "deny"
```

### Core Crate

```toml
[package]
name = "core"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { workspace = true }
serde_json = { workspace = true }
thiserror = { workspace = true }
uuid = { workspace = true }
chrono = { workspace = true }
secrecy = { workspace = true }
strum = { workspace = true }
async-trait = { workspace = true }

[dev-dependencies]
proptest = "1.4"
```

---

## Appendix B: ADR Template

```markdown
# ADR-NNN: [Title]

**Status:** Proposed | Accepted | Deprecated  
**Date:** YYYY-MM-DD  
**Deciders:** [Name(s)]

## Context

[Describe the problem and constraints]

## Decision

[Describe the chosen solution]

## Consequences

### Positive
- [Benefit 1]
- [Benefit 2]

### Negative
- [Trade-off 1]
- [Trade-off 2]

## Alternatives Considered

1. **[Alternative 1]**: [Why rejected]
2. **[Alternative 2]**: [Why rejected]
```

---

## Conclusion

This architecture plan provides a **FAANG-grade foundation** for the Alpha Vantage API Explorer that is:
- **Maintainable**: Clear boundaries, typed errors, comprehensive docs.
- **Testable**: Ports enable mocking; pure domain logic is trivial to test.
- **Extensible**: Designed for reuse as the core of a larger investment analysis platform.
- **Reliable**: No panics, graceful degradation, structured observability.

The modular monolith approach with strict hexagonal architecture ensures that as the platform grows (adding persistence, REST APIs, analytics), the core domain logic remains **pure, stable, and reusable**.
