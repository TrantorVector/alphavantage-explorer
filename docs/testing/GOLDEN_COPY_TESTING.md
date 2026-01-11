# Golden Copy Testing Guide

This document explains how to run and maintain the Golden Copy Validation Test Suite for the Alpha Vantage Explorer project.

## Overview

The golden copy tests validate that API function implementations produce outputs that exactly match the documented responses from Alpha Vantage's official documentation. These tests ensure 100% output accuracy.

## Test Coverage

**Functions Tested**: 11 API functions
- `OVERVIEW`
- `BALANCE_SHEET`
- `CASH_FLOW`
- `INCOME_STATEMENT`
- `EARNINGS`
- `EARNINGS_ESTIMATES`
- `EARNINGS_CALL_TRANSCRIPT`
- `DIVIDENDS`
- `SPLITS`
- `SHARES_OUTSTANDING`
- `INSIDER_TRANSACTIONS`

**Functions Not Tested**:
- `EARNINGS_CALENDAR` - Not included in golden copy suite
- `NEWS_SENTIMENT` - Not included in golden copy suite

## Prerequisites

### 1. Alpha Vantage API Key

You must have a valid Alpha Vantage API key set in your environment:

```bash
export ALPHAVANTAGE_API_KEY="your_api_key_here"
```

Or create a `.env` file in the project root:

```
ALPHAVANTAGE_API_KEY=your_api_key_here
```

### 2. Golden Copy Files

Ensure all golden copy files are present in `docs/golden-copy/`:
- 11 `*-input.txt` files with API parameters
- 11 `*-output.json` files with expected responses
- `not-tested.txt` listing excluded functions

### 3. Test Results Directory

The tests will automatically create a `test-results/` directory for reports and diff files.

## Running the Tests

### Run All Tests

To run the complete test suite (all 11 functions):

```bash
cargo test --package alphavantage_client --test golden_copy_validation run_all_golden_copy_tests -- --ignored --nocapture
```

**Expected Duration**: ~3-5 minutes (includes 12-second delays between tests for rate limiting)

### Run Individual Tests

To run a single function test:

```bash
# Example: Test OVERVIEW function
cargo test --package alphavantage_client --test golden_copy_validation test_overview_golden_copy -- --ignored --nocapture

# Example: Test EARNINGS_CALL_TRANSCRIPT function
cargo test --package alphavantage_client --test golden_copy_validation test_earnings_call_transcript_golden_copy -- --ignored --nocapture
```

### Run Helper Tests

The helper module has unit tests that can be run without API calls:

```bash
cargo test --package alphavantage_client --test golden_copy_helper
```

## Understanding Test Results

### Console Output

During execution, you'll see:
- ✅ **PASS**: API output matches golden copy exactly
- ❌ **FAIL**: Differences found (details shown in console and diff files)
- ⏳ Rate limit delays between tests

### Test Report

After running the full suite, check the generated report:

**Location**: `test-results/golden-copy-test-results.md`

The report includes:
- Summary statistics (total/passed/failed)
- Detailed results table
- Failure analysis with diff file references
- Environment information

### Diff Files

When tests fail, detailed differences are written to:

```
test-results/<function-name>-diff.txt
```

Each diff file shows:
- JSON path to mismatches
- Expected values
- Actual values

## Rate Limiting

> [!IMPORTANT]
> The tests respect Alpha Vantage free tier rate limits (5 calls per minute).

**Implementation**:
- 12-second delay between each test
- Tests run sequentially, not in parallel
- Total suite execution: ~3-5 minutes for 11 tests

## Common Issues and Solutions

### Issue: `ALPHAVANTAGE_API_KEY` not set

**Error**: `ALPHAVANTAGE_API_KEY environment variable must be set`

**Solution**: Set the API key in your environment:
```bash
export ALPHAVANTAGE_API_KEY="your_key"
```

### Issue: Rate limit exceeded

**Error**: `429 Too Many Requests` or similar

**Solution**: 
- Wait a few minutes before retrying
- Ensure you're running tests sequentially with the `--ignored` flag
- Check that delays between tests are working (should see "⏳ Waiting 12 seconds...")

### Issue: Golden copy file not found

**Error**: `Failed to read golden copy file`

**Solution**: 
- Verify all files exist in `docs/golden-copy/`
- Check file naming matches expected pattern: `<function-name>-output.json`
- Note: `income-statement` has a typo in the filename (`inome-statement-output.json`)

### Issue: JSON mismatch

**Error**: `Output does not match golden copy`

