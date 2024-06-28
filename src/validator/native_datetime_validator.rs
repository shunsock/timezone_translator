use chrono::NaiveDateTime;
use crate::validator::regex_matcher::ymd_matcher::ymd_matcher;
use crate::validator::regex_matcher::ymd_t_hms_matcher::ymd_t_hms_matcher;
use super::validation_error::ValidationError;
use super::regex_matcher::ymd_hms_matcher::ymd_hms_matcher;

pub(super) fn validate_string_for_native_datetime(time: &str) -> Result<NaiveDateTime, ValidationError> {
    if ymd_hms_matcher(time) {
        return NaiveDateTime::parse_from_str(
            time,
            "%Y-%m-%d %H:%M:%S"
        ).map_err(
            |_| ValidationError::InvalidTimeFormat(time.to_string())
        );
    }

    if ymd_matcher(time) {
        return NaiveDateTime::parse_from_str(
            format!("{} 00:00:00", time).as_str(),
            "%Y-%m-%d %H:%M:%S"
        ).map_err(
            |_| ValidationError::InvalidTimeFormat(time.to_string())
        );
    }

    if ymd_t_hms_matcher(time) {
        return NaiveDateTime::parse_from_str(
            time,
            "%Y-%m-%dT%H:%M:%S"
        ).map_err(
            |_| ValidationError::InvalidTimeFormat(time.to_string())
        );
    }

    Err(ValidationError::InvalidTimeFormat(time.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;

    /// Test that a valid time string is valid
    /// expected: `Ok(NaiveDateTime)`
    #[test]
    fn test_accept_ym_with_hyphen_hms() {
        // Check that a valid time string pass the validator
        let time_str = "2024-06-27 12:34:56";
        let result: Result<NaiveDateTime, ValidationError> = validate_string_for_native_datetime(time_str);
        assert!(result.is_ok());

        // Check that the result value is the same as the expected
        let expected_datetime = NaiveDateTime::parse_from_str(time_str, "%Y-%m-%d %H:%M:%S").unwrap();
        assert_eq!(result.unwrap(), expected_datetime);
    }

    /// Test that a partial string is valid
    /// expected: `Ok(NaiveDateTime)`
    #[test]
    fn test_accept_ymd_with_hyphen() {
        // Check that a valid time string pass the validator
        let time_str = "2024-06-27";
        let result: Result<NaiveDateTime, ValidationError> = validate_string_for_native_datetime(time_str);
        assert!(result.is_ok());

        // Check that the result value is the same as the expected
        let expected_datetime = NaiveDateTime::parse_from_str(
            format!("{} 00:00:00", time_str).as_str(),
            "%Y-%m-%d %H:%M:%S"
        ).unwrap();
        assert_eq!(result.unwrap(), expected_datetime);
    }

    /// Test that a valid time string is valid
    /// we expect the format to be "%Y-%m-%d %H:%M:%S" and "%Y-%m-%dT%H:%M:%S"
    /// expected: `Ok(NaiveDateTime)`
    #[test]
    fn test_accept_iso_format() {
        // Check that a valid time string pass the validator
        let time_str = "2024-06-27T12:34:56";
        let result: Result<NaiveDateTime, ValidationError> = validate_string_for_native_datetime(time_str);
        assert!(result.is_ok());

        // Check that the result value is the same as the expected
        let expected_datetime = NaiveDateTime::parse_from_str(time_str, "%Y-%m-%dT%H:%M:%S").unwrap();
        assert_eq!(result.unwrap(), expected_datetime);
    }

    /// Test that an empty string is invalid
    /// expected: `Err(ValidationError::InvalidTimeFormat)`
    #[test]
    fn test_validate_string_for_native_datetime_empty_string() {
        let time_str = "";
        let result: Result<NaiveDateTime, ValidationError> = validate_string_for_native_datetime(time_str);
        assert!(result.is_err());
    }
}
