use crate::domain::{EndpointName, SchemaTable};
use crate::error::ExplorerError;
use serde_json::Value;

/// Parses a dynamic JSON response into a list of structured `SchemaTable`s.
///
/// This function applies endpoint-specific strategies to normalize data into
/// tabular formats suitable for display and schema comparison.
///
/// # Errors
///
/// Returns `ExplorerError` if parsing fails (currently infallible, but reserved for future validation).
pub fn parse_json_to_tables(
    endpoint: EndpointName,
    json: &Value,
) -> Result<Vec<SchemaTable>, ExplorerError> {
    Ok(match endpoint {
        EndpointName::Overview | EndpointName::GlobalQuote => {
            parse_flat_map(&endpoint.to_string(), json)
        }
        EndpointName::IncomeStatement
        | EndpointName::BalanceSheet
        | EndpointName::CashFlow
        | EndpointName::Earnings => parse_financials(endpoint, json),
        EndpointName::NewsSentiment => parse_news(endpoint, json),
        EndpointName::TopGainersLosers => parse_top_movers(endpoint, json),
        // Fallback for others or unimplemented
        _ => parse_generic(&endpoint.to_string(), json),
    })
}

fn parse_flat_map(title: &str, json: &Value) -> Vec<SchemaTable> {
    if let Value::Object(map) = json {
        let mut rows = Vec::new();
        for (k, v) in map {
            rows.push(vec![k.clone(), flatten_nested(v)]);
        }
        // sort by key for consistency
        rows.sort_by(|a, b| a.first().cmp(&b.first()));

        let table = SchemaTable::new(title, vec!["Field".to_string(), "Value".to_string()], rows);
        vec![table]
    } else {
        vec![]
    }
}

fn parse_financials(endpoint: EndpointName, json: &Value) -> Vec<SchemaTable> {
    let mut tables = Vec::new();
    let keys = [
        "annualReports",
        "quarterlyReports",
        "annualEarnings",
        "quarterlyEarnings",
    ];

    if let Value::Object(map) = json {
        for key in keys {
            if let Some(Value::Array(arr)) = map.get(key) {
                let title = format!("{endpoint} - {key}");
                if let Some(table) = parse_array_to_table(title, arr) {
                    tables.push(table);
                }
            }
        }
    }

    tables
}

fn parse_news(endpoint: EndpointName, json: &Value) -> Vec<SchemaTable> {
    if let Value::Object(map) = json {
        if let Some(Value::Array(arr)) = map.get("feed") {
            let title = format!("{endpoint} - Feed");
            if let Some(table) = parse_array_to_table(title, arr) {
                return vec![table];
            }
        }
    }
    vec![]
}

fn parse_top_movers(endpoint: EndpointName, json: &Value) -> Vec<SchemaTable> {
    let mut tables = Vec::new();
    let keys = ["top_gainers", "top_losers", "most_actively_traded"];

    if let Value::Object(map) = json {
        for key in keys {
            if let Some(Value::Array(arr)) = map.get(key) {
                let title = format!("{endpoint} - {key}");
                if let Some(table) = parse_array_to_table(title, arr) {
                    tables.push(table);
                }
            }
        }
    }
    tables
}

fn parse_generic(title: &str, json: &Value) -> Vec<SchemaTable> {
    match json {
        Value::Object(_) => parse_flat_map(title, json),
        Value::Array(arr) => {
            parse_array_to_table(title.to_string(), arr).map_or_else(Vec::new, |table| vec![table])
        }
        _ => vec![],
    }
}

