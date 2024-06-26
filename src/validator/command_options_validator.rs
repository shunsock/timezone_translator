use chrono::NaiveDateTime;
use chrono_tz::Tz;
use clap::ArgMatches;
use crate::validator::validation_error::ValidationError;
use super::native_datetime_validator::validate_string_for_native_datetime;
use super::timezone_validator::validate_string_for_timezone;
use super::validated_command_options::ValidatedCommandOptions;


pub(crate) fn validate_command_options(arg: &ArgMatches) -> Result<ValidatedCommandOptions, ValidationError> {
    // arg.get_one::<String>("time") returns Option<&String>, but clap validates the required option
    // thus, we can safely unwrap the value
    let time_str: &String = arg.get_one::<String>("time").unwrap();
    let time_validated: NaiveDateTime = validate_string_for_native_datetime(&time_str)?;

    // arg.get_one::<String>("from_timezone") returns Option<&String>, but clap validates the required option
    // thus, we can safely unwrap the value
    let from_tz_str: &String = arg.get_one::<String>("from_timezone").unwrap();
    let from_tz_validated : Tz = validate_string_for_timezone(&from_tz_str)?;

    // arg.get_one::<String>("to_timezone") returns Option<&String>, but clap validates the required option
    // thus, we can safely unwrap the value
    let to_tz_str: &String = arg.get_one::<String>("to_timezone").unwrap();
    let to_tz_validated: Tz = validate_string_for_timezone(&to_tz_str)?;

    // Return validated options
    Ok(
        ValidatedCommandOptions::new(
            time_validated,
            from_tz_validated,
            to_tz_validated,
        )
    )
}
