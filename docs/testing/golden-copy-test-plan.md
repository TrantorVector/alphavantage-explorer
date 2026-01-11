# Golden Copy Confirmation Test Plan

## 1. Executive Summary

This test plan provides a comprehensive framework for validating all Alpha Vantage API function implementations against golden copy outputs extracted directly from the Alpha Vantage documentation. The test suite will ensure 100% output accuracy by comparing function responses with documented expected outputs.

**Test Coverage**: 11 API functions  
**Validation Method**: Exact JSON match (character-by-character)  
**Test Data Source**: Alpha Vantage official documentation  
**Functions Excluded**: 2 (earnings-calendar, news-sentiment)

---

## 2. Test Inventory

The following table lists all functions to be tested with their exact input parameters as documented in the `*-input.txt` files:

| # | Function Name | Symbol | Additional Parameters | Output Match | Notes |
|---|---------------|--------|----------------------|--------------|-------|
| 1 | `OVERVIEW` | `IBM` | None | | Basic company overview |
| 2 | `BALANCE_SHEET` | `IBM` | None | | Financial statement |
| 3 | `CASH_FLOW` | `IBM` | None | | Financial statement |
| 4 | `INCOME_STATEMENT` | `IBM` | None | | Financial statement |
| 5 | `EARNINGS` | `IBM` | None | | Historical earnings |
| 6 | `EARNINGS_ESTIMATES` | `IBM` | None | | Analyst estimates |
| 7 | `EARNINGS_CALL_TRANSCRIPT` | `IBM` | `quarter=2024Q1` | | Requires quarter parameter |
| 8 | `DIVIDENDS` | `IBM` | Optional: `datatype=json` | | Corporate actions |
| 9 | `SPLITS` | `IBM` | Optional: `datatype=json` | | Corporate actions |
| 10 | `SHARES_OUTSTANDING` | `MSFT` | Optional: `datatype=json` | | Note: Uses MSFT, not IBM |
| 11 | `INSIDER_TRANSACTIONS` | `IBM` | None | | Insider trading data |

### Functions Not Tested

Per `not-tested.txt`:
- `EARNINGS_CALENDAR` - Not included in this test suite
- `NEWS_SENTIMENT` - Not included in this test suite

---

## 3. Test Data Structure

### 3.1 Input Files (`*-input.txt`)
Each input file contains API parameter specifications extracted from documentation:
- **Format**: Plain text with parameter descriptions
- **Location**: `/home/preetham/Documents/alphavantage-explorer/docs/golden-copy/`
- **Naming**: `<function-name>-input.txt`
- **Content**: Required and optional parameters with example values

### 3.2 Golden Copy Output Files (`*-output.json`)
Each output file contains the exact JSON response from Alpha Vantage documentation:
- **Format**: JSON (exact copy from documentation)
- **Location**: `/home/preetham/Documents/alphavantage-explorer/docs/golden-copy/`
- **Naming**: `<function-name>-output.json`
- **Purpose**: Reference output for 100% match validation

---

## 4. Test Implementation Instructions for Antigravity

> [!IMPORTANT]
> These are step-by-step instructions for Antigravity to implement the automated test harness. **Do not execute these steps until the test plan is approved.**

### 4.1 Test File Structure

Create the following test infrastructure:

1. **Test File Location**: `tests/integration/golden_copy_validation_test.rs`
   - This should be a new integration test file in the tests directory
   - Use Rust's integration test framework

2. **Test Helper Module**: `tests/common/golden_copy_helper.rs`
   - Create helper functions for loading golden copy files
   - Implement JSON comparison logic
   - Provide test result formatting utilities

### 4.2 Test Implementation Steps

#### Step 1: Create Golden Copy Loader Function
**Objective**: Load and parse golden copy JSON files

Instructions:
- Create function `load_golden_copy(function_name: &str) -> Result<serde_json::Value>`
- Path construction: `docs/golden-copy/{function_name}-output.json`
- Use `std::fs::read_to_string` to read file contents
- Parse JSON using `serde_json::from_str`
- Return parsed JSON value for comparison
- Include comprehensive error handling for file not found, parse errors

#### Step 2: Create Input Parameter Parser
**Objective**: Extract test parameters from input files

Instructions:
- Create function `parse_input_parameters(function_name: &str) -> TestParameters`
- Define `TestParameters` struct with fields: `symbol`, `quarter` (optional), `datatype` (optional)
- Read from `docs/golden-copy/{function_name}-input.txt`
- Parse parameter values based on function requirements:
  - Extract symbol from "symbol=XXX" pattern
  - Extract quarter from "quarter=YYYYQM" pattern (for EARNINGS_CALL_TRANSCRIPT)
  - Note: Most functions use `symbol=IBM`, except SHARES_OUTSTANDING uses `symbol=MSFT`
