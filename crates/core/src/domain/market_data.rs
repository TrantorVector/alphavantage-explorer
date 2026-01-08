use serde::{Deserialize, Serialize};

/// High-level result of an API call.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum ApiResponse<T> {
    /// Request succeeded
    Success(T),
    /// API returned an error (200 OK but body contains Error Message)
    Error(ApiError),
}

/// Represents an explicit error returned by the Alpha Vantage API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub message: String,
    pub kind: ErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorKind {
    /// "Invalid API call. Please retry..."
    InvalidInput,
    /// "Thank you for using Alpha Vantage! ..." (API key limit)
    RateLimit,
    /// General catch-all
    Unknown,
}

impl ApiError {
    pub fn new(message: impl Into<String>) -> Self {
        let msg = message.into();
        let kind = if msg.contains("Invalid API call") {
            ErrorKind::InvalidInput
        } else if msg.contains("Thank you") || msg.contains("call volume") {
            ErrorKind::RateLimit
        } else {
            ErrorKind::Unknown
        };
        Self { message: msg, kind }
    }
}
