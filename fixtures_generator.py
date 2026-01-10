import json
import os
from copy import deepcopy

# Define base paths
BASE_PATH = "crates/client/fixtures/tickers"
TICKERS = ["AAPL", "NVDA", "MU"]
FILES = ["INCOME_STATEMENT", "BALANCE_SHEET", "CASH_FLOW"]

# Define Full Schemas (Templates)
# These represent a single "Report" object with ALL fields populated.
TEMPLATES = {
    "INCOME_STATEMENT": {
        "fiscalDateEnding": "2023-09-30",
        "reportedCurrency": "USD",
        "grossProfit": "10000000000",
        "totalRevenue": "20000000000",
        "costOfRevenue": "10000000000",
        "costofGoodsAndServicesSold": "10000000000",
        "operatingIncome": "5000000000",
        "sellingGeneralAndAdministrative": "2000000000",
        "researchAndDevelopment": "3000000000",
        "operatingExpenses": "5000000000",
        "investmentIncomeNet": "100000000",
        "netInterestIncome": "-100000000",
        "interestIncome": "200000000",
        "interestExpense": "300000000",
        "nonInterestIncome": "50000000",
        "otherNonOperatingIncome": "50000000",
        "depreciation": "1000000000",
        "depreciationAndAmortization": "1000000000",
        "incomeBeforeTax": "5100000000",
        "incomeTaxExpense": "1000000000",
        "interestAndDebtExpense": "300000000",
        "netIncomeFromContinuingOperations": "4100000000",
        "comprehensiveIncomeNetOfTax": "4100000000",
        "ebit": "5400000000",
        "ebitda": "6400000000",
        "netIncome": "4100000000"
    },
    "BALANCE_SHEET": {
        "fiscalDateEnding": "2023-09-30",
        "reportedCurrency": "USD",
        "totalAssets": "300000000000",
        "totalCurrentAssets": "100000000000",
        "cashAndCashEquivalentsAtCarryingValue": "20000000000",
        "cashAndShortTermInvestments": "40000000000",
        "inventory": "5000000000",
        "currentNetReceivables": "30000000000",
        "totalNonCurrentAssets": "200000000000",
        "propertyPlantEquipment": "50000000000",
        "accumulatedDepreciationAmortizationPPE": "20000000000",
        "intangibleAssets": "0",
        "intangibleAssetsExcludingGoodwill": "0",
        "goodwill": "0",
        "investments": "100000000000",
        "longTermInvestments": "100000000000",
        "shortTermInvestments": "20000000000",
        "otherCurrentAssets": "5000000000",
        "otherNonCurrentAssets": "50000000000",
        "totalLiabilities": "200000000000",
        "totalCurrentLiabilities": "100000000000",
        "currentAccountsPayable": "50000000000",
        "deferredRevenue": "5000000000",
        "currentDebt": "10000000000",
        "shortTermDebt": "10000000000",
        "totalNonCurrentLiabilities": "100000000000",
        "capitalLeaseObligations": "0",
        "longTermDebt": "90000000000",
        "currentLongTermDebt": "2000000000",
        "longTermDebtNoncurrent": "90000000000",
        "shortLongTermDebtTotal": "12000000000",
        "otherCurrentLiabilities": "33000000000",
        "otherNonCurrentLiabilities": "8000000000",
        "totalShareholderEquity": "100000000000",
        "treasuryStock": "0",
        "retainedEarnings": "50000000000",
        "commonStock": "50000000000",
        "commonStockSharesOutstanding": "10000000000"
    },
    "CASH_FLOW": {
        "fiscalDateEnding": "2023-09-30",
        "reportedCurrency": "USD",
        "operatingCashflow": "20000000000",
        "paymentsForOperatingActivities": "5000000000",
        "proceedsFromOperatingActivities": "25000000000",
        "changeInOperatingLiabilities": "1000000000",
        "changeInOperatingAssets": "-2000000000",
        "depreciationDepletionAndAmortization": "10000000000",
        "capitalExpenditures": "-5000000000",
        "changeInReceivables": "-1000000000",
        "changeInInventory": "-500000000",
        "profitLoss": "0",
        "cashflowFromInvestment": "-10000000000",
        "cashflowFromFinancing": "-15000000000",
        "proceedsFromRepaymentsOfShortTermDebt": "0",
        "paymentsForRepurchaseOfCommonStock": "-10000000000",
        "paymentsForRepurchaseOfEquity": "-10000000000",
        "paymentsForRepurchaseOfPreferredStock": "0",
        "dividendPayout": "3000000000",
        "dividendPayoutCommonStock": "3000000000",
        "dividendPayoutPreferredStock": "0",
        "proceedsFromIssuanceOfCommonStock": "0",
        "proceedsFromIssuanceOfLongTermDebtAndCapitalSecuritiesNet": "0",
        "proceedsFromIssuanceOfPreferredStock": "0",
        "proceedsFromRepurchaseOfEquity": "0",
        "proceedsFromSaleOfTreasuryStock": "0",
        "changeInCashAndCashEquivalents": "-5000000000",
        "changeInExchangeRate": "0",
        "netIncome": "10000000000"
    }
}

