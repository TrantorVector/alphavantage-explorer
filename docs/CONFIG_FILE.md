# Configuration File Support

The Alpha Vantage Explorer now supports loading API credentials and rate limits from a configuration file.

## Setup

1. **Create your configuration file** from the template:
   ```bash
   cp alphavantage.toml.template alphavantage.toml
   ```

2. **Edit `alphavantage.toml`** and replace `YOUR_API_KEY_HERE` with your actual Alpha Vantage API key:
   ```toml
   [api]
   api_key = "YOUR_ACTUAL_API_KEY"

   [rate_limit]
   daily_limit = 25  # Change this if you have a paid tier
   ```

3. **Run the CLI** - it will automatically load your configuration:
   ```bash
   cargo run -- --live-api --out-dir ./output AAPL NVDA
   ```

## Configuration File Location

The CLI searches for `alphavantage.toml` in:
- Current directory
- Parent directories (useful when running from subdirectories)

## Rate Limiting

The `daily_limit` value in the configuration file controls how many API calls you can make per day:

- **Free tier**: 25 calls/day (default)
- **Paid tiers**: Set this to your plan's limit

The rate limiter automatically:
- Tracks API calls across application restarts
- Resets at midnight UTC
- Updates when you change your daily limit

## Upgrading Your Plan

When you upgrade to a paid tier:

1. Edit `alphavantage.toml`
2. Update both `api_key` and `daily_limit`
3. Run the CLI - changes take effect immediately

No code changes required!

## Security

✅ **The `alphavantage.toml` file is git-ignored** - your API key will not be committed to version control.

⚠️ **Never commit `alphavantage.toml`** - keep your API key secure!

## Fallback Behavior

If `alphavantage.toml` is not found:
- In **mock mode**: Uses a dummy key (no configuration needed)
- In **live mode**: Falls back to:
  - `--api-key` flag
  - `ALPHA_VANTAGE_API_KEY` environment variable
  - Default rate limit of 25 calls/day

## Example Workflows

### Development (Mock Mode)
```bash
# No configuration needed
cargo run -- --out-dir ./output AAPL
```

### Production (Live Mode)
```bash
# Uses alphavantage.toml
cargo run -- --live-api --out-dir ./output AAPL NVDA MU
```

### Override with CLI
```bash  
# CLI flag takes precedence over config file
cargo run -- --live-api --api-key "DIFFERENT_KEY" --out-dir ./output AAPL
```
