use alphavantage_core::domain::{EndpointName, SchemaTable, TickerSymbol};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemaDiff {
    pub endpoint: EndpointName,
    pub union_headers: Vec<String>,
    // ticker -> (missing, extra)
    pub differences: HashMap<TickerSymbol, (Vec<String>, Vec<String>)>,
}

pub struct SchemaAnalyzerImpl;

impl Default for SchemaAnalyzerImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl SchemaAnalyzerImpl {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Computes schema differences for a single endpoint across multiple tickers.
    #[must_use]
    pub fn compute_schema_diff(
        &self,
        endpoint: EndpointName,
        tables_by_ticker: &HashMap<TickerSymbol, Vec<SchemaTable>>,
    ) -> SchemaDiff {
        let mut all_headers = HashSet::new();

        // 1. Compute Union of Headers
        // Note: An endpoint can produce MULTIPLE tables (e.g. IncomeStatement -> annual, quarterly).
        // We should probably diff BY TABLE TITLE or just aggregate all headers seen for that endpoint?
        // If we mix annual and quarterly headers, it might be messy but usually they are same.
        // Let's assume we want to diff specific tables, but endpoints are the grouping.
        // Let's collect ALL headers from ALL tables for this endpoint.

        for tables in tables_by_ticker.values() {
            for table in tables {
                for header in &table.headers {
                    all_headers.insert(header.clone());
                }
            }
        }

        let mut union_headers: Vec<String> = all_headers.into_iter().collect();
        union_headers.sort();

        // 2. Compute differences per ticker
        let mut differences = HashMap::new();

        for (ticker, tables) in tables_by_ticker {
            let mut ticker_headers = HashSet::new();
            for table in tables {
                for header in &table.headers {
                    ticker_headers.insert(header.clone());
                }
            }

            let mut missing = Vec::new();
            let extra = Vec::new(); // extra relative to what? relative to intersection?
                                    // "Extra" implies present in THIS ticker but not in OTHERS?
                                    // "Missing" implies present in UNION but not in THIS ticker.

            for h in &union_headers {
                if !ticker_headers.contains(h) {
                    missing.push(h.clone());
                }
            }

            // If "UNION" is the superset, then by definition no ticker has "extra" relative to UNION.
            // Maybe "extra" means fields present in this ticker that are NOT in the "standard" schema?
            // But we don't have a standard schema.
            // Let's redefine:
            // "Missing" = In Union but not in Ticker
            // "Present" = In Ticker
            // So we just track missing.
            // BUT, usually we want to know outliers.
            // Let's just track "Missing".
            // Wait, if one ticker has a unique field X, it goes into Union.
            // Then ALL OTHER tickers will have X as "Missing".
            // This effectively highlights the outlier.

            missing.sort();
            // We'll leave 'extra' empty for now as it doesn't make sense with union semantics
            // unless we diff against a "common intersection" baseline.

            if !missing.is_empty() {
                differences.insert(ticker.clone(), (missing, extra));
            }
        }

        SchemaDiff {
            endpoint,
            union_headers,
            differences,
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use alphavantage_core::domain::TickerSymbol;

    fn make_table(headers: Vec<&str>) -> SchemaTable {
        SchemaTable {
            title: "Test".to_string(),
            headers: headers.into_iter().map(String::from).collect(),
            rows: vec![],
            total_records: 0,
        }
    }

    #[test]
    fn test_compute_schema_diff_matching() {
        let analyzer = SchemaAnalyzerImpl::new();
        let ticker1 = TickerSymbol::new("AAPL").unwrap();
        let ticker2 = TickerSymbol::new("MSFT").unwrap();

        let tables1 = vec![make_table(vec!["A", "B"])];
        let tables2 = vec![make_table(vec!["A", "B"])];

        let mut map = HashMap::new();
        map.insert(ticker1, tables1);
        map.insert(ticker2, tables2);

        let diff = analyzer.compute_schema_diff(EndpointName::Overview, &map);

        assert_eq!(diff.union_headers, vec!["A", "B"]);
        assert!(
            diff.differences.is_empty(),
            "No differences should be found"
        );
    }

    #[test]
    fn test_compute_schema_diff_missing() {
        let analyzer = SchemaAnalyzerImpl::new();
        let ticker1 = TickerSymbol::new("AAPL").unwrap(); // Missing B
        let ticker2 = TickerSymbol::new("MSFT").unwrap(); // Has A, B

        let tables1 = vec![make_table(vec!["A"])];
        let tables2 = vec![make_table(vec!["A", "B"])];

        let mut map = HashMap::new();
        map.insert(ticker1.clone(), tables1);
        map.insert(ticker2, tables2);

        let diff = analyzer.compute_schema_diff(EndpointName::Overview, &map);

        assert_eq!(diff.union_headers, vec!["A", "B"]);
        let (missing, _) = diff.differences.get(&ticker1).unwrap();
        assert_eq!(missing, &vec!["B".to_string()]);
    }

    #[test]
    fn test_compute_schema_diff_unique_fields() {
        let analyzer = SchemaAnalyzerImpl::new();
        let aapl = TickerSymbol::new("AAPL").unwrap(); // Missing GPU
        let nvda = TickerSymbol::new("NVDA").unwrap(); // Has GPU

        let tables_aapl = vec![make_table(vec!["Rev", "EPS"])];
        let tables_nvda = vec![make_table(vec!["Rev", "EPS", "GPU"])];

        let mut map = HashMap::new();
        map.insert(aapl.clone(), tables_aapl);
        map.insert(nvda, tables_nvda);

        let diff = analyzer.compute_schema_diff(EndpointName::Overview, &map);

        assert_eq!(diff.union_headers, vec!["EPS", "GPU", "Rev"]);

        // AAPL should miss GPU
        let (missing_aapl, _) = diff.differences.get(&aapl).unwrap();
        assert_eq!(missing_aapl, &vec!["GPU".to_string()]);
    }
}
