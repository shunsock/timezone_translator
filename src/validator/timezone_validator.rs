use chrono_tz::Tz;

pub(super) fn validate_string_for_timezone(tz: &str) -> Result<Tz, String> {
    tz.parse::<Tz>().map_err(
        |_| String::from("Validation Error: Invalid timezone")
    )
}
