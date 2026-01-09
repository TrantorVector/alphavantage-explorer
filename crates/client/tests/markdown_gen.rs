#![recursion_limit = "256"]
use alphavantage_client::MarkdownWriterImpl;
use alphavantage_core::domain::EndpointName;
use alphavantage_core::logic::json_to_table::parse_json_to_tables;
use alphavantage_core::ports::MarkdownWriter;
use serde_json::json;

#[test]
fn generate_overview_markdown() {
    let json = json!({
        "Symbol": "AAPL",
        "AssetType": "Common Stock",
        "Name": "Apple Inc",
        "Description": "Apple Inc. designs, manufactures, and markets smartphones...",
        "CIK": "320193",
        "Exchange": "NASDAQ",
        "Currency": "USD",
        "Country": "USA",
        "Sector": "Technology",
        "Industry": "Consumer Electronics",
        "Address": "One Apple Park Way, Cupertino, CA, United States",
        "FiscalYearEnd": "September",
        "LatestQuarter": "2023-12-30",
        "MarketCapitalization": "2800000000000",
        "EBITDA": "130000000000",
        "PERatio": "28.5",
        "PEGRatio": "2.1",
        "BookValue": "4.5",
        "DividendPerShare": "0.95",
        "DividendYield": "0.005",
        "EPS": "6.4",
        "RevenuePerShareTTM": "24.5",
        "ProfitMargin": "0.25",
        "OperatingMarginT": "0.30",
        "ReturnOnAssetsTTM": "0.20",
        "ReturnOnEquityTTM": "1.50",
        "RevenueTTM": "385000000000",
        "GrossProfitTTM": "170000000000",
        "DilutedEPSTTM": "6.4",
        "QuarterlyEarningsGrowthYOY": "0.05",
        "QuarterlyRevenueGrowthYOY": "0.02",
        "AnalystTargetPrice": "200.0",
        "TrailingPE": "28.5",
        "ForwardPE": "26.0",
        "PriceToSalesRatioTTM": "7.5",
        "PriceToBookRatio": "40.0",
        "EVToRevenue": "7.6",
        "EVToEBITDA": "22.5",
        "Beta": "1.2",
        "52WeekHigh": "199.6",
        "52WeekLow": "124.1",
        "50DayMovingAverage": "185.0",
        "200DayMovingAverage": "175.0",
        "SharesOutstanding": "15500000000",
        "DividendDate": "2024-02-15",
        "ExDividendDate": "2024-02-09"
    });

    let tables = parse_json_to_tables(EndpointName::Overview, &json).unwrap();
    let mut writer = MarkdownWriterImpl::new();

    for table in tables {
        writer.write_table(&table).unwrap();
    }

    // We print it to stdout so we can capture it for the report
    println!(
        "--- OVERVIEW MARKDOWN BEGIN ---\n{}\n--- OVERVIEW MARKDOWN END ---",
        writer.as_str()
    );

    insta::assert_snapshot!("overview_report", writer.as_str());
}

#[test]
fn generate_income_statement_markdown() {
    let json = json!({
        "symbol": "IBM",
        "annualReports": [
            {
                "fiscalDateEnding": "2023-12-31",
                "reportedCurrency": "USD",
                "grossProfit": "32688000000",
                "totalRevenue": "61860000000",
                "costOfRevenue": "29172000000",
                "costofGoodsAndServicesSold": "29172000000",
                "operatingIncome": "2626000000",
                "sellingGeneralAndAdministrative": "18949000000",
                "researchAndDevelopment": "6986000000",
                "operatingExpenses": "29234000000",
                "investmentIncomeNet": "None",
                "netInterestIncome": "-1024000000",
                "interestIncome": "1196000000",
                "interestExpense": "2220000000",
                "nonInterestIncome": "218000000",
                "otherNonOperatingIncome": "-667000000",
                "depreciation": "4480000000",
                "depreciationAndAmortization": "4480000000",
                "incomeBeforeTax": "8712000000",
                "incomeTaxExpense": "1205000000",
                "interestAndDebtExpense": "2220000000",
                "netIncomeFromContinuingOperations": "7507000000",
                "comprehensiveIncomeNetOfTax": "7878000000",
                "ebit": "10932000000",
                "ebitda": "15412000000",
                "netIncome": "7502000000"
            },
            {
                "fiscalDateEnding": "2022-12-31",
                "reportedCurrency": "USD",
                "totalRevenue": "60530000000"
            },
            {
                "fiscalDateEnding": "2021-12-31",
                "reportedCurrency": "USD",
                "totalRevenue": "57350000000"
            },
            {
                "fiscalDateEnding": "2020-12-31",
                "reportedCurrency": "USD",
                "totalRevenue": "55000000000"
            }
        ],
        "quarterlyReports": []
    });

    let tables = parse_json_to_tables(EndpointName::IncomeStatement, &json).unwrap();
    let mut writer = MarkdownWriterImpl::new();

    for table in tables {
        writer.write_table(&table).unwrap();
    }

    println!(
        "--- INCOME STATEMENT MARKDOWN BEGIN ---\n{}\n--- INCOME STATEMENT MARKDOWN END ---",
        writer.as_str()
    );

    insta::assert_snapshot!("income_statement_report", writer.as_str());
}
