use chrono::{DateTime, Local};


/// Returns the current local date and time as a string in the format `YYYY-MM-DDTHH:MM:SS`.
/// Notice that the format does not include the timezone.
/// example: `2024-01-01T12:34:56`
/// ```rust
/// use timezone_translator::factory::local_datetime::local_datetime_string;
/// let local_datetime_str = local_datetime_string();
/// ```
pub(super) fn local_datetime_string_with_ymdthms_format() -> String {
    let local_datetime_now: DateTime<Local> = Local::now();
    local_datetime_now.format(
        "%Y-%m-%dT%H:%M:%S"
    ).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn test_check_output_match_ymdthms() {
        let local_datetime_str = local_datetime_string_with_ymdthms_format();
        let re: Regex = Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}$").unwrap();
        assert!(re.is_match(&local_datetime_str));
    }
}