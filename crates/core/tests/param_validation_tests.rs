#![allow(clippy::unwrap_used)]
//! Parameter validation tests

use alphavantage_core::domain::{validate_year, HorizonParam, QuarterParam};
use std::str::FromStr;

// Quarter validation tests

#[test]
fn test_valid_quarters() {
    assert!(matches!(QuarterParam::from_str("Q1"), Ok(QuarterParam::Q1)));
    assert!(matches!(QuarterParam::from_str("Q2"), Ok(QuarterParam::Q2)));
    assert!(matches!(QuarterParam::from_str("Q3"), Ok(QuarterParam::Q3)));
    assert!(matches!(QuarterParam::from_str("Q4"), Ok(QuarterParam::Q4)));
}

#[test]
fn test_quarter_case_insensitive() {
    assert!(matches!(QuarterParam::from_str("q1"), Ok(QuarterParam::Q1)));
    assert!(matches!(QuarterParam::from_str("q2"), Ok(QuarterParam::Q2)));
    assert!(matches!(QuarterParam::from_str("q3"), Ok(QuarterParam::Q3)));
    assert!(matches!(QuarterParam::from_str("q4"), Ok(QuarterParam::Q4)));
    assert!(matches!(QuarterParam::from_str("Q1"), Ok(QuarterParam::Q1)));
}

#[test]
fn test_invalid_quarters() {
    assert!(QuarterParam::from_str("Q5").is_err());
    assert!(QuarterParam::from_str("Q0").is_err());
    assert!(QuarterParam::from_str("INVALID").is_err());
    assert!(QuarterParam::from_str("1").is_err());
    assert!(QuarterParam::from_str("").is_err());
    assert!(QuarterParam::from_str("Quarter1").is_err());
}

#[test]
fn test_quarter_error_messages() {
    let result = QuarterParam::from_str("Q5");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Q1, Q2, Q3, or Q4"));
    assert!(err.contains("Q5"));
}

#[test]
fn test_quarter_display() {
    assert_eq!(QuarterParam::Q1.to_string(), "Q1");
    assert_eq!(QuarterParam::Q2.to_string(), "Q2");
    assert_eq!(QuarterParam::Q3.to_string(), "Q3");
    assert_eq!(QuarterParam::Q4.to_string(), "Q4");
}

#[test]
fn test_quarter_round_trip() {
    // Parse and display should be reversible
    let q1 = QuarterParam::from_str("Q1").unwrap();
    assert_eq!(q1.to_string(), "Q1");

    let q2 = QuarterParam::from_str("q2").unwrap();
    assert_eq!(q2.to_string(), "Q2");
}

// Horizon validation tests

#[test]
fn test_valid_horizons() {
    assert!(matches!(
        HorizonParam::from_str("3month"),
        Ok(HorizonParam::ThreeMonth)
    ));
    assert!(matches!(
        HorizonParam::from_str("6month"),
        Ok(HorizonParam::SixMonth)
    ));
    assert!(matches!(
        HorizonParam::from_str("12month"),
        Ok(HorizonParam::TwelveMonth)
    ));
}

#[test]
fn test_horizon_short_format() {
    assert!(matches!(
        HorizonParam::from_str("3m"),
        Ok(HorizonParam::ThreeMonth)
    ));
    assert!(matches!(
        HorizonParam::from_str("6m"),
        Ok(HorizonParam::SixMonth)
    ));
    assert!(matches!(
        HorizonParam::from_str("12m"),
        Ok(HorizonParam::TwelveMonth)
    ));
}

#[test]
fn test_invalid_horizons() {
    assert!(HorizonParam::from_str("1month").is_err());
    assert!(HorizonParam::from_str("24month").is_err());
    assert!(HorizonParam::from_str("invalid").is_err());
    assert!(HorizonParam::from_str("").is_err());
    assert!(HorizonParam::from_str("3").is_err());
}

#[test]
fn test_horizon_error_messages() {
    let result = HorizonParam::from_str("invalid");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("3month"));
    assert!(err.contains("6month"));
    assert!(err.contains("12month"));
}

#[test]
fn test_horizon_display() {
    assert_eq!(HorizonParam::ThreeMonth.to_string(), "3month");
    assert_eq!(HorizonParam::SixMonth.to_string(), "6month");
    assert_eq!(HorizonParam::TwelveMonth.to_string(), "12month");
}

#[test]
fn test_horizon_round_trip() {
    let h3 = HorizonParam::from_str("3month").unwrap();
    assert_eq!(h3.to_string(), "3month");

    let h6 = HorizonParam::from_str("6m").unwrap();
    assert_eq!(h6.to_string(), "6month");
}

// Year validation tests

#[test]
fn test_valid_years() {
    assert!(matches!(validate_year(1900), Ok(1900)));
    assert!(matches!(validate_year(2000), Ok(2000)));
    assert!(matches!(validate_year(2024), Ok(2024)));
    assert!(matches!(validate_year(2100), Ok(2100)));
}

#[test]
fn test_year_bounds() {
    // Boundary values
    assert!(validate_year(1900).is_ok());
    assert!(validate_year(2100).is_ok());

    // Just outside boundaries
    assert!(validate_year(1899).is_err());
    assert!(validate_year(2101).is_err());
}

#[test]
fn test_invalid_years() {
    assert!(validate_year(1899).is_err());
    assert!(validate_year(2101).is_err());
    assert!(validate_year(1000).is_err());
    assert!(validate_year(3000).is_err());
    assert!(validate_year(0).is_err());
}

#[test]
fn test_year_error_messages() {
    let result = validate_year(1899);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("1900"));
    assert!(err.contains("2100"));
    assert!(err.contains("1899"));
}

#[test]
fn test_current_years() {
    // Test years around current era
    assert!(validate_year(2020).is_ok());
    assert!(validate_year(2021).is_ok());
    assert!(validate_year(2022).is_ok());
    assert!(validate_year(2023).is_ok());
    assert!(validate_year(2024).is_ok());
    assert!(validate_year(2025).is_ok());
}

// Combined validation tests

#[test]
fn test_quarter_equality() {
    let q1_a = QuarterParam::from_str("Q1").unwrap();
    let q1_b = QuarterParam::from_str("q1").unwrap();
    assert_eq!(q1_a, q1_b);

    let q2 = QuarterParam::from_str("Q2").unwrap();
    assert_ne!(q1_a, q2);
}

#[test]
fn test_horizon_equality() {
    let h3_a = HorizonParam::from_str("3month").unwrap();
    let h3_b = HorizonParam::from_str("3m").unwrap();
    assert_eq!(h3_a, h3_b);

    let h6 = HorizonParam::from_str("6month").unwrap();
    assert_ne!(h3_a, h6);
}

#[test]
fn test_all_quarters_unique() {
    let q1 = QuarterParam::Q1;
    let q2 = QuarterParam::Q2;
    let q3 = QuarterParam::Q3;
    let q4 = QuarterParam::Q4;

    assert_ne!(q1, q2);
    assert_ne!(q1, q3);
    assert_ne!(q1, q4);
    assert_ne!(q2, q3);
    assert_ne!(q2, q4);
    assert_ne!(q3, q4);
}

#[test]
fn test_all_horizons_unique() {
    let h3 = HorizonParam::ThreeMonth;
    let h6 = HorizonParam::SixMonth;
    let h12 = HorizonParam::TwelveMonth;

    assert_ne!(h3, h6);
    assert_ne!(h3, h12);
    assert_ne!(h6, h12);
}
