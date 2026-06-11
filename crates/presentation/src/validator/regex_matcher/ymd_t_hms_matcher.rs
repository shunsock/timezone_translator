pub(crate) fn ymd_t_hms_matcher(text: &str) -> bool {
    regex::Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}$")
        .unwrap()
        .is_match(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that a valid time string is valid
    /// expected: `true`
    #[test]
    fn test_ymd_t_hms_matcher_valid() {
        // Check that a valid time string pass the validator
        let time_str = "2024-06-27T12:34:56";
        assert!(ymd_t_hms_matcher(time_str));
    }

    /// Test that an invalid format is invalid
    /// we expect the format to be "%Y-%m-%d %H:%M:%S"
    /// expected: `false`
    #[test]
    fn test_ymd_t_hms_matcher_invalid_format() {
        let time_str = "2024-06-27 12:34:56";
        assert!(!ymd_t_hms_matcher(time_str));
    }

    /// Test that an empty string is invalid
    /// expected: `false`
    #[test]
    fn test_ymd_t_hms_matcher_empty_string() {
        let time_str = "";
        assert!(!ymd_t_hms_matcher(time_str));
    }

    /// Test that a partial string is invalid
    /// expected: `false`
    #[test]
    fn test_ymd_t_hms_matcher_partial_string() {
        let time_str = "2024-06-27";
        assert!(!ymd_t_hms_matcher(time_str));
    }
}