- Return structured parameters for API call construction

#### Step 3: Create JSON Comparison Function
**Objective**: Implement exact JSON matching with detailed diff reporting

Instructions:
- Create function `compare_json_exact(actual: &serde_json::Value, expected: &serde_json::Value) -> ComparisonResult`
- Define `ComparisonResult` struct with fields: `matches: bool`, `differences: Vec<String>`
- Implement recursive comparison that handles:
  - Object field comparison (key-by-key)
  - Array element comparison (index-by-index, preserving order)
  - Primitive value comparison (string, number, boolean, null)
- For mismatches, record:
  - JSON path to the mismatch (e.g., `data.reports[0].fiscalDateEnding`)
  - Expected value
  - Actual value
- Return detailed comparison result

#### Step 4: Create API Client Test Wrapper
**Objective**: Execute API calls using the production client

Instructions:
- Create function `call_api_function(function_name: &str, params: TestParameters) -> Result<serde_json::Value>`
- Initialize the Alpha Vantage client with API key from environment variable `ALPHAVANTAGE_API_KEY`
- Implement function dispatch logic:
  - Map function name string to corresponding client method call
  - Pass appropriate parameters (symbol, quarter, etc.)
  - Handle async execution properly with tokio runtime
- Return raw JSON response for comparison
- Include rate limiting awareness (tests should respect API rate limits)

#### Step 5: Implement Individual Test Cases
**Objective**: Create one test function per API function

Instructions:
- Create test functions named `test_<function_name>_golden_copy()`
- Mark each with `#[tokio::test]` attribute
- Mark each with `#[ignore]` attribute (to prevent accidental execution during regular test runs)
- Each test should:
  1. Load golden copy output using helper from Step 1
  2. Parse input parameters using helper from Step 2
  3. Call API function using wrapper from Step 4
  4. Compare actual vs expected using helper from Step 3
  5. Assert that `comparison_result.matches == true`
  6. On failure, print detailed differences from `comparison_result.differences`

Test functions to create:
- `test_overview_golden_copy()`
- `test_balance_sheet_golden_copy()`
- `test_cash_flow_golden_copy()`
- `test_income_statement_golden_copy()`
- `test_earnings_golden_copy()`
- `test_earnings_estimates_golden_copy()`
- `test_earnings_call_transcript_golden_copy()`
- `test_dividends_golden_copy()`
- `test_splits_golden_copy()`
- `test_shares_outstanding_golden_copy()`
- `test_insider_transactions_golden_copy()`

#### Step 6: Create Test Suite Runner
**Objective**: Provide a mechanism to run all golden copy tests and generate report

Instructions:
- Create function `run_all_golden_copy_tests() -> TestSuiteReport`
- Define `TestSuiteReport` struct with fields:
  - `total_tests: usize`
  - `passed: usize`
  - `failed: usize`
  - `test_results: Vec<TestResult>` (where TestResult contains function name, status, details)
- Execute all 11 test functions sequentially (to respect rate limits)
- Add delay between tests (e.g., 12 seconds per Alpha Vantage free tier rate limit)
- Collect results from each test
- Generate summary report with pass/fail counts
- Create detailed markdown report: `test-results/golden-copy-test-results.md`

#### Step 7: Create Test Results Report Template
**Objective**: Generate formatted test results in markdown

Instructions:
- Create function `generate_test_report(results: TestSuiteReport) -> String`
- Generate markdown document with:
  - Executive summary (total/passed/failed counts)
  - Table matching Section 2 format with "Output Match" column populated
  - Use ✅ for exact matches, ❌ for failures
  - For failures, include detailed diff section showing what didn't match
  - Timestamp of test execution
  - API key used (masked for security)
  - Environment information (Rust version, crate version)
- Write report to: `test-results/golden-copy-test-results.md`

#### Step 8: Create Test Execution Documentation
**Objective**: Document how to run the tests

Instructions:
- Create file: `docs/testing/GOLDEN_COPY_TESTING.md`
- Include sections:
  - **Prerequisites**: API key setup, environment variables
  - **Running Tests**: Command to execute (`cargo test --test golden_copy_validation_test -- --ignored`)
  - **Rate Limiting**: Explanation of delays between tests
  - **Interpreting Results**: How to read the test report
  - **Updating Golden Copies**: Process for updating when API changes
  - **Troubleshooting**: Common issues and solutions

### 4.3 Implementation Notes

> [!WARNING]
> **Rate Limiting Consideration**: Alpha Vantage free tier allows 5 API calls per minute. The test suite must include appropriate delays (minimum 12 seconds between calls) to avoid rate limit errors.

> [!TIP]
> **Test Isolation**: Each test should be independent and idempotent. Use `#[ignore]` attribute to prevent accidental execution during normal `cargo test` runs.

