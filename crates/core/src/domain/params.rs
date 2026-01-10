use std::fmt;
use std::str::FromStr;

/// Represents a fiscal quarter
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuarterParam {
    Q1,
    Q2,
    Q3,
    Q4,
}

impl FromStr for QuarterParam {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "Q1" => Ok(Self::Q1),
            "Q2" => Ok(Self::Q2),
            "Q3" => Ok(Self::Q3),
            "Q4" => Ok(Self::Q4),
            _ => Err(format!("Invalid quarter '{s}'. Must be Q1, Q2, Q3, or Q4")),
        }
    }
}

impl fmt::Display for QuarterParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Q1 => write!(f, "Q1"),
            Self::Q2 => write!(f, "Q2"),
            Self::Q3 => write!(f, "Q3"),
            Self::Q4 => write!(f, "Q4"),
        }
    }
}

/// Represents a time horizon for calendar endpoints
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HorizonParam {
    ThreeMonth,
    SixMonth,
    TwelveMonth,
}

impl FromStr for HorizonParam {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "3month" | "3m" => Ok(Self::ThreeMonth),
            "6month" | "6m" => Ok(Self::SixMonth),
            "12month" | "12m" => Ok(Self::TwelveMonth),
            _ => Err(format!(
                "Invalid horizon '{s}'. Must be 3month, 6month, or 12month"
            )),
        }
    }
}

impl fmt::Display for HorizonParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ThreeMonth => write!(f, "3month"),
            Self::SixMonth => write!(f, "6month"),
            Self::TwelveMonth => write!(f, "12month"),
        }
    }
}

/// Validates a year parameter
///
/// # Errors
/// Returns error if year is not between 1900 and 2100
pub fn validate_year(year: u16) -> Result<u16, String> {
    if (1900..=2100).contains(&year) {
        Ok(year)
    } else {
        Err(format!(
            "Invalid year {year}. Must be between 1900 and 2100"
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quarter_parsing() {
        assert!(matches!(
            QuarterParam::from_str("Q1"),
            Ok(QuarterParam::Q1)
        ));
        assert!(matches!(
            QuarterParam::from_str("q2"),
            Ok(QuarterParam::Q2)
        ));
        assert!(QuarterParam::from_str("Q5").is_err());
        assert!(QuarterParam::from_str("invalid").is_err());
    }

    #[test]
    fn test_quarter_display() {
        assert_eq!(QuarterParam::Q1.to_string(), "Q1");
        assert_eq!(QuarterParam::Q4.to_string(), "Q4");
    }

    #[test]
    fn test_horizon_parsing() {
        assert!(matches!(
            HorizonParam::from_str("3month"),
            Ok(HorizonParam::ThreeMonth)
        ));
        assert!(matches!(
            HorizonParam::from_str("3m"),
            Ok(HorizonParam::ThreeMonth)
        ));
        assert!(HorizonParam::from_str("invalid").is_err());
    }

    #[test]
    fn test_year_validation() {
        assert!(matches!(validate_year(2024), Ok(2024)));
        assert!(validate_year(1899).is_err());
        assert!(validate_year(2101).is_err());
    }
}
