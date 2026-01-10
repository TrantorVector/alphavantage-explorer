use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

/// Configuration loaded from alphavantage.toml file
#[derive(Debug, Deserialize, Clone)]
pub struct ApiConfig {
    pub api: ApiSection,
    pub rate_limit: RateLimitSection,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ApiSection {
    pub api_key: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RateLimitSection {
    pub daily_limit: u32,
}

impl ApiConfig {
    /// Load configuration from alphavantage.toml in the project root
    pub fn load() -> Result<Self> {
        let config_path = Self::find_config_file()?;
        Self::load_from_path(&config_path)
    }

    /// Load configuration from a specific path
    pub fn load_from_path(path: &Path) -> Result<Self> {
        let contents = fs::read_to_string(path).with_context(|| {
            format!(
                "Failed to read config file at {}. \
                 Did you create alphavantage.toml from alphavantage.toml.template?",
                path.display()
            )
        })?;

        let config: Self = toml::from_str(&contents).with_context(|| {
            format!(
                "Failed to parse config file at {}. \
                 Ensure it follows the format in alphavantage.toml.template",
                path.display()
            )
        })?;

        // Validate the config
        if config.api.api_key == "YOUR_API_KEY_HERE" || config.api.api_key.is_empty() {
            anyhow::bail!(
                "API key not configured. Please edit {} and set your actual API key.",
                path.display()
            );
        }

        if config.rate_limit.daily_limit == 0 {
            anyhow::bail!(
                "Invalid daily_limit in {}. Must be greater than 0.",
                path.display()
            );
        }

        Ok(config)
    }

    /// Find the config file, starting from current directory and moving up
    fn find_config_file() -> Result<PathBuf> {
        let config_name = "alphavantage.toml";

        // Try current directory first
        let current_dir = std::env::current_dir()?;
        let config_path = current_dir.join(config_name);
        if config_path.exists() {
            return Ok(config_path);
        }

        // Try parent directories (useful when running from subdirectories)
        let mut dir = current_dir.as_path();
        while let Some(parent) = dir.parent() {
            let config_path = parent.join(config_name);
            if config_path.exists() {
                return Ok(config_path);
            }
            dir = parent;
        }

        anyhow::bail!(
            "Could not find {config_name} in current directory or any parent directory. \
             Please create it from alphavantage.toml.template"
        )
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_valid_config() {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        writeln!(
            temp_file,
            r#"
[api]
api_key = "TEST_KEY_12345"

[rate_limit]
daily_limit = 50
"#
        )
        .expect("Failed to write to temp file");

        let config = ApiConfig::load_from_path(temp_file.path()).expect("Failed to load config");
        assert_eq!(config.api.api_key, "TEST_KEY_12345");
        assert_eq!(config.rate_limit.daily_limit, 50);
    }

    #[test]
    fn test_missing_api_key() {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        writeln!(
            temp_file,
            r#"
[api]
api_key = "YOUR_API_KEY_HERE"

[rate_limit]
daily_limit = 25
"#
        )
        .expect("Failed to write to temp file");

        let result = ApiConfig::load_from_path(temp_file.path());
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("API key not configured"));
    }

    #[test]
    fn test_zero_daily_limit() {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        writeln!(
            temp_file,
            r#"
[api]
api_key = "TEST_KEY"

[rate_limit]
daily_limit = 0
"#
        )
        .expect("Failed to write to temp file");

        let result = ApiConfig::load_from_path(temp_file.path());
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("daily_limit"));
    }
}
