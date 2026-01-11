use alphavantage_core::domain::EndpointName;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Tracks progress of endpoint-fetching
pub struct ProgressReporter {
    total_endpoints: usize,
    completed_endpoints: Arc<Mutex<usize>>,
}

impl ProgressReporter {
    /// Creates a new progress reporter for tracking endpoint fetches
    #[must_use]
    pub fn new(total_endpoints: usize) -> Self {
        Self {
            total_endpoints,
            completed_endpoints: Arc::new(Mutex::new(0)),
        }
    }

    pub fn start_fetch(endpoint: EndpointName, ticker: &str) {
        tracing::info!("Fetching {} for {}...", endpoint, ticker);
    }

    pub async fn finish_fetch(
        &self,
        endpoint: EndpointName,
        ticker: &str,
        success: bool,
        error: Option<&str>,
    ) {
        if success {
            tracing::info!("✓ {} for {} succeeded", endpoint, ticker);
        } else {
            // Use error level for failures so they are visible even in non-verbose modes if needed
            tracing::error!(
                "✗ {} for {} failed: {}",
                endpoint,
                ticker,
                error.unwrap_or("unknown error")
            );
        }

        let mut completed = self.completed_endpoints.lock().await;
        *completed += 1;

        // Basic log of tokens is handled by the client rate limiter logger,
        // but we could duplicate it here if we had access to the limiter state directly.
        // For now, focusing on completion status.
    }

    pub async fn summary(&self) {
        let completed = *self.completed_endpoints.lock().await;
        tracing::info!("Completed {}/{} endpoints", completed, self.total_endpoints);
    }
}
