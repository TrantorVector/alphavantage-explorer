# Command Reference

This document provides detailed information about all Alpha Vantage Explorer CLI commands.

## Overview

The tool provides 13 granular endpoint commands, each targeting a specific Alpha Vantage API endpoint.

## Command Format

```bash
alphavantage_cli <COMMAND> --symbol <TICKER> [OPTIONS]
```

---

## Commands

### `overview`

Fetch company overview and fundamental data.

**Syntax:**
```bash
alphavantage_cli overview --symbol AAPL [--output PATH]
```

**Required Parameters:**
- `--symbol, -s`: Stock ticker symbol (e.g., AAPL, MSFT)

**Optional Parameters:**
- `--output, -o`: Custom output directory path

**Output Format:** JSON → Markdown table  
**Example:**
```bash
# Mock mode
alphavantage_cli overview --symbol AAPL

# Live API with custom output
alphavantage_cli overview --symbol NVDA --output ./reports
```

---

### `income-statement`

Fetch income statement data (annual and quarterly reports).

**Syntax:**
```bash
alphavantage_cli income-statement --symbol AAPL [--output PATH]
```

**Required Parameters:**
- `--symbol, -s`: Stock ticker symbol

**Optional Parameters:**
- `--output, -o`: Custom output directory path

**Output Format:** JSON → Markdown table

---

### `balance-sheet`

Fetch balance sheet data.

**Syntax:**
```bash
alphavantage_cli balance-sheet --symbol AAPL [--output PATH]
```

**Required Parameters:**
- `--symbol, -s`: Stock ticker symbol

**Optional Parameters:**
- `--output, -o`: Custom output directory path

**Output Format:** JSON → Markdown table

---

### `cash-flow`

Fetch cash flow statement.

**Syntax:**
```bash
alphavantage_cli cash-flow --symbol AAPL [--output PATH]
```

**Required Parameters:**
- `--symbol, -s`: Stock ticker symbol

**Optional Parameters:**
- `--output, -o`: Custom output directory path

**Output Format:** JSON → Markdown table

---

### `earnings`

Fetch earnings data (annual and quarterly).

**Syntax:**
```bash
alphavantage_cli earnings --symbol AAPL [--output PATH]
```

**Required Parameters:**
- `--symbol, -s`: Stock ticker symbol

**Optional Parameters:**
- `--output, -o`: Custom output directory path

**Output Format:** JSON → Markdown table

---

### `earnings-estimates`

Fetch earnings estimates and forecasts.

**Syntax:**
```bash
alphavantage_cli earnings-estimates --symbol AAPL [--output PATH]
```

**Required Parameters:**
- `--symbol, -s`: Stock ticker symbol

**Optional Parameters:**
- `--output, -o`: Custom output directory path

**Output Format:** JSON → Markdown table

---

### `news-sentiment`

Fetch news sentiment analysis.

**Syntax:**
```bash
alphavantage_cli news-sentiment --symbol AAPL [--limit 50] [--output PATH]
```

**Required Parameters:**
- `--symbol, -s`: Stock ticker symbol

**Optional Parameters:**
- `--limit, -l`: Maximum number of news items (default: 50)
- `--output, -o`: Custom output directory path

**Output Format:** JSON → Markdown table

**Example:**
```bash
alphavantage_cli news-sentiment --symbol TSLA --limit 100
```

---

### `insider-transactions`

Fetch insider trading activity.

**Syntax:**
```bash
alphavantage_cli insider-transactions --symbol AAPL [--output PATH]
```

**Required Parameters:**
- `--symbol, -s`: Stock ticker symbol

**Optional Parameters:**
- `--output, -o`: Custom output directory path

**Output Format:** JSON → Markdown table

---

### `dividends`

Fetch dividend payment history.

**Syntax:**
```bash
alphavantage_cli dividends --symbol AAPL [--output PATH]
```

**Required Parameters:**
- `--symbol, -s`: Stock ticker symbol

**Optional Parameters:**
- `--output, -o`: Custom output directory path

**Output Format:** JSON → Markdown table

---

### `splits`

Fetch stock split history.

**Syntax:**
```bash
alphavantage_cli splits --symbol AAPL [--output PATH]
```

**Required Parameters:**
- `--symbol, -s`: Stock ticker symbol

**Optional Parameters:**
- `--output, -o`: Custom output directory path

**Output Format:** JSON → Markdown table

---

### `shares-outstanding`

Fetch shares outstanding over time.

**Syntax:**
```bash
alphavantage_cli shares-outstanding --symbol AAPL [--output PATH]
```

**Required Parameters:**
- `--symbol, -s`: Stock ticker symbol

**Optional Parameters:**
- `--output, -o`: Custom output directory path

**Output Format:** JSON → Markdown table

---

### `earnings-calendar`

Fetch upcoming earnings calendar dates.

**Syntax:**
```bash
alphavantage_cli earnings-calendar --symbol AAPL [--horizon 3month] [--output PATH]
```

**Required Parameters:**
- `--symbol, -s`: Stock ticker symbol

**Optional Parameters:**
- `--horizon, -H`: Time horizon: `3month`, `6month`, `12month`
- `--output, -o`: Custom output directory path

**Output Format:** CSV (raw, no markdown conversion)

> **Note:** This endpoint returns CSV format. The CSV file is saved as-is without markdown transformation.

---

### `earnings-call-transcript`

Fetch earnings call transcripts.

**Syntax:**
```bash
alphavantage_cli earnings-call-transcript --symbol AAPL --year 2024 --quarter Q1 [--output PATH]
```

**Required Parameters:**
- `--symbol, -s`: Stock ticker symbol
- `--year, -y`: Year of earnings call (e.g., 2024)
- `--quarter, -q`: Quarter: `Q1`, `Q2`, `Q3`, `Q4`

**Optional Parameters:**
- `--output, -o`: Custom output directory path

**Output Format:** JSON → Markdown table

**Example:**
```bash
alphavantage_cli earnings-call-transcript --symbol AAPL --year 2023 --quarter Q4
```

---

## Global Options

All commands support these global options:

- `--log-level`: Set logging level (error, warn, info, debug, trace)
- `--log-format`: Set log format (human, json)

## Output

All successful commands print confirmation messages:
```
✓ Saved raw JSON: out/raw/overview_AAPL_20260110_230045.json
✓ Saved markdown: out/overview_AAPL_20260110_230045.md
```

For CSV endpoints:
```
✓ Saved CSV: out/raw/earnings_calendar_AAPL_20260110_230045.csv (CSV format, no markdown conversion)
```
