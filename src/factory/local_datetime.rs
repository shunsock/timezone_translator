use chrono::{DateTime, Local};


pub(super) fn local_datetime_string() -> String {
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
        let local_datetime_str = local_datetime_string();
        let re: Regex = Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}$").unwrap();
        assert!(re.is_match(&local_datetime_str));
    }
}