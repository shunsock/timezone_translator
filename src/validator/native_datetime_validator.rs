use chrono::NaiveDateTime;
use super::validation_error::ValidationError;

pub(super) fn validate_string_for_native_datetime(time: &str) -> Result<NaiveDateTime, ValidationError> {
    NaiveDateTime::parse_from_str(
        time,
        "%Y-%m-%d %H:%M:%S"
    ).map_err(
        |_| ValidationError::InvalidTimeFormat(time.to_string())
    )
}