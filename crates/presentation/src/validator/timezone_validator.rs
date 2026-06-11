use super::validation_error::ValidationError;
use chrono_tz::Tz;

pub(super) fn validate_string_for_timezone(tz: &str) -> Result<Tz, ValidationError> {
    tz.parse::<Tz>()
        .map_err(|_| ValidationError::Timezone(tz.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono_tz::Tz;

    /// Test that a valid timezone string is valid
    /// expected: Ok(Tz)
    #[test]
    fn test_validate_string_for_timezone_valid() {
        // confirm that a valid timezone string passes the validator
        let tz_str = "America/New_York";
        let result: Result<Tz, ValidationError> = validate_string_for_timezone(tz_str);
        assert!(result.is_ok());

        // confirm that the result value is the same as the expected
        let expected_tz: Tz = tz_str.parse().unwrap();
        assert_eq!(result.unwrap(), expected_tz);
    }

    /// Test that an invalid timezone is invalid
    /// expected: `Err(ValidationError::InvalidTimezone)`
    #[test]
    fn test_validate_string_for_timezone_invalid() {
        let tz_str = "Invalid/Timezone";
        let result: Result<Tz, ValidationError> = validate_string_for_timezone(tz_str);
        assert!(result.is_err());
    }

    /// Test that an empty string is invalid
    /// expected: `Err(ValidationError::InvalidTimezone)`
    #[test]
    fn test_validate_string_for_timezone_empty_string() {
        let tz_str = "";
        let result: Result<Tz, ValidationError> = validate_string_for_timezone(tz_str);
        assert!(result.is_err());
    }

    /// Test that a partial string is invalid
    /// expected: `Err(ValidationError::InvalidTimezone)`
    #[test]
    fn test_validate_string_for_timezone_partial_string() {
        let tz_str = "America";
        let result: Result<Tz, ValidationError> = validate_string_for_timezone(tz_str);
        assert!(result.is_err());
    }
}