fn parse_array_to_table(title: String, arr: &[Value]) -> Option<SchemaTable> {
    // Extract headers from the first object
    // We assume mostly consistent schema in array
    let first = arr.first()?;
    let headers = if let Value::Object(map) = first {
        let mut keys: Vec<String> = map.keys().cloned().collect();
        keys.sort(); // consistent order
        keys
    } else {
        return None; // Non-object in array not supported for now
    };

    let total_records = arr.len();
    let take_count = 3;

    let rows: Vec<Vec<String>> = arr
        .iter()
        .take(take_count)
        .map(|item| extract_row(item, &headers))
        .collect();

    let mut table = SchemaTable::new(title, headers, rows);
    table.total_records = total_records; // SchemaTable::new uses rows.len(), so we overwrite
    Some(table)
}

fn extract_row(item: &Value, headers: &[String]) -> Vec<String> {
    if let Value::Object(map) = item {
        headers
            .iter()
            .map(|h| map.get(h).map_or_else(|| "N/A".to_string(), flatten_nested))
            .collect()
    } else {
        // Should not happen if valid array of objects
        vec!["ERROR".to_string(); headers.len()]
    }
}

fn flatten_nested(value: &Value) -> String {
    match value {
        Value::Null => "N/A".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.clone(),
        Value::Array(_) | Value::Object(_) => {
            // Simple flattening: just display first few chars or type
            // Or maybe JSON stringify properly?
            // "flatten_nested" implies we want a string representation.
            // Using to_string which does JSON serialization for complex types might be too verbose,
            // but is safe.
            value.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use serde_json::json;

    #[test]
    fn test_parse_flat_map() {
        let json = json!({
            "Symbol": "AAPL",
            "AssetType": "Common Stock"
        });
        let tables = parse_json_to_tables(EndpointName::Overview, &json).unwrap();
        assert_eq!(tables.len(), 1);
        let table = &tables[0];
        assert_eq!(table.headers, vec!["Field", "Value"]);
        assert_eq!(table.rows.len(), 2);
        assert_eq!(table.rows[0], vec!["AssetType", "Common Stock"]);
        assert_eq!(table.rows[1], vec!["Symbol", "AAPL"]);
    }

    #[test]
    fn test_parse_financials() {
        let json = json!({
            "annualReports": [
                {"fiscalDateEnding": "2023", "totalRevenue": "100"},
                {"fiscalDateEnding": "2022", "totalRevenue": "90"},
                {"fiscalDateEnding": "2021", "totalRevenue": "80"},
                {"fiscalDateEnding": "2020", "totalRevenue": "70"}
            ],
            "quarterlyReports": []
        });
        let tables = parse_json_to_tables(EndpointName::IncomeStatement, &json).unwrap();
        assert_eq!(tables.len(), 1);
        let table = &tables[0];
        assert!(table.title.contains("INCOME_STATEMENT"));
        assert_eq!(table.total_records, 4);
        assert_eq!(table.rows.len(), 3); // Truncated
        assert_eq!(table.headers, vec!["fiscalDateEnding", "totalRevenue"]);
    }

    #[test]
    fn test_missing_fields_in_row() {
        // Headers derived from first row
        let json_arr = json!([
            {"col1": "A", "col2": "B"},
            {"col1": "C"} // Missing col2
        ]);
        let table = parse_array_to_table("test".to_string(), json_arr.as_array().unwrap()).unwrap();
        assert_eq!(table.headers, vec!["col1", "col2"]);
        assert_eq!(table.rows[1], vec!["C", "N/A"]);
    }

    #[test]
    fn test_parse_top_movers() {
        let json = json!({
            "top_gainers": [{"ticker": "A", "amount": "10"}],
            "top_losers": [{"ticker": "B", "amount": "-10"}],
            "most_actively_traded": [{"ticker": "C", "volume": "1000"}]
        });
        let tables = parse_json_to_tables(EndpointName::TopGainersLosers, &json).unwrap();
        assert_eq!(tables.len(), 3);
    }

    proptest! {
        #[test]
        fn doesnt_crash_on_random_json(s in "\\PC*") {
            if let Ok(json) = serde_json::from_str::<Value>(&s) {
                 let _ = parse_json_to_tables(EndpointName::Overview, &json);
            }
        }
    }
}
