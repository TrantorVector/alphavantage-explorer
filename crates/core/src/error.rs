use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExplorerError {
    #[error("Network error: {0}")]
    Network(String),

    #[error("HTTP status error: {0}")]
    HttpStatus(u16),

    #[error("Provider error: {0}")]
    ProviderError(String),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Rate limited")]
    RateLimited,

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, ExplorerError>;
