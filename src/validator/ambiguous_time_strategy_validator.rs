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