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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;

    /// Test that a valid time string is valid
    /// expected: `Ok(NaiveDateTime)`
    #[test]
    fn test_validate_string_for_native_datetime_valid() {
        // Check that a valid time string pass the validator
        let time_str = "2024-06-27 12:34:56";
        let result: Result<NaiveDateTime, ValidationError> = validate_string_for_native_datetime(time_str);
        assert!(result.is_ok());

        // Check that the result value is the same as the expected
        let expected_datetime = NaiveDateTime::parse_from_str(time_str, "%Y-%m-%d %H:%M:%S").unwrap();
        assert_eq!(result.unwrap(), expected_datetime);
    }

    /// Test that an invalid format is invalid
    /// we expect the format to be "%Y-%m-%d %H:%M:%S"
    /// expected: `Err(ValidationError::InvalidTimeFormat)`
    #[test]
    fn test_validate_string_for_native_datetime_invalid_format() {
        let time_str = "2024-06-27T12:34:56";
        let result: Result<NaiveDateTime, ValidationError> = validate_string_for_native_datetime(time_str);
        assert!(result.is_err());
    }

    /// Test that an empty string is invalid
    /// expected: `Err(ValidationError::InvalidTimeFormat)`
    #[test]
    fn test_validate_string_for_native_datetime_empty_string() {
        let time_str = "";
        let result: Result<NaiveDateTime, ValidationError> = validate_string_for_native_datetime(time_str);
        assert!(result.is_err());
    }

    /// Test that a partial string is invalid
    /// expected: `Err(ValidationError::InvalidTimeFormat)`
    #[test]
    fn test_validate_string_for_native_datetime_partial_string() {
        let time_str = "2024-06-27";
        let result: Result<NaiveDateTime, ValidationError> = validate_string_for_native_datetime(time_str);
        assert!(result.is_err());
    }
}