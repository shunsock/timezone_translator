use domain::{AmbiguousTimeStrategyParseError, ConversionTimeParseError, TimezoneParseError};

/// Aggregates the domain parse errors that user input can produce.
///
/// Each variant is transparent: the user-facing message lives with
/// the value object that failed to parse.
#[derive(thiserror::Error, Debug, PartialEq)]
pub(crate) enum ValidationError {
    #[error(transparent)]
    Time(#[from] ConversionTimeParseError),

    #[error(transparent)]
    Timezone(#[from] TimezoneParseError),

    #[error(transparent)]
    AmbiguousTimeStrategy(#[from] AmbiguousTimeStrategyParseError),
}
