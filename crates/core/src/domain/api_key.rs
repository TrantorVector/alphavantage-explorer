use crate::error::{ExplorerError, Result};
use secrecy::{ExposeSecret, Secret};
use std::fmt;
use std::str::FromStr;

/// Represents an Alpha Vantage API key.
///
/// This type wraps `secrecy::Secret` to prevent accidental logging of sensitive keys.
///
/// # Security
/// - Use `ExposeSecret` trait carefully to access the raw value.
/// - `Debug` implementation is redacted.
#[derive(Clone)]
pub struct ApiKey(Secret<String>);

impl ApiKey {
    /// Creates a new `ApiKey` from a string.
    pub fn new(key: impl Into<String>) -> Self {
        Self(Secret::new(key.into()))
    }

    /// Access the secret value
    #[must_use]
    pub fn secret(&self) -> &str {
        self.0.expose_secret()
    }
}

impl fmt::Debug for ApiKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[REDACTED]")
    }
}

impl FromStr for ApiKey {
    type Err = ExplorerError;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self::new(s))
    }
}

impl From<String> for ApiKey {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_redaction() {
        let key = ApiKey::new("my_secret_key");
        assert_eq!(format!("{key:?}"), "[REDACTED]");
    }

    #[test]
    fn test_secret_usage() {
        let key = ApiKey::new("secret");
        assert_eq!(key.secret(), "secret");
    }
}
