use crate::validator::ambiguous_time_strategy::AmbiguousTimeStrategy;
use crate::validator::validation_error::ValidationError;

pub(super) fn validate_string_for_ambiguous_time_strategy(
    ambiguous_time_strategy_str: &str,
) -> Result<AmbiguousTimeStrategy, ValidationError> {
    match ambiguous_time_strategy_str {
        "earliest" => Ok(AmbiguousTimeStrategy::Earliest),
        "latest" => Ok(AmbiguousTimeStrategy::Latest),
        _ => Err(ValidationError::InvalidAmbiguousTimeStrategy {
            ambiguous_time_strategy: ambiguous_time_strategy_str.to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that the function `validate_string_for_ambiguous_time_strategy` returns the correct
    /// expect: `AmbiguousTimeStrategy`
    #[test]
    fn test_validate_string_for_ambiguous_time_strategy() {
        assert_eq!(
            validate_string_for_ambiguous_time_strategy("earliest").unwrap(),
            AmbiguousTimeStrategy::Earliest
        );
        assert_eq!(
            validate_string_for_ambiguous_time_strategy("latest").unwrap(),
            AmbiguousTimeStrategy::Latest
        );
    }

    /// Test that the function `validate_string_for_ambiguous_time_strategy` returns an error
    /// when given an invalid string.
    /// The error should be of type `ValidationError::InvalidAmbiguousTimeStrategy`.
    /// expect: `ValidationError::InvalidAmbiguousTimeStrategy`
    #[test]
    fn test_invalid_ambiguous_time_strategy() {
        let res: Result<AmbiguousTimeStrategy, ValidationError> = validate_string_for_ambiguous_time_strategy(
            "invalid"
        );

        // check status is error
        assert!(res.is_err());

        // confirm error type
        assert_eq!(
            res.unwrap_err(),
            ValidationError::InvalidAmbiguousTimeStrategy {
                ambiguous_time_strategy: "invalid".to_string()
            }
        );
    }
}