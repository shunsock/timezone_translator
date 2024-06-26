use chrono_tz::Tz;
use super::validation_error::ValidationError;

pub(super) fn validate_string_for_timezone(tz: &str) -> Result<Tz, ValidationError> {
    tz.parse::<Tz>().map_err(
        |_| ValidationError::InvalidTimezone(tz.to_string())
    )
}
