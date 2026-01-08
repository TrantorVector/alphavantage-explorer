use strum::{Display, EnumString, IntoStaticStr};

/// Represents supported Alpha Vantage API endpoints.
///
/// Variants correspond to the `function=...` query parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, Display, IntoStaticStr)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum EndpointName {
    // Stock Time Series
    TimeSeriesIntraday,
    TimeSeriesDaily,
    TimeSeriesDailyAdjusted,
    TimeSeriesWeekly,
    TimeSeriesWeeklyAdjusted,
    TimeSeriesMonthly,
    TimeSeriesMonthlyAdjusted,

    // Quote
    GlobalQuote,
    SymbolSearch,

    // Market Status
    MarketStatus,

    // Forex (currency)
    FxIntraday,
    FxDaily,
    FxWeekly,
    FxMonthly,
    CurrencyExchangeRate,
}

impl EndpointName {
    /// Returns the exact `function` parameter string expected by the API.
    ///
    /// This relies on `strum::Display` configured with `SCREAMING_SNAKE_CASE`.
    #[must_use]
    pub fn function_name(&self) -> &str {
        self.into()
    }

    /// Determines if this endpoint is market-wide or ticker-specific.
    #[must_use]
    pub const fn is_market_wide(&self) -> bool {
        matches!(self, Self::MarketStatus)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_serialization() {
        assert_eq!(EndpointName::TimeSeriesDaily.to_string(), "TIME_SERIES_DAILY");
        assert_eq!(EndpointName::GlobalQuote.to_string(), "GLOBAL_QUOTE");
    }

    #[test]
    fn test_deserialization() {
        assert_eq!(
            EndpointName::from_str("TIME_SERIES_INTRADAY").unwrap(),
            EndpointName::TimeSeriesIntraday
        );
    }
}
