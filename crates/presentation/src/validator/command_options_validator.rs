use super::validation_error::ValidationError;
use clap::ArgMatches;
use domain::{
    AmbiguousTimeStrategy, ConversionTime, SourceTimezone, TargetTimezone, TranslationRequest,
};

/// Parses raw CLI strings into a validated `TranslationRequest`.
///
/// The `unwrap()` calls are safe: clap guarantees `time` (required)
/// and the other options (defaulted) are always present.
pub(crate) fn validate_command_options(
    arg: &ArgMatches,
) -> Result<TranslationRequest, ValidationError> {
    let time: ConversionTime = arg.get_one::<String>("time").unwrap().parse()?;
    let source: SourceTimezone = arg.get_one::<String>("from_timezone").unwrap().parse()?;
    let target: TargetTimezone = arg.get_one::<String>("to_timezone").unwrap().parse()?;
    let strategy: AmbiguousTimeStrategy = arg
        .get_one::<String>("ambiguous_time_strategy")
        .unwrap()
        .parse()?;

    Ok(TranslationRequest::new(time, source, target, strategy))
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::{Arg, Command};

    /// Fixture: builds `ArgMatches` the same shape as the real CLI.
    fn arg_matches(time: &str, from_tz: &str, to_tz: &str) -> ArgMatches {
        Command::new("test")
            .arg(Arg::new("time").required(true))
            .arg(Arg::new("from_timezone").required(true))
            .arg(Arg::new("to_timezone").required(true))
            .arg(Arg::new("ambiguous_time_strategy").default_value("earliest"))
            .get_matches_from(vec!["test", time, from_tz, to_tz])
    }

    #[test]
    fn builds_translation_request_from_valid_options() {
        // Arrange
        let matches = arg_matches("2024-06-27 12:34:56", "America/New_York", "Europe/London");

        // Act
        let request = validate_command_options(&matches).unwrap();

        // Assert
        let expected = TranslationRequest::new(
            "2024-06-27 12:34:56".parse().unwrap(),
            "America/New_York".parse().unwrap(),
            "Europe/London".parse().unwrap(),
            AmbiguousTimeStrategy::Earliest,
        );
        assert_eq!(request, expected);
    }

    #[test]
    fn rejects_invalid_time() {
        // Arrange
        let matches = arg_matches("invalid-time", "America/New_York", "Europe/London");

        // Act
        let result = validate_command_options(&matches);

        // Assert
        assert!(matches!(result, Err(ValidationError::Time(_))));
    }

    #[test]
    fn rejects_invalid_source_timezone() {
        // Arrange
        let matches = arg_matches("2024-06-27 12:34:56", "Invalid/Timezone", "Europe/London");

        // Act
        let result = validate_command_options(&matches);

        // Assert
        assert!(matches!(result, Err(ValidationError::Timezone(_))));
    }

    #[test]
    fn rejects_invalid_target_timezone() {
        // Arrange
        let matches = arg_matches(
            "2024-06-27 12:34:56",
            "America/New_York",
            "Invalid/Timezone",
        );

        // Act
        let result = validate_command_options(&matches);

        // Assert
        assert!(matches!(result, Err(ValidationError::Timezone(_))));
    }
}
