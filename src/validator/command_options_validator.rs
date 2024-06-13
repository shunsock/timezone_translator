use chrono::NaiveDateTime;
use chrono_tz::Tz;
use clap::ArgMatches;
use super::native_datetime_validator::validate_string_for_native_datetime;
use super::timezone_validator::validate_string_for_timezone;
use super::validated_command_options::ValidatedCommandOptions;


pub(crate) fn validate_command_options(arg: &ArgMatches) -> Result<ValidatedCommandOptions, String> {
    // Validate `time` option
    let time_str: &String = match arg.get_one::<String>("time") {
        Some(time) => time,
        None => return Err(String::from("Time is required")),
    };

    let time_validated: NaiveDateTime = match validate_string_for_native_datetime(&time_str) {
        Ok(time) => time,
        Err(e) => {
            return Err(String::from(format!("Invalid time {}", e)));
        }
    };

    // Validate `from_timezone` option
    let from_tz_str: &String = match arg.get_one::<String>("from_timezone") {
        Some(tz) => tz,
        None => return Err(String::from("From timezone is required")),
    };

    let from_tz_validated : Tz= match validate_string_for_timezone(&from_tz_str) {
        Ok(from_timezone) => from_timezone,
        Err(e) => return Err(String::from(format!("{}", e))),
    };

    // Validate `to_timezone` option
    let to_tz_str: &String = match arg.get_one::<String>("to_timezone") {
        Some(tz) => tz,
        None => return Err(String::from("To timezone is required")),
    };

    let to_tz_validated: Tz = match validate_string_for_timezone(&to_tz_str) {
        Ok(to_timezone) => to_timezone,
        Err(e) => return Err(String::from(format!("{}", e))),
    };

    // Return validated options
    Ok(
        ValidatedCommandOptions::new(
            time_validated,
            from_tz_validated,
            to_tz_validated,
        )
    )
}
