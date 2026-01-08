use alphavantage_core::ExplorerError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tokio::fs;
use tracing::{info, warn};

const DAILY_LIMIT: u32 = 25;
const STATE_FILENAME: &str = ".alphavantage-explorer-tokens.json";

#[derive(Debug, Serialize, Deserialize)]
struct TokenState {
    tokens_remaining: u32,
    last_reset: DateTime<Utc>,
}

impl Default for TokenState {
    fn default() -> Self {
        Self {
            tokens_remaining: DAILY_LIMIT,
            last_reset: Utc::now(),
        }
    }
}

pub struct RateLimiter {
    state: Mutex<TokenState>,
    state_path: PathBuf,
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

impl RateLimiter {
    #[must_use]
    pub fn new() -> Self {
        Self::with_path(Self::resolve_state_path())
    }

    #[must_use]
    pub fn with_path(state_path: PathBuf) -> Self {
        // We load synchronously on creation because new() is not async.
        // This is acceptable for initialization or we should accept a constructor that returns Future.
        // For CLI tools, sync init is fine.
        let state = Self::load_state_sync(&state_path).unwrap_or_default();

        // Check reset logic requires mutation.
        let limiter = Self {
            state: Mutex::new(state),
            state_path,
        };
        limiter.check_reset();
        limiter
    }

    fn resolve_state_path() -> PathBuf {
        if let Some(base_dirs) = directories::BaseDirs::new() {
            let home = base_dirs.home_dir();
            let path = home.join(STATE_FILENAME);
            if (path.exists()
                && !path
                    .metadata()
                    .map(|m| m.permissions().readonly())
                    .unwrap_or(false))
                || (!path.exists()
                    && home
                        .metadata()
                        .map(|m| !m.permissions().readonly())
                        .unwrap_or(false))
            {
                return path;
            }
        }
        PathBuf::from(format!("./{STATE_FILENAME}"))
    }

    fn load_state_sync(path: &Path) -> Option<TokenState> {
        if path.exists() {
            if let Ok(file) = std::fs::File::open(path) {
                let reader = std::io::BufReader::new(file);
                return serde_json::from_reader(reader).ok();
            }
        }
        None
    }

    async fn save_state_async(&self, state: &TokenState) {
        if let Ok(json) = serde_json::to_string(state) {
            let _ = fs::write(&self.state_path, json).await;
        }
    }

    fn check_reset(&self) {
        if let Ok(mut state) = self.state.lock() {
            let now = Utc::now();
            let last = state.last_reset;
            let today = now.date_naive();
            let last_day = last.date_naive();

            if today > last_day {
                info!("Resetting rate limit tokens for new day");
                state.tokens_remaining = DAILY_LIMIT;
                state.last_reset = now;
            }
        }
    }

    /// Blocks until a token is available or returns error if limit exhausted.
    ///
    /// # Errors
    /// Returns `ExplorerError::RateLimited` if the daily quota is exhausted.
    /// Returns `ExplorerError::Unknown` if the lock is poisoned.
    pub async fn wait(&self) -> Result<(), ExplorerError> {
        // We need to hold the lock briefly to check/update state
        // Then release it to do async I/O

        let (should_save, new_state_copy) = {
            let mut state = self
                .state
                .lock()
                .map_err(|_| ExplorerError::Unknown("Lock poisoned".into()))?;

            let now = Utc::now();
            if now.date_naive() > state.last_reset.date_naive() {
                info!("Resetting rate limit tokens (midnight UTC passed)");
                state.tokens_remaining = DAILY_LIMIT;
                state.last_reset = now;
            }

            if state.tokens_remaining > 0 {
                state.tokens_remaining -= 1;
                info!(
                    "Rate limit token consumed. Remaining: {}",
                    state.tokens_remaining
                );

                (
                    true,
                    TokenState {
                        tokens_remaining: state.tokens_remaining,
                        last_reset: state.last_reset,
                    },
                )
            } else {
                warn!("Rate limit exceeded (0 tokens remaining)");
                return Err(ExplorerError::RateLimited);
            }
        };

        if should_save {
            self.save_state_async(&new_state_copy).await;
        }
        Ok(())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use std::env;

    fn get_temp_path() -> PathBuf {
        let mut path = env::temp_dir();
        let filename = format!("test_tokens_{}.json", rand::random::<u32>());
        path.push(filename);
        path
    }

    #[tokio::test]
    async fn test_rate_limiter_enforcement() {
        let path = get_temp_path();
        let limiter = RateLimiter::with_path(path.clone());

        // Consume all 25 tokens
        for i in 0..25 {
            assert!(limiter.wait().await.is_ok(), "Call {i} failed");
        }

        // 26th call should fail
        assert!(matches!(
            limiter.wait().await,
            Err(ExplorerError::RateLimited)
        ));

        // Clean up
        let _ = std::fs::remove_file(path);
    }

    #[tokio::test]
    async fn test_persistence() {
        let path = get_temp_path();

        {
            let limiter = RateLimiter::with_path(path.clone());
            // Consume 5
            for _ in 0..5 {
                limiter.wait().await.unwrap();
            }
        } // Drop limiter

        // Reload
        let limiter = RateLimiter::with_path(path.clone());
        // Should have 20 left
        // We can't inspect state directly as fields are private
        // But we can consume 20 more, then fail on 21st
        for i in 0..20 {
            assert!(limiter.wait().await.is_ok(), "Reloaded call {i} failed");
        }
        assert!(matches!(
            limiter.wait().await,
            Err(ExplorerError::RateLimited)
        ));

        // Clean up
        let _ = std::fs::remove_file(path);
    }
}