**Possible Causes**:
1. **API output changed**: Alpha Vantage updated their API response format
2. **Implementation bug**: Client code has a bug
3. **Serialization issue**: Response deserialization/serialization differs

**Debugging Steps**:
1. Check the diff file in `test-results/<function-name>-diff.txt`
2. Compare expected vs actual values
3. If API changed, update golden copy files (see Maintenance section)
4. If implementation issue, fix the client code

## Maintenance

### When to Re-run Tests

Execute golden copy tests:
- ✅ After changes to API client implementation
- ✅ Before releasing a new version
- ✅ When Alpha Vantage announces API updates
- ✅ Quarterly (to catch undocumented API changes)

### Updating Golden Copies

If Alpha Vantage changes their API output format:

**Step 1**: Download new outputs from Alpha Vantage documentation

**Step 2**: Update golden copy files
```bash
# Replace content in docs/golden-copy/*-output.json files
# with the new documented responses
```

**Step 3**: Re-run tests to verify
```bash
cargo test --package alphavantage_client --test golden_copy_validation run_all_golden_copy_tests -- --ignored --nocapture
```

**Step 4**: Commit changes
```bash
git add docs/golden-copy/
git commit -m "Update golden copies for API version X.Y"
```

### Adding New Functions

To add a new function to the test suite:

**Step 1**: Create golden copy files
```bash
# Create in docs/golden-copy/
touch <function-name>-input.txt
touch <function-name>-output.json
```

**Step 2**: Populate with documentation data
- Copy API parameters to `*-input.txt`
- Copy expected output to `*-output.json`

**Step 3**: Add test case in `golden_copy_validation.rs`
```rust
#[tokio::test]
#[ignore]
async fn test_new_function_golden_copy() -> Result<()> {
    let client = create_client()?;
    let params = parse_input_parameters("new-function")?;
    
    execute_test("NEW_FUNCTION", "new-function", || async {
        let response = client
            .fundamentals()
            .new_function(&params.symbol)
            .send()
            .await?;
        
        Ok(serde_json::to_value(response)?)
    })
    .await?;
    
    rate_limit_delay().await;
    Ok(())
}
```

**Step 4**: Add to test suite runner (in `run_all_golden_copy_tests`)
```rust
run_test!("NEW_FUNCTION", test_new_function_golden_copy);
```

**Step 5**: Update documentation
- Update test count in this file
- Update test plan if needed

## Test Architecture

### Files

- **`golden_copy_helper.rs`**: Helper functions for loading, parsing, and comparing
- **`golden_copy_validation.rs`**: Individual test cases and suite runner
- **`docs/golden-copy/`**: Golden copy input/output files

### Key Functions

**Helper Module**:
- `load_golden_copy(function_name)` - Load expected JSON from file
- `parse_input_parameters(function_name)` - Extract test parameters
- `compare_json_exact(actual, expected)` - Deep JSON comparison

**Validation Tests**:
- `execute_test(name, function_name, api_call)` - Generic test executor
- `run_all_golden_copy_tests()` - Full suite runner with reporting
- Individual `test_*_golden_copy()` functions for each API endpoint

## Continuous Integration

To integrate with CI/CD pipelines:

```yaml
# Example GitHub Actions workflow
- name: Run Golden Copy Tests
  run: |
    export ALPHAVANTAGE_API_KEY=${{ secrets.ALPHAVANTAGE_API_KEY }}
    cargo test --package alphavantage_client --test golden_copy_validation run_all_golden_copy_tests -- --ignored --nocapture
  env:
    ALPHAVANTAGE_API_KEY: ${{ secrets.ALPHAVANTAGE_API_KEY }}
```

> [!WARNING]
> Be cautious with automated CI runs due to rate limiting. Consider:
> - Running on-demand instead of every commit
> - Using a dedicated API key for CI
> - Implementing caching mechanisms

## FAQ

**Q: Why are tests marked with `#[ignore]`?**  
A: To prevent accidental execution during normal `cargo test` runs, which would consume API quota and take ~5 minutes.

**Q: Can I run tests in parallel?**  
A: No, tests must run sequentially due to API rate limits.

**Q: What if I don't have an API key?**  
A: You cannot run the full integration tests. However, you can run the helper module unit tests without an API key.

**Q: Are golden copies version-controlled?**  
A: Yes, all golden copy files are committed to git as the source of truth.

## Support

For issues or questions:
1. Check this documentation first
2. Review test output and diff files
3. Check Alpha Vantage documentation for API changes
4. Open an issue on GitHub with test results and diff files

---

**Last Updated**: 2026-01-11  
**Test Suite Version**: 1.0
