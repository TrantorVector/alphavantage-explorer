# Alpha Vantage API Explorer

A CLI tool to validate Alpha Vantage API endpoints and generate human-readable Markdown reports. This project is built using Rust and follows a Hexagonal Architecture (Ports & Adapters) to ensure maintainability and testability.

## 🚀 Overview

The Alpha Vantage API Explorer allows developers and analysts to:
- precise validation of Alpha Vantage API responses.
- inspect data quality and consistency.
- generate reports in Markdown format.

## 🛠️ Build Instructions

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
- OpenSSL (libssl-dev on Ubuntu/Debian)

### Steps
1. Clone the repository:
   ```bash
   git clone https://github.com/TrantorVector/alphavantage-explorer.git
   cd alphavantage-explorer
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

## ⚙️ Configuration

### API Key Setup
You need an Alpha Vantage API key to use this tool. Get a free key [here](https://www.alphavantage.co/support/#api-key).

1. Copy the example environment file:
   ```bash
   cp .env.example .env
   ```

2. Edit `.env` and add your API key:
   ```env
   ALPHAVANTAGE_API_KEY=your_actual_key_here
   ```

## 📖 Usage

> **Note:** This is a work in progress.

```bash
# Run the help command
cargo run --bin alphavantage_cli -- --help

# Example command (Placeholder)
# cargo run --bin alphavantage_cli -- validate --symbol IBM
```

## 🏗️ Architecture

This project is structured as a Cargo Workspace:
- **crates/core**: Domain logic (Pure Rust, no I/O).
- **crates/client**: HTTP adapters (reqwest).
- **crates/cli**: Application layer (clap CLI).

## 🛡️ Quality Assurance

We enforce strict quality standards:
- **Clippy**: Strict linting rules.
- **Cargo Deny**: Supply chain security.
- **Micro-dependencies**: Minimized dependency tree.
