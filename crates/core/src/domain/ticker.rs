use crate::error::{ExplorerError, Result};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a validated Alpha Vantage ticker symbol.
///
/// Rules:
/// - Length: 1 to 5 characters
/// - Characters: Uppercase A-Z, 0-9
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct TickerSymbol(String);

impl TickerSymbol {
    /// Creates a new `TickerSymbol` if valid.
    ///
    /// # Errors
    /// Returns `ExplorerError::Validation` if:
    /// - Length is invalid (must be 1-5)
    /// - Contains characters other than A-Z, 0-9
    pub fn new(symbol: impl Into<String>) -> Result<Self> {
        let s = symbol.into();
        Self::validate(&s)?;
        Ok(Self(s))
    }

    /// Returns the underlying string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn validate(s: &str) -> Result<()> {
        if s.is_empty() {
            return Err(ExplorerError::Validation("Ticker cannot be empty".into()));
        }
        if s.len() > 5 {
            return Err(ExplorerError::Validation("Ticker too long (max 5 chars)".into()));
        }
        if !s.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit()) {
            return Err(ExplorerError::Validation(
                "Ticker must be uppercase alphanumeric".into(),
            ));
        }
        Ok(())
    }
}

impl fmt::Display for TickerSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for TickerSymbol {
    type Error = ExplorerError;

    fn try_from(s: String) -> Result<Self> {
        Self::new(s)
    }
}

impl From<TickerSymbol> for String {
    fn from(ticker: TickerSymbol) -> Self {
        ticker.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_valid_tickers() {
        assert!(TickerSymbol::new("IBM").is_ok());
        assert!(TickerSymbol::new("A").is_ok());
        assert!(TickerSymbol::new("123").is_ok());
        assert!(TickerSymbol::new("GOOGL").is_ok()); // 5 chars
    }

    #[test]
    fn test_invalid_tickers() {
        assert!(TickerSymbol::new("").is_err());
        assert!(TickerSymbol::new("GOOGLE").is_err()); // 6 chars
        assert!(TickerSymbol::new("ibm").is_err()); // lowercase
        assert!(TickerSymbol::new("IB M").is_err()); // space
        assert!(TickerSymbol::new("IB$").is_err()); // special char
    }

    proptest! {
        #[test]
        fn doesnt_panic(s in "\\PC*") {
            let _ = TickerSymbol::new(s);
        }
    }
}
