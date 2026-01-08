use alphavantage_core::ExplorerError;
use rand::Rng;
use std::time::Duration;
use tokio::time::sleep;
use tracing::warn;

const MAX_RETRIES: u32 = 2;
const BASE_DELAY_MS: u64 = 500;

/// Executes an async operation with retries.
///
/// # Errors
/// Returns the last error if all retries fail, or if a non-retriable error occurs.
pub async fn execute_with_retry<F, Fut, T>(operation: F) -> Result<T, ExplorerError>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T, ExplorerError>>,
{
    let mut attempt = 0;

    loop {
        match operation().await {
            Ok(val) => return Ok(val),
            Err(e) => {
                if attempt >= MAX_RETRIES {
                    return Err(e);
                }

                if is_retriable(&e) {
                    attempt += 1;
                    let delay = calculate_backoff(attempt);
                    warn!(
                        "Attempt {} failed: {}. Retrying in {}ms...",
                        attempt,
                        e,
                        delay.as_millis()
                    );
                    sleep(delay).await;
                } else {
                    return Err(e);
                }
            }
        }
    }
}

const fn is_retriable(error: &ExplorerError) -> bool {
    match error {
        ExplorerError::RateLimited | ExplorerError::Network(_) | ExplorerError::Io(_) => true,
        ExplorerError::HttpStatus(status) => {
            // Retry 429 and 5xx
            *status == 429 || *status >= 500
        }
        // Don't retry validation, parsing, etc.
        _ => false,
    }
}

fn calculate_backoff(attempt: u32) -> Duration {
    let base = BASE_DELAY_MS * 2u64.pow(attempt - 1);
    let jitter = rand::thread_rng().gen_range(0..=100);
    Duration::from_millis(base + jitter)
}