> [!CAUTION]
> **API Key Security**: Never hardcode API keys. Use environment variables (`ALPHAVANTAGE_API_KEY`) and ensure test documentation emphasizes this requirement.

---

## 5. Validation Criteria

### 5.1 Success Criteria

A test passes if and only if:
1. ✅ API call completes successfully (no network/auth errors)
2. ✅ Response JSON structure matches golden copy exactly
3. ✅ All field names match exactly (case-sensitive)
4. ✅ All field values match exactly (type and value)
5. ✅ Array ordering matches exactly
6. ✅ No extra fields in actual response
7. ✅ No missing fields from expected response

### 5.2 Failure Analysis

If a test fails, the comparison function must provide:
- **JSON path** to the first mismatch
- **Expected value** at that path
- **Actual value** at that path
- **Type mismatch details** (if types differ, e.g., string vs number)

### 5.3 Edge Cases to Handle

- **Null values**: Must match exactly
- **Empty arrays**: Must distinguish from missing arrays
- **Numeric precision**: Numbers must match exactly (consider floating point representation)
- **String whitespace**: Trailing/leading whitespace must match
- **Unicode characters**: Must handle properly in transcripts

---

## 6. Test Execution Workflow

### 6.1 Pre-Execution Checklist

Before running tests, verify:
- [ ] Alpha Vantage API key is set in environment (`ALPHAVANTAGE_API_KEY`)
- [ ] All golden copy files are present in `docs/golden-copy/`
- [ ] Test harness code has been compiled successfully
- [ ] Rate limiter is configured properly
- [ ] Network connectivity to Alpha Vantage API

### 6.2 Execution Steps

1. **Clean Build**: `cargo clean && cargo build --tests`
2. **Run Test Suite**: `cargo test --test golden_copy_validation_test -- --ignored --nocapture`
3. **Monitor Progress**: Watch console output for test progress and rate limit delays
4. **Review Results**: Check `test-results/golden-copy-test-results.md` after completion

### 6.3 Expected Execution Time

Assuming 12-second delays between tests:
- 11 tests × 12 seconds = 132 seconds (~2.2 minutes minimum)
- Plus API response time (~1-3 seconds per call)
- **Total estimated time**: ~3-5 minutes

---

## 7. Test Results Documentation

### 7.1 Report Location

Primary test results report: `test-results/golden-copy-test-results.md`

### 7.2 Report Contents

The generated report shall include:

1. **Header Section**
   - Test suite name: "Golden Copy Validation Test Results"
   - Execution timestamp
   - Test environment details
   - Summary statistics (X/11 tests passed)

2. **Results Table** (matches Section 2 format)
   - Function name
   - Input parameters
   - Output match status (✅/❌)
   - Notes/error details

3. **Detailed Failure Analysis** (if any failures)
   - Function name
   - JSON diff showing mismatches
   - Suggested remediation steps

4. **Conclusion**
   - Overall test status (PASS/FAIL)
   - Recommendations for next steps

---

## 8. Git Integration Steps

> [!IMPORTANT]
> After successful test execution, integrate test harness and results into version control.

### 8.1 Files to Commit

**New Test Infrastructure:**
- `tests/integration/golden_copy_validation_test.rs`
- `tests/common/golden_copy_helper.rs`
- `docs/testing/GOLDEN_COPY_TESTING.md`

**Test Results:**
- `test-results/golden-copy-test-results.md`

**Golden Copy Data** (if not already committed):
- `docs/golden-copy/*-input.txt` (all 11 input files)
- `docs/golden-copy/*-output.json` (all 11 output files)
- `docs/golden-copy/not-tested.txt`

### 8.2 Documentation Updates

Update the following existing documentation files:

1. **`README.md`**
   - Add section: "Testing" with subsection "Golden Copy Validation"
   - Link to detailed testing documentation
   - Badge showing test status (optional)

2. **`docs/README.md`** (if exists)
   - Add link to `GOLDEN_COPY_TESTING.md`
   - Explain purpose of golden-copy directory

