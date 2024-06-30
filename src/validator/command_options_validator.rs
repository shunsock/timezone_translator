use super::native_datetime_validator::validate_string_for_native_datetime;
use super::timezone_validator::validate_string_for_timezone;
use super::validated_command_options::ValidatedCommandOptions;
use crate::validator::ambiguous_time_strategy::AmbiguousTimeStrategy;
use crate::validator::ambiguous_time_strategy_validator::validate_string_for_ambiguous_time_strategy;
use crate::validator::validation_error::ValidationError;
use chrono::NaiveDateTime;
use chrono_tz::Tz;
use clap::ArgMatches;

pub(crate) fn validate_command_options(
    arg: &ArgMatches,
) -> Result<ValidatedCommandOptions, ValidationError> {
    // arg.get_one::<String>("time") returns Option<&String>, but clap validates the required option
    // thus, we can safely unwrap the value
    let time_str: &String = arg.get_one::<String>("time").unwrap();
    let time_validated: NaiveDateTime = validate_string_for_native_datetime(&time_str)?;

    // arg.get_one::<String>("from_timezone") returns Option<&String>, but clap validates the required option
    // thus, we can safely unwrap the value
    let from_tz_str: &String = arg.get_one::<String>("from_timezone").unwrap();
    let from_tz_validated: Tz = validate_string_for_timezone(&from_tz_str)?;

    // arg.get_one::<String>("to_timezone") returns Option<&String>, but clap validates the required option
    // thus, we can safely unwrap the value
    let to_tz_str: &String = arg.get_one::<String>("to_timezone").unwrap();
    let to_tz_validated: Tz = validate_string_for_timezone(&to_tz_str)?;

    // arg.get_one::<String>("ambiguous_time_strategy") returns Option<&String>, but clap set the default value
    // thus, we can safely unwrap the value
    let ambiguous_time_strategy_str: &String =
        arg.get_one::<String>("ambiguous_time_strategy").unwrap();
    let ambiguous_time_strategy_validated: AmbiguousTimeStrategy =
        validate_string_for_ambiguous_time_strategy(&ambiguous_time_strategy_str)?;

    // Return validated options
    Ok(ValidatedCommandOptions::new(
        time_validated,
        from_tz_validated,
        to_tz_validated,
        ambiguous_time_strategy_validated,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validator::validation_error::ValidationError;
    use chrono::NaiveDateTime;
    use chrono_tz::Tz;
    use clap::{Arg, ArgMatches, Command};

    // Helper function to create ArgMatches for testing
    fn create_arg_matches(time: &str, from_tz: &str, to_tz: &str) -> ArgMatches {
        Command::new("test")
            .arg(Arg::new("time").required(true))
            .arg(Arg::new("from_timezone").required(true))
            .arg(Arg::new("to_timezone").required(true))
            .arg(Arg::new("ambiguous_time_strategy").default_value("earliest"))
            .get_matches_from(vec!["test", time, from_tz, to_tz])
    }

    /// Test that valid command options are valid
    /// expected: `Ok(ValidatedCommandOptions)`
    #[test]
    fn test_validate_command_options_valid() {
        let matches: ArgMatches =
            create_arg_matches("2024-06-27 12:34:56", "America/New_York", "Europe/London");

        // Confirm that the validation passes
        let result: Result<ValidatedCommandOptions, ValidationError> =
            validate_command_options(&matches);
        assert!(result.is_ok());

        // Confirm that the validated options are as expected
        let validated_options: ValidatedCommandOptions = result.unwrap();
        assert_eq!(
            validated_options.time(),
            NaiveDateTime::parse_from_str("2024-06-27 12:34:56", "%Y-%m-%d %H:%M:%S").unwrap()
        );
        assert_eq!(
            validated_options.from_tz(),
            "America/New_York".parse::<Tz>().unwrap()
        );
        assert_eq!(
            validated_options.to_tz(),
            "Europe/London".parse::<Tz>().unwrap()
        );
    }

    /// Test that an invalid time is invalid
    /// expected: `Err(ValidationError::InvalidTime)`
    #[test]
    fn test_validate_command_options_invalid_time() {
        let matches: ArgMatches =
            create_arg_matches("invalid-time", "America/New_York", "Europe/London");
        let result: Result<ValidatedCommandOptions, ValidationError> =
            validate_command_options(&matches);
        assert!(result.is_err());
    }

    /// Test that an invalid from timezone is invalid
    /// expected: `Err(ValidationError::InvalidTimezone)`
    #[test]
    fn test_validate_command_options_invalid_from_timezone() {
        let matches: ArgMatches =
            create_arg_matches("2024-06-27 12:34:56", "Invalid/Timezone", "Europe/London");
        let result: Result<ValidatedCommandOptions, ValidationError> =
            validate_command_options(&matches);
        assert!(result.is_err());
    }

    /// Test that an invalid to timezone is invalid
    /// expected: `Err(ValidationError::InvalidTimezone)`
    #[test]
    fn test_validate_command_options_invalid_to_timezone() {
        let matches: ArgMatches = create_arg_matches(
            "2024-06-27 12:34:56",
            "America/New_York",
            "Invalid/Timezone",
        );
        let result: Result<ValidatedCommandOptions, ValidationError> =
            validate_command_options(&matches);
        assert!(result.is_err());
    }
}