# Ticker specific data overrides to make it look realistic (Values are approx)
DATA_OVERRIDES = {
    "AAPL": {
        "INCOME_STATEMENT": [
            {"fiscalDateEnding": "2023-09-30", "totalRevenue": "383285000000", "netIncome": "96995000000"},
            {"fiscalDateEnding": "2022-09-24", "totalRevenue": "394328000000", "netIncome": "99803000000"},
            {"fiscalDateEnding": "2021-09-25", "totalRevenue": "365817000000", "netIncome": "94680000000"}
        ],
        "quarterly_dates": ["2023-12-30", "2023-09-30", "2023-07-01"]
    },
    "NVDA": {
        "INCOME_STATEMENT": [
            {"fiscalDateEnding": "2024-01-28", "totalRevenue": "60922000000", "netIncome": "29524000000"},
            {"fiscalDateEnding": "2023-01-29", "totalRevenue": "26974000000", "netIncome": "4368000000"},
            {"fiscalDateEnding": "2022-01-30", "totalRevenue": "26914000000", "netIncome": "9752000000"}
        ],
        "quarterly_dates": ["2024-01-28", "2023-10-29", "2023-07-30"]
    },
    "MU": {
        "INCOME_STATEMENT": [
            {"fiscalDateEnding": "2023-08-31", "totalRevenue": "15537000000", "netIncome": "-5833000000"},
            {"fiscalDateEnding": "2022-09-01", "totalRevenue": "30758000000", "netIncome": "8687000000"},
            {"fiscalDateEnding": "2021-09-02", "totalRevenue": "27705000000", "netIncome": "5861000000"}
        ],
        "quarterly_dates": ["2023-11-30", "2023-08-31", "2023-06-01"]
    }
}

def generate_report_entry(template, specific_data):
    """Creates a full-schema dictionary with overwritten specific values."""
    entry = deepcopy(template)
    entry.update(specific_data)
    return entry

def main():
    for ticker in TICKERS:
        ticker_path = os.path.join(BASE_PATH, ticker)
        os.makedirs(ticker_path, exist_ok=True)
        
        # Get configured dates/data
        overrides = DATA_OVERRIDES.get(ticker, {})
        income_years = overrides.get("INCOME_STATEMENT", [])
        q_dates = overrides.get("quarterly_dates", [])
        
        for file_type in FILES:
             template = TEMPLATES[file_type]
             full_json = {"symbol": ticker, "annualReports": [], "quarterlyReports": []}
             
             # Generate 3 Annual Reports
             for i in range(3):
                 # Use date from income statement override if avail, else placeholder
                 yr_data = income_years[i] if i < len(income_years) else {}
                 # For balance/cash flow we just reuse the date
                 date = yr_data.get("fiscalDateEnding", f"2023-01-0{i+1}")
                 
                 specifics = {
                     "fiscalDateEnding": date,
                     "reportedCurrency": "USD"
                 }
                 # Inject revenue/netIncome if mapped in override (useful for Income/Cash)
                 if "totalRevenue" in yr_data and file_type == "INCOME_STATEMENT":
                     specifics["totalRevenue"] = yr_data["totalRevenue"]
                 if "netIncome" in yr_data and file_type in ["INCOME_STATEMENT", "CASH_FLOW"]:
                     specifics["netIncome"] = yr_data["netIncome"]
                     
                 full_json["annualReports"].append(generate_report_entry(template, specifics))
                 
             # Generate 3 Quarterly Reports
             for i in range(3):
                 date = q_dates[i] if i < len(q_dates) else f"2023-0{i+1}-01"
                 specifics = {
                     "fiscalDateEnding": date,
                     "reportedCurrency": "USD"
                 }
                 # Scale down values for quarterly approx (divide by 4 just for visual difference)
                 if file_type == "INCOME_STATEMENT" and len(full_json["annualReports"]) > 0:
                      ann_rev = float(full_json["annualReports"][0].get("totalRevenue", "100"))
                      specifics["totalRevenue"] = str(int(ann_rev / 4))
                      
                 full_json["quarterlyReports"].append(generate_report_entry(template, specifics))
             
             # Write File
             file_path = os.path.join(ticker_path, f"{file_type}.json")
             with open(file_path, "w") as f:
                 json.dump(full_json, f, indent=4)
             print(f"Generated {file_path}")

if __name__ == "__main__":
    main()