3. **`CHANGELOG.md`** (or create if doesn't exist)
   - Add entry for test harness addition
   - Note: "Added comprehensive golden copy validation test suite"

### 8.3 Git Commit Workflow

Execute the following git operations:

**Step 1: Stage Test Files**
```bash
git add tests/integration/golden_copy_validation_test.rs
git add tests/common/golden_copy_helper.rs
git add docs/testing/GOLDEN_COPY_TESTING.md
```

**Step 2: Stage Test Results**
```bash
git add test-results/golden-copy-test-results.md
```

**Step 3: Stage Golden Copy Data**
```bash
git add docs/golden-copy/
```

**Step 4: Stage Documentation Updates**
```bash
git add README.md
git add docs/README.md  # if exists
git add CHANGELOG.md
```

**Step 5: Commit with Descriptive Message**
```bash
git commit -m "Add golden copy validation test suite

- Implement comprehensive test harness for 11 API functions
- Add golden copy input/output files from Alpha Vantage docs
- Create detailed testing documentation
- Include test results report
- Update README with testing instructions

Tests validate 100% output accuracy against official documentation."
```

**Step 6: Push to Repository**
```bash
git push origin main
```

### 8.4 Post-Commit Verification

After pushing:
- [ ] Verify all files appear in GitHub repository
- [ ] Check that markdown files render correctly
- [ ] Confirm test results are visible and readable
- [ ] Validate links in documentation work correctly

---

## 9. Maintenance and Updates

### 9.1 When to Re-run Tests

Golden copy tests should be executed:
- ✅ After any changes to API client implementation
- ✅ Before releasing a new version
- ✅ When Alpha Vantage updates their API (update golden copies first)
- ✅ Quarterly (to catch any undocumented API changes)

### 9.2 Updating Golden Copies

If Alpha Vantage changes their API output format:

1. **Update Golden Copy Files**:
   - Download new output from Alpha Vantage documentation
   - Replace content in `docs/golden-copy/*-output.json` files
   - Document the API version/date in commit message

2. **Re-run Tests**:
   - Execute full test suite
   - Verify all tests pass with new golden copies

3. **Update Documentation**:
   - Note API version in `GOLDEN_COPY_TESTING.md`
   - Update CHANGELOG with API version change

### 9.3 Adding New Functions

When new API functions are added:

1. Create `<function-name>-input.txt` with parameters
2. Create `<function-name>-output.json` with expected output
3. Add test case following Step 5 pattern
4. Update test inventory table (Section 2)
5. Increment total test count in all documentation

---

## 10. Acceptance Criteria

This test plan is considered successfully implemented when:

- [x] Test plan document is reviewed and approved
- [ ] All 8 implementation steps (Section 4.2) are completed
- [ ] Test suite executes successfully end-to-end
- [ ] All 11 test cases pass with 100% match
- [ ] Test results report is generated automatically
- [ ] All files are committed to git repository
- [ ] Documentation is updated and pushed
- [ ] Tests can be re-run by any developer following documentation

---

## 11. Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Rate limit exceeded | Medium | High | Implement 12s delays; mark tests as `#[ignore]` |
| API output format changes | Low | High | Document golden copy update process |
| Network connectivity issues | Medium | Medium | Implement retry logic with exponential backoff |
| JSON precision mismatches | Low | Medium | Define clear numeric comparison rules |
| API key not configured | Medium | High | Clear documentation; helpful error messages |

---

## 12. Appendix: Function Details

### 12.1 Parameter Matrix

| Function | Symbol | Quarter | Datatype | Special Notes |
|----------|--------|---------|----------|---------------|
| OVERVIEW | IBM | - | - | - |
| BALANCE_SHEET | IBM | - | - | - |
| CASH_FLOW | IBM | - | - | - |
| INCOME_STATEMENT | IBM | - | - | Note: Output file has typo "inome-statement-output.json" |
| EARNINGS | IBM | - | - | - |
| EARNINGS_ESTIMATES | IBM | - | - | - |
| EARNINGS_CALL_TRANSCRIPT | IBM | 2024Q1 | - | Only function requiring quarter |
| DIVIDENDS | IBM | - | json | Datatype optional, default json |
| SPLITS | IBM | - | json | Datatype optional, default json |
| SHARES_OUTSTANDING | MSFT | - | json | Different symbol! Uses MSFT not IBM |
| INSIDER_TRANSACTIONS | IBM | - | - | - |

### 12.2 File Size Reference

For planning test execution time and memory requirements:

| Function | Output File Size | Complexity |
|----------|-----------------|------------|
| INSIDER_TRANSACTIONS | 1.5 MB | Very High |
| BALANCE_SHEET | 189 KB | High |
| CASH_FLOW | 155 KB | High |
| INCOME_STATEMENT | 122 KB | High |
| EARNINGS_ESTIMATES | 44 bytes | Very Low (likely error response) |
| EARNINGS_CALL_TRANSCRIPT | 57 KB | Medium |
| EARNINGS | 38 KB | Medium |
| DIVIDENDS | 22 KB | Low |
| SHARES_OUTSTANDING | 11 KB | Low |
| OVERVIEW | 2.6 KB | Low |
| SPLITS | 247 bytes | Very Low |

> [!NOTE]
> The `earnings-estimates-output.json` file is only 44 bytes, which suggests it may contain an error response or empty data. This should be verified during implementation.

---

## Document Control

- **Document Name**: Golden Copy Confirmation Test Plan
- **Version**: 1.0
- **Created**: 2026-01-11
- **Author**: Antigravity AI
- **Status**: Pending Approval
- **Next Review**: After user approval
