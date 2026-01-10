use chrono::Local;

/// Generates a timestamp string in `YYYYMMDD_HHMMSS` format for use in filenames
#[must_use]
pub fn generate_timestamp() -> String {
    Local::now().format("%Y%m%d_%H%M%S").to_string()
}

/// Formats a duration in seconds into a human-readable string (e.g., "5m 30s")
#[must_use]
pub fn format_duration(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    if hours > 0 {
        format!("{hours}h {minutes}m {secs}s")
    } else if minutes > 0 {
        format!("{minutes}m {secs}s")
    } else {
        format!("{secs}s")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_timestamp_format() {
        let ts = generate_timestamp();
        // Should be 15 characters: YYYYMMDD_HHMMSS
        assert_eq!(ts.len(), 15);
        assert!(ts.contains('_'));
        // Should be all digits except the underscore
        assert!(ts.chars().filter(|c| *c != '_').all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_format_duration_seconds_only() {
        assert_eq!(format_duration(0), "0s");
        assert_eq!(format_duration(45), "45s");
    }

    #[test]
    fn test_format_duration_with_minutes() {
        assert_eq!(format_duration(60), "1m 0s");
        assert_eq!(format_duration(90), "1m 30s");
        assert_eq!(format_duration(330), "5m 30s");
    }

    #[test]
    fn test_format_duration_with_hours() {
        assert_eq!(format_duration(3600), "1h 0m 0s");
        assert_eq!(format_duration(3661), "1h 1m 1s");
        assert_eq!(format_duration(7385), "2h 3m 5s");
    }
}
